import { Badge } from '@/components';
import type { ActivityLog, Task } from '@/services';
import { prettyTime } from '@/utils';
import moment from 'moment';
import { useMemo } from 'react';

type EventProps<T> = {
  data: T;
};

export function ActivitySpan({ data }: EventProps<ActivityLog>) {
  const { timeRange, duration } = useMemo(() => {
    const start = moment.unix(data.start_time).format('HH:mm:ss');
    const end = moment.unix(data.end_time).format('HH:mm:ss');
    const timeRange = `${start} - ${end}`;
    const duration = prettyTime(data.end_time - data.start_time);

    return { timeRange, duration };
  }, [data]);

  return (
    <div className='border-[1px] border-l-4 border-l-[#6E7781] border-[#D0D7DE] my-[2px] rounded-lg px-3 py-2'>
      <div className='flex items-center justify-between'>
        <div className='text-xs text-[#6E7781]'>{timeRange}</div>
        <Badge className='bg-green text-white rounded-[5px] px-1 py-[2px]'>{duration}</Badge>
      </div>
      <p className='text-xs font-semibold leading-4 text-[#6E7781]'>
        {data.name.length > 40 ? `${data.name.slice(0, 40)}...` : data.name}
      </p>
    </div>
  );
}

export function TaskSpan({ data }: EventProps<Task>) {
  const { timeRange, duration } = useMemo(() => {
    if (!data.start || !data.end) return { timeRange: '', duration: '' };

    const start = moment(data.start);
    const end = moment(data.end);
    const timeRange = `${start.hour()}:${start.minute()}:${start.second()} - ${end.hour()}:${end.minute()}:${end.second()}`;
    const duration = prettyTime((data.end - data.start) / 1000);

    return { timeRange, duration };
  }, [data]);

  return (
    <div>
      <div className='flex items-center justify-between'>
        <div>{data.name}</div>
        <div>{duration}</div>
      </div>
      <div>
        <div>{timeRange}</div>
      </div>
    </div>
  );
}
