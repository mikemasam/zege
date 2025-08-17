import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./ui/select";

export default function UIDropdown({ placeholder, name, label, items }: any) {
  return (
    <div className="grid w-full max-w-sm items-center gap-3">
      <label
        htmlFor={name}
        className="ml-1 font-medium flex items-center gap-2 text-sm leading-none select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50"
      >
        {label}
      </label>
      <Select name={name}>
        <SelectTrigger className="w-full">
          <SelectValue placeholder={placeholder} />
        </SelectTrigger>
        <SelectContent>
          {items?.map((item: any) => (
            <SelectItem value={item.value}>{item.label}</SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
}
