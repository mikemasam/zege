import api, { useApi } from "@/lib/api";
import { useState } from "react";
import { EventBox } from "./EventBox";
import QueryFilter from "./QueryFilter";
import Page from "@/components/ui/ui-page";

export default function EventsLive() {
  const query = useApi((params: any) => api.get("/events", { params }));
  const [opts, setOpts] = useState<{ selected: null | any }>({
    selected: null,
  });
  const onSelect = (item: any) => {
    setOpts((o) => ({
      ...o,
      selected: item?.ui == o.selected?.ui ? null : item,
    }));
  };
  return (
    <Page title="Live Events" desc="Live incoming events">
      <QueryFilter onChange={query.load} />
      <div className="flex flex-col gap-2">
        {query.data?.map((event: any) => (
          <EventBox
            key={event.ui}
            isSelected={opts.selected?.ui == event.ui}
            event={event}
            onSelect={onSelect}
          />
        ))}
      </div>
    </Page>
  );
}
