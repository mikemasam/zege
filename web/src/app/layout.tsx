import { useAuth } from "@/auth/use.auth";
import { Link, Navigate, Outlet } from "react-router";
import type { UserPaper } from "@/stores/auth";

export default function AppLayout() {
  const auth = useAuth();
  if (auth.loading) {
    return null;
  }
  if (!auth.valid) {
    return <Navigate to="/login" replace />;
  }
  return (
    <div className="flex min-h-screen bg-gray-50">
      <Header auth={auth} />
      <div className="flex-1 flex flex-col overflow-y-auto">
        <main className="p-6 lg:p-8">
          <Outlet />
        </main>
      </div>
    </div>
  );
}

const menu_items = [
  { label: "Overview", href: "/app", icon: "home" },
  { label: "Events", href: "/app/events/live", icon: "hive" },
  { label: "Reports", href: "/app/reports", icon: "analytics" },
  { label: "Data", href: "/app/explore", icon: "hive" },
  { label: "Users", href: "/app/users", icon: "group" },
  { label: "Roles", href: "/app/roles" },
  { label: "Buckets", href: "/app/buckets" },
];

function Menu() {
  return (
    <nav className="flex-1 px-2.5 py-5 space-y-0.5">
      <div className="px-3 mb-3 text-[11px] font-semibold text-slate-500 uppercase tracking-[0.08em]">
        Main
      </div>
      {menu_items.map((m) => (
        <Link key={m.href} to={m.href}>
          <div className="group flex items-center gap-3 px-3 py-2.5 text-sm font-medium text-slate-400 rounded-lg hover:bg-slate-800/50 hover:text-white transition-all duration-150 cursor-pointer">
            <span className="material-icons !text-[22px] text-slate-500 group-hover:text-slate-300 transition-colors duration-150">
              {m.icon ?? "business"}
            </span>
            <span>{m.label}</span>
          </div>
        </Link>
      ))}
    </nav>
  );
}

function Header({ auth }: { auth: { user: UserPaper | null; valid: boolean } }) {
  if (!auth.valid) return null;
  return (
    <aside className="w-60 min-h-screen bg-gradient-to-b from-slate-900 to-slate-950 flex flex-col shadow-xl">
      <OrganizationMenu auth={auth} />
      <Menu />

      <Link
        to="/app/account"
        className="group border-t border-slate-800 px-4 py-3.5 hover:bg-slate-800/30 transition-colors duration-150"
      >
        <div className="flex items-center gap-3">
          <div className="relative flex h-9 w-9 items-center justify-center rounded-full bg-gradient-to-br from-violet-400 to-indigo-600 flex-shrink-0 shadow-lg shadow-indigo-500/20">
            <span className="text-sm font-semibold text-white">
              {auth.user?.name?.[0] ?? "U"}
            </span>
          </div>

          <div className="min-w-0 flex-1">
            <div className="text-sm font-medium text-slate-200 truncate">
              {auth.user?.name}
            </div>
            <div className="text-xs text-slate-500 truncate">
              {auth.user?.email}
            </div>
          </div>

          <span className="material-icons !text-lg text-slate-600 group-hover:text-slate-400 transition-colors duration-150">
            chevron_right
          </span>
        </div>
      </Link>
    </aside>
  );
}

function OrganizationMenu({ auth }: { auth: { user: Pick<UserPaper, "organization"> | null } }) {
  return (
    <Link to="/app/account">
      <div
        title={auth.user?.organization?.name}
        className="flex items-center gap-2.5 px-4 py-3.5 border-b border-slate-800 hover:bg-slate-800/30 transition-colors duration-150 cursor-pointer group"
      >
        <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-slate-800 text-slate-400 flex-shrink-0 ring-1 ring-slate-700/50 group-hover:ring-slate-600/50 transition-all duration-150">
          <span className="material-icons !text-lg">business</span>
        </div>

        <div className="min-w-0 flex-1">
          <div className="text-[10px] font-semibold text-slate-500 uppercase tracking-[0.1em] mb-0.5">
            Org
          </div>
          <span className="text-sm font-semibold text-slate-200 truncate block">
            {auth.user?.organization?.name ?? "Select organization"}
          </span>
        </div>

        <span className="material-icons !text-lg text-slate-600 group-hover:text-slate-400 transition-colors duration-150">
          swap_vert
        </span>
      </div>
    </Link>
  );
}
