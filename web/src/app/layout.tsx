import { useAuth } from "@/auth/use.auth";
import Loading from "@/components/loading";
import { Link, Navigate, Outlet } from "react-router";

export default function AppLayout() {
  const auth = useAuth();
  if (auth.loading) return <Loading />;
  if (!auth.valid) {
    return <Navigate to="/login" replace />;
  }
  return (
    <div className="flex gap-2">
      <Header />
      <div className="flex-1 flex flex-col p-2 overflow-y-scroll">
        <Outlet />
      </div>
    </div>
  );
}

const menu_items = [
  { label: "Home", href: "/app" },
  { label: "Live", href: "/app/events/live" },
  { label: "Teams", href: "/app/teams" },
  { label: "Users", href: "/app/users" },
  { label: "Services", href: "/app/services" },
  { label: "Reports", href: "/app/reports" },
];
function Menu() {
  return (
    <div className="p-2 flex flex-col gap-2">
      {menu_items.map((m) => (
        <Link to={m.href}>
          <div className="p-2 border-b border-b-gray-200 hover:shadow rounded cursor-pointer">
            {m.label}
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
    <aside className="w-56 min-h-screen border-r bg-white">
      {/* Top section */}
      <div className="px-4 py-3">
        <div className="flex items-center gap-3">
          {/* Logo */}
          <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500">
            <span className="text-sm font-bold text-white">Z</span>
          </div>

          {/* Brand */}
          <div className="leading-tight">
            <h1 className="text-sm font-semibold text-gray-900">
              {auth.user?.name}
            </h1>
            <p className="text-xs text-gray-500 truncate max-w-[140px]">
              {auth.user?.email}
            </p>
          </div>
        </div>
      </div>

      {/* Menu */}
      <nav className="py-2">
        <Menu />
      </nav>
    </aside>
  );
}
