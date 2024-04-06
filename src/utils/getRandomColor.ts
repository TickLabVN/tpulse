const colorList = ['red', 'blue', 'green', 'yellow'];

export const getRandomColor = () => {
  const randomIndex = Math.floor(Math.random() * colorList.length);
  return colorList[randomIndex];
};
