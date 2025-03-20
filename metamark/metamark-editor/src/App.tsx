import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open, save } from '@tauri-apps/api/dialog';
import { listen } from '@tauri-apps/api/event';
import CodeMirror from '@uiw/react-codemirror';
import { markdown, markdownLanguage } from '@codemirror/lang-markdown';
import { languages } from '@codemirror/language-data';
import { EditorView } from '@codemirror/view';
import SplitPane from 'react-split';
import { FiFile, FiSave, FiFolder, FiUpload } from 'react-icons/fi';
import * as katex from 'katex';
import mermaid from 'mermaid';

// Initialize mermaid
mermaid.initialize({ startOnLoad: true });

// Define dark theme
const darkTheme = EditorView.theme({
  '&': {
    backgroundColor: '#1e1e1e',
    color: '#d4d4d4'
  },
  '.cm-content': {
    caretColor: '#fff'
  },
  '&.cm-focused .cm-cursor': {
    borderLeftColor: '#fff'
  },
  '.cm-gutters': {
    backgroundColor: '#1e1e1e',
    color: '#858585',
    border: 'none'
  },
  '.cm-activeLineGutter': {
    backgroundColor: '#2c313a'
  }
});

function App() {
  const [content, setContent] = useState('');
  const [preview, setPreview] = useState('');
  const [currentFile, setCurrentFile] = useState<string | null>(null);

  useEffect(() => {
    // Listen for document events
    const unsubscribe = listen('document-loaded', (event: any) => {
      setContent(event.payload as string);
    });

    return () => {
      unsubscribe.then(fn => fn());
    };
  }, []);

  useEffect(() => {
    // Update preview when content changes
    setPreview(content);

    // Render math expressions
    setTimeout(() => {
      const preview = document.getElementById('preview');
      if (preview) {
        const mathElements = preview.getElementsByClassName('math');
        Array.from(mathElements).forEach(element => {
          if (element instanceof HTMLElement) {
            const tex = element.textContent || '';
            const isDisplay = element.classList.contains('display');
            try {
              katex.render(tex, element, { displayMode: isDisplay });
            } catch (error) {
              console.error('Failed to render math:', error);
            }
          }
        });

        // Render mermaid diagrams
        const mermaidElements = preview.querySelectorAll('.mermaid') as NodeListOf<HTMLElement>;
        mermaid.init(undefined, mermaidElements);
      }
    }, 0);
  }, [content]);

  const handleNew = async () => {
    const title = prompt('Enter document title:');
    if (title) {
      try {
        const content = await invoke('new_document', { title });
        setContent(content as string);
        setCurrentFile(null);
      } catch (error) {
        console.error('Failed to create document:', error);
      }
    }
  };

  const handleOpen = async () => {
    try {
      const selected = await open({
        filters: [{
          name: 'MetaMark',
          extensions: ['mmk']
        }]
      });

      if (selected) {
        const content = await invoke('open_document', { path: selected });
        setContent(content as string);
        setCurrentFile(selected as string);
      }
    } catch (error) {
      console.error('Failed to open document:', error);
    }
  };

  const handleSave = async () => {
    try {
      if (currentFile) {
        await invoke('save_document', { content });
      } else {
        const selected = await save({
          filters: [{
            name: 'MetaMark',
            extensions: ['mmk']
          }]
        });

        if (selected) {
          await invoke('save_document', { content, path: selected });
          setCurrentFile(selected);
        }
      }
    } catch (error) {
      console.error('Failed to save document:', error);
    }
  };

  const handleExport = async () => {
    try {
      const selected = await save({
        filters: [{
          name: 'PDF',
          extensions: ['pdf']
        }, {
          name: 'HTML',
          extensions: ['html']
        }]
      });

      if (selected) {
        const format = selected.split('.').pop() || '';
        await invoke('export_document', { format, path: selected });
      }
    } catch (error) {
      console.error('Failed to export document:', error);
    }
  };

  return (
    <div className="h-screen flex flex-col bg-gray-900 text-white">
      {/* Toolbar */}
      <div className="flex items-center p-4 bg-gray-800 border-b border-gray-700">
        <button
          onClick={handleNew}
          className="p-2 hover:bg-gray-700 rounded-lg mr-2"
          title="New Document"
        >
          <FiFile />
        </button>
        <button
          onClick={handleOpen}
          className="p-2 hover:bg-gray-700 rounded-lg mr-2"
          title="Open Document"
        >
          <FiFolder />
        </button>
        <button
          onClick={handleSave}
          className="p-2 hover:bg-gray-700 rounded-lg mr-2"
          title="Save Document"
        >
          <FiSave />
        </button>
        <button
          onClick={handleExport}
          className="p-2 hover:bg-gray-700 rounded-lg"
          title="Export Document"
        >
          <FiUpload />
        </button>
        <div className="ml-4 text-gray-400">
          {currentFile || 'Untitled Document'}
        </div>
      </div>

      {/* Editor and Preview */}
      <SplitPane
        className="flex-1"
        sizes={[50, 50]}
        minSize={100}
        expandToMin={false}
        gutterSize={10}
        gutterAlign="center"
        snapOffset={30}
        dragInterval={1}
        direction="horizontal"
      >
        {/* Editor */}
        <div className="h-full overflow-auto">
          <CodeMirror
            value={content}
            height="100%"
            theme={darkTheme}
            extensions={[
              markdown({ base: markdownLanguage, codeLanguages: languages })
            ]}
            onChange={(value) => setContent(value)}
          />
        </div>

        {/* Preview */}
        <div
          id="preview"
          className="h-full overflow-auto p-4 bg-gray-800"
          dangerouslySetInnerHTML={{ __html: preview }}
        />
      </SplitPane>
    </div>
  );
}

export default App; 