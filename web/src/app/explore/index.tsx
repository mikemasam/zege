import { Button } from "@/components/ui/button";
import api, { useApi } from "@/lib/api";
import UIForm from "@/components/ui/ui-form";
import Page from "@/components/ui/ui-page";
import { SqlEditor } from "@/components/sqleditor";
import { TableView } from "@/components/report/table.view";
import { useEffect } from "react";

const default_sql =
  "select timestamp, event_name, event_type, host, message, service from zege_events order by timestamp desc limit 50";
export default function DataExplore() {
  const query = useApi((params: any) => api.post(`/data/execute`, params), {
    prefrech: false,
  });
  const onSubmit = async (form: any) => {
    query.load({ sql: form.sql });
  };
  useEffect(() => {
    onSubmit({ sql: default_sql });
  }, []);
  return (
    <Page title="Data Explore" className="!gap-1">
      <UIForm onSubmit={onSubmit} defaultValues={{ sql: default_sql }}>
        <div className="flex flex-col gap-1">
          <div className="overflow-hidden box">
            <SqlEditor name="sql" />
          </div>
          <div className="flex flex-row justify-end gap-2 box p-2">
            <Button type="submit">Run</Button>
          </div>
        </div>
      </UIForm>
      <TableView data={query.data?.data ?? []} />
    </Page>
  );
}
