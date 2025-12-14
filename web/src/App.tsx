import { Suspense } from "react";
import WebRoutes from "./web/routes";
import AppRoutes from "./app/routes";

export default function App() {
  return (
    <Suspense fallback="Loading">
      <WebRoutes />
      <AppRoutes />
    </Suspense>
  );
}
