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
    <div className={`flex flex-col gap-1`}>
      <HeaderMenu />
      <div className="flex flex-col items-start px-4 py-2 mx-2 mt-1 box">
        <h1 className="text-lg font-semibold text-gray-900">{props.title}</h1>
        <p className="text-sm text-gray-500">{props.desc}</p>
      </div>

      <div className={`flex flex-col p-2 gap-4 ${props.className ?? ""}`}>
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
    <div className="p-2 bg-blue-50 border-b border-blue-300 flex flex-row justify-end">
      {menuItems.map((item) => (
        <div className="text-xs font-semibold text-blue-800 border-l-2 border-b-gray-600 px-4">
          <a
            href="#"
            className="text-xs font-semibold text-blue-800"
            onClick={() => onClick(item)}
          >
            {item.label}
          </a>
        </div>
      ))}
    </div>
  );
}
