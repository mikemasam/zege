import UICard from "@/components/ui/ui-card";
import UIDropdown from "@/components/ui/ui-dropdown";
import UIInput from "@/components/ui/ui-input";
import UITextAreaInput from "@/components/ui/ui-text-area-input";
import { Button } from "@/components/ui/button";
import api, { useApi } from "@/lib/api";
import UIForm from "@/components/ui/ui-form";
import Page from "@/components/ui/ui-page";
import { useEffect, useState } from "react";
import { useParams } from "react-router";
import ReportRender from "@/components/report/report.render";

export default function ZegeReportEditor() {
  const [id, setId] = useState(null);
  return (
    <Page title="New Zege Report" className="space-y-4">
      <ReportForm onChange={setId} />
      {id && <ReportRender report_id={id} />}
    </Page>
  );
}

const reportTypes = [
  { label: "Table", value: "table" },
  { label: "Tiles", value: "tiles" },
  { label: "Bar", value: "bar" },
  { label: "Line", value: "line" },
];
function ReportForm({ onChange }: any) {
  const params = useParams();
  const [defaultValues, setDefaultValues] = useState<any>(undefined);
  const { data: report, load } = useApi(
    (params: any) => api.get(`/reports/${params.id}`),
    { prefrech: false },
  );
  useEffect(() => {
    if (params.id) {
      onChange(params?.id);
      console.log("loading edit report");
      load(params).then((r) => {
        if (!r?.data) return;
        setDefaultValues(() => ({
          report_name: r.data.report_name,
          report_type: r.data.report_type,
          report_sql: r.data.report_sql,
        }));
      });
    } else {
      console.log("default values for new report");
      setDefaultValues(() => ({
        report_name: "test",
        report_type: "",
        report_sql: "",
      }));
    }
  }, []);
  const onSubmit = async (form: any, e: any) => {
    onChange(null);
    if (report) {
      form.id = report.id;
    }
    const res = await api.post("/reports", form);
    console.log(res);
    if (res.status != 201) return;
    load({ id: res.data.id });
    onChange(res.data.id);
  };
  console.log(report, defaultValues);
  if (defaultValues == undefined) return null;
  return (
    <UICard className="flex flex-col gap-2">
      <UIForm onSubmit={onSubmit} defaultValues={defaultValues}>
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
          <Button type="submit">Save</Button>
        </div>
      </UIForm>
    </UICard>
  );
}
