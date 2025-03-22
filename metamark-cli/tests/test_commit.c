#include "test_framework.h"
#include "../include/cli.h"

TestResult test_commit_valid_message(void) {
    // Create test directory and file
    ASSERT(create_test_directory("test_repo"), "Failed to create test directory");
    ASSERT(write_test_file("test_repo/test.mmk", SAMPLE_MMK_CONTENT),
           "Failed to create test file");

    // Test commit command
    char *argv[] = {"mmk", "commit", "-m", "Test commit"};
    int result = handle_commit(4, argv);

    // Clean up
    remove("test_repo/test.mmk");
    remove_test_directory("test_repo");

    ASSERT(result == 0, "Commit command failed");
    TEST_PASS();
}

TestResult test_commit_empty_message(void) {
    // Test with empty message
    char *argv[] = {"mmk", "commit", "-m", ""};
    int result = handle_commit(4, argv);

    ASSERT(result == 1, "Commit command should fail with empty message");
    TEST_PASS();
}

TestResult test_commit_missing_message(void) {
    // Test with missing message argument
    char *argv[] = {"mmk", "commit", "-m"};
    int result = handle_commit(3, argv);

    ASSERT(result == 1, "Commit command should fail with missing message");
    TEST_PASS();
}

TestResult test_commit_invalid_args(void) {
    // Test with invalid arguments
    char *argv[] = {"mmk", "commit", "--message", "Test"};
    int result = handle_commit(4, argv);

    ASSERT(result == 1, "Commit command should fail with invalid arguments");
    TEST_PASS();
}

// Test suite definition
TestFunction commit_tests[] = {
    test_commit_valid_message,
    test_commit_empty_message,
    test_commit_missing_message,
    test_commit_invalid_args,
    NULL
};

TestSuite commit_suite = {
    .name = "Commit Command Tests",
    .tests = commit_tests,
    .test_count = 4
}; 