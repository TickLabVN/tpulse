import { ActivityLog, Task } from '@/services';

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
          <div key={data.name}> {data.name} </div>
          // <TimeSpan
          //   key={index}
          //   event={data}
          //   timeUnit={timeUnit}
          //   top={
          //     'start' in data
          //       ? title === moment.unix(data.start).format('HH:mm')
          //         ? 40
          //         : (Math.abs(data.start - moment(title, 'HH:mm').unix()) / timeUnit) * 40
          //       : title === moment.unix(data.from).format('HH:mm')
          //         ? 40
          //         : (Math.abs(data.from - moment(title, 'HH:mm').unix()) / timeUnit) * 40
          //   }
          // />
        ))}
      </td>
      <td className={rowStyle}>
        {tasks.map((data) => (
          <div key={data.name}> {data.name} </div>
          // <TimeSpan
          //   key={index}
          //   event={data}
          //   timeUnit={timeUnit}
          //   top={
          //     'start' in data
          //       ? title === moment.unix(data.start).format('HH:mm')
          //         ? 40
          //         : (Math.abs(data.status - moment(title, 'HH:mm').unix()) / timeUnit) * 40
          //       : title === moment.unix(data.start).format('HH:mm')
          //         ? 40
          //         : (Math.abs(data.start - moment(title, 'HH:mm').unix()) / timeUnit) * 40
          //   }
          // />
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
