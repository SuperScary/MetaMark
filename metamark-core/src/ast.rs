//! Abstract Syntax Tree (AST) definitions for MetaMark documents.
//!
//! This module defines the data structures that represent a parsed MetaMark document,
//! including metadata, blocks, inline elements, and annotations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a complete MetaMark document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Optional metadata section at the start of the document
    pub metadata: Option<Metadata>,
    /// Sequence of block-level elements that make up the document
    pub blocks: Vec<Block>,
}

/// Document metadata parsed from YAML or TOML frontmatter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// Key-value pairs of metadata information
    pub data: HashMap<String, MetaValue>,
}

/// Possible values that can appear in document metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MetaValue {
    /// String value
    String(String),
    /// Numeric value (floating point for maximum compatibility)
    Number(f64),
    /// Boolean value
    Boolean(bool),
    /// Array of metadata values
    Array(Vec<MetaValue>),
    /// Nested object of metadata values
    Object(HashMap<String, MetaValue>),
}

/// Block-level elements that can appear in a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Block {
    /// Section heading with level and optional annotations
    Heading {
        /// Heading level (1-6)
        level: u8,
        /// Heading text content
        content: String,
        /// Optional annotations attached to the heading
        annotations: Vec<Annotation>,
    },
    /// Text paragraph with inline formatting
    Paragraph {
        /// Sequence of inline elements making up the paragraph
        content: Vec<Inline>,
        /// Optional annotations attached to the paragraph
        annotations: Vec<Annotation>,
    },
    /// Custom component block with attributes
    Component {
        /// Component type name
        name: String,
        /// Component attributes as key-value pairs
        attributes: HashMap<String, String>,
        /// Nested blocks within the component
        content: Vec<Block>,
    },
    /// Fenced code block
    CodeBlock {
        /// Optional language identifier
        language: Option<String>,
        /// Code block content
        content: String,
    },
    /// Diagram block (e.g., Mermaid, PlantUML)
    Diagram {
        /// Type of diagram
        kind: DiagramType,
        /// Diagram source code
        content: String,
    },
    /// Encrypted content block
    SecureBlock {
        /// Encrypted content as bytes
        content: Vec<u8>,
        /// Encryption metadata
        encryption_info: EncryptionInfo,
    },
    /// Ordered or unordered list
    List {
        /// List items with their content and nesting level
        items: Vec<ListItem>,
        /// Whether this is an ordered (true) or unordered (false) list
        ordered: bool,
    },
    /// Single-line comment
    Comment(String),
    /// Block-level math expression
    Math(String),
}

/// Item in an ordered or unordered list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    /// Nested blocks making up the item's content
    pub content: Vec<Block>,
    /// Nesting level (0 = top level)
    pub level: usize,
}

/// Inline formatting elements that can appear within text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Inline {
    /// Plain text
    Text(String),
    /// Bold text
    Bold(Box<Inline>),
    /// Italic text
    Italic(Box<Inline>),
    /// Inline code
    Code(String),
    /// Hyperlink
    Link {
        /// Link text
        text: String,
        /// Link URL
        url: String,
    },
    /// Inline math expression
    Math(String),
}

/// Annotation attached to a block element.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    /// Annotation type (e.g., "note", "warning")
    pub kind: String,
    /// Annotation content
    pub content: String,
}

/// Types of diagrams supported in diagram blocks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagramType {
    /// Mermaid.js diagram
    Mermaid,
    /// PlantUML diagram
    PlantUML,
    /// GraphViz diagram
    GraphViz,
}

/// Metadata for encrypted content blocks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionInfo {
    /// Encryption algorithm identifier
    pub algorithm: String,
    /// Key identifier for decryption
    pub key_id: String,
    /// Initialization vector or nonce
    pub nonce: Vec<u8>,
} 