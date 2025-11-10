import { GradientTealBlue } from "@visx/gradient";
import { scaleLinear } from "@visx/scale";
import { LinePath } from "@visx/shape";
import { AxisLeft, AxisBottom } from "@visx/axis";
import { ParentSize } from "@visx/responsive";

const lineData = [
  { x: 0, y: 10 },
  { x: 1, y: 20 },
  { x: 2, y: 15 },
  { x: 3, y: 25 },
  { x: 4, y: 15 },
  { x: 5, y: 35 },
  { x: 6, y: 45 },
  { x: 7, y: 85 },
  { x: 8, y: 15 },
  { x: 9, y: 85 },
];

const margin = { top: 20, bottom: 30, left: 40, right: 20 };
export default function LineChart() {
  return (
    <div className="h-[500px]">
      <ParentSize>
        {({ width, height }) => {
          // Scales for line chart
          const xLineScale = scaleLinear({
            domain: [0, Math.max(...lineData.map((d) => d.x))],
            range: [margin.left, width - margin.right],
          });
          const yLineScale = scaleLinear({
            domain: [0, Math.max(...lineData.map((d) => d.y))],
            range: [height - margin.bottom, margin.top],
          });
          return (
            <svg width={width} height={height}>
              <GradientTealBlue id="teal" />
              <rect width={width} height={height} fill="url(#teal)" rx={4} />
              <AxisLeft
                stroke="white"
                tickStroke="white"
                tickLabelProps={{
                  fill: "white", // blue text
                }}
                scale={yLineScale}
                left={margin.left}
              />
              <AxisBottom
                stroke="white"
                tickStroke="white"
                tickLabelProps={{
                  fill: "white", // blue text
                }}
                scale={xLineScale}
                top={height - margin.bottom}
              />
              <LinePath
                data={lineData}
                x={(d) => xLineScale(d.x)}
                y={(d) => yLineScale(d.y)}
                stroke="white"
                strokeWidth={2}
              />
            </svg>
          );
        }}
      </ParentSize>
    </div>
  );
}
