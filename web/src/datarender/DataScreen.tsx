import CodeEditor from "./CodeEditor";
import TableView from "./TableView";

export default function DataScreen() {
  return (
    <div className="flex-1 flex flex-col bg-primary">
      <div className="flex flex-row justify-end p-1 gap-2">
        <button>Run All</button>
        <button>Run</button>
      </div>
      <CodeEditor />
      <TableView />
    </div>
  );
}
