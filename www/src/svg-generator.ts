import { angle_of_timestamp as angleOfTimestamp } from "roman-sunclock";

const CANVAS_X_CENTER = 125;
const CANVAS_Y_CENTER = 125;
const NINETY_DEGREE_IN_RAD = Math.PI / 2;

function calculatePoint(w: number, radius: number): [number, number] {
  return [
    Math.cos(w - NINETY_DEGREE_IN_RAD) * radius + CANVAS_X_CENTER,
    Math.sin(w - NINETY_DEGREE_IN_RAD) * radius + CANVAS_Y_CENTER,
  ];
}

function generateRomanHourLines(initialW: number, step: number): string[] {
  const content: string[] = [];
  for (let i = 1; i < 12; i++) {
    const w = initialW + i * step;
    const length = i % 3 === 0 ? 107 : 113;
    const [x1, y1] = calculatePoint(w, 119);
    const [x2, y2] = calculatePoint(w, length);
    content.push(`<line x1="${x1}" y1="${y1}" x2="${x2}" y2="${y2}" />`);
  }
  return content;
}

function generateLocalHourLines(): string[] {
  const content: string[] = [];
  for (let i = 0; i < 24; i++) {
    const w = (i * Math.PI) / 12;
    const length = i % 3 === 0 ? 80 : 85;
    const [x1, y1] = calculatePoint(w, 89);
    const [x2, y2] = calculatePoint(w, length);
    content.push(`<line x1="${x1}" y1="${y1}" x2="${x2}" y2="${y2}" />`);
  }
  return content;
}

export function generateSvg(
  nowTime: number,
  sunriseEpoch: number,
  sunsetEpoch: number
) {
  const FULL_CIRCLE = Math.PI * 2;

  const timezoneOffset = new Date().getTimezoneOffset() * 60 * 1000;
  const nowW = angleOfTimestamp(BigInt(nowTime - timezoneOffset));
  const sunriseW = angleOfTimestamp(
    BigInt(sunriseEpoch - timezoneOffset)
  );
  const sunsetW = angleOfTimestamp(BigInt(sunsetEpoch - timezoneOffset));

  const [nowPointX, nowPointY] = calculatePoint(nowW, 102);
  const [sunriseX, sunriseY] = calculatePoint(sunriseW, 105);
  const [sunsetX, sunsetY] = calculatePoint(sunsetW, 105);

  const isDayLonger = Math.PI < sunsetW - sunriseW;

  const svgContent: string[] = [];
  svgContent.push('<svg viewBox="0 0 250 250" fill="transparent">');
  svgContent.push(
    `<path d="M ${sunsetX} ${sunsetY} A 105 105 0 ${
      isDayLonger ? "0 1" : "1 1"
    } ${sunriseX} ${sunriseY}" stroke="var(--night-color)" stroke-width="30" />`
  );
  svgContent.push(
    `<path d="M ${sunriseX} ${sunriseY} A 105 105 0 ${
      isDayLonger ? "1 1" : "0 1"
    } ${sunsetX} ${sunsetY}" stroke="var(--day-color)" stroke-width="30" />`
  );
  svgContent.push('<g stroke="var(--main-color)">');
  svgContent.push(
    `<circle cx=\"${CANVAS_X_CENTER}\" cy=\"${CANVAS_Y_CENTER}\" r=\"90\" />`
  );
  svgContent.push(
    `<circle cx=\"${CANVAS_X_CENTER}\" cy=\"${CANVAS_Y_CENTER}\" r=\"120\" />`
  );
  svgContent.push(...generateLocalHourLines());
  svgContent.push('<g stroke="var(--night-color)">');
  svgContent.push(
    ...generateRomanHourLines(sunriseW, (sunsetW - sunriseW) / 12)
  );
  svgContent.push("</g>");
  svgContent.push('<g stroke="var(--day-color)">');
  svgContent.push(
    ...generateRomanHourLines(
      sunsetW,
      (FULL_CIRCLE - (sunsetW - sunriseW)) / 12
    )
  );
  svgContent.push("</g>");
  svgContent.push("</g>");
  svgContent.push(
    `<circle cx="${nowPointX}" cy="${nowPointY}" r="4" fill="red" />`
  );
  // svgContent.push(
  //   `<circle cx=\"${sunsetX}\" cy=\"${sunsetY}\" r=\"4\" fill=\"purple\" />`
  // );
  // svgContent.push(
  //   `<circle cx=\"${sunriseX}\" cy=\"${sunriseY}\" r=\"4\" fill=\"orange\" />`
  // );
  svgContent.push("</svg>");
  return svgContent.join("");
}
