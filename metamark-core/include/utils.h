#ifndef METAMARK_UTILS_H
#define METAMARK_UTILS_H

#include <stddef.h>
#include "metamark.h"

/**
 * @brief Set the last error code
 * 
 * @param error The error code to set
 */
void set_error(MetaMarkError error);

/**
 * @brief Get the last error code
 * 
 * @return MetaMarkError The last error code
 */
MetaMarkError get_last_error(void);

/**
 * @brief Convert an error code to a human-readable string
 * 
 * @param error The error code to convert
 * @return const char* The error message
 */
const char* error_to_string(MetaMarkError error);

/**
 * @brief Safely allocate memory
 * 
 * @param size The size to allocate
 * @return void* The allocated memory, or NULL on error
 */
void* safe_malloc(size_t size);

/**
 * @brief Safely reallocate memory
 * 
 * @param ptr The pointer to reallocate
 * @param size The new size
 * @return void* The reallocated memory, or NULL on error
 */
void* safe_realloc(void *ptr, size_t size);

/**
 * @brief Check if a string is a valid identifier
 * 
 * @param str The string to check
 * @return int 1 if valid, 0 if not
 */
int is_valid_identifier(const char *str);

/**
 * @brief Print a node for debugging
 * 
 * @param node The node to print
 * @param indent The indentation level
 */
void print_ast(const Node *node, int indent);

/**
 * @brief Read and parse a MetaMark file
 * 
 * @param filename The path to the file to read
 * @return Document* A new document structure, or NULL on error
 */
Document* read_metamark_file(const char *filename);

#endif /* METAMARK_UTILS_H */ 