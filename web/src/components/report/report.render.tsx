import { TableView } from "./table.view";
import { Button } from "@/components/ui/button";
import { TileView } from "./tile.view";

export default function ReportRender({
  reportQuery,
  minimal,
}: {
  minimal?: boolean;
  reportQuery: any;
}) {
  if (!reportQuery) return null;
  return (
    <div>
      {!minimal && (
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-sm font-semibold text-gray-700">Data View</h3>
          <div className="flex gap-1">
            <Button variant="outline" size="sm">Table</Button>
            <Button variant="outline" size="sm">Bar</Button>
            <Button variant="outline" size="sm">Line</Button>
          </div>
        </div>
      )}
      {reportQuery.type == "table" && <TableView data={reportQuery.data} />}
      {reportQuery.type == "tile" && <TileView data={reportQuery.data} />}
    </div>
  );
}
