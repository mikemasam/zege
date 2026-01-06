import { useEffect } from "react";
import { useNavigate } from "react-router";

export default function AppHome() {
  const nav = useNavigate();
  useEffect(() => {
    nav("/app/events/live");
  }, []);
  return <div>Home</div>;
}
