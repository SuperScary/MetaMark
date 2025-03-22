# MetaMark CLI

A command-line interface for working with MetaMark (.mmk) files. This tool provides functionality for parsing, versioning, exporting, and securing MetaMark documents.

## Prerequisites

- C compiler (GCC or Clang)
- Make
- MetaMark Core library (libmetamark.a)

## Building

### Linux/macOS

```bash
make
```

### Windows (MinGW)

```bash
mingw32-make
```

The executable will be created in the `bin` directory as `mmk` (or `mmk.exe` on Windows).

## Usage

### Basic Commands

```bash
# Parse a .mmk file
mmk parse document.mmk

# Create a new commit
mmk commit -m "Initial commit"

# Show differences between versions
mmk diff --latest
mmk diff --commit 2

# Roll back to a previous version
mmk rollback --to 1

# Export to different formats
mmk export --format pdf
mmk export --format html
mmk export --format json

# Sign a document
mmk sign --key private.pem

# Verify document signature
mmk verify document.mmk

# Show help
mmk help
```

### Test Mode

Run the CLI in test mode to verify installation:

```bash
mmk --test
```

## Development

### Project Structure

```
metamark-cli/
├── src/           # Source files
├── include/       # Header files
├── obj/          # Object files (created during build)
├── bin/          # Executable output
├── Makefile      # Build configuration
└── README.md     # This file
```

### Building from Source

1. Ensure the MetaMark Core library is built and available
2. Run `make` to build the project
3. The executable will be created in the `bin` directory

### Cleaning

To clean build artifacts:

```bash
make clean
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details. 