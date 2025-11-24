import React, { useMemo, useState } from "react";

interface JsonViewerProps {
  data: any;
}

const JsonArray = ({
  itemKey,
  arr,
  indent,
}: {
  itemKey: any;
  arr: any[];
  indent: any;
}): any => {
  const [open, setOpen] = useState<boolean>(false);
  if (arr.length === 0) return <span>[]</span>;
  if (!open) {
    return (
      <span
        onClick={() => setOpen((v) => !v)}
        className="bg-gray-200 rounded px-1 hover:bg-gray-300 hover:cursor-pointer"
      >
        <span>
          {"▶"} {"[]"}
        </span>
      </span>
    );
  }
  return (
    <span className="overflow-x-hidden">
      <span
        onClick={() => setOpen((v) => !v)}
        className="bg-gray-200 rounded px-1 hover:bg-gray-300 hover:cursor-pointer"
      >
        {"▼"}
      </span>
      [
      {arr.map((item, idx) => {
        const out = renderValue(null, item, indent + 1);
        return (
          <div key={idx} className="ml-4 overflow-x-hidden">
            {out}
            {idx < arr.length - 1 ? "," : ""}
          </div>
        );
      })}
      ]
    </span>
  );
};
const renderValue = (
  key: string | null,
  value: any,
  indent: number = 0,
): any => {
  const type = typeof value;
  switch (type) {
    case "string":
      return <span className="text-gray-700 px-1 text-wrap whitespace-pre-wrap break-words">"{value}"</span>;
    case "number":
      return <span className="text-blue-500 text-wrap whitespace-pre">{value}</span>;
    case "boolean":
      return <span className="text-purple-500">{value.toString()}</span>;
    case "object":
      if (value === null) return <span className="text-gray-500">null</span>;
      if (Array.isArray(value))
        return (
          <JsonArray itemKey={key} arr={value} indent={indent}></JsonArray>
        );
      return (
        <JsonObject itemKey={key} obj={value} indent={indent}></JsonObject>
      );
    default:
      return <span>{value}</span>;
  }
};
const JsonObject = ({ itemKey, obj, indent }: any): any => {
  const [open, setOpen] = useState<boolean>(itemKey == "data");
  const entries = useMemo(() => {
    let l = Object.entries(obj);
    l = l.filter(([_, value]) => value);
    l = l.sort((a, b) => {
      if (a[0] === "data") return -1;
      if (b[0] === "data") return 1;
      return a[0].localeCompare(b[0] as string);
    });
    return l;
  }, [obj]);
  if (!entries.length) return null;
  if (!open) {
    return (
      <span
        onClick={() => setOpen((v) => !v)}
        className="bg-gray-200 rounded px-1 hover:bg-gray-300 hover:cursor-pointer"
      >
        <span>
          {"▶"} {"{}"}
        </span>
      </span>
    );
  }
  return (
    <span>
      <span
        onClick={() => setOpen((v) => !v)}
        className="bg-gray-200 rounded px-1 hover:bg-gray-300 hover:cursor-pointer"
      >
        {"▼"}
      </span>
      {"{"}
      {entries.map(([key, value], idx) => {
        const out = renderValue(key, value, indent + 1);
        if (out === null) return null;
        return (
          <div key={key} className="ml-4 overflow-x-hidden">
            <span className="text-lime-700">"{key}"</span>: {out}
            {idx < entries.length - 1 ? "," : ""}
          </div>
        );
      })}
      {"}"}
    </span>
  );
};
const JsonViewer: React.FC<JsonViewerProps> = ({ data }) => {
  return (
    <pre className="bg-gray-100 p-2 rounded border border-gray-200 overflow-x-hidden text-xs">
      {renderValue(null, data)}
    </pre>
  );
};

export default JsonViewer;
