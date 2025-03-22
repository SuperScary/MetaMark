#include <stdlib.h>
#include <string.h>
#include "../include/metamark.h"

#define INITIAL_CHILD_CAPACITY 4

Node* create_node(NodeType type, const char *content) {
    Node *node = malloc(sizeof(Node));
    if (!node) {
        return NULL;
    }
    
    node->type = type;
    node->content = content ? strdup(content) : NULL;
    node->children = malloc(sizeof(Node*) * INITIAL_CHILD_CAPACITY);
    node->child_count = 0;
    node->child_capacity = INITIAL_CHILD_CAPACITY;
    
    if (!node->children) {
        free(node);
        return NULL;
    }
    
    return node;
}

void add_child(Node *parent, Node *child) {
    if (!parent || !child) {
        return;
    }
    
    if (parent->child_count >= parent->child_capacity) {
        size_t new_capacity = parent->child_capacity * 2;
        Node **new_children = realloc(parent->children, sizeof(Node*) * new_capacity);
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
    
    // Free all children recursively
    for (size_t i = 0; i < node->child_count; i++) {
        free_node(node->children[i]);
    }
    
    free(node->children);
    free(node->content);
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
        case NODE_DOCUMENT: return "Document";
        case NODE_METADATA: return "Metadata";
        case NODE_PARAGRAPH: return "Paragraph";
        case NODE_HEADING: return "Heading";
        case NODE_ANNOTATION: return "Annotation";
        case NODE_COMMENT: return "Comment";
        case NODE_COMPONENT: return "Component";
        case NODE_COLLAPSIBLE: return "Collapsible";
        case NODE_DIAGRAM: return "Diagram";
        case NODE_MATH: return "Math";
        case NODE_SECURE: return "Secure";
        default: return "Unknown";
    }
} 