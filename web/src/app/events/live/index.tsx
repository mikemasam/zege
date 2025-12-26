import api, { useApi } from "@/lib/api";
import { useState } from "react";
import { EventBox } from "./EventBox";
import QueryFilter from "./QueryFilter";
import Page from "@/components/ui/ui-page";

export default function EventsLive() {
  const query = useApi(() => api.get("/events"));
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
    <Page title="Live Events" loading={query.loading}>
      <QueryFilter onFilterChange={query.params} />
      <div className="grid grid-cols-8 gap-2">
        <div className="col-span-8 flex flex-col gap-2">
          {query.data?.map((event: any) => (
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
