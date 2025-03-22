/**
 * @file test_parser.c
 * @brief Test suite for the MetaMark parser
 * 
 * This file contains unit tests for the MetaMark parser implementation.
 * It tests various aspects of the parser including metadata handling,
 * AST construction, and error handling.
 */

#include <stdio.h>
#include <string.h>
#include <assert.h>
#include "../include/metamark.h"

/**
 * @brief Sample MetaMark document for testing
 * 
 * This document includes all supported MetaMark features:
 * - Frontmatter metadata
 * - Headings
 * - Paragraphs
 * - Components
 * - Annotations
 * - Comments
 */
const char *test_doc = 
"---\n"
"title: Test Document\n"
"author: John Doe\n"
"---\n"
"\n"
"# Main Heading\n"
"\n"
"This is a paragraph with some text.\n"
"\n"
"[[component:diagram]]\n"
"graph TD\n"
"A[Start] --> B[Process]\n"
"B --> C[End]\n"
"[[/component]]\n"
"\n"
"@[note:important]\n"
"This is an important note.\n"
"[/note]\n"
"\n"
"%% This is a comment block %%\n"
"Some content here\n"
"%% End comment %%\n";

/**
 * @brief Test metadata parsing functionality
 * 
 * @param doc The parsed document to test
 * 
 * This test verifies that:
 * - The document has the correct number of metadata entries
 * - Metadata keys and values are correctly parsed
 * - Metadata can be retrieved using get_metadata()
 */
void test_metadata(Document *doc) {
    assert(doc != NULL);
    assert(doc->metadata_count == 2);
    
    const char *title = get_metadata(doc, "title");
    const char *author = get_metadata(doc, "author");
    
    assert(strcmp(title, "Test Document") == 0);
    assert(strcmp(author, "John Doe") == 0);
    
    printf("Metadata test passed\n");
}

/**
 * @brief Test AST structure and content
 * 
 * @param doc The parsed document to test
 * 
 * This test verifies that:
 * - The document has a valid root node
 * - All expected nodes are present in the correct order
 * - Node types and content are correct
 * - The AST structure matches the input document
 */
void test_ast_structure(Document *doc) {
    assert(doc != NULL);
    assert(doc->root != NULL);
    assert(doc->root->type == NODE_DOCUMENT);
    
    // Check first child is metadata
    assert(doc->root->children[0]->type == NODE_METADATA);
    
    // Check heading
    Node *heading = doc->root->children[1];
    assert(heading->type == NODE_HEADING);
    assert(strcmp(heading->content, "Main Heading") == 0);
    
    // Check paragraph
    Node *paragraph = doc->root->children[2];
    assert(paragraph->type == NODE_PARAGRAPH);
    assert(strstr(paragraph->content, "This is a paragraph") != NULL);
    
    // Check component
    Node *component = doc->root->children[3];
    assert(component->type == NODE_COMPONENT);
    assert(strcmp(component->content, "diagram") == 0);
    
    // Check annotation
    Node *annotation = doc->root->children[4];
    assert(annotation->type == NODE_ANNOTATION);
    assert(strcmp(annotation->content, "important") == 0);
    
    // Check comment
    Node *comment = doc->root->children[5];
    assert(comment->type == NODE_COMMENT);
    assert(strstr(comment->content, "This is a comment block") != NULL);
    
    printf("AST structure test passed\n");
}

/**
 * @brief Test error handling functionality
 * 
 * This test verifies that:
 * - NULL input is handled gracefully
 * - Empty input produces a valid empty document
 * - Error codes are set appropriately
 */
void test_error_handling(void) {
    // Test NULL input
    Document *doc = parse_metamark(NULL);
    assert(doc == NULL);
    assert(get_last_error() == MM_ERROR_MEMORY);
    
    // Test empty input
    doc = parse_metamark("");
    assert(doc != NULL);
    assert(doc->root != NULL);
    assert(doc->root->child_count == 0);
    
    free_document(doc);
    printf("Error handling test passed\n");
}

/**
 * @brief Main test entry point
 * 
 * This function:
 * 1. Parses the test document
 * 2. Runs all test cases
 * 3. Prints the AST structure for visual inspection
 * 4. Cleans up resources
 */
int main(void) {
    printf("Running MetaMark parser tests...\n\n");
    
    // Parse test document
    Document *doc = parse_metamark(test_doc);
    assert(doc != NULL);
    
    // Run tests
    test_metadata(doc);
    test_ast_structure(doc);
    test_error_handling();
    
    // Print AST for visual inspection
    printf("\nAST Structure:\n");
    print_ast(doc->root, 0);
    
    // Cleanup
    free_document(doc);
    
    printf("\nAll tests passed!\n");
    return 0;
} 