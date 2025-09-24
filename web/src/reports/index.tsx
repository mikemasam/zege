import UICard from "@/components/Card";
import { Button } from "@/components/ui/button";
import { Link } from "react-router";

export default function ZegeReports() {
  return (
    <div>
      <UICard className="flex flex-row justify-between ">
        <span>Zege report</span>
        <Link
          to="/reports/new"
        >
          <Button>New Report</Button>
        </Link>
      </UICard>
    </div>
  );
}
