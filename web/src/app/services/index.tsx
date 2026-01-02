import Page from "@/components/ui/ui-page";
import { Button } from "@/components/ui/button";
import api, { useApi } from "@/lib/api";
import { Link } from "react-router";
import { UITable } from "@/components/ui/ui-table";

export default function ListServices() {
  const query = useApi(() => api.get("/services"));
  return (
    <Page title="Services" loading={query.loading}>
      <div className="flex flex-row justify-end">
        <Link to="/app/services/create">
          <Button variant="default">+ New Service</Button>
        </Link>
      </div>
      <UITable
        columns={[
          { key: "name", label: "Name" },
          { key: "label", label: "Label" },
          { key: "created_at", label: "Create At", type: "date" },
          { key: "apikey_value", label: "Api Key" },
        ]}
        data={query.data}
      />
    </Page>
  );
}
