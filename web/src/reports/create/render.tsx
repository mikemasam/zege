import { GradientTealBlue } from "@visx/gradient";
import { Group } from "@visx/group";
import { scaleLinear, scaleBand } from "@visx/scale";
import { LinePath, Bar } from "@visx/shape";
import { AxisLeft, AxisBottom } from "@visx/axis";
import { ParentSize } from "@visx/responsive";
import UICard from "@/components/Card";
import { Button } from "@/components/ui/button";

// Sample data
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

const barData = [
  { label: "A", value: 40 },
  { label: "B", value: 55 },
  { label: "C", value: 30 },
  { label: "D", value: 70 },
  { label: "e", value: 70 },
  { label: "f", value: 5 },
  { label: "g", value: 90 },
  { label: "h", value: 70 },
  { label: "i", value: 40 },
  { label: "j", value: 10 },
];
const margin = { top: 20, bottom: 30, left: 40, right: 20 };

export default function RenderCharts() {
  return (
    <div className="py-5 flex flex-col gap-5">
      <UICard className="flex flex-row justify-between items-center">
        <div className="text-lg">Preview</div>
        <div className="flex flex-row gap-1">
          <Button variant="outline">Data</Button>
          <Button variant="outline">Visual</Button>
        </div>
      </UICard>
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

      <UICard>
        <h3>Bar Chart</h3>
      </UICard>
      <div className="h-[500px]">
        <ParentSize>
          {({ width, height }) => {
            // Scales for bar chart
            const xBarScale = scaleBand({
              domain: barData.map((d) => d.label),
              range: [margin.left, width - margin.right],
              padding: 0.2,
            });
            const yBarScale = scaleLinear({
              domain: [0, Math.max(...barData.map((d) => d.value))],
              range: [height - margin.bottom, margin.top],
            });
            return (
              <svg width={width} height={height}>
                <GradientTealBlue id="teal" />
                <rect width={width} height={height} fill="url(#teal)" rx={4} />
                <AxisLeft
                  scale={yBarScale}
                  stroke="white"
                  tickStroke="white"
                  tickLabelProps={{
                    fill: "white", // blue text
                  }}
                  left={margin.left}
                />
                <AxisBottom
                  scale={xBarScale}
                  stroke="white"
                  tickStroke="white"
                  top={height - margin.bottom}
                  tickLabelProps={{
                    fill: "white", // blue text
                  }}
                />
                <Group>
                  {barData.map((d, i) => {
                    const barHeight =
                      height - margin.bottom - yBarScale(d.value);
                    return (
                      <Bar
                        key={`bar-${i}`}
                        x={xBarScale(d.label)}
                        y={yBarScale(d.value)}
                        width={xBarScale.bandwidth()}
                        height={barHeight}
                        fill="rgba(23, 233, 217, .8)"
                        rx={10}
                      />
                    );
                  })}
                </Group>
              </svg>
            );
          }}
        </ParentSize>
      </div>
    </div>
  );
}
