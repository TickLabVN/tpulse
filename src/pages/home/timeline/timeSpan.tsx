import { Badge } from '@/components';
import { TIMETABLE_ROW_HEIGHT, TIMETABLE_UNIT } from '@/constants';
import type { ActivityLog, CalendarEvent } from '@/services';
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
    <div className='border-[1px] border-l-4 border-l-[#6E7781] border-[#D0D7DE] rounded-lg px-2 w-full'>
      <div className='flex items-center justify-between'>
        <div className='text-xs text-[#6E7781]'>{timeRange}</div>
        <Badge className='bg-green text-white rounded-[5px]'>{duration}</Badge>
      </div>
      <p className='text-xs font-semibold leading-4 text-[#6E7781]'>
        {data.name.length > 40 ? `${data.name.slice(0, 40)}...` : data.name}
      </p>
    </div>
  );
}

export function PlanSpan({ data: event }: EventProps<CalendarEvent>) {
  const { timeRange, duration, height, top } = useMemo(() => {
    const start = moment.unix(event.start_time);
    const end = moment.unix(event.end_time);

    const startOfDay = moment().startOf('day').unix();

    const timeRange = `${start.format('hh:mm A')} - ${end.format('hh:mm A')}`;
    const duration = prettyTime(event.end_time - event.start_time);

    const height = Math.floor(((event.end_time - event.start_time) / TIMETABLE_UNIT) * TIMETABLE_ROW_HEIGHT);
    const top =
      (Math.floor((event.start_time - startOfDay) % TIMETABLE_UNIT) / TIMETABLE_UNIT) * TIMETABLE_ROW_HEIGHT;

    return { timeRange, duration, height, top };
  }, [event]);

  return (
    <div
      style={{
        height: `${height}px`,
        top: `${top}px`
      }}
      className='border-[1px] relative border-l-4 border-l-google border-[#D0D7DE] rounded-md w-full px-4 py-2 bg-white'
    >
      <div className='flex items-center justify-between'>
        <div className='text-lg text-background font-semibold'>{event.name}</div>
        <div>
          {event.source && (
            <Badge className='bg-google text-white rounded-[5px]'>
              {event.source[0].toUpperCase() + event.source.slice(1)}
            </Badge>
          )}
          <Badge className='bg-green text-white rounded-[5px]'>{duration}</Badge>
        </div>
      </div>
      <p className='text-sm font-medium'>{timeRange}</p>
    </div>
  );
}
