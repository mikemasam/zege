import api from "@/lib/api";
import { useCallback, useEffect, useState } from "react";
import { EventBox } from "./EventBox";
import QueryFilter from "./QueryFilter";
import Page from "@/components/ui/ui-page";
import { EventDetail } from "./EventDetail";

export default function EventsLive() {
  const live = useLiveEvents();
  const [opts, setOpts] = useState<{ selected: null | any }>({
    selected: null,
  });
  const onSelect = useCallback(
    (item: any) => {
      setOpts((o) => ({
        ...o,
        selected: item?.ui == o.selected?.ui ? null : item,
      }));
    },
    [setOpts],
  );
  return (
    <Page title="Live Events">
      <QueryFilter onFilterChange={live.query} />
      <div className="grid grid-cols-8 gap-2">
        <div className="col-span-8 flex flex-col gap-2">
          {live.events.map((event: any) => (
            <EventBox
              key={event.id}
              isSelected={opts.selected?.ui == event.ui}
              event={event}
              onSelect={onSelect}
            />
          ))}
        </div>
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
    query({ per_page: 50 });
  }, []);

  return {
    events: data.events,
    query,
  };
}
