import { NUM_SECS_IN_DAY, NUM_SECS_IN_HOUR } from '@/constants';
import { activityLogSvc } from '@/services';
import { useQuery } from '@tanstack/react-query';
import moment from 'moment';
import { useMemo } from 'react';
import { ActivitySpan } from './timeSpan';

export function TableRow({ index }: { index: number }) {
  const { milestone, isLastRow, rowStyle, startTime, endTime } = useMemo(() => {
    const startOfDay = moment().startOf('day');
    const startTime = startOfDay.clone().add(index * NUM_SECS_IN_HOUR, 'seconds');
    const endTime = startOfDay.clone().add((index + 1) * NUM_SECS_IN_HOUR, 'seconds');

    const milestone = endTime.format('HH:mm');
    const isLastRow = index === NUM_SECS_IN_DAY / NUM_SECS_IN_HOUR - 1;

    let rowStyle = 'px-4 border-light-gray flex-1 h-full';
    if (!isLastRow) rowStyle += ' border-b-[1px]';
    return {
      milestone,
      isLastRow,
      rowStyle,
      startTime: startTime.unix(),
      endTime: endTime.unix() - 1
    };
  }, [index]);

  const { data: activities } = useQuery({
    queryKey: ['activities', startTime, endTime],
    queryFn: () => activityLogSvc.getLogs(startTime, endTime),
    staleTime: 10_000
  });

  return (
    <div className='flex justify-between items-end h-14'>
      <div className='font-bold align-bottom w-20'>
        {!isLastRow && <div className='text-sm translate-y-1/2 text-center text-gray'>{milestone}</div>}
      </div>
      <div className={`${rowStyle} border-x-[1px]`}>
        {activities?.map((data) => (
          <ActivitySpan key={data.start_time} data={data} />
        ))}
      </div>
      <div className={`${rowStyle} border-e-[1px]`}>Row {index}</div>
      <div className='font-bold align-bottom w-20'>
        {!isLastRow && <div className='text-sm translate-y-1/2 text-center text-gray'>{milestone}</div>}
      </div>
    </div>
  );
}
