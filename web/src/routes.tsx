import { Route, Routes } from "react-router";
import Home from "./home";
import AppLayout from "./layout";
import EventsLive from "./events/live";
import EventsExplore from "./events";
import ZegeReports from "./reports";
import CreateZegeReport from "./reports/create";

export default function AppRoutes() {
  return (
    <Routes>
      <Route path="/" element={<AppLayout />}>
        <Route path="/" element={<Home />} />
        <Route path="/events" element={<EventsExplore />} />
        <Route path="/events/live" element={<EventsLive />} />
        <Route path="/reports">
          <Route index element={<ZegeReports />} />
          <Route path="new" element={<CreateZegeReport />} />
        </Route>
      </Route>
    </Routes>
  );
}
