import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
  SelectValue,
} from "@/components/ui/select";

export default function QueryFilter({
  onFilterChange,
}: {
  onFilterChange?: (filters: any) => void;
}) {
  const [open, setOpen] = useState(false);
  const [filters, setFilters] = useState({
    event_name: "",
    url: "",
    http_path: "",
    hostname: "",
    severity: "",
  });

  const handleChange = (name: string, value: string) => {
    setFilters((prev) => ({ ...prev, [name]: value }));
  };

  const handleSubmit = () => {
    onFilterChange?.(filters);
  };

  const handleReset = () => {
    const cleared = { event_name: "", url: "", http_path: "", hostname: "", severity: "" };
    setFilters(cleared);
    onFilterChange?.(cleared);
  };

  return (
    <div className="flex flex-col rounded border shadow-sm bg-white">
      {/* Header */}
      <div
        className="flex justify-between items-center px-3 py-2 border-b cursor-pointer hover:bg-gray-50"
        onClick={() => setOpen((v) => !v)}
      >
        <div className="text-sm text-gray-600 flex flex-row gap-2">
          <div className="font-bold">Filters</div>
          {Object.entries(filters)
            .filter(([key, value]) => value)
            .map(([key, value]) => (
              <div className="flex flex-row gap-1">
                <span className="italic">{key}</span> <span>:</span>{" "}
                <span>{value}</span>
              </div>
            ))}
        </div>
        <div className="text-xs text-blue-600">{open ? "Hide" : "Show"}</div>
      </div>

      {/* Body */}
      {open && (
        <div className="p-4 flex flex-col gap-4">
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
            <Input
              placeholder="Event Name"
              value={filters.event_name}
              onChange={(e) => handleChange("event_name", e.target.value)}
            />
            <Input
              placeholder="URL"
              value={filters.url}
              onChange={(e) => handleChange("url", e.target.value)}
            />
            <Input
              placeholder="PATH"
              value={filters.http_path}
              onChange={(e) => handleChange("http_path", e.target.value)}
            />
            <Input
              placeholder="Host"
              value={filters.hostname}
              onChange={(e) => handleChange("hostname", e.target.value)}
            />
            <Select
              value={filters.severity}
              onValueChange={(v) => handleChange("severity", v)}
            >
              <SelectTrigger>
                <SelectValue placeholder="Severity" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="INFO">INFO</SelectItem>
                <SelectItem value="WARN">WARN</SelectItem>
                <SelectItem value="ERROR">ERROR</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div className="flex justify-end gap-2">
            <Button variant="ghost" onClick={handleReset}>
              Reset
            </Button>
            <Button onClick={handleSubmit}>Apply</Button>
          </div>
        </div>
      )}
    </div>
  );
}
