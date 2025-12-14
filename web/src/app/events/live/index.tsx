import api from "@/lib/api";
import { useEffect, useState } from "react";
import { EventBox } from "./EventBox";
import QueryFilter from "./QueryFilter";
import Page from "@/components/ui/ui-page";

export default function EventsLive() {
  const live = useLiveEvents();
  return (
    <Page title="Live Events">
      <QueryFilter onFilterChange={live.query} />
      <div className="flex flex-col gap-2">
        {live.events.map((event: any) => (
          <EventBox key={event.id} event={event} />
        ))}
      </div>
    </Page>
  );
}

function useLiveEvents() {
  const [data, setData] = useState({
    events: [],
  });

  const query = async (params: any) => {
    const res = await api.get("/events", { params });
    if (res.status != 200) return;
    if (!Array.isArray(res.data)) return;
    setData((d) => ({ ...d, events: res.data }));
  };
  useEffect(() => {
    query({});
  }, []);

  return {
    events: data.events,
    query,
  };
}
