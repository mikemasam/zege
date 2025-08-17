import { useMemo } from "react";
import { useDataStore } from "./data.state";

type Row = Cell[];
type Cell = {
  index: number;
  name: string;
  type_name: string;
  value: any;
};
export default function TableView() {
  const { result, error } = useDataStore();

  const table = useMemo(() => {
    if (!Array.isArray(result?.data))
      return {
        items: [],
        columns: [],
        count: 0,
      };
    const items: Row[] = result.data;
    //const o = [];
    //for(let i = 0; i < result.data.length;i++){
    //let item = result.data[i];
    //}
    const item = items[0] ?? [];

    const columns = item
      .sort((a, b) => a.index - b.index)
      .map((col) => ({
        index: col.index,
        name: col.name,
        type_name: col.type_name,
      }));
    return {
      items,
      columns,
      count: items.length,
    };
  }, [result]);
  console.log(table);
  return (
    <div>
      <div className="m-2 h-[44vh] overflow-scroll table-view">
        {!!error && <div className="px-2 text-red-400 font-bold">{error}</div>}
        <table className="border-none w-full">
          <thead>
            <tr>
              <th className="text-center">{"+"}</th>
              {table.columns.map((c) => (
                <th>{c.name}</th>
              ))}
            </tr>
          </thead>
          <tbody className="overflow-scroll">
            {table.items.map((item) => (
              <tr>
                <td className="text-center">{"+"}</td>
                {item.map((c) => (
                  <td title={c.value}>{c.value}</td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <div className="flex flex-row justify-end">
      <div className="px-2 italic text-gray-400 font-bold">{table?.count}</div>
      </div>
    </div>
  );
}
