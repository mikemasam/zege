import api, { useApi } from "@/lib/api";
import { useState } from "react";
import { EventBox } from "./EventBox";
import QueryFilter from "./QueryFilter";
import Page from "@/components/ui/ui-page";
import { Button } from "@/components/ui/button";

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
      <QueryFilter
        onChange={(params: any) => query.load({ page: 0, ...params })}
      />
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

      <div className="flex flex-row justify-center py-10 gap-4">
        <Button
          onClick={() =>
            query.load({
              page: (query?.cursor?.page ?? 0) - 1,
            })
          }
        >
          Back
        </Button>
        {(query?.cursor?.page ?? 0) + 1}
        <Button
          onClick={() =>
            query.load({
              page: (query?.cursor?.page ?? 0) + 1,
            })
          }
        >
          Next
        </Button>
      </div>
    </Page>
  );
}
