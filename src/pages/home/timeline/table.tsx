import { NUM_SECS_IN_DAY, TIMETABLE_UNIT } from '@/constants';
import { calendarSvc } from '@/services';
import { useQuery } from '@tanstack/react-query';
import moment from 'moment';
import { type JSX, useCallback } from 'react';
import { TableRow } from './row';
import { TimeTableHeader } from './tableHeader';

export function TimeTable() {
  const { data: calendarEvents } = useQuery({
    queryKey: ['calendarEvents'],
    queryFn: () => {
      const startOfDay = moment().startOf('day').unix();
      const endOfDay = moment().endOf('day').unix();

      return calendarSvc.getEvents(startOfDay, endOfDay);
    }
  });

  const TimeTableBody = useCallback(() => {
    const numOfRows = NUM_SECS_IN_DAY / TIMETABLE_UNIT;
    const jsxElements: JSX.Element[] = [];

    const beginOfDay = moment().startOf('day').unix();

    for (let i = 0; i < numOfRows; i++) {
      const rowStartTime = beginOfDay + i * TIMETABLE_UNIT;
      const rowEndTime = rowStartTime + TIMETABLE_UNIT;
      const milestone = i < numOfRows - 1 ? moment.unix(rowEndTime).format('HH:mm') : undefined;

      jsxElements.push(
        <TableRow
          milestone={milestone}
          calendarEvents={
            calendarEvents?.filter((e) => {
              return e.start_time >= rowStartTime && e.start_time < rowEndTime;
            }) ?? []
          }
          afkEvents={[]}
        />
      );
    }

    return <>{jsxElements}</>;
  }, [calendarEvents]);

  return (
    <div className='rounded-2xl bg-white p-0 border-light-gray border mt-4 max-h-[75vh] overflow-y-scroll no-scrollbar'>
      <TimeTableHeader />
      <TimeTableBody />
    </div>
  );
}
