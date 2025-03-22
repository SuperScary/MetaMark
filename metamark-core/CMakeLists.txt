cmake_minimum_required(VERSION 3.15)
project(metamark-core C)

# Set C standard
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Add source files
set(SOURCES
    src/ast.c
    src/lexer.c
    src/metadata.c
    src/parser.c
    src/utils.c
)

# Add test files
set(TEST_SOURCES
    tests/test_parser.c
)

# Create static library
add_library(metamark-core STATIC ${SOURCES})

# Set include directories
target_include_directories(metamark-core
    PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# Create test executable
add_executable(test_metamark ${TEST_SOURCES})
target_link_libraries(test_metamark PRIVATE metamark-core)

# Set output directories
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

# Add test
add_test(NAME test_metamark COMMAND test_metamark) 