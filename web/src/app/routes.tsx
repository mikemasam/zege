import { Route, Routes } from "react-router";
import Home from "./home";
import AppLayout from "./layout";
import EventsLive from "./events/live";
import ZegeReports from "./reports";
import ZegeReportEditor from "./reports/create";
import ZegeReportView from "./reports/view";
import TeamCreate from "./teams/create";
import ListTeams from "./teams";
import ListUsers from "./users";
import ListRoles from "./roles";
import ListServices from "./services";

export default function AppRoutes() {
  return (
    <Routes>
      <Route path="/app" element={<AppLayout />}>
        <Route index element={<Home />} />
        <Route path="events/live" element={<EventsLive />} />
        <Route path="reports">
          <Route index element={<ZegeReports />} />
          <Route path="new" element={<ZegeReportEditor />} />
          <Route path=":id/edit" element={<ZegeReportEditor />} />
          <Route path=":id" element={<ZegeReportView />} />
        </Route>
        <Route path="teams">
          <Route index element={<ListTeams />} />
          <Route path="create" element={<TeamCreate />} />
        </Route>
        <Route path="users">
          <Route index element={<ListUsers />} />
        </Route>
        <Route path="roles">
          <Route index element={<ListRoles />} />
        </Route>
        <Route path="services">
          <Route index element={<ListServices />} />
        </Route>
      </Route>
    </Routes>
  );
}
