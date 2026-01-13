import Page from "@/components/page";
import ReportRender from "@/components/report/report.render";
import api, { useApi } from "@/lib/api";
import { useState } from "react";

export default function AppHome() {
  const query = useApi((params: any) => api.get("/reports", { params }));
  const [selected, setSelected] = useState<any>(null);
  return (
    <Page title="Overview" desc="Explore data overview" className="!gap-1">
      <div className="grid grid-cols-8 gap-4 min-h-[70vh]">
        {/* Sidebar */}
        <div
          className="
          col-span-2 flex flex-col gap-2 p-3 rounded-xl
          bg-white/60 backdrop-blur-lg
          border border-white/30
          shadow-lg shadow-black/10
        "
        >
          {query.data?.map((r: any, i: number) => (
            <MenuItem key={i} item={r} onSelect={setSelected} />
          ))}
        </div>

        {/* Content */}
        <div
          className="
          col-span-6 p-4 rounded-xl
          bg-white/50 backdrop-blur-lg
          border border-white/30
          shadow-xl shadow-black/10
        "
        >
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

function MenuItem({ item, onSelect }: { item: any; onSelect: Function }) {
  return (
    <div
      onClick={() => onSelect(item)}
      className="
        flex items-center gap-4 p-3 rounded-lg cursor-pointer
        bg-white/40 backdrop-blur
        border border-white/20
        hover:bg-white/70
        hover:shadow-md hover:shadow-blue-500/20
        transition-all
      "
    >
      <span className="material-icons text-blue-400">bubble_chart</span>

      <div className="text-sm font-mono text-slate-700 uppercase flex-1">
        {item.report_name}
      </div>

      <span className="material-icons text-blue-400 opacity-70">
        arrow_right
      </span>
    </div>
  );
}

function RenderBlockContent({ report }: { report: any }) {
  return (
    <section className="flex flex-col gap-2">
      <header
        className="
        px-1 rounded-lg
        bg-white/40 backdrop-blur
        border border-white/20
      "
      >
        <h2 className="text-lg font-semibold text-slate-700 uppercase tracking-wide">
          {report.report_name}
        </h2>
      </header>

      <div
        className="
        rounded-lg
        bg-white/40 backdrop-blur
        border border-white/20
      "
      >
        <ReportRender minimal report_id={report.id} />
      </div>
    </section>
  );
}
