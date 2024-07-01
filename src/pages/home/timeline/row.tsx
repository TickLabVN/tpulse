import type { ActivityLog, Task } from '@/services';

import { ActivitySpan, TaskSpan } from './timeSpan';

export const TimelineRow: IComponent<{
  isLastRow: boolean;
  timeUnit: number;
  displayTime: string;
  tasks: Task[];
  activities: ActivityLog[];
}> = ({ isLastRow, tasks, activities, displayTime }) => {
  let rowStyle = 'relative border-x border-light-gray h-10';
  if (!isLastRow) rowStyle += ' border-b';
  return (
    <tr id={'asdasd'}>
      <td className='font-bold px-[15px] align-bottom'>
        {!isLastRow ? <div className='text-sm translate-y-1/2 text-end text-gray'>{displayTime}</div> : null}
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
        {!isLastRow ? (
          <div className='text-sm translate-y-1/2 text-start text-gray'>{displayTime}</div>
        ) : null}
      </td>
    </tr>
  );
};
