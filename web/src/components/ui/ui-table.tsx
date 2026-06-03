import { useMemo } from "react";
import { DateTime } from "luxon";
import { Link } from "react-router";

interface Column<T> {
  key: keyof T;
  label: string;
  type?: "date" | "datetime";
  render?: (value: T[keyof T], row: T) => React.ReactNode;
}
interface Action<T> {
  label: string;
  icon?: string;
  href?: (row: T) => string;
  action?: (row: T) => void;
}

interface TableProps<T> {
  title?: string;
  data: T[];
  columns: Column<T>[];
  actions?: Action<T>[];
  className?: string;
  children?: React.ReactNode;
}

const btnClass =
  "inline-flex items-center justify-center h-8 w-8 rounded-md border border-gray-200 bg-white text-gray-500 hover:bg-gray-100 hover:text-gray-700 hover:border-gray-300 active:scale-95 transition-all duration-150 focus:outline-none focus:ring-2 focus:ring-blue-500";

function formatVal(v: unknown): string {
  if (typeof v == "string" || typeof v == "number") return String(v);
  return "";
}

export function UITable<T extends Record<string, unknown>>({
  title,
  data,
  columns,
  actions,
  className = "",
  children,
}: TableProps<T>) {
  const tableData = useMemo(() => {
    if (!Array.isArray(data)) return [];
    return data;
  }, [data]);

  return (
    <div className={`overflow-x-auto rounded-lg border border-gray-200 bg-white ${className}`}>
      {!!title && (
        <div className="border-b border-gray-200 px-5 py-3.5 flex items-center gap-4">
          <h2 className="text-sm font-semibold text-gray-700">{title}</h2>
          {children && <div className="ml-auto">{children}</div>}
        </div>
      )}
      <table className="min-w-full">
        <thead>
          <tr className="bg-gray-50 border-b border-gray-200">
            {columns.map((column) => (
              <th
                key={String(column.key)}
                className="px-4 py-3 text-left text-xs font-semibold text-gray-500 uppercase tracking-wider border-l border-gray-200 first:border-l-0"
              >
                {column.label}
              </th>
            ))}
            {actions && actions.length > 0 && (
              <th className="px-4 py-3 text-right text-xs font-semibold text-gray-500 uppercase tracking-wider w-24 border-l border-gray-200 first:border-l-0">
                Actions
              </th>
            )}
          </tr>
        </thead>
        <tbody className="divide-y divide-gray-100">
          {tableData.map((row, index) => (
            <tr key={index} className="hover:bg-gray-50 transition-colors duration-150">
              {columns.map((col) => {
                let content: React.ReactNode;
                const val = row[col.key];
                if (col.render) {
                  content = col.render(val, row);
                } else if (col.type == "date") {
                  const dt = DateTime.fromISO(formatVal(val));
                  content = dt.isValid ? dt.toFormat("yyyy-MM-dd") : formatVal(val);
                } else if (col.type == "datetime") {
                  const dt = DateTime.fromISO(formatVal(val));
                  content = dt.isValid ? dt.toFormat("yyyy-MM-dd HH:mm:ss") : formatVal(val);
                } else {
                  content = formatVal(val);
                }
                return (
                  <td
                    key={String(col.key)}
                    className="px-4 py-3 whitespace-nowrap text-sm text-gray-700 border-l border-gray-100 first:border-l-0"
                  >
                    {content}
                  </td>
                );
              })}
              {actions && actions.length > 0 && (
                <td className="px-4 py-3 whitespace-nowrap text-right border-l border-gray-100">
                  <div className="inline-flex items-center gap-1.5">
                    {actions.map((act) =>
                      act.href ? (
                        <Link
                          key={act.label}
                          to={act.href(row)}
                          aria-label={act.label}
                          title={act.label}
                          className={btnClass}
                        >
                          <span className="material-icons !text-lg">{act.icon}</span>
                        </Link>
                      ) : (
                        <button
                          key={act.label}
                          aria-label={act.label}
                          title={act.label}
                          className={btnClass}
                          onClick={() => act.action?.(row)}
                        >
                          <span className="material-icons !text-lg">{act.icon}</span>
                        </button>
                      ),
                    )}
                  </div>
                </td>
              )}
            </tr>
          ))}
        </tbody>
      </table>
      {tableData.length === 0 && (
        <div className="flex flex-col items-center justify-center py-12 text-gray-400">
          <span className="material-icons !text-3xl mb-2 text-gray-300">database</span>
          <p className="text-sm font-medium">No data available</p>
        </div>
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
