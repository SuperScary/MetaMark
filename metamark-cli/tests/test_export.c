#include "test_framework.h"
#include "../include/cli.h"

TestResult test_export_pdf(void) {
    // Create test file
    ASSERT(write_test_file("test.mmk", SAMPLE_MMK_CONTENT),
           "Failed to create test file");

    // Test export command
    char *argv[] = {"mmk", "export", "--format", "pdf"};
    int result = handle_export(4, argv);

    // Clean up
    remove("test.mmk");
    remove("test.pdf");

    ASSERT(result == 0, "PDF export failed");
    TEST_PASS();
}

TestResult test_export_html(void) {
    // Create test file
    ASSERT(write_test_file("test.mmk", SAMPLE_MMK_CONTENT),
           "Failed to create test file");

    // Test export command
    char *argv[] = {"mmk", "export", "--format", "html"};
    int result = handle_export(4, argv);

    // Clean up
    remove("test.mmk");
    remove("test.html");

    ASSERT(result == 0, "HTML export failed");
    TEST_PASS();
}

TestResult test_export_json(void) {
    // Create test file
    ASSERT(write_test_file("test.mmk", SAMPLE_MMK_CONTENT),
           "Failed to create test file");

    // Test export command
    char *argv[] = {"mmk", "export", "--format", "json"};
    int result = handle_export(4, argv);

    // Clean up
    remove("test.mmk");
    remove("test.json");

    ASSERT(result == 0, "JSON export failed");
    TEST_PASS();
}

TestResult test_export_invalid_format(void) {
    // Test with invalid format
    char *argv[] = {"mmk", "export", "--format", "invalid"};
    int result = handle_export(4, argv);

    ASSERT(result == 1, "Export command should fail with invalid format");
    TEST_PASS();
}

TestResult test_export_missing_format(void) {
    // Test with missing format argument
    char *argv[] = {"mmk", "export", "--format"};
    int result = handle_export(3, argv);

    ASSERT(result == 1, "Export command should fail with missing format");
    TEST_PASS();
}

// Test suite definition
TestFunction export_tests[] = {
    test_export_pdf,
    test_export_html,
    test_export_json,
    test_export_invalid_format,
    test_export_missing_format,
    NULL
};

TestSuite export_suite = {
    .name = "Export Command Tests",
    .tests = export_tests,
    .test_count = 5
}; 