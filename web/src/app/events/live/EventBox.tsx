import { useMemo } from "react";
import { EventDetail } from "./EventDetail";

type Severity = "INFO" | "WARN" | "ERROR" | "UNKNOWN";

type Event = {
  timestamp: string | number;
  severity?: Severity;
  event_name: string;
  message: string;
};

const severityColors: Record<Severity, string> = {
  INFO: "bg-blue-100 text-blue-800",
  WARN: "bg-yellow-100 text-yellow-800",
  ERROR: "bg-red-100 text-red-800",
  UNKNOWN: "bg-gray-200 text-gray-700",
};

export function EventBox({
  event,
  isSelected,
  onSelect,
}: {
  event: Event;
  isSelected: boolean;
  onSelect: Function;
}) {
  const severity: Severity = event.severity ?? "UNKNOWN";

  const formattedTime = useMemo(
    () => new Date(event.timestamp).toLocaleString(),
    [event.timestamp],
  );

  return (
    <div className="flex flex-col gap-4">
      <div
        role="button"
        tabIndex={0}
        className={`flex items-center gap-6 rounded p-2
                 hover:bg-blue-200 hover:shadow-sm
                 focus:outline-none focus:ring-2 focus:ring-blue-400
                 transition cursor-pointer ${isSelected ? "bg-blue-100" : "bg-gray-50"}`}
        onClick={() => onSelect(event)}
      >
        <span className="font-mono text-xs text-gray-700 whitespace-nowrap">
          {formattedTime}
        </span>

        {/* Severity */}
        <span
          className={`px-2 py-0.5 text-xs font-semibold rounded-md ${severityColors[severity]}`}
        >
          {severity}
        </span>

        {/* Event content */}
        <div className="flex min-w-0 items-center gap-2">
          <span className="text-sm font-semibold text-gray-600 whitespace-nowrap">
            {event.event_name}
          </span>

          <span className="text-gray-400">~</span>

          <span
            title={event.message}
            className="text-sm truncate text-gray-600"
          >
            {event.message}
          </span>
        </div>
      </div>
      {isSelected && <EventDetail event={event as any} />}
    </div>
  );
}
