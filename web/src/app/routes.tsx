import { Route, Routes } from "react-router";
import Home from "./home";
import AppLayout from "./layout";
import EventsLive from "./events/live";
import ZegeReports from "./reports";
import ZegeReportEditor from "./reports/create";
import ZegeReportView from "./reports/view";
import OrganizationCreate from "./organizations/create";
import ListUsers from "./users";
import ListRoles from "./roles";
import ListServices from "./services";
import ServiceCreate from "./services/create";
import AccountPage from "./account";
import UserCreate from "./users/create";

export default function AppRoutes() {
  return (
    <Routes>
      <Route path="/app" element={<AppLayout />}>
        <Route index element={<Home />} />
        <Route path="events/live" element={<EventsLive />} />
        <Route path="account" element={<AccountPage />} />
        <Route path="reports">
          <Route index element={<ZegeReports />} />
          <Route path="new" element={<ZegeReportEditor />} />
          <Route path=":id/edit" element={<ZegeReportEditor />} />
          <Route path=":id" element={<ZegeReportView />} />
        </Route>
        <Route path="organizations">
          <Route path="create" element={<OrganizationCreate />} />
        </Route>
        <Route path="users">
          <Route index element={<ListUsers />} />
          <Route path="create" element={<UserCreate />} />
        </Route>
        <Route path="roles">
          <Route index element={<ListRoles />} />
        </Route>
        <Route path="services">
          <Route index element={<ListServices />} />
          <Route path="create" element={<ServiceCreate />} />
        </Route>
      </Route>
    </Routes>
  );
}
