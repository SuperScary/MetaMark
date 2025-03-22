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
#include <stdio.h>
#include <ctype.h>
#define _GNU_SOURCE  // For strndup
#include "../include/metamark.h"
#include "../include/lexer.h"
#include "../include/utils.h"

// Custom implementation of strndup
static char* mm_strndup(const char *s, size_t n) {
    size_t len = strnlen(s, n);
    char *new = malloc(len + 1);
    if (new == NULL) {
        return NULL;
    }
    new[len] = '\0';
    return memcpy(new, s, len);
}

// Forward declarations for parser functions
static Node* parse_node(Lexer *lexer);
static Node* parse_heading(Lexer *lexer);
static Node* parse_component(Lexer *lexer);
static Node* parse_annotation(Lexer *lexer);
static Node* parse_comment(Lexer *lexer);
static Node* parse_metadata(Lexer *lexer);

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
    // Skip # characters and count level
    size_t level = 0;
    while (peek(lexer) == '#') {
        level++;
        next(lexer);
    }
    
    printf("Found heading level %zu\n", level);
    
    // Skip whitespace after #
    while (isspace(peek(lexer)) && peek(lexer) != '\n') {
        next(lexer);
    }
    
    // Read heading content until newline
    size_t start = lexer->pos;
    while (peek(lexer) != '\0' && peek(lexer) != '\n') {
        next(lexer);
    }
    
    // Skip the newline
    if (peek(lexer) == '\n') {
        next(lexer);
    }
    
    char *content = read_token_value(lexer, start, lexer->pos - 1);  // -1 to exclude newline
    if (content && *content) {
        printf("Heading content: '%s'\n", content);
        Node *node = create_node(NODE_HEADING, content);
        printf("Created heading node with type %d\n", node->type);
        free(content);
        
        // Store heading level directly in the node
        node->level = level;
        
        return node;
    }
    
    free(content);
    return NULL;
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
    
    // Read component type until ]]
    size_t start = lexer->pos;
    while (peek(lexer) != ']' && peek(lexer) != '\0') {
        next(lexer);
    }
    
    // Check for closing delimiter
    if (peek(lexer) != ']' || peek_at(lexer, 1) != ']') {
        set_error(MM_ERROR_SYNTAX);
        return NULL;
    }
    
    char *type = read_token_value(lexer, start, lexer->pos);
    printf("Component type: '%s'\n", type);
    Node *node = create_node(NODE_COMPONENT, type);
    free(type);
    
    // Skip ]] delimiter
    if (peek(lexer) == ']' && peek_at(lexer, 1) == ']') {
        lexer->pos += 2;
    }
    
    // Skip newline
    if (peek(lexer) == '\n') {
        next(lexer);
    }
    
    // Read content until [[/type]]
    start = lexer->pos;
    while (peek(lexer) != '\0') {
        if (peek(lexer) == '[' && peek_at(lexer, 1) == '[' &&
            peek_at(lexer, 2) == '/') {
            break;
        }
        next(lexer);
    }
    
    // Check for closing delimiter
    if (peek(lexer) != '[' || peek_at(lexer, 1) != '[' || peek_at(lexer, 2) != '/') {
        free_node(node);
        set_error(MM_ERROR_SYNTAX);
        return NULL;
    }
    
    char *content = read_token_value(lexer, start, lexer->pos);
    if (content) {
        printf("Component content: '%s'\n", content);
        Node *content_node = create_node(NODE_PARAGRAPH, content);
        add_child(node, content_node);
        free(content);
    }
    
    // Skip [[/type]] delimiter
    while (peek(lexer) != '\0' && peek(lexer) != ']') {
        next(lexer);
    }
    if (peek(lexer) == ']' && peek_at(lexer, 1) == ']') {
        lexer->pos += 2;
    }
    
    // Skip newline after closing delimiter
    if (peek(lexer) == '\n') {
        next(lexer);
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
    // Skip > delimiter
    next(lexer);
    
    // Skip whitespace after >
    while (isspace(peek(lexer)) && peek(lexer) != '\n') {
        next(lexer);
    }
    
    // Read annotation type until : or newline
    size_t start = lexer->pos;
    while (peek(lexer) != ':' && peek(lexer) != '\0' && peek(lexer) != '\n') {
        next(lexer);
    }
    
    // Check if we found a type
    if (lexer->pos == start) {
        set_error(MM_ERROR_SYNTAX);
        return NULL;
    }
    
    char *type = read_token_value(lexer, start, lexer->pos);
    if (!type || !is_valid_identifier(type)) {
        free(type);
        set_error(MM_ERROR_SYNTAX);
        return NULL;
    }
    
    printf("Annotation type: '%s'\n", type);
    Node *node = create_node(NODE_ANNOTATION, type);
    free(type);
    
    // Skip : delimiter if present
    if (peek(lexer) == ':') {
        next(lexer);
        
        // Skip whitespace
        while (isspace(peek(lexer)) && peek(lexer) != '\n') {
            next(lexer);
        }
        
        // Read content until newline
        start = lexer->pos;
        while (peek(lexer) != '\0' && peek(lexer) != '\n') {
            next(lexer);
        }
        
        char *content = read_token_value(lexer, start, lexer->pos);
        if (content) {
            printf("Annotation content: '%s'\n", content);
            Node *content_node = create_node(NODE_PARAGRAPH, content);
            add_child(node, content_node);
            free(content);
        }
    }
    
    // Skip newline
    if (peek(lexer) == '\n') {
        next(lexer);
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
    
    // Skip whitespace after %%
    while (isspace(peek(lexer)) && peek(lexer) != '\n') {
        next(lexer);
    }
    
    // Read comment content until next %%
    size_t start = lexer->pos;
    while (peek(lexer) != '\0') {
        if (peek(lexer) == '%' && peek_at(lexer, 1) == '%') {
            break;
        }
        next(lexer);
    }
    
    // Check if we found the closing delimiter
    if (peek(lexer) != '%' || peek_at(lexer, 1) != '%') {
        set_error(MM_ERROR_SYNTAX);
        return NULL;
    }
    
    // Trim trailing whitespace
    size_t end = lexer->pos;
    while (end > start && isspace(lexer->input[end - 1])) {
        end--;
    }
   
    char *content = read_token_value(lexer, start, end);
    if (!content) {
        content = strdup("");  // Create empty string for empty comments
        if (!content) {
            set_error(MM_ERROR_MEMORY);
            return NULL;
        }
    }
    
    printf("Comment content: '%s'\n", content);
    Node *node = create_node(NODE_COMMENT, content);
    free(content);
    
    // Skip %% delimiter
    if (peek(lexer) == '%' && peek_at(lexer, 1) == '%') {
        lexer->pos += 2;
    }
    
    // Skip newline after %%
    if (peek(lexer) == '\n') {
        next(lexer);
    }
    
    return node;
}

/**
 * @brief Parse a metadata block from the input
 * 
 * @param lexer The lexer instance
 * @return Node* A new metadata node, or NULL on error
 * 
 * Metadata blocks are delimited by --- and contain YAML-style key-value pairs.
 * The content is stored as a single string and later parsed by parse_metadata_string.
 */
static Node* parse_metadata(Lexer *lexer) {
    // Skip opening ---
    next_token(lexer);
    
    // Read until closing ---
    size_t start = lexer->pos;
    while (peek(lexer) != '\0') {
        if (peek(lexer) == '-' && 
            peek_at(lexer, 1) == '-' && 
            peek_at(lexer, 2) == '-') {
            break;
        }
        next(lexer);
    }
    
    // Check if we found the closing delimiter
    if (peek(lexer) != '-' || 
        peek_at(lexer, 1) != '-' || 
        peek_at(lexer, 2) != '-') {
        set_error(MM_ERROR_SYNTAX);
        return NULL;
    }
    
    char *content = read_token_value(lexer, start, lexer->pos);
    if (!content) {
        set_error(MM_ERROR_MEMORY);
        return NULL;
    }
    
    // Create metadata node with original content
    Node *node = create_node(NODE_METADATA, content);
    if (!node) {
        free(content);
        set_error(MM_ERROR_MEMORY);
        return NULL;
    }
    
    // Parse metadata content and create child nodes
    const char *line_start = content;
    const char *line_end;
    
    while (*line_start) {
        // Find end of line
        line_end = strchr(line_start, '\n');
        if (!line_end) {
            line_end = line_start + strlen(line_start);
        }
        
        // Skip empty lines and comments
        const char *p = line_start;
        while (p < line_end && isspace(*p)) p++;
        
        if (p < line_end && *p != '#') {
            // Look for colon
            const char *colon = memchr(line_start, ':', line_end - line_start);
            if (!colon) {
                // Invalid metadata line - no colon
                free_node(node);
                set_error(MM_ERROR_SYNTAX);
                return NULL;
            }
            
            // Extract key
            size_t key_len = colon - line_start;
            char *key = malloc(key_len + 1);
            if (key) {
                memcpy(key, line_start, key_len);
                key[key_len] = '\0';
                
                // Trim key
                char *k = key;
                while (*k && isspace(*k)) k++;
                char *k_end = key + key_len - 1;
                while (k_end > k && isspace(*k_end)) k_end--;
                k_end[1] = '\0';
                
                // Extract value
                const char *v = colon + 1;
                while (v < line_end && isspace(*v)) v++;
                size_t value_len = line_end - v;
                char *value = malloc(value_len + 1);
                if (value) {
                    memcpy(value, v, value_len);
                    value[value_len] = '\0';
                    
                    // Trim value
                    char *v_end = value + value_len - 1;
                    while (v_end > value && isspace(*v_end)) v_end--;
                    v_end[1] = '\0';
                    
                    // Create child node
                    char *child_content = malloc(strlen(k) + strlen(value) + 2);
                    if (child_content) {
                        sprintf(child_content, "%s:%s", k, value);
                        Node *child = create_node(NODE_PARAGRAPH, child_content);
                        if (child) {
                            add_child(node, child);
                        }
                        free(child_content);
                    }
                    free(value);
                }
                free(key);
            }
        }
        
        // Move to next line
        if (*line_end == '\n') {
            line_start = line_end + 1;
        } else {
            break;
        }
    }
    
    // Skip closing ---
    if (peek_at(lexer, 0) == '-' && 
        peek_at(lexer, 1) == '-' && 
        peek_at(lexer, 2) == '-') {
        lexer->pos += 3;
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
    char current = peek(lexer);
    
    // Skip empty lines
    while (current == '\n' || current == '\r') {
        next(lexer);
        current = peek(lexer);
    }
    
    // Skip leading whitespace
    while (isspace(current)) {
        next(lexer);
        current = peek(lexer);
    }
    
    // Handle different node types based on the current character
    if (current == '#') {
        return parse_heading(lexer);
    } else if (current == '[' && peek_at(lexer, 1) == '[') {
        return parse_component(lexer);
    } else if (current == '>') {
        return parse_annotation(lexer);
    } else if (current == '%' && peek_at(lexer, 1) == '%') {
        return parse_comment(lexer);
    } else if (current == '-' && peek_at(lexer, 1) == '-' && peek_at(lexer, 2) == '-') {
        return parse_metadata(lexer);
    } else if (current != '\0') {
        // For text tokens, collect all text until a special token or double newline
        size_t start = lexer->pos;
        size_t consecutive_newlines = 0;
        
        while (peek(lexer) != '\0') {
            current = peek(lexer);
            if (current == '\n') {
                consecutive_newlines++;
                next(lexer);
                if (consecutive_newlines >= 2) {
                    break;
                }
            } else if (current == '#' || 
                      (current == '[' && peek_at(lexer, 1) == '[') ||
                      (current == '>' && peek_at(lexer, 1) != '>') ||
                      (current == '%' && peek_at(lexer, 1) == '%') ||
                      (current == '-' && peek_at(lexer, 1) == '-' && peek_at(lexer, 2) == '-')) {
                break;
            } else {
                consecutive_newlines = 0;
                next(lexer);
            }
        }
        
        // Trim trailing newlines
        size_t end = lexer->pos;
        while (end > start && (lexer->input[end - 1] == '\n' || lexer->input[end - 1] == '\r')) {
            end--;
        }
        
        char *content = read_token_value(lexer, start, end);
        if (content && *content) {  // Only create node if content is not empty
            Node *node = create_node(NODE_PARAGRAPH, content);
            free(content);
            
            // Skip any remaining newlines
            while (peek(lexer) == '\n' || peek(lexer) == '\r') {
                next(lexer);
            }
            
            return node;
        }
        free(content);
    }
    
    return NULL;
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
    if (!input) {
        set_error(MM_ERROR_INVALID);
        return NULL;
    }
    
    // Skip leading whitespace
    while (*input && isspace(*input)) {
        input++;
    }
    
    // Check for empty input
    if (!*input) {
        set_error(MM_ERROR_SYNTAX);
        return NULL;
    }
    
    Lexer lexer;
    lexer_init(&lexer, input);
    
    Document *doc = malloc(sizeof(Document));
    if (!doc) {
        set_error(MM_ERROR_MEMORY);
        lexer_free(&lexer);
        return NULL;
    }
    
    doc->metadata = NULL;
    doc->metadata_count = 0;
    doc->root = create_node(NODE_DOCUMENT, NULL);
    if (!doc->root) {
        set_error(MM_ERROR_MEMORY);
        free(doc);
        lexer_free(&lexer);
        return NULL;
    }
    
    // Parse metadata if present (delimited by ---)
    if (peek_at(&lexer, 0) == '-' && 
        peek_at(&lexer, 1) == '-' && 
        peek_at(&lexer, 2) == '-') {
        Node *metadata_node = parse_metadata(&lexer);
        if (metadata_node) {
            add_child(doc->root, metadata_node);
            parse_metadata_node(doc, metadata_node);
        }
    }
    
    // Parse document content
    while (peek(&lexer) != '\0') {
        Node *node = parse_node(&lexer);
        if (node) {
            add_child(doc->root, node);
        } else {
            // Skip any remaining whitespace or empty lines
            while (isspace(peek(&lexer))) {
                next(&lexer);
            }
        }
    }
    
    // Verify document structure
    if (doc->root->child_count == 0) {
        set_error(MM_ERROR_SYNTAX);
        free_document(doc);
        lexer_free(&lexer);
        return NULL;
    }
    
    lexer_free(&lexer);
    return doc;
} 