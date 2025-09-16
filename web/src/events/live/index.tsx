import api from "@/lib/api";
import { useEffect, useState } from "react";
import { EventBox } from "./EventBox";
import QueryFilter from "./QueryFilter";

export default function EventsLive() {
  const live = useLiveEvents();
  return (
    <div className="flex flex-col py-4 gap-2">
      <div className="px-2 py-4">Live Events</div>
      <QueryFilter onFilterChange={live.query} />
      <div className="flex flex-col gap-2">
        {live.events.map((event: any) => (
          <EventBox key={event.id} event={event} />
        ))}
      </div>
    </div>
  );
}

function useLiveEvents() {
  const [data, setData] = useState({
    events: [],
  });

  const query = async (params: any) => {
    const res = await api.get("/events", { params });
    if (res.status != 200) return;
    if (!Array.isArray(res.data.data)) return;
    setData((d) => ({ ...d, events: res.data.data }));
  };
  useEffect(() => {
    query({});
  }, []);

  return {
    events: data.events,
    query,
  };
}
