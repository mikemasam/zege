import { useMemo } from "react";

interface TileProps<T> {
  data: T[]; // Array of row data
  className?: string; // Optional Tailwind classes for wrapper
}
export function TileView<T extends Record<string, unknown>>({
  data,
  className = "",
}: TileProps<T>) {
  const tileData = useMemo(() => {
    if (!Array.isArray(data) || !data?.length) return [];
    return data;
  }, [data]);
  return (
    <div className="flex flex-row flex-wrap gap-4 py-2">
      {tileData.map((row: any, index) => (
        <div
          key={index}
          className="rounded-lg border p-4 shadow hover:shadow-md transition-shadow bg-white flex flex-row items-center justify-between gap-4"
        >
          <div className="text-sm text-gray-500">{row.label}</div>
          <div className="font-semibold underline text-right">{row.value}</div>
        </div>
      ))}
    </div>
  );
}
