import * as React from "react";

import { cn } from "@/lib/utils";
import { useController } from "react-hook-form";

export default function UITextAreaInput({
  label,
  name,
  className,
  ...props
}: React.ComponentProps<"textarea"> & { label: string; name: string }) {
  const methods = useController({ name });
  return (
    <div className="grid w-full items-center gap-2 py-2">
      <label
        htmlFor={name}
        className="flex px-[2px] items-center gap-2 leading-tight font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50"
      >
        {label}
      </label>
      <textarea
        name={name}
        data-slot="input"
        rows={5}
        className={cn(
          "bg-white file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input flex  w-full min-w-0 rounded-md border px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50",
          "focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[1px]",
          "aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
          className,
        )}
        value={methods.field.value}
        onChange={methods.field.onChange}
        {...props}
      />
    </div>
  );
}
