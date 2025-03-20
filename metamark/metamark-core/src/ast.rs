use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a complete MetaMark document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub metadata: Metadata,
    pub content: Vec<Block>,
    pub annotations: Vec<Annotation>,
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub authors: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub version: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}

/// Block-level elements in the document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Block {
    Heading {
        level: u8,
        content: String,
        id: String,
    },
    Paragraph {
        content: Vec<Inline>,
    },
    CodeBlock {
        language: String,
        content: String,
    },
    List {
        items: Vec<ListItem>,
        ordered: bool,
    },
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    BlockQuote {
        content: Vec<Block>,
    },
}

/// Inline elements within blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Inline {
    Text(String),
    Bold(String),
    Italic(String),
    Code(String),
    Link {
        text: String,
        url: String,
    },
    Math {
        content: String,
        display: bool,
    },
}

/// List items can contain nested blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    pub content: Vec<Block>,
    pub checked: Option<bool>,
}

/// Annotations for collaborative editing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub id: String,
    pub author: String,
    pub created_at: String,
    pub target: AnnotationTarget,
    pub content: String,
    pub resolved: bool,
}

/// Target of an annotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationTarget {
    pub block_id: String,
    pub range: Option<Range>,
}

/// Text range for annotations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Document {
    pub fn new(title: String) -> Self {
        Self {
            metadata: Metadata {
                title,
                authors: Vec::new(),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                version: crate::VERSION.to_string(),
                tags: Vec::new(),
                custom: HashMap::new(),
            },
            content: Vec::new(),
            annotations: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.content.push(block);
        self.metadata.updated_at = chrono::Utc::now().to_rfc3339();
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.push(annotation);
        self.metadata.updated_at = chrono::Utc::now().to_rfc3339();
    }
} 