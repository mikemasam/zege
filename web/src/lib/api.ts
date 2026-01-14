import { useNotifyStore } from "@/stores/notify";
import axios, { AxiosError } from "axios";
import { useEffect, useState } from "react";

console.log("ENV URL", import.meta.env.VITE_API_URL || "/api/v1");
const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL || "/api/v1",
  timeout: 10000,
  validateStatus: () => true,
  headers: {
    "Content-Type": "application/json",
  },
});

// Request interceptor: attach auth token
api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem("authorization");
    if (token && config.headers) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    if ((config as any).meta?.notify !== false) {
      (config as any).requestId = useNotifyStore.getState().add_task();
    } else {
      (config as any).requestId = Math.random();
    }
    return config;
  },
  (error) => ({
    status: 0,
    message: error?.message ?? "request failed",
    data: null,
  }),
);

// Response interceptor: handle errors / auto logout on 401
api.interceptors.response.use(
  (response) => {
    const requestId = (response.config as any).requestId;
    useNotifyStore.getState().remove_task(requestId);
    if (response.data) return response.data;

    return {
      status: response.status,
      message: response.statusText ?? "Request failed",
      data: null,
    };
  },
  (err: AxiosError) => {
    const requestId = (err?.config as any)?.requestId || "unknown";
    useNotifyStore.getState().remove_task(requestId);
    return {
      status: err?.status ?? 0,
      message: err?.message ?? "Request failed",
      data: null,
    };
  },
);

const initial: {
  status: number;
  data: any;
  cursor?: null | { page: number; per_page: number };
  message: string;
} = {
  status: 0,
  data: null,
  cursor: null,
  message: "",
};

export function useApi(callback: Function, opts?: { prefrech?: boolean }) {
  const [params, setParams] = useState<any>({});
  const [result, setResult] = useState(initial);
  const [loading, setLoading] = useState(false);
  const load = async (_params?: any) => {
    const __params = {
      ...params,
      ..._params,
    };
    setParams((p: any) => ({ ...p, ..._params }));
    setLoading(true);
    return Promise.resolve(callback(__params))
      .then((res) => {
        console.log("loaded", res);
        setResult(res);
        return res;
      })
      .catch(() => {
        setResult({
          status: 0,
          data: null,
          message: "Request failed",
        });
        return null;
      })
      .finally(() => {
        setLoading(false);
      });
  };
  useEffect(() => {
    if (opts?.prefrech !== false) load();
  }, []);
  return {
    params,
    result,
    data: result.data,
    cursor: result.cursor,
    error: result.status == 0 ? result.message : null,
    loading,
    load,
  };
}
export default api;
