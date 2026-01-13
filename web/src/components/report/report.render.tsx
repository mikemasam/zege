import api, { useApi } from "@/lib/api";
import { TableView } from "./table.view";
import UICard from "@/components/ui/ui-card";
import { Button } from "@/components/ui/button";
import { useEffect, useMemo, useState } from "react";
import { TileView } from "./tile.view";

export default function ReportRender({
  report_id,
  minimal,
}: {
  minimal?: boolean;
  report_id: number;
}) {
  const [type, setType] = useState("");
  const query = useApi(() => api.get(`/reports/${report_id}/read`), {
    prefrech: false,
  });
  const [report, data] = useMemo(() => {
    const output = query.data;
    if (!output) return [null, null];
    return [output.report, output.data];
  }, [query.data]);
  useEffect(() => {
    if (!report) return;
    setType(report.report_type);
  }, [report]);
  useEffect(() => {
    if (report_id) {
      query.load();
    }
  }, [report_id]);
  console.log(report, data);
  return (
    <div>
      {!minimal && (
        <UICard className="flex flex-row justify-between items-center">
          <div className="text-lg">Data View</div>
          <div className="flex flex-row gap-1">
            <Button variant="outline">Table</Button>
            <Button variant="outline">Bar</Button>
            <Button variant="outline">Line</Button>
          </div>
        </UICard>
      )}
      {type == "table" && <TableView data={data} />}
      {type == "tile" && <TileView data={data} />}
    </div>
  );
}
