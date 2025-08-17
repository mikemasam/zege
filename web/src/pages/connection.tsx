import UIDropdown from "@/components/dropdown";
import { Button } from "@/components/ui/button";
import { useLocation } from "wouter";

const tables = [{ name: "users" }, { name: "products" }, { name: "orders" }];
export default function DatabaseConnection() {
  return (
    <div className="p-2 flex flex-col gap-4 overflow-auto h-full">
      <DbConnection />
      <DbSchema />
      <DbObject />
      <TableList
        tables={tables}
        onView={(name: any) => console.log("View table:", name)}
      />
    </div>
  );
}

function TableList({ tables, onView }: any) {
  return (
    <div className="overflow-x-auto overflow-y-auto border-2 border-muted h-full flex-1">
      <table className="min-w-full divide-y divide-muted rounded-lg">
        <tbody className="divide-y border-secondary">
          {tables.map((table: any, idx: any) => (
            <tr
              key={idx}
              className="hover:bg-muted transition-colors cursor-pointer"
            >
              <td className="px-2 py-1 whitespace-nowrap font-mono text-sm font-medium">
                {table.name}
              </td>
              <td className="px-2 py-1 whitespace-nowrap text-right text-sm">
                <span
                  className="material-icons text-sm"
                  onClick={() => onView(table.name)}
                >
                  visibility
                </span>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function DbConnection() {
  const [, navigate] = useLocation();
  const cons = [{ label: "Deba Backup", value: "debaback" }];
  return (
    <div className="flex flex-row items-end gap-2">
      <UIDropdown
        label="Connection"
        name="connection"
        placeholder="Select Connection"
        items={cons}
      />
      <Button
        variant="ghost"
        size="sm"
        className="mb-1"
        onClick={() => navigate("/connections/new")}
      >
        <span className="material-icons">add</span>
      </Button>
    </div>
  );
}

function DbSchema() {
  const cons = [{ label: "public", value: "public" }];
  return (
    <div className="flex flex-row items-end gap-2">
      <UIDropdown
        label="Schema"
        name="schema"
        placeholder="Select Schema"
        items={cons}
      />
    </div>
  );
}

function DbObject() {
  const cons = [{ label: "Tables", value: "table" }];
  return (
    <div className="flex flex-row items-end gap-2">
      <UIDropdown
        label="Object"
        name="object"
        placeholder="Select Object"
        items={cons}
      />
    </div>
  );
}
