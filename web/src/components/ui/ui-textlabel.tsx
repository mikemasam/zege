
export default function TextLabel({ label, desc }: { label?: string; desc?: string }) {
  return (
    <div className="flex flex-col gap-0.5">
      <span className="text-xs font-medium text-gray-400 uppercase tracking-wider">{label}</span>
      <span className="text-sm font-semibold text-gray-800 truncate">{desc}</span>
    </div>
  );
}
