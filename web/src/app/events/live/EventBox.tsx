import { useMemo } from "react";
import { EventDetail } from "./EventDetail";
import { DateTime } from "luxon";

type Event = {
  timestamp: string;
  event_name: string;
  service: string;
  message: string;
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
  const formattedTime = useMemo(() => {
    return DateTime.fromISO(event.timestamp).toFormat("HH:mm dd/MM/yyyy");
  }, [event.timestamp]);

  return (
    <div className="flex flex-col">
      <div
        role="button"
        tabIndex={0}
        className={`flex items-center gap-6 rounded p-2 box-selectable ${isSelected ? "bg-blue-500" : ""}`}
        onClick={() => onSelect(event)}
      >
        <span className="text-xs whitespace-nowrap font-bold">
          {formattedTime}
        </span>

        {/* Severity */}
        <span
          className={`px-2 py-0.5 text-xs font-semibold rounded-md bg-blue-100 text-blue-800`}
        >
          {event.service}
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
      {isSelected && (
        <div className="mb-4">
          <EventDetail event={event as any} />
        </div>
      )}
    </div>
  );
}
