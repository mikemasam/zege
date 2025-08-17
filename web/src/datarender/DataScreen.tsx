import { useRef, useState } from "react";
import CodeEditor, { type CodeEditorRef } from "./CodeEditor";
import TableView from "./TableView";
import { execQuery, useDataStore } from "./data.state";
import React from "react";
import { Button } from "@/components/ui/button";

export default function DataScreen() {
  const codeEditorRef = useRef<CodeEditorRef>(null);

  const handleGetBlock = () => {
    const sql = codeEditorRef.current?.getCursorBlockText();
    execQuery(sql);
  };

  const handleGetSelection = () => {
    const sql = codeEditorRef.current?.getCurrentSelection();
    execQuery(sql);
  };
  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      <div className="flex flex-row justify-end p-1 gap-2">
        <Button onClick={handleGetBlock}>Run Block</Button>
        <Button onClick={handleGetSelection}>Run Selection</Button>
      </div>
      <div>
        <div className="h-[450px]">
          <CodeEditor ref={codeEditorRef} />
        </div>
        <div>
          <TableView />
        </div>
      </div>
    </div>
  );
}
