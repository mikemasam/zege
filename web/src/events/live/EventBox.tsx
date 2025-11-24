import { useState } from "react";
import JsonViewer from "./JsonViewer";

const severityColors: Record<string, string> = {
  INFO: "bg-blue-100 text-blue-800",
  WARN: "bg-yellow-100 text-yellow-800",
  ERROR: "bg-red-100 text-red-800",
};

function KeyValueGrid({ data }: { data: Record<string, any> }) {
  return (
    <div className="grid grid-cols-3 gap-1 text-xs">
      {Object.entries(data)
        .filter(([_, v]) => v !== null && v !== undefined && v !== "")
        .map(([k, v]) => (
          <div
            key={k}
            className="flex flex-col border rounded p-1 gap-2 tile-content"
          >
            <span className="font-semibold text-gray-500">{k}</span>
            <span className="font-mono break-all">
              {typeof v === "object" ? JSON.stringify(v) : String(v)}
            </span>
          </div>
        ))}
    </div>
  );
}

export function EventBox({ event }: { event: any }) {
  return (
    <div className="flex flex-col gap-2 border rounded p-3 bg-gray-200 transition">
      <div className="flex items-center gap-2">
        <span
          className={`px-2 py-0.5 text-xs font-semibold rounded-md ${
            severityColors[event.severity] || "bg-gray-100 text-gray-800"
          }`}
        >
          {event.severity || "UNKNOWN"}
        </span>
        <div className="font-semibold text-sm flex-1 text-gray-600">
          {event.event_name}
        </div>
        <span className="font-mono text-xs text-gray-800">
          {new Date(event.timestamp).toLocaleString()}
        </span>
      </div>

      {event.message && 
      <div className="text-sm text-gray-700 tile-content text-wrap  overflow-x-hidden break-all">{event.message}</div>}

      <div className="flex flex-row gap-2">
        <div className="flex flex-wrap gap-1 flex-1">
          {(event.tags || []).map((t: string) => (
            <span
              key={t}
              className="text-xs px-2 py-0.5 bg-gray-200 rounded-full text-gray-700"
            >
              {t}
            </span>
          ))}
          {event.labels &&
            Object.entries(event.labels).map(([k, v]: any) => (
              <span
                key={k}
                className="text-xs px-2 py-0.5 bg-gray-100 rounded-full text-gray-600"
              >
                {k}: {v}
              </span>
            ))}
        </div>
      </div>
      <JsonViewer data={event} />
    </div>
  );
}

