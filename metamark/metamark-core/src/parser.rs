use crate::{
    ast::{Annotation, Block, Document, Inline, ListItem, Metadata},
    lexer::Token,
    Error, Result,
};
use std::collections::HashMap;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Document> {
        let metadata = self.parse_metadata()?;
        let content = self.parse_blocks()?;
        let annotations = self.parse_annotations()?;

        Ok(Document {
            metadata,
            content,
            annotations,
        })
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn parse_metadata(&mut self) -> Result<Metadata> {
        match self.current_token() {
            Some(Token::MetadataStart) => {
                self.advance();
                let mut metadata = Metadata {
                    title: String::new(),
                    authors: Vec::new(),
                    created_at: chrono::Utc::now().to_rfc3339(),
                    updated_at: chrono::Utc::now().to_rfc3339(),
                    version: crate::VERSION.to_string(),
                    tags: Vec::new(),
                    custom: HashMap::new(),
                };

                while let Some(token) = self.current_token() {
                    match token {
                        Token::MetadataEnd => {
                            self.advance();
                            return Ok(metadata);
                        }
                        Token::MetadataValue(value) => {
                            // Parse metadata values based on key
                            // This is a simplified implementation
                            metadata.title = value.clone();
                            self.advance();
                        }
                        _ => return Err(Error::parser("Invalid metadata token")),
                    }
                }
                Err(Error::parser("Unexpected end of metadata"))
            }
            _ => Err(Error::parser("Expected metadata start")),
        }
    }

    fn parse_blocks(&mut self) -> Result<Vec<Block>> {
        let mut blocks = Vec::new();
        while let Some(token) = self.current_token() {
            match token {
                Token::Heading { level, content } => {
                    blocks.push(Block::Heading {
                        level: *level,
                        content: content.clone(),
                        id: self.generate_id(content),
                    });
                    self.advance();
                }
                Token::ParagraphStart => {
                    self.advance();
                    let content = self.parse_inline_content()?;
                    blocks.push(Block::Paragraph { content });
                }
                Token::CodeBlockStart { language } => {
                    self.advance();
                    let content = self.parse_code_content()?;
                    blocks.push(Block::CodeBlock {
                        language: language.clone(),
                        content,
                    });
                }
                Token::ListItemStart { ordered, number } => {
                    self.advance();
                    let items = self.parse_list_items()?;
                    blocks.push(Block::List {
                        items,
                        ordered: *ordered,
                    });
                }
                Token::EOF => break,
                _ => self.advance(),
            }
        }
        Ok(blocks)
    }

    fn parse_inline_content(&mut self) -> Result<Vec<Inline>> {
        let mut content = Vec::new();
        while let Some(token) = self.current_token() {
            match token {
                Token::Text(text) => {
                    content.push(Inline::Text(text.clone()));
                    self.advance();
                }
                Token::Bold(text) => {
                    content.push(Inline::Bold(text.clone()));
                    self.advance();
                }
                Token::Italic(text) => {
                    content.push(Inline::Italic(text.clone()));
                    self.advance();
                }
                Token::InlineCode(code) => {
                    content.push(Inline::Code(code.clone()));
                    self.advance();
                }
                Token::ParagraphEnd => {
                    self.advance();
                    break;
                }
                _ => self.advance(),
            }
        }
        Ok(content)
    }

    fn parse_code_content(&mut self) -> Result<String> {
        let mut content = String::new();
        while let Some(token) = self.current_token() {
            match token {
                Token::Text(text) => {
                    content.push_str(text);
                    self.advance();
                }
                Token::CodeBlockEnd => {
                    self.advance();
                    break;
                }
                _ => self.advance(),
            }
        }
        Ok(content)
    }

    fn parse_list_items(&mut self) -> Result<Vec<ListItem>> {
        let mut items = Vec::new();
        while let Some(token) = self.current_token() {
            match token {
                Token::Text(text) => {
                    items.push(ListItem {
                        content: vec![Block::Paragraph {
                            content: vec![Inline::Text(text.clone())],
                        }],
                        checked: None,
                    });
                    self.advance();
                }
                Token::ListItemEnd => {
                    self.advance();
                    break;
                }
                _ => self.advance(),
            }
        }
        Ok(items)
    }

    fn parse_annotations(&mut self) -> Result<Vec<Annotation>> {
        // Simplified annotation parsing
        Ok(Vec::new())
    }

    fn generate_id(&self, content: &str) -> String {
        content
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_basic_document() {
        let input = "---\ntitle: Test Document\n---\n# Heading 1\nSome text\n";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let doc = parser.parse().unwrap();

        assert_eq!(doc.metadata.title, "Test Document");
        assert_eq!(doc.content.len(), 2);
    }
} 