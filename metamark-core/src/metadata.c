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
    char *line;
    char *str_copy = strdup(metadata_str);
    char *line_start = str_copy;
    
    while ((line = strsep(&line_start, "\n")) != NULL) {
        trim_whitespace(line);
        
        // Skip empty lines
        if (line[0] == '\0') {
            continue;
        }
        
        // Skip comments
        if (line[0] == '#') {
            continue;
        }
        
        char *key = extract_key(line);
        char *value = extract_value(line);
        
        if (key && value) {
            add_metadata(doc, key, value);
        }
        
        free(key);
        free(value);
    }
    
    free(str_copy);
} 