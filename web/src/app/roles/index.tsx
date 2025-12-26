import Page from "@/components/ui/ui-page";
import api, { useApi } from "@/lib/api";
import Loading from "@/components/loading";
import { UITable } from "@/components/ui/ui-table";

export default function ListRoles() {
  const { data, loading } = useApi(() => api.get("/roles"));
  if (loading) return <Loading />;
  return (
    <Page title="Roles">
      <UITable columns={[{ key: "name", label: "Name" }]} data={data} />
    </Page>
  );
}
