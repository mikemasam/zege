import React, { useMemo, useState } from "react";

// Props for the generic table component
interface Column<T> {
  key: keyof T; // Accessor for data
  label: string; // Header label
  render?: (value: T[keyof T], row: T) => React.ReactNode; // Optional custom renderer
}

interface TableProps<T> {
  data: T[]; // Array of row data
  columns: Column<T>[]; // Column definitions
  className?: string; // Optional Tailwind classes for wrapper
}

export function UITable<T extends Record<string, unknown>>({
  data,
  columns,
  className = "",
}: TableProps<T>) {
  // Memoize sorted data if needed; extend for sorting later
  const tableData = useMemo(() => {
    if (!Array.isArray(data)) return [];
    return data;
  }, [data]);

  return (
    <div className={`overflow-x-auto ${className} border-l-2 border-blue-300`}>
      <table className="min-w-full divide-y divide-gray-200">
        <thead className="bg-gray-50">
          <tr>
            {columns.map((column) => (
              <th
                key={String(column.key)}
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                {column.label}
              </th>
            ))}
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {tableData.map((row, index) => (
            <tr key={index} className="hover:bg-gray-50">
              {columns.map((column) => (
                <td
                  key={String(column.key)}
                  className="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
                >
                  {column.render
                    ? column.render(row[column.key], row)
                    : String(row[column.key])}
                </td>
              ))}
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
