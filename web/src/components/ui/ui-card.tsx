export default function UICard({ children, className }: any) {
  return (
    <div
      className={`
        border-[1px] border-gray-200 
        p-4 rounded 
        bg-blue-50/20 
        box
        ${className}
      `}
    >
      {children}
    </div>
  );
}
