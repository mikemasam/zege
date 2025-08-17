import { Editor } from "@monaco-editor/react";
import { forwardRef, useImperativeHandle, useRef } from "react";

export interface CodeEditorRef {
  getCursorBlockText: () => string | undefined;
  getCurrentSelection: () => string | undefined;
}

const CodeEditor = forwardRef<CodeEditorRef>((_, ref) => {
  const editorRef = useRef<any>(null);
  const handleEditorMount = (editor: any) => (editorRef.current = editor);
  useImperativeHandle(ref, () => ({
    getCursorBlockText() {
      if (!editorRef.current) {
        console.log("editor not setup");
        return;
      }

      const model = editorRef.current.getModel();
      const position = editorRef.current.getPosition();
      if (!model || !position) {
        console.log("no model/position", model, position);
        return;
      }

      const totalLines = model.getLineCount();
      const currentLine = position.lineNumber;

      // Find start of block
      let startLine = currentLine;
      while (startLine > 1) {
        const lineText = model.getLineContent(startLine - 1).trim();
        if (lineText === "" || lineText.endsWith(";")) break;
        startLine--;
      }

      // Find end of block
      let endLine = currentLine;
      while (endLine < totalLines) {
        const lineText = model.getLineContent(endLine + 1).trim();
        if (lineText === "" || lineText.endsWith(";")) break;
        endLine++;
      }

      return model.getValueInRange({
        startLineNumber: startLine,
        startColumn: 1,
        endLineNumber: endLine,
        endColumn: model.getLineMaxColumn(endLine),
      });
    },

    getCurrentSelection() {
      if (!editorRef.current) {
        console.log("editor not setup");
        return;
      }
      const selection = editorRef.current.getSelection();
      return editorRef.current.getModel().getValueInRange(selection);
    },
  }));
  return (
    <div className="border-4 border-muted rounded-md m-2 h-full">
      <Editor
        defaultLanguage="sql"
        defaultValue="SELECT * FROM users;"
        theme="vs-light"
        onMount={handleEditorMount}
        options={{
          //minimap: { enabled: false },
          //lineNumbers: "off",
          fontSize: 16,
          scrollBeyondLastLine: false,
          cursorBlinking: "smooth",
          tabSize: 2, // Tab size
          insertSpaces: true, // Convert tabs to spaces
          renderWhitespace: "all", // Show whitespace characters
          folding: true, // Enable code folding
          quickSuggestions: true, // Live suggestions
          snippetSuggestions: "inline", // Show snippets inline
        }}
      />
    </div>
  );
});

export default CodeEditor;
