import Editor from "@monaco-editor/react";
import { useController } from "react-hook-form";

export function SqlEditor({ name }: { name: string }) {
  const methods = useController({ name });
  return (
    <Editor
      height="400px"
      defaultLanguage="sql"
      value={methods.field.value}
      onChange={methods.field.onChange}
      theme="vs-light"
      options={{
        fontSize: 18,
        minimap: { enabled: false },
        wordWrap: "on",
        automaticLayout: true,
        padding: {
          top: 16,
          bottom: 16,
        },
      }}
    />
  );
}
