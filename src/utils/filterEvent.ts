import { EventData, TaskData } from '@/interfaces';
import moment from 'moment';
export const filterEvent = (rawData: EventData[] | TaskData[], title: string, timeUnit: number) => {
  const filteredData = rawData.filter((data) => {
    const startHeight = data.start;
    const endHeight = moment(title, 'HH:mm').unix();
    return endHeight - startHeight < timeUnit - 1 && endHeight >= startHeight;
  });
  return filteredData;
};
