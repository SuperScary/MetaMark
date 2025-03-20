use crate::{
    ast::{Block, Document, Inline},
    document::DocumentManager,
    lexer::Lexer,
    parser::Parser,
    security::Security,
};
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_document_lifecycle() {
    let temp_dir = tempdir().unwrap();
    let manager = DocumentManager::new(temp_dir.path());

    // Create document
    let mut doc = manager.create_document("Test Document").unwrap();
    doc.add_block(Block::Heading {
        level: 1,
        content: "Hello, MetaMark!".to_string(),
        id: "hello-metamark".to_string(),
    });

    // Save document
    let path = temp_dir.path().join("test.mmk");
    manager.save_document(&doc, &path, false).unwrap();

    // Load document
    let loaded_doc = manager.load_document(&path, None).unwrap();
    assert_eq!(loaded_doc.metadata.title, "Test Document");
}

#[test]
fn test_document_encryption() {
    let temp_dir = tempdir().unwrap();
    let manager = DocumentManager::new(temp_dir.path());
    let security = Security::new();

    // Create and encrypt document
    let doc = manager.create_document("Secret Document").unwrap();
    let path = temp_dir.path().join("secret.mmk");
    manager.save_document(&doc, &path, true).unwrap();

    // Try to load without key (should fail)
    assert!(manager.load_document(&path, None).is_err());

    // Generate key and try again
    let key = security.generate_key().unwrap();
    let loaded_doc = manager.load_document(&path, Some(&key)).unwrap();
    assert_eq!(loaded_doc.metadata.title, "Secret Document");
}

#[test]
fn test_document_parsing() {
    let input = r#"---
title: Test Document
version: 1.0.0
---

# Heading 1

This is a paragraph with **bold** and *italic* text.

```rust
fn main() {
    println!("Hello!");
}
```"#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let doc = parser.parse().unwrap();

    assert_eq!(doc.metadata.title, "Test Document");
    assert_eq!(doc.metadata.version, "1.0.0");

    // Check document structure
    match &doc.content[0] {
        Block::Heading { level, content, .. } => {
            assert_eq!(*level, 1);
            assert_eq!(content, "Heading 1");
        }
        _ => panic!("Expected heading"),
    }
}

#[test]
fn test_document_export() {
    let manager = DocumentManager::new(tempdir().unwrap().path());
    let mut doc = manager.create_document("Export Test").unwrap();

    doc.add_block(Block::Heading {
        level: 1,
        content: "Test".to_string(),
        id: "test".to_string(),
    });

    doc.add_block(Block::Paragraph {
        content: vec![
            Inline::Text("Hello ".to_string()),
            Inline::Bold("world".to_string()),
            Inline::Text("!".to_string()),
        ],
    });

    let exported = manager.export_mmk(&doc).unwrap();
    assert!(exported.contains("# Test"));
    assert!(exported.contains("Hello **world**!"));
}

#[test]
fn test_document_metadata() {
    let manager = DocumentManager::new(tempdir().unwrap().path());
    let doc = manager.create_document("Metadata Test").unwrap();

    assert_eq!(doc.metadata.title, "Metadata Test");
    assert!(!doc.metadata.created_at.is_empty());
    assert!(!doc.metadata.updated_at.is_empty());
    assert_eq!(doc.metadata.version, crate::VERSION);
} 