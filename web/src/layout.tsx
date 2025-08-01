import DataScreen from "./datarender/DataScreen";

export default function AppLayout() {
  return (
    <div className="flex h-screen border border-gray-600">
      <div className="w-80 border-r border-gray-600 flex flex-col overflow-x-hidden bg-primary">
        <div className="flex flex-row gap-2 p-2 overflow-scroll">
          <button>History</button>
          <button>Databases</button>
          <button>Collections</button>
        </div>
        Title
      </div>
      <div className="flex-1 flex">
        <DataScreen />
      </div>
    </div>
  );
}
