#include "test_framework.h"
#include <sys/stat.h>
#include <errno.h>
#ifdef _WIN32
#include <direct.h>
#endif

// Test statistics
size_t total_tests = 0;
size_t passed_tests = 0;
size_t failed_tests = 0;

// Test data
const char *SAMPLE_MMK_CONTENT = 
    "# Title\n"
    "This is a sample MetaMark document.\n"
    "\n"
    "## Section 1\n"
    "Some content here.\n"
    "\n"
    "## Section 2\n"
    "More content here.\n";

const char *SAMPLE_MMK_SIGNED_CONTENT = 
    "-----BEGIN META MARK-----\n"
    "# Title\n"
    "This is a sample signed MetaMark document.\n"
    "-----END META MARK-----\n"
    "-----BEGIN SIGNATURE-----\n"
    "Sample signature data\n"
    "-----END SIGNATURE-----\n";

const char *SAMPLE_PRIVATE_KEY = 
    "-----BEGIN PRIVATE KEY-----\n"
    "Sample private key data\n"
    "-----END PRIVATE KEY-----\n";

const char *SAMPLE_PUBLIC_KEY = 
    "-----BEGIN PUBLIC KEY-----\n"
    "Sample public key data\n"
    "-----END PUBLIC KEY-----\n";

void run_test(TestFunction test) {
    total_tests++;
    TestResult result = test();
    printf("Running test: %s\n", result.name);
    
    if (result.passed) {
        passed_tests++;
        printf("✓ PASSED: %s\n", result.name);
    } else {
        failed_tests++;
        printf("✗ FAILED: %s - %s\n", result.name, result.message);
    }
}

void run_test_suite(TestSuite suite) {
    printf("\nRunning test suite: %s\n", suite.name);
    printf("================================\n");
    
    for (size_t i = 0; i < suite.test_count; i++) {
        run_test(suite.tests[i]);
    }
    
    printf("================================\n");
}

void print_test_summary(void) {
    printf("\nTest Summary:\n");
    printf("Total tests: %zu\n", total_tests);
    printf("Passed: %zu\n", passed_tests);
    printf("Failed: %zu\n", failed_tests);
    printf("Success rate: %.1f%%\n", 
           total_tests > 0 ? (float)passed_tests / total_tests * 100 : 0.0f);
}

char* read_test_file(const char *filename) {
    FILE *file = fopen(filename, "rb");
    if (!file) {
        return NULL;
    }

    fseek(file, 0, SEEK_END);
    long size = ftell(file);
    fseek(file, 0, SEEK_SET);

    char *content = malloc(size + 1);
    if (!content) {
        fclose(file);
        return NULL;
    }

    size_t read = fread(content, 1, size, file);
    fclose(file);

    if (read != (size_t)size) {
        free(content);
        return NULL;
    }

    content[size] = '\0';
    return content;
}

bool write_test_file(const char *filename, const char *content) {
    FILE *file = fopen(filename, "wb");
    if (!file) {
        return false;
    }

    size_t written = fwrite(content, 1, strlen(content), file);
    fclose(file);

    return written == strlen(content);
}

bool create_test_directory(const char *dirname) {
#ifdef _WIN32
    return _mkdir(dirname) == 0 || errno == EEXIST;
#else
    return mkdir(dirname, 0755) == 0 || errno == EEXIST;
#endif
}

bool remove_test_directory(const char *dirname) {
#ifdef _WIN32
    return _rmdir(dirname) == 0;
#else
    return rmdir(dirname) == 0;
#endif
} 