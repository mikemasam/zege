import React, { useMemo, useState } from "react";
import { DateTime } from "luxon";

// Props for the generic table component
interface Column<T> {
  key: keyof T; // Accessor for data
  label: string; // Header label
  type?: "date" | "datetime";
  render?: (value: T[keyof T], row: T) => React.ReactNode; // Optional custom renderer
}
interface Action<T> {
  label: string; // Header label
  icon?: string;
  action?: (row: T) => void; // For clickable actions (e.g., onClick handler)
}

interface TableProps<T> {
  title?: string;
  data: T[]; // Array of row data
  columns: Column<T>[]; // Column definitions
  actions?: Action<T>[]; // Action definitions
  className?: string; // Optional Tailwind classes for wrapper
  children?: React.ReactNode;
}

export function UITable<T extends Record<string, unknown>>({
  title,
  data,
  columns,
  actions,
  className = "",
  children,
}: TableProps<T>) {
  // Memoize sorted data if needed; extend for sorting later
  const tableData = useMemo(() => {
    if (!Array.isArray(data)) return [];
    return data;
  }, [data]);

  return (
    <div className={`overflow-x-auto ${className} box`}>
      {!!title && (
        <div className="border-b border-gray-200 px-6 py-4 flex flex-row items-center">
          <h2 className="text-sm font-medium text-gray-700 uppercase tracking-wide">
            {title}
          </h2>
          {children}
        </div>
      )}
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
            {actions && actions.length > 0 && (
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                *
              </th>
            )}
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {tableData.map((row, index) => (
            <tr key={index} className="hover:bg-gray-50">
              {columns.map((col) => {
                if (col.type == "date") {
                  return (
                    <td
                      key={String(col.key)}
                      className="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
                    >
                      {DateTime.fromISO(String(row[col.key])).toFormat(
                        "yyyy-MM-dd",
                      )}
                    </td>
                  );
                }
                if (col.type == "datetime") {
                  return (
                    <td
                      key={String(col.key)}
                      className="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
                    >
                      {DateTime.fromISO(String(row[col.key])).toFormat(
                        "yyyy-MM-dd HH:mm:ss",
                      )}
                    </td>
                  );
                }
                if (col.render) {
                  return (
                    <td
                      key={String(col.key)}
                      className="px-4 py-2 flex flex-row items-center"
                    >
                      {col.render(row[col.key], row)}
                    </td>
                  );
                }
                return (
                  <td
                    key={String(col.key)}
                    className="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
                  >
                    {String(row[col.key])}
                  </td>
                );
              })}

              {actions?.map((act) => {
                return (
                  <td className="px-4 py-2">
                    <button
                      aria-label={act.label}
                      title={act.label}
                      className="text-blue-600 hover:underline cursor-pointer hover:shadow-2xl"
                      onClick={() => act.action?.(row)}
                    >
                      <span className="material-icons">{act.icon}</span>
                    </button>
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
