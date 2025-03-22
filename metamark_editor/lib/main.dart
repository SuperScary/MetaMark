import 'package:flutter/material.dart';
import 'package:desktop_window/desktop_window.dart';
import 'dart:io';
import 'package:provider/provider.dart';
import 'editor/editor.dart';
import 'preview/preview.dart';
import 'theme/app_theme.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  if (Platform.isWindows || Platform.isLinux || Platform.isMacOS) {
    DesktopWindow.setMinWindowSize(const Size(1200, 800));
  }
  runApp(const MetaMarkEditor());
}

class MetaMarkEditor extends StatelessWidget {
  const MetaMarkEditor({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'MetaMark Editor',
      theme: AppTheme.lightTheme,
      darkTheme: AppTheme.darkTheme,
      home: const EditorScreen(),
    );
  }
}

class EditorScreen extends StatefulWidget {
  const EditorScreen({super.key});

  @override
  State<EditorScreen> createState() => _EditorScreenState();
}

class _EditorScreenState extends State<EditorScreen> {
  String _content = '';

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Row(
        children: [
          // Editor pane (left side)
          Expanded(
            flex: 1,
            child: EditorPane(
              onContentChanged: (content) {
                setState(() {
                  _content = content;
                });
              },
            ),
          ),
          // Preview pane (right side)
          Expanded(
            flex: 1,
            child: PreviewPane(
              content: _content,
            ),
          ),
        ],
      ),
    );
  }
}
