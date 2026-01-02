import UICard from "@/components/ui/ui-card";
import UIInput from "@/components/ui/ui-input";
import { Button } from "@/components/ui/button";
import api from "@/lib/api";
import UIForm from "@/components/ui/ui-form";
import Page from "@/components/ui/ui-page";
import { useNavigate } from "react-router";
export default function UserCreate() {
  const nav = useNavigate();
  const onSubmit = async (form: any) => {
    const res = await api.post("/users", form);
    if (res.status != 201) return;
    nav("/app/users");
  };
  return (
    <Page title="New User" className="space-y-4">
      <UICard className="flex flex-col gap-2">
        <UIForm onSubmit={onSubmit} defaultValues={{}}>
          <UIInput
            label="Display Name"
            name="name"
            placeholder="Enter display name"
          ></UIInput>
          <UIInput label="Email" name="email" placeholder="Email"></UIInput>
          <UIInput label="Password" name="password" placeholder="********" />
          <div className="flex flex-row justify-end gap-2">
            <Button type="submit">Save</Button>
          </div>
        </UIForm>
      </UICard>
    </Page>
  );
}
