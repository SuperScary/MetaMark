#ifndef METAMARK_CLI_H
#define METAMARK_CLI_H

#include <stdint.h>
#include <stdbool.h>
#include "metamark.h"

// Command handler function type
typedef int (*CommandHandler)(int argc, char *argv[]);

// Command structure
typedef struct {
    const char *name;
    const char *description;
    CommandHandler handler;
} Command;

// Command handlers
int handle_parse(int argc, char *argv[]);
int handle_commit(int argc, char *argv[]);
int handle_diff(int argc, char *argv[]);
int handle_rollback(int argc, char *argv[]);
int handle_export(int argc, char *argv[]);
int handle_sign(int argc, char *argv[]);
int handle_verify(int argc, char *argv[]);
int handle_help(int argc, char *argv[]);

// Utility functions
void print_help(void);
void print_error(const char *message);
void print_success(const char *message);

// File operations
int read_file_content(const char *filename, char **content, size_t *size);
int write_file_content(const char *filename, const char *content, size_t size);

// Export functions
int export_to_pdf(const Node *doc, const char *output_path);
int export_to_html(const Node *doc, const char *output_path);
int export_to_json(const Node *doc, const char *output_path);

// Security functions
int sign_file(const char *file, const char *private_key_path);
int verify_signature(const char *file);

// Version control functions
int create_commit(const char *message, const char *author);
int get_commit_history(void);
int rollback_to_commit(int commit_id);

#endif // METAMARK_CLI_H 