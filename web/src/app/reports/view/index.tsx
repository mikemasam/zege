import ReportRender from "@/components/report/report.render";
import { TableView } from "@/components/report/table.view";
import Page from "@/components/ui/ui-page";
import api, { useApi } from "@/lib/api";
import { DateTime } from "luxon";
import { useParams } from "react-router";

export default function ZegeReportView() {
  const params = useParams();
  const {
    data: report,
    loading,
    error,
  } = useApi(() => api.get(`/reports/${params?.id}`), {});
  return (
    <Page
      title={report?.report_name ?? ""}
      desc={
        report?.created_at
          ? DateTime.fromISO(report.created_at).toRelative()
          : ""
      }
      className="space-y-4"
    >
      {loading && <p className="text-gray-500 text-sm">Loading reports...</p>}
      {error && <p className="text-red-500 text-sm">Failed to load reports.</p>}
      {report && <Item item={report} />}
      {report && <ReportRender report_id={report.id} />}
    </Page>
  );
}

function Item({ item }: { item: any }) {
  return (
    <div className="flex items-center justify-between border-l-2 border-blue-300 p-2 rounded">
      <div className="flex flex-col gap-2">
        <span className="text-sm text-gray-500">
          {item.report_type.toUpperCase()}
        </span>
        <span className="text-sm text-gray-500">{item.report_sql}</span>
      </div>
    </div>
  );
}
