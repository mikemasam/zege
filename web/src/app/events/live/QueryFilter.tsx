import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import EventsSummary from "./EventsSummary";

export default function QueryFilter({ onChange }: { onChange?: Function }) {
  const [filters, setFilters] = useState({
    search: "",
  });
  const [summary_opened, setSummaryOpen] = useState(false);

  const handleChange = (name: string, value: string) => {
    setFilters((prev) => ({ ...prev, [name]: value }));
  };

  const handleSubmit = () => {
    onChange?.(filters);
  };

  console.log(filters);

  return (
    <div className="flex flex-col gap-1">
      <div className="flex flex-col box !border-green-400">
        <div className="p-2 flex flex-row items-center gap-4">
          <span
            className="material-icons text-gray-500 hover:text-blue-500 cursor-pointer"
            onClick={() => setSummaryOpen((v) => !v)}
          >
            {!summary_opened ? "insights" : "arrow_circle_up"}
          </span>
          <Input
            placeholder="Search event_name:service:host"
            value={filters.search}
            onChange={(e) => handleChange("search", e.target.value)}
          />

          <div className="flex justify-end gap-2">
            <Button variant="ghost" onClick={handleSubmit}>
              Run
            </Button>
          </div>
        </div>
      </div>
      {summary_opened && <EventsSummary />}
    </div>
  );
}
