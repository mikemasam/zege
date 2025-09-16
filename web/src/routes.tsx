import { Route, Routes } from "react-router";
import Home from "./home";
import AppLayout from "./layout";
import EventsLive from "./events/live";
import EventsExplore from "./events";

export default function AppRoutes() {
  return (
    <Routes>
      <Route path="/" element={<AppLayout />}>
        <Route path="/" element={<Home />} />
        <Route path="/events" element={<EventsExplore />} />
        <Route path="/events/live" element={<EventsLive />} />
      </Route>
    </Routes>
  );
}
