export default function Loading() {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/5 backdrop-blur-sm animate-fade-in">
      <div className="relative flex flex-row gap-10 items-center">
        <div className="h-12 w-12 animate-spin rounded-full border-4 border-gray-300 border-t-gray-900" />
        <div className="absolute inset-0 h-12 w-12 rounded-full animate-ping border border-gray-400/40" />
        <div className="text-gray-500 font-bold">Loading...</div>
      </div>
    </div>
  );
}
