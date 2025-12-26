import Page from "@/components/ui/ui-page";
import api, { useApi } from "@/lib/api";
import Loading from "@/components/loading";
import { UITable } from "@/components/ui/ui-table";

export default function ListUsers() {
  const { data, loading } = useApi(() => api.get("/users"));
  if (loading) return <Loading />;
  return (
    <Page title="Teams">
      <UITable
        columns={[
          { key: "name", label: "Name" },
          { key: "email", label: "Email" },
        ]}
        data={data}
      />
    </Page>
  );
}
