#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include "../include/metamark.h"

#define INITIAL_METADATA_CAPACITY 8

static void trim_whitespace(char *str) {
    char *end;
    
    // Trim leading spaces
    while (isspace((unsigned char)*str)) str++;
    
    if (*str == 0) return;
    
    // Trim trailing spaces
    end = str + strlen(str) - 1;
    while (end > str && isspace((unsigned char)*end)) end--;
    
    end[1] = '\0';
}

static char* extract_key(const char *line) {
    const char *colon = strchr(line, ':');
    if (!colon) {
        return NULL;
    }
    
    size_t key_len = colon - line;
    char *key = malloc(key_len + 1);
    if (!key) {
        return NULL;
    }
    
    strncpy(key, line, key_len);
    key[key_len] = '\0';
    trim_whitespace(key);
    
    return key;
}

static char* extract_value(const char *line) {
    const char *colon = strchr(line, ':');
    if (!colon) {
        return NULL;
    }
    
    const char *value_start = colon + 1;
    while (isspace((unsigned char)*value_start)) value_start++;
    
    return strdup(value_start);
}

void add_metadata(Document *doc, const char *key, const char *value) {
    if (!doc || !key || !value) {
        return;
    }
    
    if (doc->metadata_count == 0) {
        doc->metadata = malloc(sizeof(MetadataPair) * INITIAL_METADATA_CAPACITY);
        if (!doc->metadata) {
            return;
        }
        doc->metadata_count = 0;
    }
    
    if (doc->metadata_count >= INITIAL_METADATA_CAPACITY) {
        size_t new_capacity = INITIAL_METADATA_CAPACITY * 2;
        MetadataPair *new_metadata = realloc(doc->metadata, 
                                           sizeof(MetadataPair) * new_capacity);
        if (!new_metadata) {
            return;
        }
        doc->metadata = new_metadata;
    }
    
    doc->metadata[doc->metadata_count].key = strdup(key);
    doc->metadata[doc->metadata_count].value = strdup(value);
    doc->metadata_count++;
}

const char* get_metadata(const Document *doc, const char *key) {
    if (!doc || !key) {
        return NULL;
    }
    
    for (size_t i = 0; i < doc->metadata_count; i++) {
        if (strcmp(doc->metadata[i].key, key) == 0) {
            return doc->metadata[i].value;
        }
    }
    
    return NULL;
}

// Parse YAML-style metadata from a string
static void parse_metadata_string(Document *doc, const char *metadata_str) {
    if (!metadata_str) return;
    
    const char *line_start = metadata_str;
    const char *line_end;
    
    while (line_start && *line_start) {
        // Find end of line
        line_end = strchr(line_start, '\n');
        
        // Process the line
        size_t line_len = line_end ? (line_end - line_start) : strlen(line_start);
        char *line = malloc(line_len + 1);
        if (!line) break;
        
        strncpy(line, line_start, line_len);
        line[line_len] = '\0';
        
        trim_whitespace(line);
        
        // Skip empty lines and comments
        if (*line != '\0' && *line != '#') {
            char *key = extract_key(line);
            char *value = extract_value(line);
            
            if (key && value) {
                add_metadata(doc, key, value);
            }
            
            free(key);
            free(value);
        }
        
        free(line);
        
        // Move to next line
        if (line_end) {
            line_start = line_end + 1;
        } else {
            break;
        }
    }
}

// Public function to parse metadata from a node
void parse_metadata_node(Document *doc, const Node *node) {
    if (!doc || !node || node->type != NODE_METADATA || !node->content) {
        return;
    }
    
    parse_metadata_string(doc, node->content);
} 