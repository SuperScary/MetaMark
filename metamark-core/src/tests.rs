use crate::{ast::*, error::*, parse_metamark};

#[test]
fn test_complete_document() {
    let input = r#"---
title: Test Document
author: John Doe
tags:
  - test
  - example
---

# Main Heading @[important: true]

This is a paragraph with **bold** and *italic* text.
It also has `inline code` and [links](https://example.com).

[[component: type="card"]]
## Card Title
Some card content with a list:
- Item 1
- Item 2
[[/component]]

%% This is a comment

```rust
fn main() {
    println!("Hello, world!");
}
```

1. First ordered item
2. Second ordered item
   - Nested unordered item
"#;

    let result = parse_metamark(input);
    assert!(result.is_ok());

    let doc = result.unwrap();
    
    // Test metadata
    let metadata = doc.metadata.unwrap();
    match &metadata.data.get("title").unwrap() {
        MetaValue::String(s) => assert_eq!(s, "Test Document"),
        _ => panic!("Expected string value for title"),
    }

    // Test document structure
    assert!(doc.blocks.len() >= 6); // At least 6 top-level blocks

    // Test heading with annotation
    match &doc.blocks[0] {
        Block::Heading { level, content, annotations } => {
            assert_eq!(*level, 1);
            assert_eq!(content.trim(), "Main Heading");
            assert_eq!(annotations.len(), 1);
            assert_eq!(annotations[0].kind, "important");
            assert_eq!(annotations[0].content, "true");
        }
        _ => panic!("Expected heading as first block"),
    }
}

#[test]
fn test_error_handling() {
    // Test invalid metadata
    let input = r#"---
invalid: yaml: : :
---
"#;
    assert!(parse_metamark(input).is_err());

    // Test unclosed component
    let input = "[[component: type=\"card\"]]\nContent";
    assert!(parse_metamark(input).is_err());
}

#[test]
fn test_nested_structures() {
    let input = r#"[[component: type="card"]]
# Heading inside component @[note: nested]
- List inside component
  - Nested list item
  [[component: type="alert"]]
  Nested component content
  [[/component]]
[[/component]]"#;

    let result = parse_metamark(input);
    assert!(result.is_ok());
}

#[test]
fn test_inline_formatting() {
    let input = "This has **bold**, *italic*, and `code` with a [link](url).";
    let doc = parse_metamark(input).unwrap();

    match &doc.blocks[0] {
        Block::Paragraph { content, .. } => {
            assert!(content.len() > 4); // Should have multiple inline elements
        }
        _ => panic!("Expected paragraph"),
    }
}

#[test]
fn test_code_blocks() {
    let input = r#"```rust
fn test() -> Result<(), Error> {
    Ok(())
}
```"#;

    let doc = parse_metamark(input).unwrap();
    match &doc.blocks[0] {
        Block::CodeBlock { language, content } => {
            assert_eq!(language.as_ref().unwrap(), "rust");
            assert!(content.contains("fn test()"));
        }
        _ => panic!("Expected code block"),
    }
}

// Additional test cases would be added for:
// - Complex metadata structures
// - Edge cases in parsing
// - Error conditions
// - Boundary conditions
// - etc. 