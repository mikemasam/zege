import Page from "@/components/ui/ui-page";
import { Button } from "@/components/ui/button";
import api, { useApi } from "@/lib/api";
import { Link } from "react-router";
import Loading from "@/components/loading";
import { UITable } from "@/components/ui/ui-table";

export default function ListTeams() {
  const { data, result, loading, error } = useApi(() => api.get("/teams"));
  console.log(data, result);
  if (loading) return <Loading />;
  return (
    <Page title="Teams">
      <div className="flex flex-row justify-end">
        <Link to="/app/teams/create">
          <Button variant="default">+ New Team</Button>
        </Link>
      </div>
      <UITable columns={[{ key: "name", label: "Name" }]} data={data} />
    </Page>
  );
}

