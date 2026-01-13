import { useAuth } from "@/auth/use.auth";
import { Link, Navigate, Outlet } from "react-router";

export default function AppLayout() {
  const auth = useAuth();
  if (auth.loading) {
    return null;
  }
  if (!auth.valid) {
    return <Navigate to="/login" replace />;
  }
  return (
    <div className="flex gap-0 bg-[#f7f5f5]">
      <Header />
      <div className="flex-1 flex flex-col overflow-y-scroll">
        <Outlet />
      </div>
    </div>
  );
}

const menu_items = [
  { label: "Home", href: "/app", icon: "home" },
  { label: "Events", href: "/app/events/live", icon: "hive" },
  { label: "Data", href: "/app/explore", icon: "hive" },
  { label: "Users", href: "/app/users", icon: "group" },
  { label: "Roles", href: "/app/roles" },
  { label: "Buckets", href: "/app/buckets" },
  { label: "Reports", href: "/app/reports", icon: "analytics" },
];
function Menu() {
  return (
    <div className="p-2 flex flex-col gap-2">
      {menu_items.map((m) => (
        <Link to={m.href}>
          <div className="p-2 border-b border-b-gray-200 hover:shadow cursor-pointer flex flex-row items-center gap-2 bg-white rounded-xl">
            <span className="material-icons !text-base text-gray-500">
              {m.icon ?? "business"}
            </span>
            <span>{m.label}</span>
          </div>
        </Link>
      ))}
    </div>
  );
}
function Header() {
  const auth = useAuth();
  if (!auth.valid) return null;
  return (
    <aside className="w-56 min-h-screen border-r flex flex-col">
      <OrganizationMenu auth={auth} />

      <Link to="/app/account">
        <div className="flex items-center gap-3 bg-white  px-2 py-2 mb-2 rounded">
          <div className="flex h-8 w-8 items-center justify-center rounded-full bg-gray-900">
            <span className="text-xs font-medium text-white">
              {auth.user?.name?.[0] ?? "U"}
            </span>
          </div>

          <div className="min-w-0">
            <div className="text-sm font-medium text-gray-900 truncate">
              {auth.user?.name}
            </div>
            <div className="text-xs text-gray-400 truncate">
              {auth.user?.email}
            </div>
          </div>
        </div>
      </Link>
      <Menu />
    </aside>
  );
}

function OrganizationMenu({ auth }: any) {
  return (
    <Link to="/app/account">
      <div
        title={auth.user?.organization?.name}
        className="flex flex-row items-center gap-1.5 text-sm font-semibold text-gray-700 bg-blue-50 border-b border-blue-300 px-2 py-1 cursor-pointer hover:bg-gray-200 hover:shadow-sm active:scale-[0.98] transition"
      >
        <span className="material-icons !text-base text-gray-500">
          business
        </span>

        <span className="truncate flex-1">
          {auth.user?.organization?.name ?? "Select organization"}
        </span>

        <span className="material-icons !text-base text-gray-400">
          swap_vert
        </span>
      </div>
    </Link>
  );
}
