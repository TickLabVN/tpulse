export const prettyTime = (time: number) => {
  const hours = Math.floor(time / 3600);
  const minutes = Math.floor((time % 3600) / 60);
  const seconds = Math.floor(time % 60);

  let str = '';
  if (hours > 0) str += `${hours}h`;
  if (minutes > 0) str += `${minutes}m`;
  if (seconds > 0) str += `${seconds}s`;
  return str;
};
