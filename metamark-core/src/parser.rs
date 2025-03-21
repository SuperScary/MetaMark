use crate::ast::{Annotation, Block, Document, Inline, ListItem};
use crate::error::{MetaMarkError, MetaMarkResult};
use crate::lexer::{Lexer, Token};
use crate::metadata::parse_metadata;
use std::collections::HashMap;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
    line: usize,
    column: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let (token, line, column) = match lexer.next_token() {
            Some(Ok((t, l, c))) => (Some(t), l, c),
            _ => (None, 1, 1),
        };

        Self {
            lexer,
            current_token: token,
            line,
            column,
        }
    }

    pub fn parse(&mut self) -> MetaMarkResult<Document> {
        let mut metadata = None;
        let mut blocks = Vec::new();

        // Check for metadata section
        if self.current_token == Some(Token::MetaDelimiter) {
            let mut meta_content = String::new();
            self.advance()?; // Skip first delimiter

            while let Some(token) = &self.current_token {
                if token == &Token::MetaDelimiter {
                    self.advance()?;
                    metadata = Some(parse_metadata(&meta_content)?);
                    break;
                }
                match token {
                    Token::Text => {
                        meta_content.push_str(self.lexer.tokens.slice());
                    }
                    Token::Newline => {
                        meta_content.push('\n');
                    }
                    Token::Whitespace => {
                        meta_content.push_str(self.lexer.tokens.slice());
                    }
                    _ => {
                        return Err(MetaMarkError::ParserError {
                            line: self.line,
                            column: self.column,
                            message: format!("Unexpected token in metadata: {:?}", token),
                        });
                    }
                }
                self.advance()?;
            }
        }

        // Parse blocks
        while let Some(token) = &self.current_token {
            match token {
                Token::Heading => blocks.push(self.parse_heading()?),
                Token::UnorderedListMarker | Token::OrderedListMarker => {
                    blocks.push(self.parse_list()?);
                }
                Token::ComponentStart => blocks.push(self.parse_component()?),
                Token::CodeBlockStart => blocks.push(self.parse_code_block()?),
                Token::Comment => blocks.push(self.parse_comment()?),
                Token::Text => blocks.push(self.parse_paragraph()?),
                Token::Newline | Token::Whitespace => {
                    self.advance()?;
                }
                _ => {
                    return Err(MetaMarkError::ParserError {
                        line: self.line,
                        column: self.column,
                        message: format!("Unexpected token: {:?}", token),
                    })
                }
            }
        }

        Ok(Document { metadata, blocks })
    }

    fn advance(&mut self) -> MetaMarkResult<()> {
        match self.lexer.next_token() {
            Some(Ok((token, line, column))) => {
                self.current_token = Some(token);
                self.line = line;
                self.column = column;
                Ok(())
            }
            Some(Err(msg)) => Err(MetaMarkError::ParserError {
                line: self.line,
                column: self.column,
                message: msg,
            }),
            None => {
                self.current_token = None;
                Ok(())
            }
        }
    }

    fn parse_heading(&mut self) -> MetaMarkResult<Block> {
        let level = self.lexer.tokens.slice().chars().filter(|c| *c == '#').count() as u8;
        self.advance()?;

        let mut content = String::new();
        let mut annotations = Vec::new();

        while let Some(token) = &self.current_token {
            match token {
                Token::Text => {
                    content.push_str(self.lexer.tokens.slice());
                    self.advance()?;
                }
                Token::Annotation => {
                    annotations.push(self.parse_annotation()?);
                }
                Token::Newline => {
                    self.advance()?;
                    break;
                }
                _ => self.advance()?,
            }
        }

        Ok(Block::Heading {
            level,
            content,
            annotations,
        })
    }

    fn parse_list(&mut self) -> MetaMarkResult<Block> {
        let ordered = matches!(self.current_token, Some(Token::OrderedListMarker));
        let mut items = Vec::new();

        while let Some(token) = &self.current_token {
            match token {
                Token::UnorderedListMarker | Token::OrderedListMarker => {
                    let spaces = self.lexer.tokens.slice().chars().take_while(|c| *c == ' ').count();
                    let level = spaces / 2;
                    self.advance()?;

                    let mut content = Vec::new();
                    while let Some(inner_token) = &self.current_token {
                        match inner_token {
                            Token::Text => {
                                content.push(Block::Paragraph {
                                    content: vec![Inline::Text(self.lexer.tokens.slice().to_string())],
                                    annotations: Vec::new(),
                                });
                                self.advance()?;
                            }
                            Token::Newline => {
                                self.advance()?;
                                break;
                            }
                            _ => self.advance()?,
                        }
                    }

                    items.push(ListItem {
                        content,
                        level,
                    });
                }
                Token::Newline => {
                    self.advance()?;
                }
                _ => break,
            }
        }

        Ok(Block::List { items, ordered })
    }

    fn parse_component(&mut self) -> MetaMarkResult<Block> {
        let component_text = self.lexer.tokens.slice();
        let name_start = component_text.find(':').unwrap_or(0) + 1;
        let component_str = component_text[name_start..component_text.len() - 2].trim();
        
        self.advance()?;

        let mut content = Vec::new();
        let mut attributes = HashMap::new();
        let name;

        // Parse attributes from component string
        if let Some(attr_start) = component_str.find(' ') {
            let (name_part, attrs_str) = component_str.split_at(attr_start);
            name = name_part.to_string();
            
            for attr in attrs_str.split_whitespace() {
                if let Some((key, value)) = attr.split_once('=') {
                    attributes.insert(
                        key.trim().to_string(),
                        value.trim_matches('"').to_string(),
                    );
                }
            }
        } else {
            name = component_str.to_string();
        }

        while let Some(token) = &self.current_token {
            match token {
                Token::ComponentEnd => {
                    self.advance()?;
                    break;
                }
                Token::ComponentStart => content.push(self.parse_component()?),
                Token::Heading => content.push(self.parse_heading()?),
                Token::Text => content.push(self.parse_paragraph()?),
                Token::UnorderedListMarker | Token::OrderedListMarker => {
                    content.push(self.parse_list()?);
                }
                Token::Newline | Token::Whitespace => {
                    self.advance()?;
                }
                _ => self.advance()?,
            }
        }

        Ok(Block::Component {
            name,
            attributes,
            content,
        })
    }

    fn parse_code_block(&mut self) -> MetaMarkResult<Block> {
        let start_text = self.lexer.tokens.slice();
        let language = if start_text.len() > 4 {
            Some(start_text[3..start_text.len() - 1].to_string())
        } else {
            None
        };
        
        self.advance()?;

        let mut content = String::new();
        while let Some(token) = &self.current_token {
            match token {
                Token::CodeBlockEnd => {
                    self.advance()?;
                    break;
                }
                _ => {
                    content.push_str(self.lexer.tokens.slice());
                    self.advance()?;
                }
            }
        }

        Ok(Block::CodeBlock {
            language,
            content,
        })
    }

    fn parse_comment(&mut self) -> MetaMarkResult<Block> {
        let comment = self.lexer.tokens.slice()[3..].trim().to_string();
        self.advance()?;
        Ok(Block::Comment(comment))
    }

    fn parse_paragraph(&mut self) -> MetaMarkResult<Block> {
        let mut content = Vec::new();
        let mut annotations = Vec::new();

        while let Some(token) = &self.current_token {
            match token {
                Token::Text => {
                    content.push(Inline::Text(self.lexer.tokens.slice().to_string()));
                    self.advance()?;
                }
                Token::Bold => {
                    let text = self.lexer.tokens.slice();
                    content.push(Inline::Bold(Box::new(Inline::Text(
                        text[2..text.len() - 2].to_string(),
                    ))));
                    self.advance()?;
                }
                Token::Italic => {
                    let text = self.lexer.tokens.slice();
                    content.push(Inline::Italic(Box::new(Inline::Text(
                        text[1..text.len() - 1].to_string(),
                    ))));
                    self.advance()?;
                }
                Token::InlineCode => {
                    let text = self.lexer.tokens.slice();
                    content.push(Inline::Code(text[1..text.len() - 1].to_string()));
                    self.advance()?;
                }
                Token::Link => {
                    let text = self.lexer.tokens.slice();
                    let (text_part, url_part) = text.split_once("](").unwrap();
                    content.push(Inline::Link {
                        text: text_part[1..].to_string(),
                        url: url_part[..url_part.len() - 1].to_string(),
                    });
                    self.advance()?;
                }
                Token::Annotation => {
                    annotations.push(self.parse_annotation()?);
                }
                Token::Newline => {
                    self.advance()?;
                    break;
                }
                _ => self.advance()?,
            }
        }

        Ok(Block::Paragraph {
            content,
            annotations,
        })
    }

    fn parse_annotation(&mut self) -> MetaMarkResult<Annotation> {
        let text = self.lexer.tokens.slice();
        let content = text
            .trim_start_matches("@[")
            .trim_end_matches("]")
            .to_string();
        let parts: Vec<&str> = content.splitn(2, ": ").collect();
        
        self.advance()?;

        if parts.len() != 2 {
            return Err(MetaMarkError::ParserError {
                line: self.line,
                column: self.column,
                message: "Invalid annotation format".to_string(),
            });
        }

        Ok(Annotation {
            kind: parts[0].to_string(),
            content: parts[1].to_string(),
        })
    }
} 