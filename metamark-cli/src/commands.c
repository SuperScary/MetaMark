#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "../include/cli.h"

// Stub implementations of core functions
int rollback_to_commit(int commit_id) { return 0; }
int sign_file(const char *file, const char *private_key_path) { return 0; }
int verify_signature(const char *file) { return 0; }

int handle_parse(int argc, char *argv[]) {
    if (argc < 3) {
        return 1;
    }
    // Check if file exists
    FILE *file = fopen(argv[2], "r");
    if (!file) {
        return 1;
    }
    fclose(file);
    // Stub implementation for testing
    return 0;
}

int handle_commit(int argc, char *argv[]) {
    if (argc < 4 || strcmp(argv[2], "-m") != 0) {
        return 1;
    }
    // Check for empty message
    if (strlen(argv[3]) == 0) {
        return 1;
    }
    // Stub implementation for testing
    return 0;
}

int handle_diff(int argc, char *argv[]) {
    if (argc < 2) {
        print_error("Usage: mmk diff [--latest | --commit N]");
        return 1;
    }

    // TODO: Implement diff logic
    print_error("Diff functionality not implemented yet");
    return 1;
}

int handle_rollback(int argc, char *argv[]) {
    if (argc != 4 || strcmp(argv[2], "--to") != 0) {
        print_error("Usage: mmk rollback --to N");
        return 1;
    }

    int commit_id = atoi(argv[3]);
    return rollback_to_commit(commit_id);
}

int handle_export(int argc, char *argv[]) {
    if (argc < 4 || strcmp(argv[2], "--format") != 0) {
        return 1;
    }
    // Check for valid format
    const char *format = argv[3];
    if (strcmp(format, "pdf") != 0 && 
        strcmp(format, "html") != 0 && 
        strcmp(format, "json") != 0) {
        return 1;
    }
    // Stub implementation for testing
    return 0;
}

int handle_sign(int argc, char *argv[]) {
    if (argc != 4 || strcmp(argv[2], "--key") != 0) {
        print_error("Usage: mmk sign --key private.pem");
        return 1;
    }

    return sign_file(argv[1], argv[3]);
}

int handle_verify(int argc, char *argv[]) {
    if (argc != 3) {
        print_error("Usage: mmk verify <file.mmk>");
        return 1;
    }

    return verify_signature(argv[2]);
}

int handle_help(int argc, char *argv[]) {
    print_help();
    return 0;
}

void print_help(void) {
    printf("MetaMark CLI - Command Line Interface for .mmk files\n\n");
    printf("Usage: mmk <command> [options]\n\n");
    printf("Commands:\n");
    printf("  parse <file.mmk>        Parse and display the AST\n");
    printf("  commit -m \"message\"     Create a new commit\n");
    printf("  diff [--latest|--commit N] Show differences\n");
    printf("  rollback --to N         Roll back to version N\n");
    printf("  export --format [pdf|html|json] Export document\n");
    printf("  sign --key private.pem   Sign the document\n");
    printf("  verify <file.mmk>        Verify signature\n");
    printf("  help                     Show this help\n");
    printf("\nOptions:\n");
    printf("  --test                   Run in test mode\n");
}

void print_error(const char *message) {
    fprintf(stderr, "Error: %s\n", message);
}

void print_success(const char *message) {
    printf("Success: %s\n", message);
} 