import { FormProvider, useForm } from "react-hook-form";

export default function UIForm({ defaultValues, children, onSubmit }: any) {
  const methods = useForm({ defaultValues });
  return (
    <FormProvider {...methods}>
      <form onSubmit={methods.handleSubmit(onSubmit)}>{children}</form>
    </FormProvider>
  );
}
