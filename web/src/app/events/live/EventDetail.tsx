import { useMemo, useState } from "react";
import JsonViewer from "./JsonViewer";

type Severity = "INFO" | "WARN" | "ERROR" | "UNKNOWN";

type Event = {
  timestamp: string | number;
  severity?: Severity;
  event_name: string;
  message: string;
  tags: any;
  labels: any;
};

const severityColors: Record<Severity, string> = {
  INFO: "bg-blue-100 text-blue-800",
  WARN: "bg-yellow-100 text-yellow-800",
  ERROR: "bg-red-100 text-red-800",
  UNKNOWN: "bg-gray-200 text-gray-700",
};
export function EventDetail({ event }: { event: Event }) {
  if (!event) return null;
  const severity: Severity = event.severity ?? "UNKNOWN";
  return (
    <div className="flex flex-col gap-2 p-3 box">
      <div className="flex flex-row items-center justify-between gap-2">
        <span
          className={`py-0.5 text-xs font-semibold rounded-md ${
            severityColors[severity] || "bg-gray-100 text-gray-800"
          }`}
        >
          {severity}
        </span>
        <div className="font-semibold text-sm flex-1 text-gray-600">
          {event.event_name}
        </div>
        <span className="font-mono text-xs text-gray-800">
          {new Date(event.timestamp).toLocaleString()}
        </span>
      </div>

      {event.message && (
        <div className="text-sm text-gray-700 tile-content text-wrap  overflow-x-hidden break-all">
          {event.message}
        </div>
      )}

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
