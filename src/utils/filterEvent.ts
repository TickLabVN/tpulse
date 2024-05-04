import { EventData, TaskData } from '@/interfaces';
import moment from 'moment';
export const filterEvent = (
  rawData: EventData[] | TaskData[] | undefined,
  title: string,
  timeUnit: number
) => {
  if (!rawData) return [];
  const filteredData = rawData.filter((data) => {
    const startHeight = 'from' in data ? data.from : data.start;
    const endHeight = moment(title, 'HH:mm').unix();
    return endHeight - startHeight < timeUnit - 1 && endHeight >= startHeight;
  });
  return filteredData;
};
