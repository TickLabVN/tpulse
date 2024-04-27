import { useState, useEffect } from 'react';
import { EventData, TaskData } from '@/interfaces';
import { prettyHour } from '@/utils';
import { Badge } from '@/components';
import moment from 'moment';
interface EventProps {
  event: EventData | TaskData;
  timeUnit: number;
  top: number;
}

export function Event({ event, timeUnit, top }: EventProps) {
  const eventColor = {
    backgroundColor: '#D0D7DE',
    textColor: '#6E7781',
    borderColor: '#6E7781'
  };
  let title = '';
  const start = event.start;
  const end = event.end;
  let icon = '';
  let taskName = '';
  let projectName = '';
  if ('color' in event) {
    eventColor.backgroundColor = event.color.backgroundColor;
    eventColor.textColor = event.color.textColor;
    eventColor.borderColor = event.color.borderColor;
    taskName = event.taskName;
    projectName = event.projectName;
  } else {
    title = event.title;
    icon = event.icon;
  }
  const [height, setHeight] = useState(0);
  const getEventPlacement = () => {
    setHeight(((end - start) / timeUnit) * 40);
  };

  useEffect(() => {
    getEventPlacement();
  }, [timeUnit, start, end]);
  const firstContent = () => {
    return (
      <div className='flex items-center gap-x-3'>
        <div
          className={taskName ? 'text-[14px] font-[500] leading-[16.94px]' : 'text-xs font-normal leading-3'}
        >
          {moment.unix(start).format('HH:mm')} - {moment.unix(end).format('HH:mm')}
        </div>
        {projectName ? (
          <div className='flex items-center px-3 py-1 gap-x-1'>
            <span
              className='w-2 h-2 rounded-full'
              style={{
                backgroundColor: eventColor.borderColor
              }}
            ></span>
            <div className='text-xs font-normal leading-3'>{projectName}</div>
          </div>
        ) : (
          <Badge className='!bg-green !text-xs !font-[500] !rounded-[5px] !px-1 !py-0.5 text-white !leading-3 !border-none ml-auto'>
            {prettyHour(end - start)}
          </Badge>
        )}
      </div>
    );
  };
  const secondContent = () => {
    return (
      <div className='flex items-center justify-between'>
        <div className='flex items-center gap-x-1'>
          {icon && <img src={icon} alt='vsc-icon' className='w-5 h-5 rounded-full' />}
          <span className='text-xs font-semibold'>{title}</span>
          {taskName && <span className='text-lg font-semibold leading-5'>{taskName}</span>}
        </div>
        {taskName && (
          <Badge className='!bg-accent-blue !text-xs !font-[500] !rounded-[5px] !px-1 !py-0.5 text-white !leading-3 !border-none'>
            {prettyHour(end - start)}
          </Badge>
        )}
      </div>
    );
  };
  return (
    <div
      className='absolute flex justify-center w-3/4 overflow-hidden -translate-x-1/2 cursor-pointer select-none left-1/2'
      style={{
        ...eventColor,
        top: `${top}px`,
        height: `${height}px`,
        borderRadius: '10px',
        zIndex: 1
      }}
    >
      <span
        className='shrink-0 w-[5px]'
        style={{
          backgroundColor: eventColor.borderColor
        }}
      ></span>
      <div className='flex flex-col flex-1 py-1.5 px-3  gap-y-1 '>
        {'color' in event ? (
          <>
            {secondContent()}
            {firstContent()}
          </>
        ) : (
          <>
            {firstContent()}
            {secondContent()}
          </>
        )}
      </div>
    </div>
  );
}
