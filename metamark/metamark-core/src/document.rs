use crate::{
    ast::{Block, Document, Metadata},
    lexer::Lexer,
    parser::Parser,
    security::Security,
    Error, Result,
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentInfo {
    pub path: PathBuf,
    pub metadata: Metadata,
    pub encrypted: bool,
}

pub struct DocumentManager {
    security: Security,
    working_dir: PathBuf,
}

impl DocumentManager {
    pub fn new<P: AsRef<Path>>(working_dir: P) -> Self {
        Self {
            security: Security::new(),
            working_dir: working_dir.as_ref().to_path_buf(),
        }
    }

    pub fn create_document(&self, title: &str) -> Result<Document> {
        Ok(Document::new(title.to_string()))
    }

    pub fn save_document(&self, doc: &Document, path: &Path, encrypt: bool) -> Result<()> {
        let content = serde_json::to_string_pretty(doc)
            .map_err(|e| Error::serialization(format!("Failed to serialize document: {}", e)))?;

        let final_content = if encrypt {
            let key = self.security.generate_key()
                .map_err(|e| Error::security(format!("Failed to generate encryption key: {:?}", e)))?;
            let encrypted = self.security.encrypt(&key, content.as_bytes())?;
            base64::encode(encrypted)
        } else {
            content
        };

        fs::write(path, final_content)
            .map_err(|e| Error::Io(e))?;
        Ok(())
    }

    pub fn load_document(&self, path: &Path, key: Option<&[u8]>) -> Result<Document> {
        let content = fs::read(path)
            .map_err(|e| Error::Io(e))?;

        let decoded = if let Some(key) = key {
            let encrypted = base64::decode(&String::from_utf8(content.clone())
                .map_err(|e| Error::security(format!("Invalid UTF-8 in encrypted content: {}", e)))?)
                .map_err(|e| Error::security(format!("Failed to decode base64: {}", e)))?;
            let decrypted = self.security.decrypt(key, &encrypted)?;
            String::from_utf8(decrypted)
                .map_err(|e| Error::security(format!("Invalid UTF-8 in decrypted content: {}", e)))?
        } else {
            String::from_utf8(content)
                .map_err(|e| Error::security(format!("Invalid UTF-8: {}", e)))?
        };

        serde_json::from_str(&decoded)
            .map_err(|e| Error::serialization(format!("Failed to deserialize document: {}", e)))
    }

    pub fn parse_mmk(&self, content: &str) -> Result<Document> {
        let mut lexer = Lexer::new(content);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    pub fn export_mmk(&self, doc: &Document) -> Result<String> {
        let mut output = String::new();

        // Write metadata
        output.push_str("---\n");
        output.push_str(&format!("title: {}\n", doc.metadata.title));
        output.push_str(&format!("version: {}\n", doc.metadata.version));
        if !doc.metadata.authors.is_empty() {
            output.push_str(&format!("authors: {}\n", doc.metadata.authors.join(", ")));
        }
        if !doc.metadata.tags.is_empty() {
            output.push_str(&format!("tags: {}\n", doc.metadata.tags.join(", ")));
        }
        for (key, value) in &doc.metadata.custom {
            output.push_str(&format!("{}: {}\n", key, value));
        }
        output.push_str("---\n\n");

        // Write content
        for block in &doc.content {
            match block {
                Block::Heading { level, content, .. } => {
                    output.push_str(&format!("{} {}\n\n", "#".repeat(*level as usize), content));
                }
                Block::Paragraph { content } => {
                    for inline in content {
                        match inline {
                            crate::ast::Inline::Text(text) => output.push_str(text),
                            crate::ast::Inline::Bold(text) => output.push_str(&format!("**{}**", text)),
                            crate::ast::Inline::Italic(text) => output.push_str(&format!("*{}*", text)),
                            crate::ast::Inline::Code(text) => output.push_str(&format!("`{}`", text)),
                            crate::ast::Inline::Link { text, url } => {
                                output.push_str(&format!("[{}]({})", text, url))
                            }
                            crate::ast::Inline::Math { content, display } => {
                                if *display {
                                    output.push_str(&format!("${}$", content))
                                } else {
                                    output.push_str(&format!("$${}$$", content))
                                }
                            }
                        }
                    }
                    output.push_str("\n\n");
                }
                Block::CodeBlock { language, content } => {
                    output.push_str(&format!("```{}\n{}\n```\n\n", language, content));
                }
                Block::List { items, ordered } => {
                    for (i, item) in items.iter().enumerate() {
                        let prefix = if *ordered {
                            format!("{}. ", i + 1)
                        } else {
                            "- ".to_string()
                        };
                        output.push_str(&prefix);
                        if let Some(checked) = item.checked {
                            output.push_str(if checked { "[x] " } else { "[ ] " });
                        }
                        // Simplified list item content rendering
                        if let Some(Block::Paragraph { content }) = item.content.first() {
                            for inline in content {
                                match inline {
                                    crate::ast::Inline::Text(text) => output.push_str(text),
                                    _ => output.push_str(&format!("{:?}", inline)),
                                }
                            }
                        }
                        output.push_str("\n");
                    }
                    output.push_str("\n");
                }
                Block::Table { headers, rows } => {
                    // Write table headers
                    output.push_str("|");
                    for header in headers {
                        output.push_str(&format!(" {} |", header));
                    }
                    output.push_str("\n|");
                    
                    // Write header separator
                    for _ in headers {
                        output.push_str(" --- |");
                    }
                    output.push_str("\n");

                    // Write table rows
                    for row in rows {
                        output.push_str("|");
                        for cell in row {
                            output.push_str(&format!(" {} |", cell));
                        }
                        output.push_str("\n");
                    }
                    output.push_str("\n");
                }
                Block::BlockQuote { content } => {
                    for block in content {
                        let block_content = self.export_mmk(&Document {
                            metadata: doc.metadata.clone(),
                            content: vec![block.clone()],
                            annotations: Vec::new(),
                        })?;
                        
                        // Add quote prefix to each line
                        for line in block_content.lines() {
                            if !line.is_empty() {
                                output.push_str(&format!("> {}\n", line));
                            } else {
                                output.push_str(">\n");
                            }
                        }
                    }
                    output.push_str("\n");
                }
            }
        }

        Ok(output)
    }

    pub fn list_documents(&self) -> Result<Vec<DocumentInfo>> {
        let mut documents = Vec::new();
        for entry in fs::read_dir(&self.working_dir)
            .map_err(|e| Error::Io(e))?
        {
            let entry = entry.map_err(|e| Error::Io(e))?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "mmk") {
                match self.load_document(&path, None) {
                    Ok(doc) => {
                        documents.push(DocumentInfo {
                            path,
                            metadata: doc.metadata,
                            encrypted: false, // This is a simplification
                        });
                    }
                    Err(e) => {
                        eprintln!("Failed to load document {}: {}", path.display(), e);
                        continue;
                    }
                }
            }
        }
        Ok(documents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_document_lifecycle() {
        let temp_dir = tempdir().unwrap();
        let manager = DocumentManager::new(temp_dir.path());

        // Create and save document
        let mut doc = manager.create_document("Test Document").unwrap();
        doc.add_block(Block::Heading {
            level: 1,
            content: "Hello, MetaMark!".to_string(),
            id: "hello-metamark".to_string(),
        });

        let path = temp_dir.path().join("test.mmk");
        manager.save_document(&doc, &path, false).unwrap();

        // Load and verify
        let loaded_doc = manager.load_document(&path, None).unwrap();
        assert_eq!(loaded_doc.metadata.title, "Test Document");
    }

    #[test]
    fn test_encrypted_document() {
        let temp_dir = tempdir().unwrap();
        let manager = DocumentManager::new(temp_dir.path());
        let doc = manager.create_document("Secret Document").unwrap();

        let path = temp_dir.path().join("secret.mmk");
        manager.save_document(&doc, &path, true).unwrap();

        // Try to load without key (should fail)
        assert!(manager.load_document(&path, None).is_err());
    }
} 