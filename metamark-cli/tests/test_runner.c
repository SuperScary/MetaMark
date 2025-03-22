#include "test_framework.h"

// Forward declarations of test suites
extern TestSuite parse_suite;
extern TestSuite commit_suite;
extern TestSuite export_suite;

int main(void) {
    printf("MetaMark CLI Test Suite\n");
    printf("======================\n\n");

    // Run all test suites
    run_test_suite(parse_suite);
    run_test_suite(commit_suite);
    run_test_suite(export_suite);

    // Print final summary
    print_test_summary();

    // Return 0 if all tests passed, 1 if any failed
    return failed_tests > 0 ? 1 : 0;
} 