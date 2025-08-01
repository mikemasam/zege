export default function TableView() {
  return (
    <div className="border border-gray-500 m-2 h-[450px]">
      <table className="table-auto border-none w-full table-view">
        <thead className="">
          <tr>
            <th>Name</th>
            <th>Age</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>Victor</td>
            <td>28</td>
          </tr>
        </tbody>
      </table>
    </div>
  );
}
