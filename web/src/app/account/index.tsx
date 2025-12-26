import { useAuth } from "@/auth/use.auth";
import Loading from "@/components/loading";
import { Button } from "@/components/ui/button";
import Page from "@/components/ui/ui-page";
import { UITable } from "@/components/ui/ui-table";
import api, { useApi } from "@/lib/api";
import { Link } from "react-router";

export default function AccountPage() {
  const auth = useAuth();
  const { data, result, loading, error } = useApi(() =>
    api.get("/organizations"),
  );
  if (auth.loading) return <Loading />;
  return (
    <Page>
      <div className="space-y-6">
        <div className="flex items-center justify-between p-5">
          <div>
            <h1 className="text-xl font-semibold text-gray-900">Account</h1>
            <p className="text-sm text-gray-500">
              Manage your account and organization details
            </p>
          </div>
        </div>

        <div className="rounded border-x-2 border-blue-300 bg-white p-6">
          <div className="flex flex-col gap-4">
            <TextLabel label="Name" desc={auth.user?.name} />
            <TextLabel label="Email" desc={auth.user?.email} />
            <TextLabel
              label="Organization"
              desc={auth.user?.organization?.name}
            />
          </div>
        </div>
        <UITable
          title="Organizations"
          columns={[
            { key: "name", label: "Name" },
            { key: "created_at", label: "Created At" },
          ]}
          data={data}
        />
        <div className="flex flex-row justify-end">
          <Link to="/app/organizations/create">
            <Button variant="default" className="px-4">
              + New Organization
            </Button>
          </Link>
        </div>
      </div>
    </Page>
  );
}

function TextLabel({ label, desc }: { label?: string; desc?: string }) {
  return (
    <div className="flex flex-col gap-1">
      <label className="text-sm text-gray-500 tracking-snug">{label}</label>
      <span className="text-sm text-gray-600 leading-wide font-bold">
        {desc}
      </span>
    </div>
  );
}
