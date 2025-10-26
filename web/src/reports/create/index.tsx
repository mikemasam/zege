import UICard from "@/components/Card";
import UIDropdown from "@/components/dropdown";
import UIInput from "@/components/Input";
import UITextAreaInput from "@/components/TextAreaInput";
import { Button } from "@/components/ui/button";
import RenderCharts from "./render";
import { useForm } from "react-hook-form";
import api from "@/lib/api";
import type { FormEvent } from "react";
import UIForm from "@/components/ui-form";

export default function CreateZegeReport() {
  return (
    <div className="flex flex-col gap-4 p-2">
      <UICard>
        <div>Create Zege report</div>
      </UICard>
      <ReportForm />
      <RenderCharts />
    </div>
  );
}

const reportTypes = [
  { label: "Bar", value: "bar" },
  { label: "Line", value: "line" },
  { label: "Table", value: "table" },
];
function ReportForm() {
  const onSubmit = async (form: any, e: any) => {
    console.log(form);
    const res = await api.post("/reports", form);
    console.log(res);
  };
  return (
    <UICard className="flex flex-col gap-2">
      <UIForm
        onSubmit={onSubmit}
        defaultValues={{
          report_name: "test",
          report_type: "",
          report_sql: "",
        }}
      >
        <UIInput
          label="Name"
          name="report_name"
          placeholder="Enter report name"
        ></UIInput>
        <UIDropdown
          label="Type"
          name="report_type"
          placeholder="Select report type"
          items={reportTypes}
        />
        <UITextAreaInput
          label="SQL"
          name="report_sql"
          placeholder="Enter report sql"
        ></UITextAreaInput>
        <div className="flex flex-row justify-end gap-2">
          <Button variant="outline">Preview</Button>
          <Button type="submit">Save</Button>
        </div>
      </UIForm>
    </UICard>
  );
}
