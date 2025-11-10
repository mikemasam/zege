import { Route, Routes } from "react-router";
import Home from "./home";
import AppLayout from "./layout";
import EventsLive from "./events/live";
import ZegeReports from "./reports";
import ZegeReportEditor from "./reports/create";
import ZegeReportView from "./reports/view";

export default function AppRoutes() {
  return (
    <Routes>
      <Route path="/" element={<AppLayout />}>
        <Route path="/" element={<Home />} />
        <Route path="/events/live" element={<EventsLive />} />
        <Route path="/reports">
          <Route index element={<ZegeReports />} />
          <Route path="new" element={<ZegeReportEditor />} />
          <Route path=":id/edit" element={<ZegeReportEditor />} />
          <Route path=":id" element={<ZegeReportView />} />
        </Route>
      </Route>
    </Routes>
  );
}
