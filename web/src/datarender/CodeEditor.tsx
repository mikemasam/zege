import { Editor } from "@monaco-editor/react";
import { useRef } from "react";

export default function CodeEditor() {
    const editorRef = useRef(null);
  return (
    <div className="border border-gray-500 m-2 h-[450px] py-2">
      <Editor
        defaultLanguage="sql"
        defaultValue="SELECT * FROM users;"
        theme="vs-dark"
        options={{
          //minimap: { enabled: false },
          lineNumbers: "off",
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
}
