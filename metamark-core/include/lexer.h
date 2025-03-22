/**
 * @file lexer.h
 * @brief Lexer types and token definitions for MetaMark
 */

#ifndef METAMARK_LEXER_H
#define METAMARK_LEXER_H

#include <stddef.h>

/**
 * @brief Token types recognized by the lexer
 */
typedef enum {
    TOKEN_EOF,              ///< End of file
    TOKEN_TEXT,             ///< Regular text content
    TOKEN_NEWLINE,          ///< Newline character
    TOKEN_HEADING,          ///< Heading marker (#)
    TOKEN_COMPONENT_START,  ///< Component start ([[)
    TOKEN_COMPONENT_END,    ///< Component end (]])
    TOKEN_ANNOTATION_START, ///< Annotation start (@[)
    TOKEN_ANNOTATION_END,   ///< Annotation end (])
    TOKEN_COMMENT_START,    ///< Comment start (%%)
    TOKEN_COMMENT_END,      ///< Comment end (%%)
    TOKEN_METADATA_START,   ///< Metadata start (---)
    TOKEN_METADATA_END,     ///< Metadata end (---)
    TOKEN_ERROR            ///< Error token
} TokenType;

/**
 * @brief Lexer state structure
 */
typedef struct {
    const char *input;      ///< Input text
    size_t pos;            ///< Current position in input
    size_t length;         ///< Length of input text
    TokenType current;     ///< Current token type
    char *token_value;     ///< Current token value
} Lexer;

/**
 * @brief Initialize a new lexer
 * 
 * @param lexer The lexer to initialize
 * @param input The input text to tokenize
 */
void lexer_init(Lexer *lexer, const char *input);

/**
 * @brief Free lexer resources
 * 
 * @param lexer The lexer to free
 */
void lexer_free(Lexer *lexer);

/**
 * @brief Get the next token from the input
 * 
 * @param lexer The lexer instance
 * @return TokenType The type of the next token
 */
TokenType next_token(Lexer *lexer);

/**
 * @brief Peek at the next character without consuming it
 * 
 * @param lexer The lexer instance
 * @param offset The offset from the current position
 * @return char The next character, or '\0' if at end
 */
char peek_at(const Lexer *lexer, size_t offset);

/**
 * @brief Peek at the next character without consuming it
 * 
 * @param lexer The lexer instance
 * @return char The next character, or '\0' if at end
 */
char peek(const Lexer *lexer);

/**
 * @brief Advance to the next character
 * 
 * @param lexer The lexer instance
 * @return char The character that was advanced past
 */
char next(Lexer *lexer);

/**
 * @brief Skip whitespace characters
 * 
 * @param lexer The lexer instance
 */
void skip_whitespace(Lexer *lexer);

/**
 * @brief Read a token value from the input
 * 
 * @param lexer The lexer instance
 * @param start Starting position
 * @param end Ending position
 * @return char* The token value, or NULL on error
 */
char* read_token_value(const Lexer *lexer, size_t start, size_t end);

#endif // METAMARK_LEXER_H 