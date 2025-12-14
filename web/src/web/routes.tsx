import { Outlet, Route, Routes } from "react-router";
import Home from "./home";
import LoginPage from "./login";
import SignupPage from "./signup";

function WebLayout() {
  return (
    <div className="flex-1 flex flex-col overflow-y-scroll">
      <Outlet />
    </div>
  );
}
export default function WebRoutes() {
  return (
    <Routes>
      <Route path="/" element={<WebLayout />}>
        <Route path="/" element={<Home />} />
        <Route path="/login" element={<LoginPage />} />
        <Route path="/signup" element={<SignupPage />} />
      </Route>
    </Routes>
  );
}
