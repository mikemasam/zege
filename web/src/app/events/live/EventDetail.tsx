import { useMemo, useState } from "react";
import JsonViewer from "./JsonViewer";

type Event = {
  timestamp: string | number;
  event_name: string;
  message: string;
};

export function EventDetail({ event }: { event: Event }) {
  if (!event) return null;
  return (
    <div className="flex flex-col gap-2 p-3 box">
      <div className="flex flex-row items-center justify-between gap-2">
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
      <JsonViewer data={event} />
    </div>
  );
}
