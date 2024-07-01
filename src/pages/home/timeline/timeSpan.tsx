import type { ActivityLog, Task } from '@/services';
import { prettyTime } from '@/utils';
import { useMemo } from 'react';

type EventProps<T> = {
  data: T;
};

export function ActivitySpan({ data }: EventProps<ActivityLog>) {
  const { timeRange, duration } = useMemo(() => {
    const start = new Date(data.start_time);
    const end = new Date(data.end_time);
    const timeRange = `${start.getHours()}:${start.getMinutes()} - ${end.getHours()}:${end.getMinutes()}`;
    const duration = prettyTime((end.getTime() - start.getTime()) / 1000);

    return { timeRange, duration };
  }, [data]);

  return (
    <div>
      <div className='flex items-center justify-between'>
        <div>{timeRange}</div>
        <div>{duration}</div>
      </div>
      <div>
        <div>{data.name}</div>
        <div>{data.category_tag}</div>
      </div>
    </div>
  );
}

export function TaskSpan({ data }: EventProps<Task>) {
  const { timeRange, duration } = useMemo(() => {
    if (!data.start || !data.end) return { timeRange: '', duration: '' };

    const start = new Date(data.start);
    const end = new Date(data.end);
    const timeRange = `${start.getHours()}:${start.getMinutes()} - ${end.getHours()}:${end.getMinutes()}`;
    const duration = prettyTime((end.getTime() - start.getTime()) / 1000);

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
