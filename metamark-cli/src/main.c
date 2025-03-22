#include <stdio.h>
#include <string.h>
#include "../include/cli.h"

// Command registry
static const Command commands[] = {
    {"parse", "Parse and display the AST of a .mmk file", handle_parse},
    {"commit", "Create a new commit with a message", handle_commit},
    {"diff", "Show differences between versions", handle_diff},
    {"rollback", "Roll back to a previous version", handle_rollback},
    {"export", "Export document to various formats", handle_export},
    {"sign", "Sign the document cryptographically", handle_sign},
    {"verify", "Verify document signature", handle_verify},
    {"help", "Show this help message", handle_help},
    {NULL, NULL, NULL}  // End marker
};

int main(int argc, char *argv[]) {
    if (argc < 2) {
        print_help();
        return 1;
    }

    // Check for test mode
    if (strcmp(argv[1], "--test") == 0) {
        // TODO: Implement test mode
        printf("Test mode not implemented yet\n");
        return 0;
    }

    // Find and execute the command
    for (const Command *cmd = commands; cmd->name != NULL; cmd++) {
        if (strcmp(argv[1], cmd->name) == 0) {
            return cmd->handler(argc, argv);
        }
    }

    // Command not found
    print_error("Unknown command. Use 'mmk help' for usage information.");
    return 1;
} 