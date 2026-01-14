import Page from "@/components/page";
import ReportRender from "@/components/report/report.render";
import { useReport } from "@/components/report/use.report";
import api, { useApi } from "@/lib/api";
import { useEffect, useState } from "react";

export default function AppHome() {
  const query = useApi((params: any) => api.get("/reports/pin", { params }));
  const [selected, setSelected] = useState<any>(null);
  useEffect(() => {
    const items = query.data;
    if (!selected && Array.isArray(items)) {
      setSelected(items[0]);
    }
  }, [query.data]);
  return (
    <Page title="Overview" desc="Explore data overview" className="!gap-1">
      <div className="grid grid-cols-8 gap-4 min-h-[70vh]">
        <div className="col-span-2 flex flex-col gap-4 pt-4">
          <div>
            {query.data?.map((r: any, i: number) => (
              <MenuItem
                key={i}
                item={r}
                isSelected={selected?.id == r.id}
                onSelect={setSelected}
              />
            ))}
          </div>
        </div>
        <div className="col-span-6 box !border-none !shadow-none">
          {selected ? (
            <RenderBlockContent report={selected} />
          ) : (
            <div className="text-slate-400 text-center mt-20">
              Select a report
            </div>
          )}
        </div>
      </div>
    </Page>
  );
}

function MenuItem({
  item,
  onSelect,
  isSelected,
}: {
  isSelected: boolean;
  item: any;
  onSelect: Function;
}) {
  return (
    <div
      onClick={() => onSelect(item)}
      className={`
        flex items-center gap-4 p-3 rounded-lg cursor-pointer
        bg-white/40 backdrop-blur
        border border-white/20
        hover:bg-white/70
        hover:shadow-md hover:shadow-blue-500/20
        transition-all text-blue-400
        ${isSelected ? "!bg-blue-100 " : ""}
      `}
    >
      <span className="material-icons ">bubble_chart</span>

      <div className="text-sm font-mono text-slate-700 uppercase flex-1">
        {item.report_name}
      </div>

      <span className="material-icons opacity-70">arrow_right</span>
    </div>
  );
}

function RenderBlockContent({ report }: { report: any }) {
  const reportQuery = useReport({
    report_id: report?.id,
    autorefresh: true,
  });
  return (
    <section className="flex flex-col gap-4 p-4">
      <ReportRender minimal reportQuery={reportQuery} />
    </section>
  );
}
