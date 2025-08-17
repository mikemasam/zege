import React, { useState } from "react";
import UIInput from "@/components/Input";
import UIDropdown from "@/components/dropdown";
import { Button } from "@/components/ui/button";

export default function NewDatabaseConnectionForm() {
  const [formData, setFormData] = useState({
    name: "",
    type: "postgres",
    host: "",
    port: "",
    username: "",
    dbpassword: "",
    database: "",
  });

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>,
  ) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    console.log("Form submitted:", formData);
    // Send to API here
  };

  return (
    <form onSubmit={handleSubmit} className="max-w-md mx-auto p-6 space-y-4">
      <h2 className="text-xl font-bold">Database Connection</h2>
      <UIInput
        label="Name"
        name="name"
        value={formData.name}
        onChange={handleChange}
      />
      <UIDropdown
        label="Database Type"
        placeholder="Select Database"
        name="type"
        value={formData.type}
        onChange={handleChange}
        items={[
          { value: "postgres", label: "PostgreSQL" },
          { value: "mysql", label: "MySQL" },
          { value: "mariadb", label: "MariaDB" },
        ]}
      />

      <UIInput
        label="Database Host"
        name="host"
        value={formData.host}
        onChange={handleChange}
      />
      <UIInput
        label="Database Username"
        name="username"
        value={formData.username}
        onChange={handleChange}
      />
      <UIInput
        label="Database Password"
        name="dbpassword"
        value={formData.dbpassword}
        onChange={handleChange}
      />

      <UIInput
        label="Database Name"
        placeholder="Database Name"
        name="database"
        value={formData.database}
        onChange={handleChange}
      />
      <Button type="submit">Save Connection</Button>
    </form>
  );
}

function FormSelect({ items, label, name, value, onChange }: any) {
  return (
    <div>
      <label className="block text-sm font-medium">Database Type</label>
      <select
        name="type"
        value={value}
        onChange={onChange}
        className="text-sm block w-full border-2 border-gray-600 rounded-lg p-2 hover:outline-none focus:outline-none"
      >
        {items?.map?.((i: any) => <option value={i.value}>{i.label}</option>)}
      </select>
    </div>
  );
}
