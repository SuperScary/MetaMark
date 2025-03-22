import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

// Define the Node structure that matches the C struct
class Node extends Struct {
  external Pointer<Utf8> type;
  external Pointer<Utf8> content;
  external Pointer<Node> next;
  external Pointer<Node> child;
}

// Define the library name based on the platform
String _getLibraryName() {
  if (Platform.isWindows) {
    return 'metamark.dll';
  } else if (Platform.isMacOS) {
    return 'libmetamark.dylib';
  } else {
    return 'libmetamark.so';
  }
}

class MetaMarkFFI {
  static final DynamicLibrary _lib = DynamicLibrary.open(_getLibraryName());
  
  // Function signatures
  static final _parseMetamark = _lib.lookupFunction<
    Pointer<Node> Function(Pointer<Utf8>),
    Pointer<Node> Function(Pointer<Utf8>)
  >('parse_metamark');

  static final _renderMetamarkHtml = _lib.lookupFunction<
    Pointer<Utf8> Function(Pointer<Node>),
    Pointer<Utf8> Function(Pointer<Node>)
  >('render_metamark_html');

  static final _freeAst = _lib.lookupFunction<
    Void Function(Pointer<Node>),
    void Function(Pointer<Node>)
  >('free_ast');

  /// Parses a MetaMark string and returns a pointer to the AST
  static Pointer<Node> parseMetamark(String input) {
    final inputPtr = input.toNativeUtf8();
    try {
      return _parseMetamark(inputPtr);
    } finally {
      calloc.free(inputPtr);
    }
  }

  /// Renders the AST to HTML
  static String renderMetamarkHtml(Pointer<Node> ast) {
    final resultPtr = _renderMetamarkHtml(ast);
    try {
      return resultPtr.toDartString();
    } finally {
      calloc.free(resultPtr);
    }
  }

  /// Frees the AST memory
  static void freeAst(Pointer<Node> ast) {
    _freeAst(ast);
  }

  /// Convenience method to parse and render in one step
  static String parseAndRender(String input) {
    final ast = parseMetamark(input);
    try {
      return renderMetamarkHtml(ast);
    } finally {
      freeAst(ast);
    }
  }
} 