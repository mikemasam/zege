import api, { useApi } from "@/lib/api";
import { useState, useMemo, useEffect, useRef } from "react";

export type useReportType = any;
type UseReportOps = {
  report_id?: number;
  autorefresh?: boolean;
};
export function useReport(ops?: UseReportOps): useReportType {
  const [type, setType] = useState("");
  const query = useApi(
    (params: any) =>
      api.post(`/reports/${params.report_id}/read`, {}, {
        meta: {
          notify: ops?.autorefresh !== true,
        },
      } as any),
    {
      prefrech: false,
    },
  );
  const [report, data] = useMemo(() => {
    const output = query.data;
    if (!output) return [null, null];
    return [output.report, output.data];
  }, [query.data]);
  useEffect(() => {
    if (!report) return;
    setType(report.report_type);
  }, [report]);
  useEffect(() => {
    if (ops?.report_id) {
      query.load({ report_id: ops.report_id });
    }
  }, [ops?.report_id]);
  usePolling(query.load, ops?.autorefresh ? 1000 * 4 : 0);
  return { report, type, data, query };
}

export function usePolling(clb: () => void, delay = 5000) {
  const ref = useRef({ clb: clb });
  ref.current.clb = clb;
  useEffect(() => {
    if (delay < 1000) return;
    const id = setInterval(() => {
      ref.current.clb?.();
    }, delay);
    return () => clearInterval(id);
  }, [delay]);
}
