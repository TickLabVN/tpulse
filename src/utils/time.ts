export const formatTime = (time: number) => {
  const hours = Math.floor(time / 3600);
  const minutes = Math.floor((time % 3600) / 60);
  const seconds = time % 60;

  return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
};

export const convertTimeToNumber = (time: string) => {
  const timeComponents = time.split(':').map((component) => parseInt(component, 10) || 0);
  return (
    timeComponents[0] +
    timeComponents[1] / 60 +
    (timeComponents[2] || 0) / 3600 +
    (timeComponents[3] || 0) / 3600000
  );
};

export const convertNumberToTime = (currentHour: number) => {
  const formattedHour = Math.floor(currentHour).toString().padStart(2, '0');
  const formattedMinute = Math.floor((currentHour % 1) * 60)
    .toString()
    .padStart(2, '0');
  const formattedSecond = Math.floor(((currentHour * 60) % 1) * 60)
    .toString()
    .padStart(2, '0');
  const formattedMillisecond = Math.floor(((currentHour * 60 * 60) % 1) * 1000)
    .toString()
    .padStart(3, '0');
  let formattedTime = formattedHour + ':' + formattedMinute;
  formattedTime = formattedSecond === '000' ? formattedTime + ':' + formattedSecond : formattedTime;
  formattedTime = formattedMillisecond === '000' ? formattedTime : formattedTime + '.' + formattedMillisecond;
  return formattedTime;
};
