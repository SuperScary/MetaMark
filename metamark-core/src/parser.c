/**
 * @file parser.c
 * @brief MetaMark document parser implementation
 * 
 * This file implements the core parsing logic for MetaMark documents.
 * It uses a recursive descent parser to build an Abstract Syntax Tree (AST)
 * from the input text.
 */

#include <stdlib.h>
#include <string.h>
#include "../include/metamark.h"
#include "../include/lexer.h"

// Forward declarations for parser functions
static Node* parse_heading(Lexer *lexer);
static Node* parse_paragraph(Lexer *lexer);
static Node* parse_component(Lexer *lexer);
static Node* parse_annotation(Lexer *lexer);
static Node* parse_comment(Lexer *lexer);
static Node* parse_secure_block(Lexer *lexer);

/**
 * @brief Parse a heading node from the input
 * 
 * @param lexer The lexer instance
 * @return Node* A new heading node, or NULL on error
 * 
 * Headings start with one or more # characters, followed by whitespace
 * and the heading text. The number of # characters determines the heading level.
 */
static Node* parse_heading(Lexer *lexer) {
    char *content = NULL;
    size_t level = 0;
    
    // Count heading level (number of # characters)
    while (peek(lexer) == '#') {
        level++;
        next(lexer);
    }
    
    // Skip whitespace after #
    skip_whitespace(lexer);
    
    // Read heading content until newline
    size_t start = lexer->pos;
    while (peek(lexer) != '\0' && peek(lexer) != '\n') {
        next(lexer);
    }
    
    content = read_token_value(lexer, start, lexer->pos);
    Node *node = create_node(NODE_HEADING, content);
    
    // Store heading level as metadata
    char level_str[2] = {(char)('0' + level), '\0'};
    Node *level_node = create_node(NODE_METADATA, level_str);
    add_child(node, level_node);
    
    return node;
}

/**
 * @brief Parse a paragraph node from the input
 * 
 * @param lexer The lexer instance
 * @return Node* A new paragraph node, or NULL on error
 * 
 * Paragraphs are blocks of text separated by blank lines.
 * The parser collects text until it encounters two consecutive newlines.
 */
static Node* parse_paragraph(Lexer *lexer) {
    size_t start = lexer->pos;
    int consecutive_newlines = 0;
    
    while (peek(lexer) != '\0') {
        if (peek(lexer) == '\n') {
            consecutive_newlines++;
            next(lexer);
            if (consecutive_newlines >= 2) {
                break;
            }
        } else {
            consecutive_newlines = 0;
            next(lexer);
        }
    }
    
    char *content = read_token_value(lexer, start, lexer->pos);
    return create_node(NODE_PARAGRAPH, content);
}

/**
 * @brief Parse a component block from the input
 * 
 * @param lexer The lexer instance
 * @return Node* A new component node, or NULL on error
 * 
 * Component blocks are delimited by [[ and ]] and have the format:
 * [[type:content]]. The type determines how the content should be processed.
 */
static Node* parse_component(Lexer *lexer) {
    // Skip [[ delimiter
    next(lexer);
    next(lexer);
    
    // Read component type until :
    size_t start = lexer->pos;
    while (peek(lexer) != ':' && peek(lexer) != '\0') {
        next(lexer);
    }
    
    char *type = read_token_value(lexer, start, lexer->pos);
    Node *node = create_node(NODE_COMPONENT, type);
    
    // Skip : separator
    if (peek(lexer) == ':') {
        next(lexer);
    }
    
    // Parse component content until ]]
    while (peek(lexer) != '\0') {
        if (peek(lexer) == ']' && peek(lexer + 1) == ']') {
            lexer->pos += 2;
            break;
        }
        
        Node *child = parse_node(lexer);
        if (child) {
            add_child(node, child);
        }
    }
    
    return node;
}

/**
 * @brief Parse an annotation from the input
 * 
 * @param lexer The lexer instance
 * @return Node* A new annotation node, or NULL on error
 * 
 * Annotations are delimited by @[ and ] and have the format:
 * @[type:content]. They are used for inline notes and comments.
 */
static Node* parse_annotation(Lexer *lexer) {
    // Skip @[ delimiter
    next(lexer);
    next(lexer);
    
    // Read annotation type until :
    size_t start = lexer->pos;
    while (peek(lexer) != ':' && peek(lexer) != '\0') {
        next(lexer);
    }
    
    char *type = read_token_value(lexer, start, lexer->pos);
    Node *node = create_node(NODE_ANNOTATION, type);
    
    // Skip : separator
    if (peek(lexer) == ':') {
        next(lexer);
    }
    
    // Parse annotation content until ]
    while (peek(lexer) != '\0') {
        if (peek(lexer) == ']') {
            next(lexer);
            break;
        }
        
        Node *child = parse_node(lexer);
        if (child) {
            add_child(node, child);
        }
    }
    
    return node;
}

/**
 * @brief Parse a comment block from the input
 * 
 * @param lexer The lexer instance
 * @return Node* A new comment node, or NULL on error
 * 
 * Comment blocks are delimited by %% and are not rendered in the output.
 * They can span multiple lines.
 */
static Node* parse_comment(Lexer *lexer) {
    // Skip %% delimiter
    next(lexer);
    next(lexer);
    
    // Read comment content until next %%
    size_t start = lexer->pos;
    while (peek(lexer) != '\0') {
        if (peek(lexer) == '%' && peek(lexer + 1) == '%') {
            lexer->pos += 2;
            break;
        }
        next(lexer);
    }
    
    char *content = read_token_value(lexer, start, lexer->pos);
    return create_node(NODE_COMMENT, content);
}

/**
 * @brief Parse a secure block from the input
 * 
 * @param lexer The lexer instance
 * @return Node* A new secure node, or NULL on error
 * 
 * Secure blocks contain encrypted or sensitive content.
 * They are processed differently from regular content.
 */
static Node* parse_secure_block(Lexer *lexer) {
    Node *node = create_node(NODE_SECURE, NULL);
    
    // Parse secure block content
    while (peek(lexer) != '\0') {
        Node *child = parse_node(lexer);
        if (child) {
            add_child(node, child);
        }
    }
    
    return node;
}

/**
 * @brief Parse a single node based on the current token
 * 
 * @param lexer The lexer instance
 * @return Node* A new node, or NULL on error
 * 
 * This function determines the type of node to parse based on the current token
 * and delegates to the appropriate parsing function.
 */
static Node* parse_node(Lexer *lexer) {
    TokenType token = next_token(lexer);
    
    switch (token) {
        case TOKEN_HEADING:
            return parse_heading(lexer);
        case TOKEN_COMPONENT_START:
            return parse_component(lexer);
        case TOKEN_ANNOTATION_START:
            return parse_annotation(lexer);
        case TOKEN_COMMENT_START:
            return parse_comment(lexer);
        case TOKEN_TEXT:
            return parse_paragraph(lexer);
        case TOKEN_NEWLINE:
            return parse_node(lexer); // Skip newlines
        default:
            return NULL;
    }
}

/**
 * @brief Parse a complete MetaMark document
 * 
 * @param input The input text to parse
 * @return Document* A new document structure, or NULL on error
 * 
 * This is the main entry point for parsing MetaMark documents.
 * It handles both the frontmatter metadata and the document content.
 */
Document* parse_metamark(const char *input) {
    Lexer lexer;
    lexer_init(&lexer, input);
    
    Document *doc = malloc(sizeof(Document));
    if (!doc) {
        lexer_free(&lexer);
        return NULL;
    }
    
    doc->metadata = NULL;
    doc->metadata_count = 0;
    doc->root = create_node(NODE_DOCUMENT, NULL);
    
    // Parse metadata if present (delimited by ---)
    if (peek(&lexer) == '-' && peek(&lexer + 1) == '-' && peek(&lexer + 2) == '-') {
        lexer.pos += 3;
        Node *metadata_node = create_node(NODE_METADATA, NULL);
        
        // Parse metadata content until next ---
        while (peek(&lexer) != '\0') {
            if (peek(&lexer) == '-' && peek(&lexer + 1) == '-' && peek(&lexer + 2) == '-') {
                lexer.pos += 3;
                break;
            }
            
            Node *child = parse_node(&lexer);
            if (child) {
                add_child(metadata_node, child);
            }
        }
        
        add_child(doc->root, metadata_node);
    }
    
    // Parse document content
    while (peek(&lexer) != '\0') {
        Node *node = parse_node(&lexer);
        if (node) {
            add_child(doc->root, node);
        }
    }
    
    lexer_free(&lexer);
    return doc;
} 