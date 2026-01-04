import Page from "@/components/ui/ui-page";
import { Button } from "@/components/ui/button";
import { Link } from "react-router";
import api, { useApi } from "@/lib/api";
import { UITable } from "@/components/ui/ui-table";

export default function ListUsers() {
  const query = useApi(() => api.get("/users"));
  return (
    <Page title="Users">
      <div className="flex flex-row justify-end">
        <Link to="/app/users/create">
          <Button variant="default">+ Add User</Button>
        </Link>
      </div>
      <UITable
        columns={[
          { key: "name", label: "Name" },
          { key: "email", label: "Email" },
        ]}
        data={query.data}
      />
    </Page>
  );
}
