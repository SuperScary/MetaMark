/**
 * @file metamark.h
 * @brief Core MetaMark parsing library header
 * 
 * This header defines the public API for the MetaMark parsing library.
 * MetaMark is a Markdown-inspired format that supports extended features
 * like components, annotations, and secure blocks.
 */

#ifndef METAMARK_H
#define METAMARK_H

#include <stddef.h>

/**
 * @brief Node types in the Abstract Syntax Tree (AST)
 * 
 * Each node in the AST represents a different element in the MetaMark document.
 * The type determines how the node should be processed and rendered.
 */
typedef enum {
    NODE_DOCUMENT,    ///< Root node of the document
    NODE_METADATA,    ///< Document metadata (frontmatter)
    NODE_PARAGRAPH,   ///< Regular text paragraph
    NODE_HEADING,     ///< Document heading (h1-h6)
    NODE_ANNOTATION,  ///< Inline annotation (@[type:content])
    NODE_COMMENT,     ///< Comment block (%%content%%)
    NODE_COMPONENT,   ///< Special component block ([[type:content]])
    NODE_COLLAPSIBLE, ///< Collapsible section
    NODE_DIAGRAM,     ///< Diagram component
    NODE_MATH,        ///< Mathematical expression
    NODE_SECURE       ///< Encrypted/secure block
} NodeType;

/**
 * @brief Structure representing a node in the AST
 * 
 * Each node can have multiple children, forming a tree structure.
 * The content field stores the actual text content of the node.
 */
typedef struct Node {
    NodeType type;           ///< Type of the node
    char *content;          ///< Text content of the node
    struct Node **children; ///< Array of child nodes
    size_t child_count;     ///< Number of child nodes
    size_t child_capacity;  ///< Current capacity of children array
} Node;

/**
 * @brief Key-value pair for document metadata
 * 
 * Used to store frontmatter metadata like title, author, etc.
 */
typedef struct {
    char *key;   ///< Metadata key
    char *value; ///< Metadata value
} MetadataPair;

/**
 * @brief Complete document structure
 * 
 * Contains both the document metadata and the root node of the AST.
 */
typedef struct {
    MetadataPair *metadata;     ///< Array of metadata key-value pairs
    size_t metadata_count;      ///< Number of metadata entries
    Node *root;                 ///< Root node of the document AST
} Document;

/**
 * @brief Error codes for the MetaMark library
 */
typedef enum {
    MM_SUCCESS = 0,        ///< Operation successful
    MM_ERROR_MEMORY,       ///< Memory allocation failed
    MM_ERROR_SYNTAX,       ///< Syntax error in document
    MM_ERROR_IO           ///< Input/output error
} MetaMarkError;

/**
 * @brief Parse a MetaMark document from a string
 * 
 * @param input The MetaMark document text to parse
 * @return Document* A new document structure, or NULL on error
 * 
 * This is the main entry point for parsing MetaMark documents.
 * The function creates a complete AST representation of the document,
 * including metadata and all document elements.
 */
Document* parse_metamark(const char *input);

/**
 * @brief Free a document and all its resources
 * 
 * @param doc The document to free
 * 
 * This function properly frees all memory allocated for the document,
 * including the AST, metadata, and all node contents.
 */
void free_document(Document *doc);

/**
 * @brief Create a new AST node
 * 
 * @param type The type of node to create
 * @param content The text content of the node
 * @return Node* A new node, or NULL on error
 */
Node* create_node(NodeType type, const char *content);

/**
 * @brief Add a child node to a parent node
 * 
 * @param parent The parent node
 * @param child The child node to add
 * 
 * The child node is added to the parent's children array.
 * The array is automatically resized if needed.
 */
void add_child(Node *parent, Node *child);

/**
 * @brief Free a node and all its children
 * 
 * @param node The node to free
 */
void free_node(Node *node);

/**
 * @brief Add a metadata key-value pair to a document
 * 
 * @param doc The document to add metadata to
 * @param key The metadata key
 * @param value The metadata value
 */
void add_metadata(Document *doc, const char *key, const char *value);

/**
 * @brief Get a metadata value by key
 * 
 * @param doc The document to search
 * @param key The metadata key to look up
 * @return const char* The metadata value, or NULL if not found
 */
const char* get_metadata(const Document *doc, const char *key);

/**
 * @brief Print the AST structure for debugging
 * 
 * @param root The root node to print
 * @param indent The current indentation level
 */
void print_ast(const Node *root, int indent);

/**
 * @brief Convert a node type to a string
 * 
 * @param type The node type to convert
 * @return const char* A string representation of the node type
 */
const char* node_type_to_string(NodeType type);

/**
 * @brief Get the last error that occurred
 * 
 * @return MetaMarkError The last error code
 */
MetaMarkError get_last_error(void);

/**
 * @brief Convert an error code to a string
 * 
 * @param error The error code to convert
 * @return const char* A string description of the error
 */
const char* error_to_string(MetaMarkError error);

#endif // METAMARK_H 