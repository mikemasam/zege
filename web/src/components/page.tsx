import Loading from "./ui/ui-loading";
import { useNavigate } from "react-router";
import { useAuthStore } from "@/stores/auth";

type PageProps = {
  title?: string | null;
  desc?: string | null;
  children: React.ReactNode;
  className?: string;
};
export default function Page(props: PageProps) {
  return (
    <div className="flex flex-col">
      <HeaderMenu />
      <div className="flex flex-col items-start px-5 py-3 mx-4 mt-3 bg-white rounded-lg border-l-4 border-blue-300 shadow-sm">
        <h1 className="text-lg font-semibold text-gray-900">{props.title}</h1>
        {props.desc && <p className="text-sm text-gray-500">{props.desc}</p>}
      </div>

      <div className={`flex flex-col p-3 gap-4 ${props.className ?? ""}`}>
        {props.children}
      </div>
      <Loading />
    </div>
  );
}

type MenuItem = { label: string; key: string };
const menuItems: MenuItem[] = [
  { label: "Profile", key: "account" },
  { label: "Logout", key: "logout" },
];
function HeaderMenu() {
  const auth = useAuthStore();
  const navigate = useNavigate();
  const onClick = (item: MenuItem) => {
    if (item.key == "logout") {
      auth.logout();
      navigate("/login");
    }
    if (item.key == "account") {
      navigate("/app/account");
    }
  };
  return (
    <div className="flex items-center justify-end gap-1 px-5 py-2 border-b border-gray-100">
      {menuItems.map((item) => (
        <button
          key={item.key}
          onClick={() => onClick(item)}
          className="text-xs font-medium text-gray-400 hover:text-gray-700 transition-colors px-2.5 py-1.5 rounded-md hover:bg-gray-100"
        >
          {item.label}
        </button>
      ))}
    </div>
  );
}
