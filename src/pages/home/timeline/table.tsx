import { NUM_SECS_IN_DAY, TIMETABLE_UNIT } from '@/constants';
import { TableRow } from './row';
import { TimeTableHeader } from './tableHeader';

export function TimeTable() {
  return (
    <div className='rounded-2xl bg-white p-0 border-light-gray border mt-4 max-h-[75vh] overflow-y-scroll no-scrollbar'>
      <TimeTableHeader />
      {Array.from({ length: NUM_SECS_IN_DAY / TIMETABLE_UNIT }).map((_, index) => (
        // biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
        <TableRow key={index} index={index} />
      ))}
    </div>
  );
}
