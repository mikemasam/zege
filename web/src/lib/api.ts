import axios, { AxiosError } from "axios";
import { useEffect, useState } from "react";

const api = axios.create({
  baseURL: import.meta.env.API_URL || "http://localhost:3432/api/v1",
  timeout: 10000,
  headers: {
    "Content-Type": "application/json",
  },
});

// Request interceptor: attach auth token
api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem("token");
    if (token && config.headers) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error),
);

// Response interceptor: handle errors / auto logout on 401
api.interceptors.response.use(
  (response) => response.data,
  (error: AxiosError) => {
    if (error.response?.status === 401) {
      // optional: remove token and redirect to login
      localStorage.removeItem("token");
      window.location.href = "/login";
    }
    return Promise.reject(error);
  },
);

export function useApi(
  callback: any,
  opts?: { prefrech?: boolean },
  params?: any,
) {
  const [result, setResult] = useState({
    status: 0,
    data: null,
    message: "",
  } as {
    status: number;
    data: any;
    message: string;
  });
  const [loading, setLoading] = useState(false);
  const load = async (params?: any) => {
    setLoading(true);
    return Promise.resolve(callback(params))
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
  useEffect(
    () => {
      if (opts?.prefrech !== false) load();
    },
    Array.isArray(params) ? params : [],
  );
  return {
    result,
    data: result.data,
    error: result.status == 0 ? result.message : null,
    loading,
    load,
  };
}
export default api;
