import UICard from "@/components/Card";
import UIDropdown from "@/components/dropdown";
import UIInput from "@/components/Input";
import UITextAreaInput from "@/components/TextAreaInput";
import { Button } from "@/components/ui/button";
import RenderCharts from "./render";

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
  return (
    <UICard className="flex flex-col gap-2">
      <UIInput label="Name" placeholder="Enter report name"></UIInput>
      <UIDropdown
        label="Type"
        placeholder="Select report type"
        items={reportTypes}
      />
      <UITextAreaInput
        label="SQL"
        placeholder="Enter report sql"
      ></UITextAreaInput>
      <div className="flex flex-row justify-end gap-2">
        <Button variant="outline">Preview</Button>
        <Button>Save</Button>
      </div>
    </UICard>
  );
}

