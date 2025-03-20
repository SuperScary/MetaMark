use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{char, digit1, line_ending, space0, space1},
    combinator::{map, opt, recognize},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Metadata
    MetadataStart,
    MetadataEnd,
    MetadataKey(String),
    MetadataValue(String),

    // Block Elements
    Heading { level: u8, content: String },
    ParagraphStart,
    ParagraphEnd,
    CodeBlockStart { language: String },
    CodeBlockEnd,
    ListItemStart { ordered: bool, number: Option<u32> },
    ListItemEnd,
    BlockQuoteStart,
    BlockQuoteEnd,

    // Inline Elements
    Text(String),
    Bold(String),
    Italic(String),
    InlineCode(String),
    Link { text: String, url: String },
    Math { content: String, display: bool },

    // Special
    Newline,
    Whitespace(String),
    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> crate::Result<Vec<Token>> {
        let mut tokens = Vec::new();
        while !self.is_eof() {
            let token = self.next_token()?;
            tokens.push(token);
        }
        tokens.push(Token::EOF);
        Ok(tokens)
    }

    fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }

    fn next_token(&mut self) -> crate::Result<Token> {
        let input = &self.input[self.position..];
        match self.parse_token(input) {
            Ok((remaining, token)) => {
                self.position = self.input.len() - remaining.len();
                Ok(token)
            }
            Err(_) => Err(crate::Error::lexer("Failed to parse token")),
        }
    }

    fn parse_token(&self, input: &str) -> IResult<&str, Token> {
        alt((
            self.parse_metadata_token,
            self.parse_heading,
            self.parse_code_block,
            self.parse_list_item,
            self.parse_block_quote,
            self.parse_inline_elements,
            self.parse_whitespace,
            self.parse_newline,
        ))(input)
    }

    fn parse_metadata_token(input: &str) -> IResult<&str, Token> {
        alt((
            map(tag("---\n"), |_| Token::MetadataStart),
            map(tag("---"), |_| Token::MetadataEnd),
            map(
                tuple((
                    take_while1(|c: char| c.is_alphanumeric() || c == '_'),
                    tag(":"),
                    space0,
                    take_until("\n"),
                )),
                |(key, _, _, value)| Token::MetadataValue(value.trim().to_string()),
            ),
        ))(input)
    }

    // Additional parsing methods would be implemented here...
    // This is a basic implementation that would need to be expanded
    // based on the full MetaMark syntax specification
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_metadata() {
        let input = "---\ntitle: Test Document\n---\n";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::MetadataStart);
        assert_eq!(
            tokens[1],
            Token::MetadataValue("Test Document".to_string())
        );
        assert_eq!(tokens[2], Token::MetadataEnd);
    }
} 