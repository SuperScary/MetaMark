# MetaMark Editor

A cross-platform desktop editor for MetaMark (.mmk) files, built with Flutter.

## Features

- Modern IDE-like interface
- Real-time preview of MetaMark content
- Syntax highlighting for .mmk files
- Support for custom blocks and components
- Math equation rendering
- Native integration with metamark-core via FFI
- Cross-platform support (Windows, macOS, Linux)

## Prerequisites

- Flutter SDK (latest stable version)
- Dart SDK (latest stable version)
- C compiler (for FFI)
- metamark-core library (built as a shared library)

## Building metamark-core

Before running the editor, you need to build the metamark-core library as a shared library:

### Windows
```bash
cd metamark-core
gcc -shared -o metamark.dll src/*.c
```

### macOS
```bash
cd metamark-core
gcc -shared -o libmetamark.dylib src/*.c
```

### Linux
```bash
cd metamark-core
gcc -shared -o libmetamark.so src/*.c
```

## Setup

1. Clone the repository:
```bash
git clone https://github.com/yourusername/metamark.git
cd metamark/metamark_editor
```

2. Install dependencies:
```bash
flutter pub get
```

3. Build the editor:
```bash
flutter build windows  # For Windows
flutter build macos   # For macOS
flutter build linux   # For Linux
```

## Running the Editor

```bash
flutter run -d windows  # For Windows
flutter run -d macos    # For macOS
flutter run -d linux    # For Linux
```

## Project Structure

```
metamark_editor/
├── lib/
│   ├── main.dart           # Application entry point
│   ├── editor/             # Editor pane implementation
│   ├── preview/            # Preview pane implementation
│   ├── parser/             # FFI bindings for metamark-core
│   ├── theme/              # Application theme
│   └── utils/              # Utility functions
├── assets/                 # Static assets
├── test/                   # Test files
└── pubspec.yaml           # Project configuration
```

## Development

The editor is built with Flutter and uses the following key packages:

- `flutter_code_editor`: Code editing functionality
- `flutter_highlight`: Syntax highlighting
- `flutter_markdown`: Markdown rendering
- `flutter_math_fork`: Math equation rendering
- `ffi`: Foreign Function Interface for C library integration
- `provider`: State management
- `shared_preferences`: Local storage
- `path_provider`: File system access

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
