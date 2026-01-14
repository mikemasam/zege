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
    <div className="">
      {!minimal && (
        <div className="flex flex-row justify-between items-center p-2">
          <div className="text-lg">Data View</div>
          <div className="flex flex-row gap-1">
            <Button variant="outline">Table</Button>
            <Button variant="outline">Bar</Button>
            <Button variant="outline">Line</Button>
          </div>
        </div>
      )}
      {reportQuery.type == "table" && <TableView data={reportQuery.data} />}
      {reportQuery.type == "tile" && <TileView data={reportQuery.data} />}
    </div>
  );
}
