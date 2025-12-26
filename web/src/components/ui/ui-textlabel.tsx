
export default function TextLabel({ label, desc }: { label?: string; desc?: string }) {
  return (
    <div className="flex flex-col gap-1">
      <label className="text-sm text-gray-500 tracking-snug">{label}</label>
      <span className="text-sm text-gray-600 leading-wide font-bold">
        {desc}
      </span>
    </div>
  );
}
