import { useEffect } from "react";
import { useAuthStore } from "@/stores/auth";

export function useAuth() {
  const { user, loading, load } = useAuthStore();
  useEffect(() => {
    load();
  }, [load]);

  return {
    user,
    valid: !!user,
    loading,
  };
}

export function authorize_by_token(token: string) {
  if (!token) return false;
  useAuthStore.getState().logout();
  localStorage.setItem("authorization", token);
  useAuthStore.getState().load();
  return true;
}
