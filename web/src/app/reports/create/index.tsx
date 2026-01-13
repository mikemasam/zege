import UIDropdown from "@/components/ui/ui-dropdown";
import UIInput from "@/components/ui/ui-input";
import { Button } from "@/components/ui/button";
import api, { useApi } from "@/lib/api";
import UIForm from "@/components/ui/ui-form";
import Page from "@/components/ui/ui-page";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";
import ReportRender from "@/components/report/report.render";
import { SqlEditor } from "@/components/sqleditor";

export default function ZegeReportEditor() {
  const [id, setId] = useState(null);
  //{id && <ReportRender report_id={id} />}
  return (
    <Page title="New Report" className="space-y-4">
      <ReportForm onChange={setId} />
    </Page>
  );
}

const reportTypes = [
  { label: "Table", value: "table" },
  { label: "Tile", value: "tile" },
  { label: "Bar", value: "bar" },
  { label: "Line", value: "line" },
];
function ReportForm({ onChange }: any) {
  const navigate = useNavigate();
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
    navigate(`/app/reports/${res.data.id}`);
    //load({ id: res.data.id });
    //onChange(res.data.id);
  };
  console.log(report, defaultValues);
  if (defaultValues == undefined) return null;
  return (
    <UIForm onSubmit={onSubmit} defaultValues={defaultValues}>
      <div className="flex flex-col gap-4">
        <div className="flex flex-col gap-2 box p-2">
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
        </div>
        <div className="overflow-hidden box">
          <SqlEditor name="report_sql" />
        </div>
        <div className="flex flex-row justify-end gap-2 box p-2">
          <Button type="submit">Save</Button>
        </div>
      </div>
    </UIForm>
  );
}
