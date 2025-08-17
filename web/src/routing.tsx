import { useLocation, Switch, Route } from "wouter";
import MenuItem from "./components/menu-item";
import AppLayout from "./layout";
import NewDatabaseConnectionForm from "./pages/new.connection";
import DatabaseConnection from "./pages/connection";

export default function Routing() {
  const [location, navigate] = useLocation();
  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      <div className="p-2 flex flex-row gap-2 border-b">
        <MenuItem icon="dataset" onClick={() => navigate("/")} />
        <MenuItem icon="settings" />
      </div>
      <Switch>
        <Route path="/" component={DatabaseConnection} />
        <Route path="/connections/new" component={NewDatabaseConnectionForm} />
      </Switch>
    </div>
  );
}
