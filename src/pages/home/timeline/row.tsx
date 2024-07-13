import type { ActivityLog, Task } from '@/services';
import type { Moment } from 'moment';
import { useMemo } from 'react';
import { ActivitySpan, TaskSpan } from './timeSpan';

export const TimelineRow: IComponent<{
  timeUnit: number;
  displayTime: Moment;
  tasks: Task[];
  activities: ActivityLog[];
}> = ({ tasks, activities, displayTime }) => {
  const [milestone, isLastRow, rowStyle] = useMemo(() => {
    const milestone = displayTime.format('HH:mm');
    const isLastRow = milestone === '00:00';
    let rowStyle = 'px-4 border-x border-light-gray top-0 !max-h-12 overflow-hidden';
    if (!isLastRow) rowStyle += ' border-b';
    return [milestone, isLastRow, rowStyle];
  }, [displayTime]);

  return (
    <tr className='h-12 overflow-visible'>
      <td className='font-bold px-[15px] align-bottom'>
        {!isLastRow && <div className='text-sm translate-y-1/2 text-end text-gray'>{milestone}</div>}
      </td>
      <td className={rowStyle}>
        {activities.map((data) => (
          <ActivitySpan key={data.name} data={data} />
        ))}
      </td>
      <td className={rowStyle}>
        {tasks.map((data) => (
          <TaskSpan key={data.name} data={data} />
        ))}
      </td>
      <td className='font-bold px-[15px] align-bottom'>
        {!isLastRow && <div className='text-sm translate-y-1/2 text-start text-gray'>{milestone}</div>}
      </td>
    </tr>
  );
};
