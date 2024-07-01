export const prettyTime = (time: number) => {
  const hours = Math.floor(time / 3600);
  const minutes = Math.floor((time % 3600) / 60);

  let str = `${hours}h`;
  if (minutes > 0) {
    str += `${minutes}m`;
  }
  return str;
};
