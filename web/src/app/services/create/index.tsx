import UICard from "@/components/ui/ui-card";
import UIInput from "@/components/ui/ui-input";
import { Button } from "@/components/ui/button";
import api from "@/lib/api";
import UIForm from "@/components/ui/ui-form";
import Page from "@/components/ui/ui-page";
import { useNavigate } from "react-router";
export default function ServiceCreate() {
  const nav = useNavigate();
  const onSubmit = async (form: any) => {
    const res = await api.post("/services", form);
    if (res.status != 201) return;
    nav("/app/services");
  };
  return (
    <Page title="New Service" className="space-y-4">
      <UICard className="flex flex-col gap-2">
        <UIForm onSubmit={onSubmit} defaultValues={{}}>
          <UIInput label="Name" name="name" placeholder="Enter name"></UIInput>
          <UIInput
            label="Display Label"
            name="label"
            placeholder="Service key"
          ></UIInput>
          <UIInput
            label="Description"
            name="description"
            placeholder="Service Description"
          ></UIInput>
          <div className="flex flex-row justify-end gap-2">
            <Button type="submit">Save</Button>
          </div>
        </UIForm>
      </UICard>
    </Page>
  );
}
