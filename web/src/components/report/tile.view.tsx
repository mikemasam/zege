interface TileItem {
  label: React.ReactNode;
  value: React.ReactNode;
  hint?: React.ReactNode;
}

interface TileViewProps {
  data: TileItem[];
  className?: string;
}

function formatValue(v: React.ReactNode): React.ReactNode {
  if (typeof v == "number") {
    return v.toLocaleString();
  }
  return v;
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
            className="rounded-xl bg-white border border-gray-100 shadow-sm hover:shadow-lg hover:-translate-y-0.5 transition-all duration-200 p-5 flex flex-col justify-center min-h-[110px] overflow-hidden cursor-pointer"
          >
            <div className="text-xs font-semibold uppercase tracking-wider text-gray-500 mb-1.5 truncate">
              {item.label}
            </div>
            <div className="text-3xl font-bold text-gray-900 tracking-tight leading-none truncate">
              {formatValue(item.value)}
            </div>
            {item.hint && (
              <div className="mt-2 text-xs text-gray-400 truncate">{item.hint}</div>
            )}
          </div>
      ))}
    </div>
  );
}
