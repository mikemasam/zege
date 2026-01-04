import Page from "@/components/ui/ui-page";
import { Button } from "@/components/ui/button";
import api, { useApi } from "@/lib/api";
import { Link } from "react-router";
import { UITable } from "@/components/ui/ui-table";

export default function ListBuckets() {
  const query = useApi(() => api.get("/buckets"));
  return (
    <Page title="Buckets">
      <div className="flex flex-row justify-end">
        <Link to="/app/buckets/create">
          <Button variant="default">+ New Bucket</Button>
        </Link>
      </div>
      <UITable
        columns={[
          { key: "name", label: "Name" },
          { key: "created_at", label: "Create At", type: "date" },
          { key: "bucket_key", label: "Bucket Key" },
        ]}
        data={query.data}
      />
    </Page>
  );
}
