import { FormProvider, useForm } from "react-hook-form";

export default function UIForm({ defaultValues, className, children, onSubmit }: any) {
  const methods = useForm({ defaultValues });
  return (
    <FormProvider {...methods}>
      <form onSubmit={methods.handleSubmit(onSubmit)} className={className}>{children}</form>
    </FormProvider>
  );
}
