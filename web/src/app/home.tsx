import Page from "@/components/page";
import ReportRender from "@/components/report/report.render";
import { useReport } from "@/components/report/use.report";
import api, { useApi } from "@/lib/api";
import { Link } from "react-router";
import { useEffect, useState } from "react";

const quick_links = [
  { label: "Live Events", href: "/app/events/live", icon: "hive", color: "bg-blue-500" },
  { label: "Data Explore", href: "/app/explore", icon: "explore", color: "bg-violet-500" },
  { label: "Reports", href: "/app/reports", icon: "analytics", color: "bg-emerald-500" },
  { label: "Users", href: "/app/users", icon: "group", color: "bg-amber-500" },
];

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
    <Page title="Overview" desc="Explore data overview">
      <div className="grid grid-cols-2 sm:grid-cols-4 gap-3">
        {quick_links.map((link) => (
          <Link key={link.href} to={link.href}>
            <div className="flex items-center gap-3 px-4 py-3 bg-white rounded-lg border border-gray-200 shadow-sm hover:shadow-md hover:-translate-y-0.5 transition-all duration-200 cursor-pointer">
              <div className={`flex h-9 w-9 items-center justify-center rounded-lg ${link.color} text-white shadow-sm`}>
                <span className="material-icons !text-xl">{link.icon}</span>
              </div>
              <span className="text-sm font-medium text-gray-700">{link.label}</span>
            </div>
          </Link>
        ))}
      </div>

      <div className="lg:grid grid-cols-8 gap-5">
        <div className="col-span-2 flex flex-col gap-1.5">
          <div className="flex items-center gap-2 px-1 mb-1">
            <span className="material-icons !text-base text-gray-400">push_pin</span>
            <span className="text-xs font-semibold uppercase tracking-wider text-gray-400">Pinned</span>
          </div>
          {query.data?.map((r: any, i: number) => (
            <MenuItem
              key={i}
              item={r}
              isSelected={selected?.id == r.id}
              onSelect={setSelected}
            />
          ))}
          {(!query.data || query.data.length === 0) && (
            <div className="text-xs text-gray-400 px-1 py-4 text-center">
              No pinned reports yet
            </div>
          )}
        </div>
        <div className="col-span-6">
          {selected ? (
            <RenderBlockContent report={selected} />
          ) : (
            <div className="flex flex-col items-center justify-center text-center py-20 text-gray-400">
              <span className="material-icons !text-4xl mb-3 text-gray-300">
                dashboard
              </span>
              <p className="text-sm font-medium">Select a report to view</p>
            </div>
          )}
        </div>
      </div>
    </Page>
  );
}

type MenuItemType = {
  isSelected: boolean;
  item: any;
  onSelect: Function;
};
function MenuItem({ item, onSelect, isSelected }: MenuItemType) {
  return (
    <div
      onClick={() => onSelect(item)}
      className={`
        flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer
        transition-all duration-150 group
        ${
          isSelected
            ? "bg-blue-50 border-l-[3px] border-blue-500 shadow-sm"
            : "bg-white border border-gray-100 hover:border-gray-200 hover:shadow-sm hover:bg-gray-50 border-l-[3px] border-l-transparent"
        }
      `}
    >
      <span className={`material-icons !text-lg transition-colors duration-150 ${isSelected ? "text-blue-600" : "text-gray-400 group-hover:text-blue-500"}`}>
        bubble_chart
      </span>
      <div className={`text-sm font-medium flex-1 truncate ${isSelected ? "text-blue-800" : "text-slate-700"}`}>
        {item.report_name}
      </div>
      <span className={`material-icons !text-lg transition-all duration-150 ${isSelected ? "text-blue-400 translate-x-0.5" : "text-gray-300 group-hover:text-gray-400"}`}>
        chevron_right
      </span>
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
