import * as React from "react";

import { Input } from "./input";
import { useController } from "react-hook-form";

export default function UIInput({
  label,
  name,
  ...props
}: React.ComponentProps<"input"> & { label: string; name: string }) {
  const methods = useController({ name });
  return (
    <div className="grid w-full items-center gap-2 py-2">
      <label
        htmlFor={name}
        className="flex px-[2px] text-gray-600 items-center gap-2 leading-tight font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50"
      >
        {label}
      </label>
      <Input
        className="not-dark:bg-white focus-visible:ring-[1px]"
        name={name}
        value={methods.field.value}
        onChange={methods.field.onChange}
        {...props}
      />
    </div>
  );
}
