import { useMemo } from "react";

interface TableProps<T> {
  data: T[]; // Array of row data
  className?: string; // Optional Tailwind classes for wrapper
}

export function TableView<T extends Record<string, unknown>>({
  data,
  className = "",
}: TableProps<T>) {
  // Memoize sorted data if needed; extend for sorting later
  const tableData = useMemo(() => {
    if (!Array.isArray(data) || !data?.length) return [];
    return data;
  }, [data]);
  const columns = useMemo(() => {
    if (!Array.isArray(data) || !data?.length) return [];
    return Object.keys(data[0]).map((c) => ({
      key: c,
      label: c,
    }));
  }, [data]);

  return (
    <div className={`overflow-x-auto rounded-lg border border-gray-200 ${className}`}>
      <table className="min-w-full">
        <thead>
          <tr className="bg-gray-50">
            <th className="px-3 py-3 text-left text-xs font-semibold text-gray-500 uppercase tracking-wider w-10">
              #
            </th>
            {columns.map((column) => (
              <th
                key={String(column.key)}
                className="px-3 py-3 text-left text-xs font-semibold text-gray-500 uppercase tracking-wider border-l border-gray-200"
              >
                {column.label}
              </th>
            ))}
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-100">
          {tableData.map((row, index) => (
            <tr key={index} className="hover:bg-gray-50 transition-colors duration-150">
              <td className="px-3 py-2.5 whitespace-nowrap text-sm text-gray-400">
                {index + 1}
              </td>
              {columns.map((column) => {
                const value = row[column.key];
                let _val: any = "";
                if (typeof value == "string" || typeof value == "number") {
                  _val = value;
                } else if (value === null) {
                  _val = "";
                }
                return (
                  <td
                    key={String(column.key)}
                    className="px-3 py-2.5 whitespace-nowrap text-sm text-gray-700 border-l border-gray-100 max-w-xs truncate"
                  >
                    {_val}
                  </td>
                );
              })}
            </tr>
          ))}
        </tbody>
      </table>
      {tableData.length === 0 && (
        <div className="flex flex-col items-center justify-center py-12 text-gray-400">
          <span className="material-icons !text-3xl mb-2 text-gray-300">table_rows</span>
          <p className="text-sm font-medium">No data available</p>
        </div>
      )}
    </div>
  );
}
