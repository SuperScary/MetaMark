#include <string.h>
#include <ctype.h>
#include "../include/metamark.h"

// Token types for the lexer
typedef enum {
    TOKEN_EOF,
    TOKEN_NEWLINE,
    TOKEN_HEADING,
    TOKEN_METADATA_START,
    TOKEN_METADATA_END,
    TOKEN_COMPONENT_START,
    TOKEN_COMPONENT_END,
    TOKEN_ANNOTATION_START,
    TOKEN_ANNOTATION_END,
    TOKEN_COMMENT_START,
    TOKEN_COMMENT_END,
    TOKEN_TEXT
} TokenType;

// Lexer state
typedef struct {
    const char *input;
    size_t pos;
    size_t length;
    TokenType current_token;
    char *token_value;
} Lexer;

// Initialize lexer
static void lexer_init(Lexer *lexer, const char *input) {
    lexer->input = input;
    lexer->pos = 0;
    lexer->length = strlen(input);
    lexer->token_value = NULL;
}

// Free lexer resources
static void lexer_free(Lexer *lexer) {
    if (lexer->token_value) {
        free(lexer->token_value);
    }
}

// Get next character without advancing
static char peek(Lexer *lexer) {
    if (lexer->pos >= lexer->length) {
        return '\0';
    }
    return lexer->input[lexer->pos];
}

// Get next character and advance
static char next(Lexer *lexer) {
    char c = peek(lexer);
    if (c != '\0') {
        lexer->pos++;
    }
    return c;
}

// Skip whitespace
static void skip_whitespace(Lexer *lexer) {
    while (isspace(peek(lexer)) && peek(lexer) != '\n') {
        next(lexer);
    }
}

// Read a token value
static char* read_token_value(Lexer *lexer, size_t start, size_t end) {
    size_t len = end - start;
    char *value = malloc(len + 1);
    if (value) {
        strncpy(value, lexer->input + start, len);
        value[len] = '\0';
    }
    return value;
}

// Get next token
static TokenType next_token(Lexer *lexer) {
    skip_whitespace(lexer);
    
    char c = peek(lexer);
    if (c == '\0') {
        return TOKEN_EOF;
    }

    size_t start = lexer->pos;
    
    // Handle special tokens
    if (c == '\n') {
        next(lexer);
        return TOKEN_NEWLINE;
    }
    
    if (c == '#') {
        next(lexer);
        return TOKEN_HEADING;
    }
    
    if (c == '-' && peek(lexer + 1) == '-' && peek(lexer + 2) == '-') {
        lexer->pos += 3;
        return TOKEN_METADATA_START;
    }
    
    if (c == '[' && peek(lexer + 1) == '[') {
        lexer->pos += 2;
        return TOKEN_COMPONENT_START;
    }
    
    if (c == ']' && peek(lexer + 1) == ']') {
        lexer->pos += 2;
        return TOKEN_COMPONENT_END;
    }
    
    if (c == '@' && peek(lexer + 1) == '[') {
        lexer->pos += 2;
        return TOKEN_ANNOTATION_START;
    }
    
    if (c == '%' && peek(lexer + 1) == '%') {
        lexer->pos += 2;
        return TOKEN_COMMENT_START;
    }
    
    // Read text token
    while (peek(lexer) != '\0' && !isspace(peek(lexer)) && 
           peek(lexer) != '#' && peek(lexer) != '[' && 
           peek(lexer) != '@' && peek(lexer) != '%') {
        next(lexer);
    }
    
    if (lexer->token_value) {
        free(lexer->token_value);
    }
    lexer->token_value = read_token_value(lexer, start, lexer->pos);
    
    return TOKEN_TEXT;
}

// Public API implementation
Document* parse_metamark(const char *input) {
    Lexer lexer;
    lexer_init(&lexer, input);
    
    // TODO: Implement full parsing logic using the lexer
    
    lexer_free(&lexer);
    return NULL; // Placeholder
} 