import Loading from "../loading";

type PageProps = {
  title?: string | null;
  desc?: string | null;
  children: React.ReactNode;
  loading?: boolean;
  className?: string;
};
export default function Page(props: PageProps) {
  return (
    <div className={`flex flex-col gap-2`}>
      <div className="p-4 bg-blue-50 border-b border-blue-300"></div>
      <div className="flex flex-col items-start px-4 py-3 m-2 box">
        <h1 className="text-lg font-semibold text-gray-900">{props.title}</h1>
        <p className="text-sm text-gray-500">{props.desc}</p>
      </div>

      {props.loading && <Loading />}
      <div className={`flex flex-col p-2 gap-4 ${props.className ?? ""}`}>
        {!props.loading && props.children}
      </div>
    </div>
  );
}
