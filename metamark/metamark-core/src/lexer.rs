use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{char as nom_char, digit1, line_ending, space0, space1},
    combinator::{map, opt, recognize},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult, Parser, AsChar, InputTakeAtPosition, InputLength, InputTake, error::Error,
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
        match Self::parse_token(input) {
            Ok((remaining, token)) => {
                self.position = self.input.len() - remaining.len();
                Ok(token)
            }
            Err(_) => Err(crate::Error::lexer("Failed to parse token")),
        }
    }

    fn parse_token(input: &str) -> IResult<&str, Token> {
        alt((
            Self::parse_metadata_token,
            Self::parse_heading,
            Self::parse_code_block,
            Self::parse_list_item,
            Self::parse_block_quote,
            Self::parse_inline_elements,
            Self::parse_whitespace,
            Self::parse_newline,
        ))(input)
    }

    fn parse_metadata_token(input: &str) -> IResult<&str, Token> {
        alt((
            map(tag::<&str, _, Error<&str>>("---\n"), |_| Token::MetadataStart),
            map(tag::<&str, _, Error<&str>>("---"), |_| Token::MetadataEnd),
            map(
                tuple((
                    take_while1(|c: char| c.is_alphanumeric() || c == '_'),
                    tag::<&str, _, Error<&str>>(":"),
                    space0::<&str, Error<&str>>,
                    take_until("\n"),
                )),
                |(key, _, _, value)| Token::MetadataKey(key.to_string()),
            ),
        ))(input)
    }

    fn parse_heading(input: &str) -> IResult<&str, Token> {
        map(
            tuple((
                many1(nom_char::<&str, Error<&str>>('#')),
                space1::<&str, Error<&str>>,
                take_until("\n"),
            )),
            |(hashes, _, content)| Token::Heading {
                level: hashes.len() as u8,
                content: content.trim().to_string(),
            },
        )(input)
    }

    fn parse_code_block(input: &str) -> IResult<&str, Token> {
        alt((
            map(
                tuple((
                    tag::<&str, _, Error<&str>>("```"),
                    take_while1(|c: char| c.is_alphanumeric() || c == '_'),
                    line_ending::<&str, Error<&str>>,
                )),
                |(_, lang, _)| Token::CodeBlockStart {
                    language: lang.to_string(),
                },
            ),
            map(tag::<&str, _, Error<&str>>("```"), |_| Token::CodeBlockEnd),
        ))(input)
    }

    fn parse_list_item(input: &str) -> IResult<&str, Token> {
        alt((
            // Ordered list item
            map(
                tuple((
                    digit1::<&str, Error<&str>>,
                    tag::<&str, _, Error<&str>>(". "),
                )),
                |(num, _)| Token::ListItemStart {
                    ordered: true,
                    number: Some(num.parse().unwrap_or(1)),
                },
            ),
            // Unordered list item
            map(
                tag::<&str, _, Error<&str>>("- "),
                |_| Token::ListItemStart {
                    ordered: false,
                    number: None,
                },
            ),
        ))(input)
    }

    fn parse_block_quote(input: &str) -> IResult<&str, Token> {
        map(
            tag::<&str, _, Error<&str>>("> "),
            |_| Token::BlockQuoteStart,
        )(input)
    }

    fn parse_inline_elements(input: &str) -> IResult<&str, Token> {
        alt((
            // Bold
            map(
                delimited(tag::<&str, _, Error<&str>>("**"), take_until("**"), tag::<&str, _, Error<&str>>("**")),
                |content: &str| Token::Bold(content.to_string()),
            ),
            // Italic
            map(
                delimited(tag::<&str, _, Error<&str>>("*"), take_until("*"), tag::<&str, _, Error<&str>>("*")),
                |content: &str| Token::Italic(content.to_string()),
            ),
            // Inline code
            map(
                delimited(tag::<&str, _, Error<&str>>("`"), take_until("`"), tag::<&str, _, Error<&str>>("`")),
                |content: &str| Token::InlineCode(content.to_string()),
            ),
            // Link
            map(
                tuple((
                    delimited(tag::<&str, _, Error<&str>>("["), take_until("]"), tag::<&str, _, Error<&str>>("]")),
                    delimited(tag::<&str, _, Error<&str>>("("), take_until(")"), tag::<&str, _, Error<&str>>(")")),
                )),
                |(text, url)| Token::Link {
                    text: text.to_string(),
                    url: url.to_string(),
                },
            ),
            // Math
            map(
                alt((
                    delimited(tag::<&str, _, Error<&str>>("$$"), take_until("$$"), tag::<&str, _, Error<&str>>("$$")),
                    delimited(tag::<&str, _, Error<&str>>("$"), take_until("$"), tag::<&str, _, Error<&str>>("$")),
                )),
                |content| Token::Math {
                    content: content.to_string(),
                    display: true,
                },
            ),
            // Plain text
            map(
                take_while1(|c: char| !matches!(c, '*' | '`' | '[' | '$' | '\n' | '#' | '-' | '>')),
                |text: &str| Token::Text(text.to_string()),
            ),
        ))(input)
    }

    fn parse_whitespace(input: &str) -> IResult<&str, Token> {
        map(
            space1::<&str, Error<&str>>,
            |s: &str| Token::Whitespace(s.to_string()),
        )(input)
    }

    fn parse_newline(input: &str) -> IResult<&str, Token> {
        map(
            line_ending::<&str, Error<&str>>,
            |_| Token::Newline,
        )(input)
    }
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
            Token::MetadataKey("title".to_string())
        );
        assert_eq!(tokens[2], Token::MetadataEnd);
    }

    #[test]
    fn test_heading() {
        let input = "# Test Heading\n";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens[0],
            Token::Heading {
                level: 1,
                content: "Test Heading".to_string(),
            }
        );
    }

    #[test]
    fn test_inline_elements() {
        let input = "**bold** *italic* `code` [link](url) $math$";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Bold("bold".to_string()));
        assert_eq!(tokens[2], Token::Italic("italic".to_string()));
        assert_eq!(tokens[4], Token::InlineCode("code".to_string()));
        assert_eq!(
            tokens[6],
            Token::Link {
                text: "link".to_string(),
                url: "url".to_string(),
            }
        );
        assert_eq!(
            tokens[8],
            Token::Math {
                content: "math".to_string(),
                display: true,
            }
        );
    }
} 