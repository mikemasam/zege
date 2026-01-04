import { create } from "zustand";

type NotifyLoadQueue = { key: string; message?: string };
type NotifyState = {
  queue: NotifyLoadQueue[];
  loading: boolean;
  add_task: (key?: string, message?: string) => string;
  remove_task: (key: string) => void;
};

export const useNotifyStore = create<NotifyState>((set, get) => ({
  queue: [],
  loading: false,
  add_task: (_key?: string, message?: string) => {
    const key = _key ?? String(Math.random());
    const _q = [
      ...get().queue,
      {
        key,
        message,
      },
    ];
    set({ queue: _q, loading: _q.length > 0 });
    return key;
  },
  remove_task: async (key: string) => {
    const _q = [...get().queue.filter((q) => q.key != key)];
    set({ queue: _q, loading: _q.length > 0 });
  },
}));
