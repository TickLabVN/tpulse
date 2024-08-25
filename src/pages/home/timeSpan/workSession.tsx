import { TIMETABLE_ROW_HEIGHT, TIMETABLE_UNIT } from '@/constants';
import type { WorkSession } from '@/db';
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
      className='absolute rounded-sm left-4 right-4 bg-blue-400 text-right text-white font-semibold text-sm z-[2] px-1'
      style={{
        minHeight: `${height}px`,
        top: `${top}px`
      }}
    >
      {time}
    </div>
  );
}
