
interface MenuItemProps {
  icon: string;
  onClick?: () => void;
  className?: string;
}

export default function UIMenuItem({ icon, onClick, className = "" }: MenuItemProps) {
  return (
    <div
      onClick={onClick}
      className={`flex items-center justify-center w-10 h-10 rounded-lg
                  border border-gray-400 hover:border-blue-500
                  hover:bg-blue-50 cursor-pointer
                  transition-all duration-200 ${className}`}
    >
      <span className="material-icons text-gray-600 hover:text-blue-600 transition-colors duration-200">
        {icon}
      </span>
    </div>
  );
}
