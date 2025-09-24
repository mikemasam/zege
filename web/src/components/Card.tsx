
export default function UICard({ children, className }: any) {
  return (
    <div
      className={`
        border border-gray-200 
        p-4 rounded 
        bg-blue-50/20 
        shadow shadow-blue-50 
        ${className}
      `}
    >
      {children}
    </div>
  );
}
