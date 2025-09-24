import * as React from "react";

import { Input } from "./ui/input";

export default function UIInput({
  label,
  name,
  ...props
}: React.ComponentProps<"input"> & { label: string }) {
  return (
    <div className="grid w-full items-center gap-2 py-2">
      <label
        htmlFor={name}
        className="flex px-[2px] text-gray-600 items-center gap-2 leading-tight font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50"
      >
        {label}
      </label>
      <Input className="bg-white focus-visible:ring-[1px]" name={name} {...props} />
    </div>
  );
}
