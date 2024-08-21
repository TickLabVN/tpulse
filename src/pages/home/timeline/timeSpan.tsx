import { Badge } from '@/components';
import { TIMETABLE_ROW_HEIGHT, TIMETABLE_UNIT } from '@/constants';
import type { CalendarEvent, WorkSession } from '@/services';
import { prettyTime } from '@/utils';
import moment from 'moment';
import { useMemo } from 'react';

type EventProps<T> = {
  data: T;
};

export function WorkSessionSpan({ data }: EventProps<WorkSession>) {
  const { height, top, time } = useMemo(() => {
    const startTime = data.start_time;
    const endTime = data.end_time ?? moment().unix();

    const startOfDay = moment().startOf('day').unix();
    const height = Math.floor(((endTime - startTime) / TIMETABLE_UNIT) * TIMETABLE_ROW_HEIGHT);
    let top = (startTime - startOfDay) % TIMETABLE_UNIT;
    top = Math.round((top / TIMETABLE_UNIT) * TIMETABLE_ROW_HEIGHT);

    const time = prettyTime(endTime - startTime);
    return { height, top, time };
  }, [data]);

  return (
    <div
      className='absolute rounded-sm left-4 right-4 bg-primary text-right text-white font-semibold text-sm z-[2] px-1'
      style={{
        minHeight: `${height}px`,
        top: `${top}px`
      }}
    >
      {time}
    </div>
  );
}

export function CalendarSpan({ data: event }: EventProps<CalendarEvent>) {
  const { timeRange, duration, height, top, spanStyle, titleStyle } = useMemo(() => {
    const start = moment.unix(event.start_time);
    const end = moment.unix(event.end_time);

    const startOfDay = moment().startOf('day').unix();

    const timeRange = `${start.format('hh:mm')} - ${end.format('hh:mm')}`;
    const duration = prettyTime(event.end_time - event.start_time);

    const height = Math.floor(((event.end_time - event.start_time) / TIMETABLE_UNIT) * TIMETABLE_ROW_HEIGHT);
    const top =
      (Math.floor((event.start_time - startOfDay) % TIMETABLE_UNIT) / TIMETABLE_UNIT) * TIMETABLE_ROW_HEIGHT;

    let spanStyle = '';
    let titleStyle = '';
    if (height <= TIMETABLE_ROW_HEIGHT / 4) {
      spanStyle = 'px-1 py-0';
      titleStyle = 'text-sm';
    } else if (height <= TIMETABLE_ROW_HEIGHT / 2) {
      spanStyle = 'px-1 py-0';
      titleStyle = 'text-md';
    } else {
      spanStyle = 'px-4 py-2';
      titleStyle = 'text-lg';
    }
    return { timeRange, duration, height, top, spanStyle, titleStyle };
  }, [event]);

  return (
    <div
      style={{
        height: `${height}px`,
        top: `${top}px`,
        left: '16px',
        right: '16px'
      }}
      className={`${spanStyle} absolute border-[1px] border-l-4 border-l-google border-[#D0D7DE] rounded-md bg-white z-[2]`}
    >
      <div className='flex items-center justify-between'>
        <div className={`${titleStyle} text-background font-semibold`}>{event.name}</div>
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
