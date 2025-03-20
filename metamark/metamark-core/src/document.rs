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
            let key = self.security.generate_key().map_err(|e| {
                Error::security(format!("Failed to generate encryption key: {:?}", e))
            })?;
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
            let encrypted = base64::decode(&content)
                .map_err(|e| Error::security(format!("Failed to decode base64: {}", e)))?;
            let decrypted = self.security.decrypt(key, &encrypted)?;
            String::from_utf8(decrypted)
                .map_err(|e| Error::security(format!("Invalid UTF-8: {}", e)))?
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
        output.push_str("---\n\n");

        // Write content
        for block in &doc.content {
            match block {
                Block::Heading { level, content, .. } => {
                    output.push_str(&format!("{} {}\n\n", "#".repeat(*level as usize), content));
                }
                Block::Paragraph { content } => {
                    // Simplified paragraph rendering
                    output.push_str(&format!("{:?}\n\n", content));
                }
                Block::CodeBlock { language, content } => {
                    output.push_str(&format!("```{}\n{}\n```\n\n", language, content));
                }
                // Add other block types as needed
                _ => output.push_str(&format!("{:?}\n\n", block)),
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
                if let Ok(doc) = self.load_document(&path, None) {
                    documents.push(DocumentInfo {
                        path,
                        metadata: doc.metadata,
                        encrypted: false, // This is a simplification
                    });
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
} 