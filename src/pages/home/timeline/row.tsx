import type { ActivityLog, Task } from '@/services';
import type { Moment } from 'moment';
import { useCallback, useMemo } from 'react';
import { ActivitySpan, TaskSpan } from './timeSpan';

const ROW_HEIGHT = 48; // 48px

export const TimelineRow: IComponent<{
  index: number;
  style: unknown;
  timeUnit: number;
  displayTime: Moment;
  tasks: Task[];
  activities: ActivityLog[];
}> = ({ tasks, activities, displayTime, timeUnit }) => {
  const [milestone, isLastRow, rowStyle] = useMemo(() => {
    const milestone = displayTime.format('HH:mm');
    const isLastRow = milestone === '00:00';
    let rowStyle = `px-4 border-x border-light-gray top-0 !max-h-[${ROW_HEIGHT}px] overflow-visible relative`;
    if (!isLastRow) rowStyle += ' border-b';
    return [milestone, isLastRow, rowStyle];
  }, [displayTime]);

  const calcSpanHeight = useCallback(
    (start: number, end: number) => {
      const duration = end - start;
      return (duration / timeUnit) * ROW_HEIGHT;
    },
    [timeUnit]
  );

  const calcTopPosition = useCallback(
    (start: number) => {
      const duration = start - displayTime.unix() + timeUnit;
      return (duration / timeUnit) * ROW_HEIGHT;
    },
    [displayTime, timeUnit]
  );

  return (
    <tr className='h-12 overflow-visible'>
      <td className='font-bold px-[15px] align-bottom'>
        {!isLastRow && <div className='text-sm translate-y-1/2 text-end text-gray'>{milestone}</div>}
      </td>
      <td className={rowStyle}>
        {activities.map((data) => (
          <ActivitySpan
            key={data.start_time}
            data={data}
            height={calcSpanHeight(data.start_time, data.end_time)}
            top={calcTopPosition(data.start_time)}
          />
        ))}
      </td>
      <td className={rowStyle}>
        {tasks.map((data) =>
          data.start && data.end ? (
            <TaskSpan
              key={data.id}
              data={data}
              height={calcSpanHeight(data.start, data.end)}
              top={calcTopPosition(data.start)}
            />
          ) : null
        )}
      </td>
      <td className='font-bold px-[15px] align-bottom'>
        {!isLastRow && <div className='text-sm translate-y-1/2 text-start text-gray'>{milestone}</div>}
      </td>
    </tr>
  );
};
