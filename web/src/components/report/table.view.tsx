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
    <div className={`overflow-x-auto ${className} border-l-2 border-blue-300`}>
      <table className="min-w-full divide-y divide-gray-200">
        <thead className="bg-gray-50">
          <tr>
            <th className="px-2 py-1 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              *
            </th>
            {columns.map((column) => (
              <th
                key={String(column.key)}
                className="px-2 py-1 text-left text-xs font-medium text-gray-500 uppercase tracking-wider border-l-2 border-gray-200"
              >
                {column.label}
              </th>
            ))}
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {tableData.map((row, index) => (
            <tr key={index} className="hover:bg-gray-50">
              <td className="px-2 py-1 whitespace-nowrap text-sm text-gray-900">
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
                    className="px-2 py-1 whitespace-nowrap text-sm text-gray-900  border-l-2 border-gray-200"
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
        <div className="text-center py-4 text-gray-500">No data available</div>
      )}
    </div>
  );
}

// Usage Example:
// const users = [
//   { id: 1, name: 'John Doe', email: 'john@example.com', age: 30 },
//   { id: 2, name: 'Jane Smith', email: 'jane@example.com', age: 25 },
// ];
//
// const columns = [
//   { key: 'name', label: 'Name' },
//   { key: 'email', label: 'Email' },
//   { key: 'age', label: 'Age' },
//   {
//     key: 'actions',
//     label: 'Actions',
//     render: (value, row) => <button className="text-blue-600">Edit</button>,
//   },
// ];
//
// <GenericTable data={users} columns={columns} />
