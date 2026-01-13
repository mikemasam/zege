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

export type UserPaper = {
  id: number;
  name: string;
  email: string;
  organization: Org;
  role: Role;
};
export type ConfigPaper = {
  features: {
    landing: boolean;
    signup: boolean;
    create_organization: boolean;
    login: boolean;
  };
};
type AuthState = {
  user: UserPaper | null;
  config: ConfigPaper | null;
  loading: boolean;
  ready: boolean;
  load: () => Promise<void>;
  logout: () => void;
};

export const useAuthStore = create<AuthState>((set, get) => ({
  user: null,
  config: null,
  loading: true,
  ready: false,
  load: async () => {
    set({ loading: true });
    console.log("Loading at ", get());
    const res = await loadAuth();
    const user = res?.user;
    const config = res?.config;
    set({ user, config, loading: false, ready: true });
  },
  logout: () => {
    localStorage.removeItem("authorization");
    set({ user: null, loading: false, ready: false });
  },
}));

async function loadAuth() {
  const res = await api.get("/auth/papers-please");
  if (res.status != 200) return false;
  if (!res.data) return false;
  return res.data;
}
