import axios from "axios";
import { create } from "zustand";

type DataStore = {
  result: any;
  error: string | null;
};
export const useDataStore = create<DataStore>((set) => ({
  result: [],
  error: null,
  //increasePopulation: () => set((state: any) => ({ bears: state.bears + 1 })),
  //removeAllBears: () => set({ bears: 0 }),
}));

export const execQuery = async (sql: string | undefined) => {
  if (!sql) return;
  const payload = {
    sql,
    connection_id: "sample",
  };
  const res = await axios.post("http://localhost:3000/editor/query", payload);
  useDataStore.setState({
    result: res.data?.data,
    error: res.data?.error,
  });
  //console.log(res.data);
};
