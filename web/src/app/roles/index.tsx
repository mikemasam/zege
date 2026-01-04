import Page from "@/components/ui/ui-page";
import api, { useApi } from "@/lib/api";
import { UITable } from "@/components/ui/ui-table";

export default function ListRoles() {
  const query = useApi(() => api.get("/roles"));
  return (
    <Page title="Roles">
      <UITable columns={[{ key: "name", label: "Name" }]} data={query.data} />
    </Page>
  );
}
