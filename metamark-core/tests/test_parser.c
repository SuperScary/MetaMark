/**
 * @file test_parser.c
 * @brief Test suite for the MetaMark parser
 * 
 * This file contains unit tests for the MetaMark parser implementation.
 * It tests various aspects of the parser including metadata handling,
 * AST construction, and error handling.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "../include/metamark.h"
#include "../include/utils.h"

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
void test_metadata() {
    printf("Testing metadata parsing...\n");
    
    const char *input = "---\ntitle: Test Document\nauthor: John Doe\n---\n";
    Document *doc = parse_metamark(input);
    assert(doc != NULL);
    
    Node *root = doc->root;
    assert(root != NULL);
    assert(root->type == NODE_DOCUMENT);
    assert(root->child_count == 1);
    
    Node *metadata = root->children[0];
    assert(metadata != NULL);
    assert(metadata->type == NODE_METADATA);
    assert(metadata->child_count == 2);
    
    // Check metadata content
    assert(strstr(metadata->content, "title: Test Document") != NULL);
    assert(strstr(metadata->content, "author: John Doe") != NULL);
    
    free_document(doc);
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
void test_ast_structure() {
    printf("Testing AST structure...\n");
    
    const char *input = "# Main Heading\n\n"
                       "This is a paragraph with some text.\n\n"
                       "[[diagram]]\ngraph TD\nA[Start] --> B[Process]\nB --> C[End]\n[[/diagram]]\n\n"
                       "> important: This is an important note.\n\n"
                       "%% This is a comment block %%\n"
                       "Some content here\n"
                       "%% End comment %%\n";
    
    Document *doc = parse_metamark(input);
    assert(doc != NULL);
    
    Node *root = doc->root;
    assert(root != NULL);
    assert(root->type == NODE_DOCUMENT);
    
    // Print AST structure
    printf("\nAST Structure:\n");
    print_ast(root, 0);
    printf("\nChild count: %zu\n", root->child_count);
    
    assert(root->child_count == 7);  // heading + paragraph + component + annotation + comment + paragraph + comment
    
    // Test heading
    Node *heading = root->children[0];
    assert(heading != NULL);
    assert(heading->type == NODE_HEADING);
    assert(strcmp(heading->content, "Main Heading") == 0);
    
    // Test paragraph
    Node *paragraph = root->children[1];
    assert(paragraph != NULL);
    assert(paragraph->type == NODE_PARAGRAPH);
    assert(strcmp(paragraph->content, "This is a paragraph with some text.") == 0);
    
    // Test component
    Node *component = root->children[2];
    assert(component != NULL);
    assert(component->type == NODE_COMPONENT);
    assert(strcmp(component->content, "diagram") == 0);
    assert(component->child_count == 1);
    assert(component->children[0]->type == NODE_PARAGRAPH);
    assert(strstr(component->children[0]->content, "graph TD") != NULL);
    
    // Test annotation
    Node *annotation = root->children[3];
    assert(annotation != NULL);
    assert(annotation->type == NODE_ANNOTATION);
    assert(strcmp(annotation->content, "important") == 0);
    assert(annotation->child_count == 1);
    assert(annotation->children[0]->type == NODE_PARAGRAPH);
    assert(strcmp(annotation->children[0]->content, "This is an important note.") == 0);
    
    // Test first comment
    Node *comment1 = root->children[4];
    assert(comment1 != NULL);
    assert(comment1->type == NODE_COMMENT);
    assert(strcmp(comment1->content, "This is a comment block") == 0);
    
    // Test paragraph between comments
    Node *paragraph2 = root->children[5];
    assert(paragraph2 != NULL);
    assert(paragraph2->type == NODE_PARAGRAPH);
    assert(strcmp(paragraph2->content, "Some content here") == 0);
    
    // Test second comment
    Node *comment2 = root->children[6];
    assert(comment2 != NULL);
    assert(comment2->type == NODE_COMMENT);
    assert(strcmp(comment2->content, "End comment") == 0);
    
    free_document(doc);
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
void test_error_handling() {
    printf("Testing error handling...\n");
    
    // Test NULL input
    Document *doc = parse_metamark(NULL);
    assert(doc == NULL);
    assert(get_last_error() == MM_ERROR_INVALID);
    
    // Test empty input
    doc = parse_metamark("");
    assert(doc == NULL);
    assert(get_last_error() == MM_ERROR_SYNTAX);
    
    // Test invalid metadata
    doc = parse_metamark("---\ninvalid metadata\n---\n");
    assert(doc == NULL);
    assert(get_last_error() == MM_ERROR_SYNTAX);
    
    // Test unclosed metadata
    doc = parse_metamark("---\ntitle: Test\n");
    assert(doc == NULL);
    assert(get_last_error() == MM_ERROR_SYNTAX);
    
    // Test invalid component
    doc = parse_metamark("[[invalid component\n");
    assert(doc == NULL);
    assert(get_last_error() == MM_ERROR_SYNTAX);
    
    // Test unclosed component
    doc = parse_metamark("[[diagram]]\ncontent\n");
    assert(doc == NULL);
    assert(get_last_error() == MM_ERROR_SYNTAX);
    
    // Test invalid annotation
    doc = parse_metamark("> invalid annotation\n");
    assert(doc == NULL);
    assert(get_last_error() == MM_ERROR_SYNTAX);
    
    // Test unclosed comment
    doc = parse_metamark("%% unclosed comment\n");
    assert(doc == NULL);
    assert(get_last_error() == MM_ERROR_SYNTAX);
    
    printf("Error handling test passed\n");
}

void test_complex_document() {
    printf("Testing complex document parsing...\n");
    
    const char *input = "---\ntitle: Complex Test\ndescription: A test with nested structures\n---\n\n"
                       "# Main Section\n\n"
                       "This is a paragraph with **bold** and *italic* text.\n\n"
                       "[[diagram]]\n"
                       "graph TD\n"
                       "    A[Start] --> B[Process]\n"
                       "    B --> C[Decision]\n"
                       "    C -->|Yes| D[Action 1]\n"
                       "    C -->|No| E[Action 2]\n"
                       "[[/diagram]]\n\n"
                       "> important: This is a critical note about the process.\n"
                       "> It spans multiple lines.\n\n"
                       "%% This is a detailed comment about the implementation %%\n"
                       "Some implementation details here.\n"
                       "%% End implementation comment %%\n\n"
                       "## Subsection\n\n"
                       "More content here.\n\n"
                       "[[table]]\n"
                       "| Header 1 | Header 2 |\n"
                       "|----------|----------|\n"
                       "| Cell 1   | Cell 2   |\n"
                       "[[/table]]\n\n"
                       "> warning: This is a warning about the table format.\n\n"
                       "%% Final comment %%\n";
    
    Document *doc = parse_metamark(input);
    assert(doc != NULL);
    
    Node *root = doc->root;
    assert(root != NULL);
    assert(root->type == NODE_DOCUMENT);
    
    // Verify document structure
    size_t expected_nodes = 13;  // metadata + heading + paragraph + diagram + annotation + comment + paragraph + comment + heading + paragraph + table + annotation + comment
    assert(root->child_count == expected_nodes);
    
    // Print document structure for debugging
    printf("\nComplex Document Structure:\n");
    print_ast(root, 0);
    
    // Test nested structures
    Node *diagram = root->children[3];
    assert(diagram != NULL);
    assert(diagram->type == NODE_COMPONENT);
    assert(strcmp(diagram->content, "diagram") == 0);
    assert(diagram->child_count == 1);
    assert(diagram->children[0]->type == NODE_PARAGRAPH);
    
    // Test multi-line annotation
    Node *annotation = root->children[4];
    assert(annotation != NULL);
    assert(annotation->type == NODE_ANNOTATION);
    assert(strcmp(annotation->content, "important") == 0);
    assert(annotation->child_count == 1);
    
    // Test nested headings
    Node *subheading = root->children[8];
    assert(subheading != NULL);
    assert(subheading->type == NODE_HEADING);
    assert(strcmp(subheading->content, "Subsection") == 0);
    
    // Test table component
    Node *table = root->children[10];
    assert(table != NULL);
    assert(table->type == NODE_COMPONENT);
    assert(strcmp(table->content, "table") == 0);
    assert(table->child_count == 1);
    assert(table->children[0]->type == NODE_PARAGRAPH);
    
    free_document(doc);
    printf("Complex document test passed\n");
}

void test_edge_cases() {
    printf("Testing edge cases...\n");
    
    // Test empty lines
    const char *input1 = "\n\n\n# Title\n\n\nContent\n\n\n";
    Document *doc = parse_metamark(input1);
    assert(doc != NULL);
    assert(doc->root->child_count == 2);  // heading + paragraph
    free_document(doc);
    
    // Test whitespace in metadata
    const char *input2 = "---\n  title  :  Test  \n  author  :  User  \n---\n";
    doc = parse_metamark(input2);
    assert(doc != NULL);
    Node *metadata = doc->root->children[0];
    assert(strstr(metadata->content, "title  :  Test") != NULL);
    assert(strstr(metadata->content, "author  :  User") != NULL);
    free_document(doc);
    
    // Test empty components
    const char *input3 = "[[empty]]\n[[/empty]]\n";
    doc = parse_metamark(input3);
    assert(doc != NULL);
    Node *component = doc->root->children[0];
    assert(component->type == NODE_COMPONENT);
    assert(strcmp(component->content, "empty") == 0);
    assert(component->child_count == 0);
    free_document(doc);
    
    // Test empty annotations
    const char *input4 = "> note:\n";
    doc = parse_metamark(input4);
    assert(doc != NULL);
    Node *annotation = doc->root->children[0];
    assert(annotation->type == NODE_ANNOTATION);
    assert(strcmp(annotation->content, "note") == 0);
    assert(annotation->child_count == 0);
    free_document(doc);
    
    // Test empty comments
    const char *input5 = "%% %%\n";
    doc = parse_metamark(input5);
    assert(doc != NULL);
    Node *comment = doc->root->children[0];
    assert(comment->type == NODE_COMMENT);
    assert(strcmp(comment->content, "") == 0);
    free_document(doc);
    
    printf("Edge cases test passed\n");
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
    
    test_metadata();
    test_ast_structure();
    test_error_handling();
    test_complex_document();
    test_edge_cases();
    
    printf("\nAll tests passed!\n");
    return 0;
} 