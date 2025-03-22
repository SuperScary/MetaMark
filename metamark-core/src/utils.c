/**
 * @file utils.c
 * @brief Utility functions for the MetaMark library
 * 
 * This file contains various utility functions used throughout the MetaMark library,
 * including error handling, string manipulation, file I/O, and debugging tools.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include "../include/metamark.h"
#include "../include/lexer.h"

/**
 * @brief The last error that occurred in the library
 * 
 * This static variable stores the most recent error code.
 * It is updated by various functions when errors occur.
 */
static MetaMarkError last_error = MM_ERROR_NONE;

/**
 * @brief Set the error code for the library
 * 
 * @param error The error code to set
 * 
 * This function sets the most recent error code for the library.
 */
void set_error(MetaMarkError error) {
    last_error = error;
}

/**
 * @brief Get the last error that occurred
 * 
 * @return MetaMarkError The last error code
 * 
 * This function returns the most recent error code stored in last_error.
 * It can be used to check for errors after operations that might fail.
 */
MetaMarkError get_last_error(void) {
    return last_error;
}

/**
 * @brief Convert an error code to a human-readable string
 * 
 * @param error The error code to convert
 * @return const char* A string description of the error
 * 
 * This function provides a human-readable description of each error code.
 * It is useful for error reporting and debugging.
 */
const char* error_to_string(MetaMarkError error) {
    switch (error) {
        case MM_ERROR_NONE:
            return "No error";
        case MM_ERROR_MEMORY:
            return "Memory allocation error";
        case MM_ERROR_IO:
            return "I/O error";
        case MM_ERROR_SYNTAX:
            return "Syntax error";
        case MM_ERROR_INVALID:
            return "Invalid argument";
        default:
            return "Unknown error";
    }
}

/**
 * @brief Safely duplicate a string
 * 
 * @param str The string to duplicate
 * @return char* A new copy of the string, or NULL on error
 * 
 * This function creates a new copy of the input string using malloc.
 * It updates last_error if memory allocation fails.
 */
char* str_dup(const char *str) {
    if (!str) {
        last_error = MM_ERROR_MEMORY;
        return NULL;
    }
    
    char *dup = strdup(str);
    if (!dup) {
        last_error = MM_ERROR_MEMORY;
    }
    return dup;
}

/**
 * @brief Trim whitespace from both ends of a string
 * 
 * @param str The string to trim
 * @return char* The trimmed string
 * 
 * This function modifies the input string in-place to remove leading
 * and trailing whitespace characters.
 */
char* str_trim(char *str) {
    if (!str) {
        return NULL;
    }
    
    // Trim leading spaces
    while (isspace((unsigned char)*str)) str++;
    
    if (*str == 0) {
        return str;
    }
    
    // Trim trailing spaces
    char *end = str + strlen(str) - 1;
    while (end > str && isspace((unsigned char)*end)) end--;
    
    end[1] = '\0';
    return str;
}

/**
 * @brief Read the entire contents of a file
 * 
 * @param filename The name of the file to read
 * @return char* The file contents as a string, or NULL on error
 * 
 * This function reads the entire contents of a file into memory.
 * It handles file I/O errors and memory allocation failures.
 */
char* read_file(const char *filename) {
    FILE *file = fopen(filename, "r");
    if (!file) {
        last_error = MM_ERROR_IO;
        return NULL;
    }
    
    // Get file size
    fseek(file, 0, SEEK_END);
    long size = ftell(file);
    fseek(file, 0, SEEK_SET);
    
    if (size < 0) {
        fclose(file);
        last_error = MM_ERROR_IO;
        return NULL;
    }
    
    // Allocate buffer
    char *buffer = malloc(size + 1);
    if (!buffer) {
        fclose(file);
        last_error = MM_ERROR_MEMORY;
        return NULL;
    }
    
    // Read file
    size_t read = fread(buffer, 1, size, file);
    buffer[read] = '\0';
    
    fclose(file);
    return buffer;
}

/**
 * @brief Safely allocate memory
 * 
 * @param size The number of bytes to allocate
 * @return void* The allocated memory, or NULL on error
 * 
 * This function wraps malloc and updates last_error if allocation fails.
 */
void* safe_malloc(size_t size) {
    void *ptr = malloc(size);
    if (!ptr) {
        last_error = MM_ERROR_MEMORY;
    }
    return ptr;
}

/**
 * @brief Safely reallocate memory
 * 
 * @param ptr The pointer to reallocate
 * @param size The new size in bytes
 * @return void* The reallocated memory, or NULL on error
 * 
 * This function wraps realloc and updates last_error if reallocation fails.
 */
void* safe_realloc(void *ptr, size_t size) {
    void *new_ptr = realloc(ptr, size);
    if (!new_ptr) {
        last_error = MM_ERROR_MEMORY;
    }
    return new_ptr;
}

/**
 * @brief Check if a string is a valid identifier
 * 
 * @param str The string to check
 * @return int 1 if valid, 0 if invalid
 * 
 * A valid identifier starts with a letter or underscore and contains
 * only letters, numbers, and underscores.
 */
int is_valid_identifier(const char *str) {
    if (!str || !*str) {
        return 0;
    }
    
    // First character must be a letter or underscore
    if (!isalpha((unsigned char)*str) && *str != '_') {
        return 0;
    }
    
    // Remaining characters can be letters, numbers, or underscores
    for (const char *p = str + 1; *p; p++) {
        if (!isalnum((unsigned char)*p) && *p != '_') {
            return 0;
        }
    }
    
    return 1;
}

/**
 * @brief Print token information for debugging
 * 
 * @param token The token type to print
 * @param value The token value to print
 * 
 * This function prints the type and value of a token for debugging purposes.
 */
void debug_print_token(TokenType token, const char *value) {
    printf("Token: %d, Value: %s\n", token, value ? value : "NULL");
}

/**
 * @brief Print the AST structure for debugging
 * 
 * @param node The root node to print
 * @param indent The current indentation level
 * 
 * This function recursively prints the AST structure with proper indentation.
 * It shows the type, content, and number of children for each node.
 */
void debug_print_node(const Node *node, int indent) {
    if (!node) {
        return;
    }
    
    for (int i = 0; i < indent; i++) {
        printf("  ");
    }
    
    printf("Node(type=%s, content=%s, children=%zu)\n",
           node_type_to_string(node->type),
           node->content ? node->content : "NULL",
           node->child_count);
    
    for (size_t i = 0; i < node->child_count; i++) {
        debug_print_node(node->children[i], indent + 1);
    }
}

/**
 * @brief Read and parse a MetaMark file
 * 
 * @param filename The path to the file to read
 * @return Document* A new document structure, or NULL on error
 * 
 * This function reads the contents of a file and parses it as a MetaMark document.
 * It handles file I/O errors and memory allocation failures.
 */
Document* read_metamark_file(const char *filename) {
    char *content = read_file(filename);
    if (!content) {
        return NULL;
    }
    
    Document *doc = parse_metamark(content);
    free(content);
    return doc;
} 