import Page from "@/components/ui/ui-page";
import { Button } from "@/components/ui/button";
import { Link } from "react-router";
import api, { useApi } from "@/lib/api";
import { UITable } from "@/components/ui/ui-table";

export default function ZegeReports() {
  const query = useApi(() => api.get("/reports"));
  return (
    <Page title="Zege Reports">
      <div className="flex flex-row justify-end">
        <Link to="/app/reports/new">
          <Button variant="default">+ New Report</Button>
        </Link>
      </div>

      <UITable
        columns={[
          { key: "report_name", label: "Name" },
          {
            key: "report_type",
            label: "Type",
          },
        ]}
        data={query.data}
        actions={[
          {
            label: "Edit",
            icon: "edit",
            href: (i: any) => {
              return `/app/reports/${i.id}/edit`;
            },
          },
          {
            label: "View",
            icon: "arrow_right_alt",
            href: (i: any) => {
              return `/app/reports/${i.id}`;
            },
          },
        ]}
      />
    </Page>
  );
}

function Item({ item }: { item: any }) {
  return (
    <li className="flex items-center justify-between p-4 hover:bg-gray-50 transition box">
      <div className="flex flex-col">
        <span className="font-medium text-gray-900">{item.report_name}</span>
        <span className="text-sm text-gray-500">
          {item.report_type.toUpperCase()} •{" "}
          {new Date(item.created_at).toLocaleDateString()}
        </span>
      </div>

      <div className="flex items-center gap-2">
        <Link to={""}>
          <Button variant="outline" size="sm">
            Edit
          </Button>
        </Link>
        <Link to={`/app/reports/${item.id}`}>
          <Button variant="outline" size="sm">
            View
          </Button>
        </Link>
      </div>
    </li>
  );
}
