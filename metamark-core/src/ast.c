/**
 * @file ast.c
 * @brief Abstract Syntax Tree implementation for MetaMark
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../include/metamark.h"

Node* create_node(NodeType type, const char *content) {
    Node *node = malloc(sizeof(Node));
    if (!node) {
        return NULL;
    }
    
    node->type = type;
    node->content = content ? strdup(content) : NULL;
    node->children = NULL;
    node->child_count = 0;
    node->child_capacity = 0;
    
    return node;
}

void add_child(Node *parent, Node *child) {
    if (!parent || !child) {
        return;
    }
    
    if (parent->child_count >= parent->child_capacity) {
        size_t new_capacity = parent->child_capacity == 0 ? 4 : parent->child_capacity * 2;
        Node **new_children = realloc(parent->children, new_capacity * sizeof(Node*));
        if (!new_children) {
            return;
        }
        
        parent->children = new_children;
        parent->child_capacity = new_capacity;
    }
    
    parent->children[parent->child_count++] = child;
}

void free_node(Node *node) {
    if (!node) {
        return;
    }
    
    // Free content
    if (node->content) {
        free(node->content);
    }
    
    // Free children recursively
    for (size_t i = 0; i < node->child_count; i++) {
        free_node(node->children[i]);
    }
    
    // Free children array
    if (node->children) {
        free(node->children);
    }
    
    free(node);
}

void free_document(Document *doc) {
    if (!doc) {
        return;
    }
    
    // Free metadata
    for (size_t i = 0; i < doc->metadata_count; i++) {
        free(doc->metadata[i].key);
        free(doc->metadata[i].value);
    }
    free(doc->metadata);
    
    // Free AST
    free_node(doc->root);
    
    free(doc);
}

void print_ast(const Node *root, int indent) {
    if (!root) {
        return;
    }
    
    // Print indentation
    for (int i = 0; i < indent; i++) {
        printf("  ");
    }
    
    // Print node type and content
    printf("%s", node_type_to_string(root->type));
    if (root->content) {
        printf(": %s", root->content);
    }
    printf("\n");
    
    // Print children
    for (size_t i = 0; i < root->child_count; i++) {
        print_ast(root->children[i], indent + 1);
    }
}

const char* node_type_to_string(NodeType type) {
    switch (type) {
        case NODE_DOCUMENT:
            return "DOCUMENT";
        case NODE_METADATA:
            return "METADATA";
        case NODE_HEADING:
            return "HEADING";
        case NODE_PARAGRAPH:
            return "PARAGRAPH";
        case NODE_COMPONENT:
            return "COMPONENT";
        case NODE_ANNOTATION:
            return "ANNOTATION";
        case NODE_COMMENT:
            return "COMMENT";
        case NODE_SECURE:
            return "SECURE";
        default:
            return "UNKNOWN";
    }
} 