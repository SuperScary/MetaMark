#ifndef TEST_FRAMEWORK_H
#define TEST_FRAMEWORK_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// Test statistics
extern size_t total_tests;
extern size_t passed_tests;
extern size_t failed_tests;

// Test result structure
typedef struct {
    const char *name;
    bool passed;
    const char *message;
} TestResult;

// Test function type
typedef TestResult (*TestFunction)(void);

// Test suite structure
typedef struct {
    const char *name;
    TestFunction *tests;
    size_t test_count;
} TestSuite;

// Test utilities
#define ASSERT(condition, message) \
    if (!(condition)) { \
        return (TestResult){__func__, false, message}; \
    }

#define TEST_PASS() \
    return (TestResult){__func__, true, NULL}

// Test runner functions
void run_test(TestFunction test);
void run_test_suite(TestSuite suite);
void print_test_summary(void);

// Test file utilities
char* read_test_file(const char *filename);
bool write_test_file(const char *filename, const char *content);
bool create_test_directory(const char *dirname);
bool remove_test_directory(const char *dirname);

// Test data
extern const char *SAMPLE_MMK_CONTENT;
extern const char *SAMPLE_MMK_SIGNED_CONTENT;
extern const char *SAMPLE_PRIVATE_KEY;
extern const char *SAMPLE_PUBLIC_KEY;

#endif // TEST_FRAMEWORK_H 