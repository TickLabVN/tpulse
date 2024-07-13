import { Badge } from '@/components';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip';
import type { ActivityLog, Task } from '@/services';
import { prettyTime } from '@/utils';
import moment from 'moment';
import { useMemo } from 'react';

type EventProps<T> = {
  height: number;
  top: number;
  data: T;
};

export function ActivitySpan({ data, height, top }: EventProps<ActivityLog>) {
  const { timeRange, duration } = useMemo(() => {
    const start = moment.unix(data.start_time).format('HH:mm:ss');
    const end = moment.unix(data.end_time).format('HH:mm:ss');
    const timeRange = `${start} - ${end}`;
    const duration = prettyTime(data.end_time - data.start_time);

    return { timeRange, duration };
  }, [data]);

  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger>
          <div
            className='absolute overflow-hidden border-[1px] border-l-4 border-l-[#6E7781] border-[#D0D7DE] rounded-lg px-2'
            style={{
              height: `${height}px`,
              top: `${top}px`
            }}
          >
            <div className='flex items-center justify-between'>
              <div className='text-xs text-[#6E7781]'>{timeRange}</div>
              <Badge className='bg-green text-white rounded-[5px]'>{duration}</Badge>
            </div>
            <p className='text-xs font-semibold leading-4 text-[#6E7781]'>
              {data.name.length > 40 ? `${data.name.slice(0, 40)}...` : data.name}
            </p>
          </div>
        </TooltipTrigger>
        <TooltipContent>{data.name}</TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
}

export function TaskSpan({ data, height, top }: EventProps<Task>) {
  const { timeRange, duration } = useMemo(() => {
    if (!data.start || !data.end) return { timeRange: '', duration: '' };

    const start = moment(data.start);
    const end = moment(data.end);
    const timeRange = `${start.hour()}:${start.minute()}:${start.second()} - ${end.hour()}:${end.minute()}:${end.second()}`;
    const duration = prettyTime((data.end - data.start) / 1000);

    return { timeRange, duration };
  }, [data]);

  return (
    <div
      className='overflow-hidden'
      style={{
        height: `${height}px`,
        top: `${top}px`
      }}
    >
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
