import { useAuthStore } from "@/stores/auth";
import { useEffect } from "react";

export function useAuth() {
  const auth = useAuthStore();
  useEffect(() => {
    if (!auth.ready) {
      auth.load();
    }
  }, [auth.ready]);
  return {
    ...auth,
    valid: !!auth.user,
  };
}

export function authorize_by_token(token: string) {
  if (!token) return false;
  useAuthStore.getState().logout();
  localStorage.setItem("authorization", token);
  useAuthStore.getState().load();
  return true;
}
