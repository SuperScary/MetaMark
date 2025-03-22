#include "test_framework.h"
#include "../include/cli.h"

TestResult test_parse_valid_file(void) {
    // Create a test file
    ASSERT(write_test_file("test.mmk", SAMPLE_MMK_CONTENT), 
           "Failed to create test file");

    // Test parse command
    char *argv[] = {"mmk", "parse", "test.mmk"};
    int result = handle_parse(3, argv);
    
    // Clean up
    remove("test.mmk");
    
    ASSERT(result == 0, "Parse command failed");
    TEST_PASS();
}

TestResult test_parse_invalid_file(void) {
    // Test with non-existent file
    char *argv[] = {"mmk", "parse", "nonexistent.mmk"};
    int result = handle_parse(3, argv);
    
    ASSERT(result == 1, "Parse command should fail for invalid file");
    TEST_PASS();
}

TestResult test_parse_invalid_args(void) {
    // Test with missing file argument
    char *argv[] = {"mmk", "parse"};
    int result = handle_parse(2, argv);
    
    ASSERT(result == 1, "Parse command should fail with missing argument");
    TEST_PASS();
}

TestResult test_parse_empty_file(void) {
    // Create an empty test file
    ASSERT(write_test_file("empty.mmk", ""), 
           "Failed to create empty test file");

    // Test parse command
    char *argv[] = {"mmk", "parse", "empty.mmk"};
    int result = handle_parse(3, argv);
    
    // Clean up
    remove("empty.mmk");
    
    ASSERT(result == 0, "Parse command should handle empty file");
    TEST_PASS();
}

// Test suite definition
TestFunction parse_tests[] = {
    test_parse_valid_file,
    test_parse_invalid_file,
    test_parse_invalid_args,
    test_parse_empty_file,
    NULL
};

TestSuite parse_suite = {
    .name = "Parse Command Tests",
    .tests = parse_tests,
    .test_count = 4
}; 