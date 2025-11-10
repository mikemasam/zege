export default function Page({ title, children, className }: any) {
  return (
    <div className={`flex flex-col gap-4 ${className ?? ""}`}>
      <div className="p-4 bg-blue-100 border border-blue-300 rounded-md flex flex-row items-center justify-between">
        <h2 className="text-xl font-semibold">{title}</h2>
      </div>
      {children}
    </div>
  );
}
