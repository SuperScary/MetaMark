import 'package:flutter/material.dart';
import 'package:code_text_field/code_text_field.dart';
import 'package:flutter_highlight/themes/monokai-sublime.dart';
import 'package:highlight/languages/markdown.dart';
import 'package:file_picker/file_picker.dart';
import '../utils/file_utils.dart';
import 'dart:io';
import 'package:flutter/services.dart';

class Tab {
  String title;
  String? filePath;
  final CodeController controller;
  bool hasUnsavedChanges;

  Tab({
    required this.title,
    this.filePath,
    required this.controller,
    this.hasUnsavedChanges = false,
  });
}

class EditorPane extends StatefulWidget {
  final Function(String)? onContentChanged;
  
  const EditorPane({
    super.key,
    this.onContentChanged,
  });

  @override
  State<EditorPane> createState() => _EditorPaneState();
}

class _EditorPaneState extends State<EditorPane> {
  final List<Tab> _tabs = [];
  int _currentTabIndex = 0;
  final _focusNode = FocusNode();
  bool _showFindReplace = false;
  final _findController = TextEditingController();
  final _replaceController = TextEditingController();
  int _currentMatchIndex = -1;
  List<TextEditingController> _findReplaceControllers = [];
  final ScrollController _editorScrollController = ScrollController();
  final ScrollController _lineNumberScrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    _createNewTab();
    _loadLastFile();
    _editorScrollController.addListener(_syncLineNumberScroll);
  }

  void _syncLineNumberScroll() {
    _lineNumberScrollController.jumpTo(_editorScrollController.offset);
  }

  void _createNewTab({String? filePath, String? content}) {
    final controller = CodeController(
      text: content ?? '\n',  // Initialize with newline to ensure proper layout
      language: markdown,
      params: const EditorParams(
        tabSpaces: 2,
      ),
      stringMap: {
        // MetaMark-specific elements
        r'\[\[component:.*?\]\]': const TextStyle(color: Colors.purple, fontWeight: FontWeight.bold),
        r'\[\[/component\]\]': const TextStyle(color: Colors.purple, fontWeight: FontWeight.bold),
        r'\[\[collapse:.*?\]\]': const TextStyle(color: Colors.blue, fontWeight: FontWeight.bold),
        r'\[\[/collapse\]\]': const TextStyle(color: Colors.blue, fontWeight: FontWeight.bold),
        r'\[\[secure\]\]': const TextStyle(color: Colors.red, fontWeight: FontWeight.bold),
        r'\[\[/secure\]\]': const TextStyle(color: Colors.red, fontWeight: FontWeight.bold),
        r'\[\[.*?\]\]': const TextStyle(color: Colors.purple),
        
        // Math equations
        r'\$.*?\$': const TextStyle(color: Colors.orange),
        
        // Comments
        r'%.*$': const TextStyle(color: Colors.grey),
        
        // Headers
        r'^#+\s.*$': const TextStyle(color: Colors.blue, fontWeight: FontWeight.bold),
        
        // Lists
        r'^\s*[-*+]\s.*$': const TextStyle(color: Colors.green),
        r'^\s*\d+\.\s.*$': const TextStyle(color: Colors.green),
        
        // Code blocks
        r'```.*$': const TextStyle(color: Colors.purple),
        r'`.*?`': const TextStyle(color: Colors.purple),
        
        // Emphasis
        r'\*\*.*?\*\*': const TextStyle(fontWeight: FontWeight.bold),
        r'\*.*?\*': const TextStyle(fontStyle: FontStyle.italic),
        r'__.*?__': const TextStyle(fontWeight: FontWeight.bold),
        r'_.*?_': const TextStyle(fontStyle: FontStyle.italic),
        
        // Links and images
        r'\[.*?\]\(.*?\)': const TextStyle(color: Colors.blue),
        r'!\[.*?\]\(.*?\)': const TextStyle(color: Colors.blue),
      },
    );
    controller.addListener(_onCodeChanged);
    
    setState(() {
      _tabs.add(Tab(
        title: filePath?.split(Platform.pathSeparator).last ?? 'Untitled',
        filePath: filePath,
        controller: controller,
      ));
      _currentTabIndex = _tabs.length - 1;
      
      // Force a layout update after tab creation
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (mounted) setState(() {});
      });
    });
  }

  void _onCodeChanged() {
    final currentTab = _tabs[_currentTabIndex];
    currentTab.hasUnsavedChanges = true;
    widget.onContentChanged?.call(currentTab.controller.text);
  }

  Future<void> _loadLastFile() async {
    final lastFile = await FileUtils.getLastOpenedFile();
    if (lastFile != null && await FileUtils.fileExists(lastFile)) {
      await _openFileFromPath(lastFile);
    }
  }

  Future<void> _openFileFromPath(String filePath) async {
    if (!FileUtils.isMetaMarkFile(filePath)) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Only .mmk files are supported')),
      );
      return;
    }

    try {
      final content = await FileUtils.readFile(filePath);
      _createNewTab(filePath: filePath, content: content);
      await FileUtils.saveLastOpenedFile(filePath);
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error opening file: $e')),
      );
    }
  }

  Future<bool> _closeTab(int index) async {
    final tab = _tabs[index];
    if (tab.hasUnsavedChanges) {
      final result = await showDialog<bool>(
        context: context,
        builder: (context) => AlertDialog(
          title: const Text('Unsaved Changes'),
          content: Text('Do you want to save changes to ${tab.title}?'),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context, false),
              child: const Text('Don\'t Save'),
            ),
            TextButton(
              onPressed: () => Navigator.pop(context, null),
              child: const Text('Cancel'),
            ),
            TextButton(
              onPressed: () => Navigator.pop(context, true),
              child: const Text('Save'),
            ),
          ],
        ),
      );

      if (result == null) return false;
      if (result) {
        await _saveFile();
      }
    }

    setState(() {
      tab.controller.removeListener(_onCodeChanged);
      tab.controller.dispose();
      _tabs.removeAt(index);
      if (_currentTabIndex >= _tabs.length) {
        _currentTabIndex = _tabs.length - 1;
      }
    });
    return true;
  }

  Future<void> _saveFile() async {
    final currentTab = _tabs[_currentTabIndex];
    if (currentTab.filePath == null) {
      final result = await FilePicker.platform.saveFile(
        dialogTitle: 'Save MetaMark file',
        fileName: 'untitled.mmk',
        allowedExtensions: ['mmk'],
        type: FileType.custom,
      );

      if (result != null) {
        currentTab.filePath = result;
        currentTab.title = result.split(Platform.pathSeparator).last;
      } else {
        return;
      }
    }

    try {
      await FileUtils.writeFile(currentTab.filePath!, currentTab.controller.text);
      await FileUtils.saveLastOpenedFile(currentTab.filePath!);
      setState(() {
        currentTab.hasUnsavedChanges = false;
      });
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('File saved successfully')),
      );
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error saving file: $e')),
      );
    }
  }

  void _toggleFindReplace() {
    setState(() {
      _showFindReplace = !_showFindReplace;
      if (!_showFindReplace) {
        _findController.clear();
        _replaceController.clear();
        _currentMatchIndex = -1;
      }
    });
  }

  void _findNext() {
    if (_findController.text.isEmpty) return;
    
    final text = _tabs[_currentTabIndex].controller.text;
    final searchText = _findController.text;
    final matches = <int>[];
    
    int index = 0;
    while ((index = text.toLowerCase().indexOf(searchText.toLowerCase(), index)) != -1) {
      matches.add(index);
      index += searchText.length;
    }
    
    if (matches.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('No matches found')),
      );
      return;
    }
    
    setState(() {
      _currentMatchIndex = (_currentMatchIndex + 1) % matches.length;
      _tabs[_currentTabIndex].controller.selection = TextSelection(
        baseOffset: matches[_currentMatchIndex],
        extentOffset: matches[_currentMatchIndex] + searchText.length,
      );
    });
  }

  void _replaceCurrent() {
    if (_findController.text.isEmpty || _currentMatchIndex == -1) return;
    
    final text = _tabs[_currentTabIndex].controller.text;
    final searchText = _findController.text;
    final replaceText = _replaceController.text;
    final matches = <int>[];
    
    int index = 0;
    while ((index = text.toLowerCase().indexOf(searchText.toLowerCase(), index)) != -1) {
      matches.add(index);
      index += searchText.length;
    }
    
    if (matches.isEmpty) return;
    
    final newText = text.substring(0, matches[_currentMatchIndex]) +
        replaceText +
        text.substring(matches[_currentMatchIndex] + searchText.length);
    
    _tabs[_currentTabIndex].controller.text = newText;
    _findNext();
  }

  void _replaceAll() {
    if (_findController.text.isEmpty) return;
    
    final text = _tabs[_currentTabIndex].controller.text;
    final searchText = _findController.text;
    final replaceText = _replaceController.text;
    
    final newText = text.replaceAll(searchText, replaceText);
    _tabs[_currentTabIndex].controller.text = newText;
  }

  void _switchToTab(int index) {
    setState(() {
      _currentTabIndex = index;
      // Request focus in the next frame to ensure the UI has updated
      WidgetsBinding.instance.addPostFrameCallback((_) {
        _focusNode.requestFocus();
      });
    });
  }

  String _getLineAndColumn() {
    if (_tabs.isEmpty) return '';
    
    final controller = _tabs[_currentTabIndex].controller;
    final text = controller.text;
    final position = controller.selection.baseOffset;
    
    if (position < 0) return 'Line 1, Column 1';
    
    // Count newlines before the cursor to get line number
    int line = 1;
    int lastNewline = -1;
    for (int i = 0; i < position; i++) {
      if (text[i] == '\n') {
        line++;
        lastNewline = i;
      }
    }
    
    // Column is the number of characters after the last newline
    final column = position - lastNewline;
    
    return 'Line $line, Column $column';
  }

  @override
  void dispose() {
    for (final tab in _tabs) {
      tab.controller.removeListener(_onCodeChanged);
      tab.controller.dispose();
    }
    _findController.dispose();
    _replaceController.dispose();
    _focusNode.dispose();
    _editorScrollController.dispose();
    _lineNumberScrollController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        // Tabs
        Container(
          height: 40,
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.surfaceVariant,
            border: Border(
              bottom: BorderSide(
                color: Theme.of(context).dividerColor,
              ),
            ),
          ),
          child: ListView.builder(
            scrollDirection: Axis.horizontal,
            itemCount: _tabs.length,
            itemBuilder: (context, index) {
              final tab = _tabs[index];
              final isSelected = index == _currentTabIndex;
              return GestureDetector(
                onTap: () => _switchToTab(index),
                child: Container(
                  padding: const EdgeInsets.symmetric(horizontal: 16),
                  decoration: BoxDecoration(
                    color: isSelected
                        ? Theme.of(context).colorScheme.primaryContainer
                        : Colors.transparent,
                    border: Border(
                      right: BorderSide(
                        color: Theme.of(context).dividerColor,
                      ),
                    ),
                  ),
                  child: Row(
                    children: [
                      Text(
                        tab.title + (tab.hasUnsavedChanges ? ' *' : ''),
                        style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                              color: isSelected
                                  ? Theme.of(context).colorScheme.onPrimaryContainer
                                  : null,
                            ),
                      ),
                      if (_tabs.length > 1) ...[
                        const SizedBox(width: 8),
                        IconButton(
                          icon: const Icon(Icons.close, size: 16),
                          onPressed: () => _closeTab(index),
                          padding: EdgeInsets.zero,
                          constraints: const BoxConstraints(),
                        ),
                      ],
                    ],
                  ),
                ),
              );
            },
          ),
        ),
        // Top bar with actions
        Container(
          height: 48,
          padding: const EdgeInsets.symmetric(horizontal: 16),
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.surfaceVariant,
            border: Border(
              bottom: BorderSide(
                color: Theme.of(context).dividerColor,
              ),
            ),
          ),
          child: Row(
            children: [
              IconButton(
                icon: const Icon(Icons.add),
                onPressed: () => _createNewTab(),
              ),
              IconButton(
                icon: const Icon(Icons.folder_open),
                onPressed: _openFile,
              ),
              IconButton(
                icon: const Icon(Icons.save),
                onPressed: _saveFile,
              ),
              IconButton(
                icon: const Icon(Icons.find_replace),
                onPressed: _toggleFindReplace,
              ),
            ],
          ),
        ),
        // Find/Replace bar
        if (_showFindReplace)
          Container(
            height: 48,
            padding: const EdgeInsets.symmetric(horizontal: 16),
            decoration: BoxDecoration(
              color: Theme.of(context).colorScheme.surfaceVariant,
              border: Border(
                bottom: BorderSide(
                  color: Theme.of(context).dividerColor,
                ),
              ),
            ),
            child: Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: _findController,
                    decoration: const InputDecoration(
                      hintText: 'Find',
                      border: InputBorder.none,
                    ),
                    onSubmitted: (_) => _findNext(),
                  ),
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: TextField(
                    controller: _replaceController,
                    decoration: const InputDecoration(
                      hintText: 'Replace',
                      border: InputBorder.none,
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                IconButton(
                  icon: const Icon(Icons.search),
                  onPressed: _findNext,
                ),
                IconButton(
                  icon: const Icon(Icons.find_replace),
                  onPressed: _replaceCurrent,
                ),
                IconButton(
                  icon: const Icon(Icons.all_inclusive),
                  onPressed: _replaceAll,
                ),
              ],
            ),
          ),
        // Code editor
        Expanded(
          child: Container(
            color: const Color(0xFF272822), // Monokai background color
            child: _tabs.isEmpty
                ? const Center(child: Text('No tabs open'))
                : Focus(
                    focusNode: _focusNode,
                    child: ScrollConfiguration(
                      behavior: ScrollConfiguration.of(context).copyWith(scrollbars: false),
                      child: Row(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Container(
                            width: 48,
                            color: const Color(0xFF272822),
                            child: SingleChildScrollView(
                              controller: _lineNumberScrollController,
                              physics: const NeverScrollableScrollPhysics(),
                              child: _buildLineNumbers(),
                            ),
                          ),
                          Expanded(
                            child: SingleChildScrollView(
                              controller: _editorScrollController,
                              child: CodeField(
                                controller: _tabs[_currentTabIndex].controller,
                                textStyle: const TextStyle(
                                  fontFamily: 'monospace',
                                  fontSize: 14,
                                  color: Colors.white,
                                  height: 1.5,
                                ),
                                background: const Color(0xFF272822),
                                wrap: false,
                                minLines: 1,
                                maxLines: null,
                                enabled: true,
                                horizontalScroll: true,
                                expands: false,
                                lineNumbers: false,
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
          ),
        ),
        // Status bar
        Container(
          height: 24,
          padding: const EdgeInsets.symmetric(horizontal: 8),
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.surfaceVariant,
            border: Border(
              top: BorderSide(
                color: Theme.of(context).dividerColor,
              ),
            ),
          ),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              if (_tabs.isNotEmpty)
                Text(
                  _getLineAndColumn(),
                  style: Theme.of(context).textTheme.bodySmall,
                ),
            ],
          ),
        ),
      ],
    );
  }

  Future<void> _openFile() async {
    final result = await FilePicker.platform.pickFiles(
      type: FileType.custom,
      allowedExtensions: ['mmk'],
    );

    if (result != null && result.files.single.path != null) {
      await _openFileFromPath(result.files.single.path!);
    }
  }

  Widget _buildLineNumbers() {
    if (_tabs.isEmpty) return Container();
    
    final text = _tabs[_currentTabIndex].controller.text;
    final lines = text.split('\n');
    final lineCount = lines.length > 0 ? lines.length : 1;
    
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: List.generate(
        lineCount,
        (index) => Container(
          height: 21,
          alignment: Alignment.centerRight,
          padding: const EdgeInsets.only(right: 8),
          child: Text(
            '${index + 1}'.padLeft(3),
            style: const TextStyle(
              color: Color(0xFF90908A),
              fontSize: 14,
              fontFamily: 'monospace',
              height: 1.5,
            ),
          ),
        ),
      ),
    );
  }
} 