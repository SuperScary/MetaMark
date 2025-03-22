import 'package:flutter/material.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import 'package:flutter_math_fork/flutter_math.dart';
import 'package:markdown/markdown.dart' as md;

class PreviewPane extends StatelessWidget {
  final String content;
  
  const PreviewPane({
    super.key,
    this.content = '',
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        // Preview header
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
              Text(
                'Preview',
                style: Theme.of(context).textTheme.titleMedium,
              ),
            ],
          ),
        ),
        // Preview content
        Expanded(
          child: LayoutBuilder(
            builder: (context, constraints) {
              return SingleChildScrollView(
                padding: const EdgeInsets.all(16),
                child: ConstrainedBox(
                  constraints: BoxConstraints(
                    minWidth: constraints.maxWidth - 32,
                    maxWidth: constraints.maxWidth - 32,
                  ),
                  child: Markdown(
                    data: content.isEmpty ? '''
# Welcome to MetaMark Editor

This is a preview of your MetaMark content. The preview will update in real-time as you edit the file.

## Features

- **Bold** and *italic* text
- Math equations: \$E = mc^2\$
- Code blocks:
```python
def hello():
    print("Hello, World!")
```

## Custom Blocks

[[component: type="card"]]
This is a custom card component.
[[/component]]

[[collapse: title="Click to expand"]]
This is a collapsible section.
[[/collapse]]

[[secure]]
This is an encrypted block.
[[/secure]]
''' : content,
                    builders: {
                      'math': MathElementBuilder(),
                      'component': ComponentBuilder(),
                      'collapse': CollapseBuilder(),
                      'secure': SecureBlockBuilder(),
                    },
                    shrinkWrap: true,
                    selectable: true,
                  ),
                ),
              );
            },
          ),
        ),
      ],
    );
  }
}

class MathElementBuilder extends MarkdownElementBuilder {
  @override
  Widget? visitText(md.Text text, TextStyle? preferredStyle) {
    return LayoutBuilder(
      builder: (context, constraints) {
        return Math.tex(
          text.text,
          textStyle: preferredStyle,
          textScaleFactor: 1.2,
        );
      }
    );
  }
}

class ComponentBuilder extends MarkdownElementBuilder {
  @override
  Widget? visitElementAfter(md.Element element, TextStyle? preferredStyle) {
    return ConstrainedBox(
      constraints: const BoxConstraints(minWidth: double.infinity),
      child: Card(
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Text(element.textContent),
        ),
      ),
    );
  }
}

class CollapseBuilder extends MarkdownElementBuilder {
  @override
  Widget? visitElementAfter(md.Element element, TextStyle? preferredStyle) {
    return ConstrainedBox(
      constraints: const BoxConstraints(minWidth: double.infinity),
      child: ExpansionTile(
        title: Text(element.attributes['title'] ?? 'Collapsible Section'),
        children: [
          Padding(
            padding: const EdgeInsets.all(16),
            child: Text(element.textContent),
          ),
        ],
      ),
    );
  }
}

class SecureBlockBuilder extends MarkdownElementBuilder {
  @override
  Widget? visitElementAfter(md.Element element, TextStyle? preferredStyle) {
    return ConstrainedBox(
      constraints: const BoxConstraints(minWidth: double.infinity),
      child: Card(
        color: Colors.grey[200],
        child: const Padding(
          padding: EdgeInsets.all(16),
          child: Row(
            children: [
              Icon(Icons.lock),
              SizedBox(width: 8),
              Expanded(child: Text('Encrypted Content')),
            ],
          ),
        ),
      ),
    );
  }
} 