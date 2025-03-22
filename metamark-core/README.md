# MetaMark Core

A C-based core library for parsing and processing MetaMark documents. This library provides the foundation for the MetaMark markup system, handling the parsing of `.mmk` files and exposing a clean API for downstream tools.

## Features

- Lexical analysis of MetaMark documents
- Abstract Syntax Tree (AST) construction
- YAML-style frontmatter metadata parsing
- Support for extended MetaMark blocks:
  - Collapsible sections
  - Annotations
  - Diagrams
  - Math expressions
  - Secure blocks
- Clean C API for embedding in other tools

## Building

### Prerequisites

- GCC or compatible C compiler
- Make

### Build Commands

```bash
# Build the library
make

# Run tests
make test

# Clean build artifacts
make clean
```

The library will be built as `lib/libmetamark.a`.

## API Usage

```c
#include <metamark.h>

// Parse a MetaMark document
Document* doc = parse_metamark(input_text);

// Access metadata
const char* title = get_metadata(doc, "title");

// Print the AST
print_ast(doc->root, 0);

// Clean up
free_document(doc);
```

## Project Structure

```
metamark-core/
├── include/
│   └── metamark.h      # Public API header
├── src/
│   ├── lexer.c         # Tokenization
│   ├── parser.c        # AST construction
│   ├── ast.c          # AST manipulation
│   ├── metadata.c     # Frontmatter parsing
│   └── utils.c        # Utility functions
├── tests/
│   └── test_parser.c  # Test suite
├── Makefile
└── README.md
```

## License

MIT License - see LICENSE file for details 