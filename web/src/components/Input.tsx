import * as React from "react";

import { Input } from "./ui/input";

export default function UIInput({
  label,
  name,
  ...props
}: React.ComponentProps<"input"> & { label: string }) {
  return (
    <div className="grid w-full max-w-sm items-center gap-3">
      <label
        htmlFor={name}
        className="flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50"
      >
        {label}
      </label>
      <Input name={name} {...props} />
    </div>
  );
}
