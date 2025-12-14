import ReportRender from "@/components/report/report.render";
import { TableView } from "@/components/report/table.view";
import Page from "@/components/ui/ui-page";
import api, { useApi } from "@/lib/api";
import { useParams } from "react-router";

export default function ZegeReportView() {
  const params = useParams();
  console.log(params);
  const {
    data: report,
    loading,
    error,
  } = useApi(() => api.get(`/reports/${params?.id}`), {}, [params]);
  return (
    <Page
      title={`${report?.report_name ?? ""} ~ Zege Report`}
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
        <span className="text-xl font-medium text-gray-900">
          {item.report_name}
        </span>
        <span className="text-sm text-gray-500">
          {item.report_type.toUpperCase()} •{" "}
          {new Date(item.created_at).toLocaleDateString()}
        </span>
        <span className="text-sm text-gray-500">{item.report_sql}</span>
      </div>
    </div>
  );
}
