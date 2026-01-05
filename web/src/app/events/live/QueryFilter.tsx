import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

export default function QueryFilter({ onChange }: { onChange?: Function }) {
  const [filters, setFilters] = useState({
    search: "",
  });

  const handleChange = (name: string, value: string) => {
    setFilters((prev) => ({ ...prev, [name]: value }));
  };

  const handleSubmit = () => {
    onChange?.(filters);
  };

  console.log(filters);

  return (
    <div className="flex flex-col box">
      <div className="p-2 flex flex-row gap-4">
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
  );
}
