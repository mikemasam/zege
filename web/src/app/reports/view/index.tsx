import ReportRender from "@/components/report/report.render";
import { useReport } from "@/components/report/use.report";
import Page from "@/components/ui/ui-page";
import api, { useApi } from "@/lib/api";
import { DateTime } from "luxon";
import { useParams } from "react-router";

export default function ZegeReportView() {
  const params = useParams();
  const { data: report, error } = useApi(
    () => api.get(`/reports/${params?.id}`),
    {},
  );
  const reportQuery = useReport({
    report_id: report?.id,
  });
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
      {error && <p className="text-red-500 text-sm">Failed to load reports.</p>}
      {report && <Item item={report} />}
      {report && (
        <div className="box p-2">
          <ReportRender reportQuery={reportQuery} />
        </div>
      )}
    </Page>
  );
}

function Item({ item }: { item: any }) {
  return (
    <div className="flex items-center justify-between p-2 box">
      <div className="flex flex-col gap-2">
        <span className="text-sm text-gray-500">
          {item.report_type.toUpperCase()}
        </span>
        <span className="text-sm text-gray-500">{item.report_sql}</span>
      </div>
    </div>
  );
}
