import { useAuth } from "@/auth/use.auth";
import { Button } from "@/components/ui/button";
import Page from "@/components/ui/ui-page";
import { UITable } from "@/components/ui/ui-table";
import TextLabel from "@/components/ui/ui-textlabel";
import api, { useApi } from "@/lib/api";
import { Link } from "react-router";

export default function AccountPage() {
  const auth = useAuth();
  const query = useApi(() => api.get("/organizations"));
  const switchOrg = async (org: any) => {
    const _ = await api.post("/auth/switch-organization", {
      org_id: org.id,
    });
    query.load();
    auth.load();
  };
  return (
    <Page title="Account" desc="Manage your account and organization details">
      <div className="space-y-6">
        <div className="box p-6">
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
            {
              key: "name",
              label: "Name",
              render: (v, row) => {
                return (
                  <>
                    {row.is_current && (
                      <span className="material-icons !text-lg pr-2 text-green-800">
                        check_circle
                      </span>
                    )}
                    {!row.is_current && (
                      <span className="material-icons !text-lg pr-2 text-gray-200">
                        do_not_disturb_on
                      </span>
                    )}
                    {v}
                  </>
                );
              },
            },
            { key: "created_at", label: "Created At", type: "datetime" },
          ]}
          actions={[
            {
              label: "Switch to this organization",
              icon: "compare_arrows",
              action: (a) => {
                switchOrg(a);
              },
            },
          ]}
          data={query.data}
        >
          <div className="flex flex-row justify-end flex-1">
            <Link to="/app/organizations/create">
              <Button variant="default">+</Button>
            </Link>
          </div>
        </UITable>
      </div>
    </Page>
  );
}
