export function asNumber(bigint: BigInt): number {
  return Number(bigint.toString());
}

export function formatClockTime(hours: number, minutes: number): string {
  return `${hours.toString().padStart(2, "0")}:${minutes
    .toString()
    .padStart(2, "0")}`;
}
