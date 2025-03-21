//! Lexical analysis for MetaMark documents.
//! 
//! This module provides tokenization of MetaMark syntax using the `logos` lexer generator.
//! It handles all MetaMark-specific tokens including metadata delimiters, components,
//! annotations, formatting, and more.

use logos::Logos;

/// Represents all possible token types in a MetaMark document.
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    /// Heading marker (e.g., "# ", "## ", etc.)
    #[regex(r"#+ ", priority = 2)]
    Heading,

    /// Metadata section delimiter "---" followed by newline
    #[regex(r"---[\r\n]+", priority = 2)]
    MetaDelimiter,

    /// Start of a component block (e.g., "[[component:type=\"card\"]]")
    #[regex(r"\[\[component:[^\]]+\]\]", priority = 2)]
    ComponentStart,

    /// End of a component block "[[/component]]"
    #[regex(r"\[\[/component\]\]", priority = 2)]
    ComponentEnd,

    /// Annotation marker (e.g., "@[note: text]")
    #[regex(r"@\[[^\]]+\]", priority = 2)]
    Annotation,

    /// Comment line (e.g., "%% This is a comment")
    #[regex(r"%% [^\r\n]*", priority = 2)]
    Comment,

    /// Start of a code block with optional language (e.g., "```rust\n")
    #[regex(r"```[a-zA-Z0-9]*[\r\n]+", priority = 3)]
    CodeBlockStart,

    /// End of a code block "```\n"
    #[regex(r"```[\r\n]+", priority = 2)]
    CodeBlockEnd,

    /// Bold text marker (e.g., "**bold**")
    #[regex(r"\*\*[^*]+\*\*", priority = 2)]
    Bold,

    /// Italic text marker (e.g., "*italic*")
    #[regex(r"\*[^*]+\*", priority = 2)]
    Italic,

    /// Inline code marker (e.g., "`code`")
    #[regex(r"`[^`]+`", priority = 2)]
    InlineCode,

    /// Link with text and URL (e.g., "[text](url)")
    #[regex(r"\[[^\]]+\]\([^\)]+\)", priority = 2)]
    Link,

    /// Inline math expression (e.g., "$x^2$")
    #[regex(r"\$[^$]+\$", priority = 2)]
    InlineMath,

    /// Block math expression (e.g., "$$\sum_{i=1}^n x_i$$")
    #[regex(r"\$\$[^$]+\$\$", priority = 2)]
    BlockMath,

    /// Unordered list item marker "- "
    #[regex(r"- ", priority = 2)]
    UnorderedListMarker,

    /// Ordered list item marker (e.g., "1. ")
    #[regex(r"\d+\. ", priority = 2)]
    OrderedListMarker,

    /// Regular text content
    #[regex(r"[^\r\n\s][^\r\n]*", priority = 1)]
    Text,

    /// Whitespace (spaces and tabs)
    #[regex(r"[ \t]+", priority = 2)]
    Whitespace,

    /// Newline sequence
    #[regex(r"[\r\n]+", priority = 2)]
    Newline,

    /// Error token for invalid input
    Error
}

/// A lexical analyzer for MetaMark documents that tracks line and column positions.
pub struct Lexer<'a> {
    /// The underlying logos lexer
    pub(crate) tokens: logos::Lexer<'a, Token>,
    /// Current line number (1-based)
    pub line: usize,
    /// Current column number (1-based)
    pub column: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer for the given input string.
    ///
    /// # Arguments
    ///
    /// * `input` - The MetaMark document text to tokenize
    ///
    /// # Returns
    ///
    /// A new `Lexer` instance initialized to the start of the input
    pub fn new(input: &'a str) -> Self {
        Self {
            tokens: Token::lexer(input),
            line: 1,
            column: 1,
        }
    }

    /// Advances to and returns the next token in the input.
    ///
    /// This method tracks line and column positions as it processes the input.
    /// It returns `None` when the input is exhausted.
    ///
    /// # Returns
    ///
    /// * `Some(Ok((token, line, column)))` - The next valid token with its position
    /// * `Some(Err(message))` - An error occurred at the current position
    /// * `None` - No more tokens available
    pub fn next_token(&mut self) -> Option<Result<(Token, usize, usize), String>> {
        let token = self.tokens.next()?;
        let slice = self.tokens.slice();
        
        let current_line = self.line;
        let current_column = self.column;

        // Update position
        for c in slice.chars() {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }

        Some(match token {
            Ok(Token::Error) => Err(format!("Invalid token at line {}, column {}", 
                current_line, current_column)),
            Ok(token) => Ok((token, current_line, current_column)),
            Err(_) => Err(format!("Failed to lex token at line {}, column {}", 
                current_line, current_column))
        })
    }
} 