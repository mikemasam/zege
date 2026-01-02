import { create } from "zustand";
import api from "@/lib/api";

type Org = {
  id: number;
  name: string;
};

type Role = {
  id: number;
  name: string;
};

export type UserPapers = {
  id: number;
  name: string;
  email: string;
  organization: Org;
  role: Role;
};

type AuthState = {
  user: UserPapers | null;
  loading: boolean;
  _booted: boolean;
  load: () => Promise<void>;
  logout: () => void;
};

export const useAuthStore = create<AuthState>((set, get) => ({
  user: null,
  loading: true,
  _booted: false,
  load: async () => {
    set({ loading: true });
    const user = await loadAuth();
    set({ user, loading: false, _booted: true });
  },
  logout: () => {
    localStorage.removeItem("authorization");
    set({ user: null, _booted: false });
  },
}));

async function loadAuth() {
  const res = await api.get("/auth/papers-please");
  if (res.status != 200) return false;
  if (!res.data) return false;
  return res.data;
}
