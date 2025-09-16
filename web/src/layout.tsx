import { Outlet } from "react-router";

export default function AppLayout() {
  return (
    <div className="flex gap-2">
      <Header />
      <div className="flex-1 flex flex-col p-2 overflow-x-hidden">
        <Outlet />
      </div>
    </div>
  );
}

const menu_items = [
  { label: "Home", href: "/" },
  { label: "Events", href: "/events" },
  { label: "Live Events", href: "/events/live" },
];
function Menu() {
  return (
    <div className="p-2 flex flex-col gap-2">
      {menu_items.map((m) => (
        <a href={m.href}>
          <div className="p-2 border-b border-b-gray-200 hover:shadow-md rounded-md cursor-pointer">
            {m.label}
          </div>
        </a>
      ))}
    </div>
  );
}
function Header() {
  return (
    <div className="shadow-md w-50 min-h-screen ">
      <div className="mx-auto flex items-center justify-between px-4 py-4">
        {/* Logo + Title */}
        <div className="flex items-center space-x-3">
          {/* Logo Circle with Z */}
          <div className="w-10 h-10 rounded-lg bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 flex items-center justify-center">
            <span className="text-xl font-extrabold text-white">Z</span>
          </div>
          <div>
            <h1 className="text-2xl font-bold tracking-wide drop-shadow">
              Zege
            </h1>
            <p className="text-sm text-gray-500">Event Logger</p>
          </div>
        </div>
      </div>
      <Menu />
    </div>
  );
}
