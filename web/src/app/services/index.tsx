import Page from "@/components/ui/ui-page";
import { Button } from "@/components/ui/button";
import api, { useApi } from "@/lib/api";
import { Link } from "react-router";
import Loading from "@/components/loading";
import { UITable } from "@/components/ui/ui-table";

export default function ListServices() {
  const { data, result, loading, error } = useApi(() => api.get("/services"));
  console.log(data, result);
  if (loading) return <Loading />;
  return (
    <Page title="Services">
      <div className="flex flex-row justify-end">
        <Link to="/app/services/create">
          <Button variant="default">+ New Service</Button>
        </Link>
      </div>
      <UITable
        columns={[
          { key: "name", label: "Name" },
          { key: "label", label: "Label" },
          { key: "created_at", label: "Create At" },
        ]}
        data={data}
      />
    </Page>
  );
}
