import api, { useApi } from "@/lib/api";
import { TableView } from "./table.view";
import UICard from "@/components/ui/ui-card";
import { Button } from "@/components/ui/button";

export default function ReportRender({ report_id }: { report_id: number }) {
  const { data, loading, error } = useApi(
    () => api.get(`/reports/${report_id}/read`),
    {},
    [report_id],
  );
  console.log(data);
  return (
    <div>
      <UICard className="flex flex-row justify-between items-center">
        <div className="text-lg">Data View</div>
        <div className="flex flex-row gap-1">
          <Button variant="outline">Table</Button>
          <Button variant="outline">Bar</Button>
          <Button variant="outline">Line</Button>
        </div>
      </UICard>
      <TableView data={data} />
    </div>
  );
}
