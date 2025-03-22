/**
 * @file lexer.c
 * @brief Lexer implementation for MetaMark
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include "../include/metamark.h"
#include "../include/lexer.h"

void lexer_init(Lexer *lexer, const char *input) {
    lexer->input = input;
    lexer->pos = 0;
    lexer->length = strlen(input);
    lexer->current = TOKEN_EOF;
    lexer->token_value = NULL;
}

void lexer_free(Lexer *lexer) {
    if (lexer->token_value) {
        free(lexer->token_value);
    }
}

char peek_at(const Lexer *lexer, size_t offset) {
    size_t pos = lexer->pos + offset;
    if (pos >= lexer->length) {
        return '\0';
    }
    return lexer->input[pos];
}

char peek(const Lexer *lexer) {
    return peek_at(lexer, 0);
}

char next(Lexer *lexer) {
    if (lexer->pos >= lexer->length) {
        return '\0';
    }
    return lexer->input[lexer->pos++];
}

void skip_whitespace(Lexer *lexer) {
    while (peek(lexer) != '\0' && isspace((unsigned char)peek(lexer))) {
        next(lexer);
    }
}

char* read_token_value(const Lexer *lexer, size_t start, size_t end) {
    if (start >= end || end > lexer->length) {
        return NULL;
    }
    
    size_t length = end - start;
    char *value = malloc(length + 1);
    if (!value) {
        return NULL;
    }
    
    strncpy(value, lexer->input + start, length);
    value[length] = '\0';
    return value;
}

static int is_metadata_delimiter(const Lexer *lexer) {
    return peek_at(lexer, 0) == '-' && 
           peek_at(lexer, 1) == '-' && 
           peek_at(lexer, 2) == '-';
}

TokenType next_token(Lexer *lexer) {
    if (lexer->token_value) {
        free(lexer->token_value);
        lexer->token_value = NULL;
    }
    
    skip_whitespace(lexer);
    
    char c = peek(lexer);
    if (c == '\0') {
        return TOKEN_EOF;
    }
    
    // Handle newlines
    if (c == '\n') {
        next(lexer);
        return TOKEN_NEWLINE;
    }
    
    // Handle headings
    if (c == '#') {
        next(lexer);
        return TOKEN_HEADING;
    }
    
    // Handle component blocks
    if (c == '[' && peek_at(lexer, 1) == '[') {
        lexer->pos += 2;
        return TOKEN_COMPONENT_START;
    }
    
    if (c == ']' && peek_at(lexer, 1) == ']') {
        lexer->pos += 2;
        return TOKEN_COMPONENT_END;
    }
    
    // Handle annotations
    if (c == '@' && peek_at(lexer, 1) == '[') {
        lexer->pos += 2;
        return TOKEN_ANNOTATION_START;
    }
    
    if (c == ']') {
        next(lexer);
        return TOKEN_ANNOTATION_END;
    }
    
    // Handle comments
    if (c == '%' && peek_at(lexer, 1) == '%') {
        lexer->pos += 2;
        return TOKEN_COMMENT_START;
    }
    
    // Handle metadata delimiters
    if (is_metadata_delimiter(lexer)) {
        lexer->pos += 3;
        return TOKEN_METADATA_START;
    }
    
    // Default to text token
    size_t start = lexer->pos;
    while (peek(lexer) != '\0' && !isspace((unsigned char)peek(lexer))) {
        next(lexer);
    }
    
    lexer->token_value = read_token_value(lexer, start, lexer->pos);
    return TOKEN_TEXT;
} 