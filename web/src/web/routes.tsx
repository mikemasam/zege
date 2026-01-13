import { Outlet, Route, Routes, useNavigate } from "react-router";
import Home from "./home";
import LoginPage from "./login";
import SignupPage from "./signup";
import { useAuth } from "@/auth/use.auth";

function WebLayout() {
  const auth = useAuth();
  if (!auth.ready) {
    return null;
  }
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
