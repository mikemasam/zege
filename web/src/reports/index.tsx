import Page from "@/components/ui/ui-page";
import { Button } from "@/components/ui/button";
import api, { useApi } from "@/lib/api";
import { Link } from "react-router";

export default function ZegeReports() {
  const { data, result, loading, error } = useApi(() => api.get("/reports"));
  console.log(data, result)
  return (
    <Page title="Zege Reports">
      <div className="flex flex-row justify-end">
        <Link to="/reports/new">
          <Button variant="default">+ New Report</Button>
        </Link>
      </div>

      {/* Reports List */}
      <div className="space-y-2">
        {loading && <p className="text-gray-500 text-sm">Loading reports...</p>}
        {error && (
          <p className="text-red-500 text-sm">Failed to load reports.</p>
        )}

        {data && data.length > 0 ? (
          <ul className="divide-y divide-gray-200 border rounded-xl">
            {data.map((item: any) => (
              <Item key={item.id} item={item} />
            ))}
          </ul>
        ) : (
          !loading && <p className="text-gray-500 text-sm">No reports found.</p>
        )}
      </div>
    </Page>
  );
}

function Item({ item }: { item: any }) {
  return (
    <li className="flex items-center justify-between p-4 hover:bg-gray-50 transition">
      <div className="flex flex-col">
        <span className="font-medium text-gray-900">{item.report_name}</span>
        <span className="text-sm text-gray-500">
          {item.report_type.toUpperCase()} •{" "}
          {new Date(item.created_at).toLocaleDateString()}
        </span>
      </div>

      <div className="flex items-center gap-2">
        <Link to={`/reports/${item.id}/edit`}>
          <Button variant="outline" size="sm">
            Edit 
          </Button>
        </Link>
        <Link to={`/reports/${item.id}`}>
          <Button variant="outline" size="sm">
            View
          </Button>
        </Link>
      </div>
    </li>
  );
}
