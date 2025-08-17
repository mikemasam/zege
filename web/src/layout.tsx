import DataScreen from "./datarender/DataScreen";
import Routing from "./routing";

export default function AppLayout() {
  return (
    <div className="flex h-screen border border-gray-600 overflow-hidden">
      <div className="w-80 border-r border-gray-600 flex flex-col overflow-hidden">
        <Routing />
      </div>
      <div className="flex-1 flex overflow-hidden">
        <DataScreen />
      </div>
    </div>
  );
}
