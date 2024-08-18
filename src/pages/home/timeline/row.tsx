import { TIMETABLE_ROW_HEIGHT } from '@/constants';
import type { CalendarEvent } from '@/services';
import { useMemo } from 'react';
import { CalendarSpan } from './timeSpan';

type TableRowProps = {
  milestone?: string;
  calendarEvents: CalendarEvent[];
  afkEvents: unknown[];
};

export function TableRow({ calendarEvents, milestone }: TableRowProps) {
  const rowStyle = useMemo(() => {
    let rowStyle = 'border-light-gray flex-1 h-full relative';
    if (milestone) rowStyle += ' border-b-[1px]';
    return rowStyle;
  }, [milestone]);

  return (
    <div
      style={{
        height: `${TIMETABLE_ROW_HEIGHT}px`,
        maxHeight: `${TIMETABLE_ROW_HEIGHT}px`
      }}
      className='flex justify-between items-end overflow-visible'
    >
      <div className='font-bold align-bottom w-20'>
        {milestone && <div className='text-sm translate-y-1/2 text-center text-gray'>{milestone}</div>}
      </div>
      <div className={`${rowStyle} border-x-[1px]`} />
      <div className={`${rowStyle} border-e-[1px] z-0`}>
        {calendarEvents.map((e) => (
          <CalendarSpan key={e.id} data={e} />
        ))}
      </div>
      <div className='font-bold align-bottom w-20'>
        {milestone && <div className='text-sm translate-y-1/2 text-center text-gray'>{milestone}</div>}
      </div>
    </div>
  );
}
