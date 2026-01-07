import api, { useApi } from "@/lib/api";
import Utils from "@/lib/utils";
import { useEffect, useState } from "react";

export default function EventsSummary() {
  const [rg, setRg] = useState("30m");
  const query = useApi(
    (params: any) => api.get("/events/summary", { params }),
    {
      prefrech: false,
    },
  );
  useEffect(() => {
    try {
      const dt = Utils.parseRelativeTime(rg);
      query.load({
        from: dt.toISO(),
      });
    } catch (e) {}
  }, [rg]);
  return (
    <div className="flex flex-col p-2 gap-2 box !border-green-400">
      <div className="flex flex-row gap-2 py-2">
        {time_rangs.map((t) => (
          <div
            onClick={() => setRg(t.value)}
            className={`rounded bg-blue-50 text-gray-500 font-bold px-2 text-xs cursor-pointer hover:bg-blue-400 hover:text-white ${t.value == rg ? "bg-blue-500 text-white" : ""}`}
          >
            {t.label}
          </div>
        ))}
      </div>
      {query.data?.map((section: any) => (
        <div key={section.label} className="flex flex-col gap-2">
          {/* Section label */}
          <div className="text-gray-500 text-xs uppercase border-b border-blue-200 py-2 font-bold">
            {section.label}
          </div>

          {/* Items */}
          <div className="flex flex-row flex-wrap gap-2">
            {section.items.map((item: any) => (
              <div
                key={item.label}
                className={`flex justify-between gap-2 items-center px-2 py-1 rounded border border-blue-200 text-gray-600  hover:bg-blue-800 hover:text-white`}
              >
                <span className="font-medium text-xs">{item.label}</span>
                <span className="px-2 py-0.5 rounded text-xs font-semibold">
                  {item.count}
                </span>
              </div>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}

const time_rangs = [
  { label: "5m", value: "5m" },
  { label: "10m", value: "10m" },
  { label: "30m", value: "30m" },
  { label: "1h", value: "1h" },
  { label: "3h", value: "3h" },
  { label: "12h", value: "12h" },
  { label: "1d", value: "1d" },
  { label: "1w", value: "1w" },
  { label: "1M", value: "1M" },
];
