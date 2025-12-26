import Loading from "../loading";

type PageProps = {
  title?: string;
  children: React.ReactNode;
  loading?: boolean;
  className?: string;
};
export default function Page(props: PageProps) {
  return (
    <div className={`flex flex-col gap-4`}>
      <div className="p-4 bg-blue-50 border-b border-blue-300 rounded flex flex-row items-center justify-between">
        {!!props.title && (
          <h2 className="text-xl font-semibold">{props.title}</h2>
        )}
      </div>

      {props.loading && <Loading />}
      <div className={`flex flex-col p-2 gap-4 ${props.className ?? ""}`}>
        {!props.loading && props.children}
      </div>
    </div>
  );
}
