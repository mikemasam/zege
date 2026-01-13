interface TileItem {
  label: React.ReactNode;
  value: React.ReactNode;
  hint?: React.ReactNode;
}

interface TileViewProps {
  data: TileItem[];
  className?: string;
}

export function TileView({ data, className = "" }: TileViewProps) {
  if (!data?.length) return null;

  return (
    <div
      className={`
        grid gap-4
        grid-cols-[repeat(auto-fill,minmax(220px,1fr))]
        ${className}
      `}
    >
      {data.map((item, i) => (
        <div
          key={i}
          className="
            relative
            rounded-lg
            bg-white
            px-5 py-4
            shadow-sm
            ring-1 ring-slate-200
            transition
            hover:shadow-md
          "
        >
          {/* Label */}
          <div className="text-xs font-medium uppercase tracking-wide text-slate-500">
            {item.label}
          </div>

          {/* Value */}
          <div className="mt-2 text-3xl font-semibold text-slate-900">
            {item.value}
          </div>

          {/* Optional hint */}
          {item.hint && (
            <div className="mt-1 text-xs text-slate-400">{item.hint}</div>
          )}
        </div>
      ))}
    </div>
  );
}
