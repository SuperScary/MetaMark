//! MetaMark core library for parsing and representing MetaMark documents.
//!
//! MetaMark is a Markdown-inspired format that adds support for metadata, annotations,
//! collapsible blocks, diagrams, and encrypted content. This library provides the core
//! functionality for parsing .mmk files and building an Abstract Syntax Tree (AST).
//!
//! # Features
//!
//! - YAML/TOML frontmatter metadata
//! - Markdown-compatible formatting
//! - Component blocks with attributes
//! - Annotations and comments
//! - Code blocks with syntax highlighting
//! - Diagram blocks (Mermaid, PlantUML, GraphViz)
//! - Encrypted content regions
//!
//! # Example
//!
//! ```rust
//! use metamark_core::parse_metamark;
//!
//! let input = r#"---
//! title: My Document
//! author: John Doe
//! ---
//!
//! # Hello World @[note: This is important]
//!
//! This is a paragraph with **bold** and *italic* text.
//!
//! [[component: type="card"]]
//! ## Card Title
//! Card content here
//! [[/component]]
//! "#;
//!
//! let doc = parse_metamark(input).unwrap();
//! ```

pub mod ast;
pub mod error;
pub mod lexer;
pub mod metadata;
pub mod parser;

use error::MetaMarkResult;
use parser::Parser;
use std::time::Instant;

/// Parse a MetaMark document string into an AST.
///
/// This is the main entry point for parsing MetaMark documents. It processes
/// the input string and returns a complete document AST that can be used for
/// further processing, rendering, or analysis.
///
/// # Arguments
///
/// * `input` - The MetaMark document string to parse
///
/// # Returns
///
/// * `Ok(Document)` - Successfully parsed document AST
/// * `Err(MetaMarkError)` - If any parsing errors occur
///
/// # Example
///
/// ```rust
/// use metamark_core::parse_metamark;
///
/// let input = r#"# Hello World
///
/// This is a test paragraph with **bold** text.
/// "#;
///
/// let doc = parse_metamark(input).unwrap();
/// assert_eq!(doc.blocks.len(), 2); // Heading and paragraph
/// ```
pub fn parse_metamark(input: &str) -> MetaMarkResult<ast::Document> {
    let mut parser = Parser::new(input);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Block, DiagramType, Inline, ListItem, Metadata, MetaValue};
    use std::collections::HashMap;

    #[test]
    fn test_basic_document() {
        let input = r#"# Hello World

This is a test paragraph with **bold** text.
"#;

        let doc = parse_metamark(input).unwrap();
        assert!(doc.metadata.is_none());
        assert_eq!(doc.blocks.len(), 2);

        match &doc.blocks[0] {
            Block::Heading { level, content, .. } => {
                assert_eq!(*level, 1);
                assert_eq!(content.trim(), "Hello World");
            }
            _ => panic!("Expected heading"),
        }
    }

    #[test]
    fn test_metadata() {
        let input = r#"---
title: Test Document
author: John Doe
tags:
  - rust
  - documentation
numbers:
  - 1
  - 2.5
settings:
  debug: true
  cache_size: 1000
---

# Content"#;

        let doc = parse_metamark(input).unwrap();
        assert!(doc.metadata.is_some());
        
        if let Some(metadata) = doc.metadata {
            // Test string value
            match &metadata.data.get("title").unwrap() {
                MetaValue::String(s) => assert_eq!(s, "Test Document"),
                _ => panic!("Expected string value"),
            }

            // Test array of strings
            match &metadata.data.get("tags").unwrap() {
                MetaValue::Array(arr) => {
                    assert_eq!(arr.len(), 2);
                    match &arr[0] {
                        MetaValue::String(s) => assert_eq!(s, "rust"),
                        _ => panic!("Expected string value"),
                    }
                }
                _ => panic!("Expected array"),
            }

            // Test array of numbers
            match &metadata.data.get("numbers").unwrap() {
                MetaValue::Array(arr) => {
                    assert_eq!(arr.len(), 2);
                    match &arr[1] {
                        MetaValue::Number(n) => assert_eq!(*n, 2.5),
                        _ => panic!("Expected number value"),
                    }
                }
                _ => panic!("Expected array"),
            }

            // Test nested object
            match &metadata.data.get("settings").unwrap() {
                MetaValue::Object(obj) => {
                    match obj.get("debug").unwrap() {
                        MetaValue::Boolean(b) => assert!(*b),
                        _ => panic!("Expected boolean value"),
                    }
                    match obj.get("cache_size").unwrap() {
                        MetaValue::Number(n) => assert_eq!(*n, 1000.0),
                        _ => panic!("Expected number value"),
                    }
                }
                _ => panic!("Expected object"),
            }
        }
    }

    #[test]
    fn test_components() {
        let input = r#"[[component: type="card" theme="dark"]]
# Card Title
Some content here
- List item 1
- List item 2
[[/component]]"#;

        let doc = parse_metamark(input).unwrap();
        assert_eq!(doc.blocks.len(), 1);

        match &doc.blocks[0] {
            Block::Component { name, attributes, content } => {
                assert_eq!(name, "type=\"card\"");
                assert_eq!(attributes.get("theme").unwrap(), "dark");
                assert_eq!(content.len(), 3); // Heading, paragraph, and list
                
                match &content[0] {
                    Block::Heading { level, content, .. } => {
                        assert_eq!(*level, 1);
                        assert_eq!(content, "Card Title");
                    }
                    _ => panic!("Expected heading"),
                }
            }
            _ => panic!("Expected component"),
        }
    }

    #[test]
    fn test_annotations() {
        let input = r#"# Important Section @[warning: Critical information] @[note: Review required]

This paragraph needs attention @[todo: Update content]."#;

        let doc = parse_metamark(input).unwrap();
        
        match &doc.blocks[0] {
            Block::Heading { annotations, .. } => {
                assert_eq!(annotations.len(), 2);
                assert_eq!(annotations[0].kind, "warning");
                assert_eq!(annotations[0].content, "Critical information");
                assert_eq!(annotations[1].kind, "note");
            }
            _ => panic!("Expected heading"),
        }

        match &doc.blocks[1] {
            Block::Paragraph { annotations, .. } => {
                assert_eq!(annotations.len(), 1);
                assert_eq!(annotations[0].kind, "todo");
            }
            _ => panic!("Expected paragraph"),
        }
    }

    #[test]
    fn test_lists() {
        let input = r#"1. First item
2. Second item
   - Nested unordered
   - Another nested
3. Third item

- Unordered one
  1. Nested ordered
  2. Another ordered
- Unordered two"#;

        let doc = parse_metamark(input).unwrap();
        assert_eq!(doc.blocks.len(), 2);

        match &doc.blocks[0] {
            Block::List { items, ordered } => {
                assert!(*ordered);
                assert_eq!(items.len(), 3);
                
                // Check second item's nested list
                match &items[1].content[1] {
                    Block::List { items, ordered } => {
                        assert!(!*ordered);
                        assert_eq!(items.len(), 2);
                        assert_eq!(items[0].level, 1);
                    }
                    _ => panic!("Expected nested list"),
                }
            }
            _ => panic!("Expected ordered list"),
        }

        match &doc.blocks[1] {
            Block::List { items, ordered } => {
                assert!(!*ordered);
                assert_eq!(items.len(), 2);
                
                // Check first item's nested list
                match &items[0].content[1] {
                    Block::List { items, ordered } => {
                        assert!(*ordered);
                        assert_eq!(items.len(), 2);
                        assert_eq!(items[0].level, 1);
                    }
                    _ => panic!("Expected nested list"),
                }
            }
            _ => panic!("Expected unordered list"),
        }
    }

    #[test]
    fn test_code_blocks() {
        let input = r#"```rust
fn main() {
    println!("Hello, world!");
}
```

```
Plain code block
```"#;

        let doc = parse_metamark(input).unwrap();
        assert_eq!(doc.blocks.len(), 2);

        match &doc.blocks[0] {
            Block::CodeBlock { language, content } => {
                assert_eq!(language.as_ref().unwrap(), "rust");
                assert!(content.contains("println!"));
            }
            _ => panic!("Expected code block"),
        }

        match &doc.blocks[1] {
            Block::CodeBlock { language, content } => {
                assert!(language.is_none());
                assert_eq!(content.trim(), "Plain code block");
            }
            _ => panic!("Expected code block"),
        }
    }

    #[test]
    fn test_inline_formatting() {
        let input = "This has **bold** and *italic* text with `code` and [link](https://example.com).";

        let doc = parse_metamark(input).unwrap();
        
        match &doc.blocks[0] {
            Block::Paragraph { content, .. } => {
                assert_eq!(content.len(), 8); // Text + Bold + Text + Italic + Text + Code + Text + Link
                
                // Check bold
                match &content[1] {
                    Inline::Bold(inner) => {
                        match &**inner {
                            Inline::Text(text) => assert_eq!(text, "bold"),
                            _ => panic!("Expected text inside bold"),
                        }
                    }
                    _ => panic!("Expected bold"),
                }

                // Check link
                match &content[7] {
                    Inline::Link { text, url } => {
                        assert_eq!(text, "link");
                        assert_eq!(url, "https://example.com");
                    }
                    _ => panic!("Expected link"),
                }
            }
            _ => panic!("Expected paragraph"),
        }
    }

    #[test]
    fn test_diagrams() {
        let input = r#"```mermaid
graph TD;
    A-->B;
    B-->C;
```

```plantuml
@startuml
Alice -> Bob: Hello
@enduml
```"#;

        let doc = parse_metamark(input).unwrap();
        
        match &doc.blocks[0] {
            Block::Diagram { kind, content } => {
                assert!(matches!(kind, DiagramType::Mermaid));
                assert!(content.contains("graph TD"));
            }
            _ => panic!("Expected diagram"),
        }

        match &doc.blocks[1] {
            Block::Diagram { kind, content } => {
                assert!(matches!(kind, DiagramType::PlantUML));
                assert!(content.contains("@startuml"));
            }
            _ => panic!("Expected diagram"),
        }
    }

    #[test]
    fn test_comments() {
        let input = r#"%% This is a comment
# Heading
%% Another comment
Some text"#;

        let doc = parse_metamark(input).unwrap();
        assert_eq!(doc.blocks.len(), 4);

        match &doc.blocks[0] {
            Block::Comment(text) => {
                assert_eq!(text, "This is a comment");
            }
            _ => panic!("Expected comment"),
        }
    }

    #[test]
    fn test_math() {
        let input = r#"Inline math: $x^2 + y^2 = z^2$

Block math:
$$
\int_0^\infty e^{-x} dx = 1
$$"#;

        let doc = parse_metamark(input).unwrap();
        
        match &doc.blocks[0] {
            Block::Paragraph { content, .. } => {
                match &content[2] {
                    Inline::Math(text) => {
                        assert_eq!(text, "x^2 + y^2 = z^2");
                    }
                    _ => panic!("Expected inline math"),
                }
            }
            _ => panic!("Expected paragraph"),
        }

        match &doc.blocks[2] {
            Block::Math(text) => {
                assert!(text.contains(r"\int"));
            }
            _ => panic!("Expected block math"),
        }
    }

    #[test]
    fn test_error_handling() {
        // Test invalid metadata
        let input = r#"---
invalid: : : :
---"#;
        assert!(parse_metamark(input).is_err());

        // Test unclosed component
        let input = "[[component: type=\"card\"]]\nContent";
        assert!(parse_metamark(input).is_err());

        // Test unclosed code block
        let input = "```rust\nfn main() {}\n";
        assert!(parse_metamark(input).is_err());

        // Test invalid annotation format
        let input = "@[invalid]";
        assert!(parse_metamark(input).is_err());
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    fn measure_performance<F, T>(name: &str, f: F) -> T 
    where
        F: FnOnce() -> T
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();

        println!(
            "Performance - {}: {:.2}ms",
            name,
            duration.as_secs_f64() * 1000.0
        );

        result
    }

    #[test]
    fn test_small_document_performance() {
        let input = r#"---
title: Test Document
author: Test Author
---

# Section 1

This is a test paragraph."#;
        
        println!("Generated document:\n{}", input);
        
        measure_performance("Small Document Parse", || {
            let doc = parse_metamark(&input).unwrap();
            assert!(doc.blocks.len() > 0);
        });
    }

    #[test]
    fn test_medium_document_performance() {
        let input = r#"---
title: Test Document
author: Test Author
tags:
  - test
  - performance
---

# Section 1

This is a test paragraph.

## Subsection 1.1

- List item 1
- List item 2
  - Nested item 2.1
  - Nested item 2.2

```rust
fn test() {
    println!("Hello");
}
```

# Section 2

Another paragraph with **bold** and *italic* text."#;
        
        measure_performance("Medium Document Parse", || {
            let doc = parse_metamark(&input).unwrap();
            assert!(doc.blocks.len() > 0);
        });
    }

    #[test]
    fn test_large_document_performance() {
        let mut input = String::from("---\ntitle: Large Test\nauthor: Test\n---\n\n");
        
        // Add 100 sections
        for i in 1..=100 {
            input.push_str(&format!("# Section {}\n\nParagraph {}.\n\n", i, i));
        }
        
        measure_performance("Large Document Parse", || {
            let doc = parse_metamark(&input).unwrap();
            assert!(doc.blocks.len() > 0);
        });
    }

    #[test]
    fn test_complex_nesting_performance() {
        let mut input = String::from("# Deep Nesting Test\n\n");
        
        // Add 10 levels of nested components
        let mut current = String::new();
        for i in 0..10 {
            current.push_str(&format!("[[component: type=\"level{}\"]]
# Level {i}
Some content at level {i}\n", i));
        }
        
        // Close all components
        current.push_str(&"\n[[/component]]\n".repeat(10));
        input.push_str(&current);

        measure_performance("Complex Nesting Parse", || {
            let doc = parse_metamark(&input).unwrap();
            assert!(doc.blocks.len() > 0);
        });
    }

    #[test]
    fn test_metadata_performance() {
        let mut input = String::from("---\n");
        input.push_str("title: Large Metadata Test\n");
        input.push_str("author: Test Author\n");
        input.push_str("tags:\n");
        
        // Add 1000 tags
        for i in 0..1000 {
            input.push_str(&format!("  - tag{}\n", i));
        }
        
        input.push_str("---\n\n# Content\n");

        measure_performance("Large Metadata Parse", || {
            let doc = parse_metamark(&input).unwrap();
            assert!(doc.metadata.is_some());
        });
    }

    #[test]
    fn test_mixed_content_performance() {
        let mut input = String::from("---\ntitle: Mixed Content Test\n---\n\n");

        // Add various block types
        for i in 0..50 {
            // Heading with annotations
            input.push_str(&format!("# Section {i} @[note: Important] @[status: Draft]\n\n"));

            // Lists
            input.push_str("- Item 1\n  - Nested 1.1\n  - Nested 1.2\n- Item 2\n\n");

            // Component
            input.push_str(&format!("[[component: type=\"card{i}\"]]
## Card Content
Some text here
[[/component]]\n\n"));

            // Code block
            input.push_str("```rust\nfn test() {}\n```\n\n");

            // Math
            input.push_str("Inline math: $x_{i}^2$\n\n$$\\sum_{j=0}^{i} j$$\n\n");
        }

        measure_performance("Mixed Content Parse", || {
            let doc = parse_metamark(&input).unwrap();
            assert!(doc.blocks.len() > 0);
        });
    }

    #[test]
    fn test_repeated_parse_performance() {
        let input = r#"---
title: Test Document
author: Test Author
---

# Section 1

This is a test paragraph.

## Subsection 1.1

- List item 1
- List item 2"#;
        
        measure_performance("Repeated Parse (10 times)", || {
            for _ in 0..10 {
                let doc = parse_metamark(&input).unwrap();
                assert!(doc.blocks.len() > 0);
            }
        });
    }
} 