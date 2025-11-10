import { GradientTealBlue } from "@visx/gradient";
import { Group } from "@visx/group";
import { scaleLinear, scaleBand } from "@visx/scale";
import { Bar } from "@visx/shape";
import { AxisLeft, AxisBottom } from "@visx/axis";
import { ParentSize } from "@visx/responsive";
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

export default function BarChart() {
  return (
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
                  const barHeight = height - margin.bottom - yBarScale(d.value);
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
  );
}
