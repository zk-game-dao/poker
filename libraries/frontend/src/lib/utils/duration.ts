export const secondsToString = (seconds: bigint): string => {
  const hours = Math.floor(Number(seconds) / 3600);
  const minutes = Math.floor((Number(seconds) % 3600) / 60);
  const remainingSeconds = Number(seconds) % 60;

  return `${hours > 0 ? `${hours}h ` : ""}${minutes > 0 ? `${minutes}m ` : ""}${remainingSeconds}s`;
};

export const nanosecondsToString = (nanoseconds: bigint): string =>
  secondsToString(nanoseconds / 1_000_000_000n);
