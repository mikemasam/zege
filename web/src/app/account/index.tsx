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
        <div className="bg-white rounded-lg border border-gray-200 shadow-sm p-6">
          <div className="flex items-center gap-4 pb-5 mb-5 border-b border-gray-100">
            <div className="flex h-12 w-12 items-center justify-center rounded-full bg-gradient-to-br from-blue-500 to-indigo-600 text-white text-lg font-bold shadow-sm flex-shrink-0">
              {auth.user?.name?.[0] ?? "U"}
            </div>
            <div className="min-w-0">
              <h2 className="text-base font-semibold text-gray-900 truncate">
                {auth.user?.name}
              </h2>
              <p className="text-sm text-gray-500 truncate">
                {auth.user?.email}
              </p>
            </div>
          </div>
          <div className="grid grid-cols-2 gap-6">
            <TextLabel label="Organization" desc={auth.user?.organization?.name} />
            <TextLabel label="Role" desc={auth.user?.role?.name} />
          </div>
        </div>

        <UITable
          title="Organizations"
          columns={[
            {
              key: "name",
              label: "Name",
              render: (v, row: any) => (
                <span className="inline-flex items-center gap-2">
                  {row.is_current ? (
                    <span className="material-icons !text-lg text-emerald-600">check_circle</span>
                  ) : (
                    <span className="material-icons !text-lg text-gray-300">radio_button_unchecked</span>
                  )}
                  <span className={row.is_current ? "font-medium text-gray-900" : "text-gray-700"}>
                    {v}
                  </span>
                </span>
              ),
            },
            { key: "created_at", label: "Created", type: "datetime" },
          ]}
          actions={[
            {
              label: "Switch to this organization",
              icon: "compare_arrows",
              action: (a) => switchOrg(a),
            },
          ]}
          data={query.data}
        >
          {auth.config?.features?.create_organization && (
            <Link to="/app/organizations/create">
              <Button variant="outline" size="sm">+ New Organization</Button>
            </Link>
          )}
        </UITable>
      </div>
    </Page>
  );
}
