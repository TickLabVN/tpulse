import { Badge } from '@/components';
import { ActivityLog, Task } from '@/services';
import { prettyHour } from '@/utils';
import moment from 'moment';
import { useEffect, useMemo, useState } from 'react';

interface EventProps {
  event: ActivityLog | Task;
  timeUnit: number;
  top: number;
}

export function TimeSpan({ event, timeUnit, top }: EventProps) {
  const [eventColor, setEventColor] = useState({
    backgroundColor: '#DAE2EB',
    textColor: '#6E7781',
    borderColor: '#6E7781'
  });

  const { title, start, end, icon, name, projectName } = useMemo(() => {
    if ('start' in event) {
      setEventColor({
        backgroundColor: '#E9D8FC',
        borderColor: '#907BFD',
        textColor: '#071A29'
      });
      return {
        title: event.name,
        icon: '',
        start: event.start,
        end: event.end,
        name: event.name,
        projectName: ''
      };
    } else {
      return {
        title: event.name,
        icon: '',
        start: event.start_time,
        end: event.end_time,
        name: event.name,
        projectName: event.category_tag
      };
    }
  }, [event]);

  const [height, setHeight] = useState(0);

  useEffect(() => {
    if (!start || !end) return;
    setHeight(((end - start) / timeUnit) * 40);
  }, [timeUnit, start, end]);

  const firstContent = () => {
    return start && end ? (
      <div className={`flex items-center gap-x-3`}>
        <div className={name ? 'text-[14px] font-[500] leading-[16.94px]' : `text-xs font-normal leading-3 `}>
          {moment.unix(start!).format('HH:mm A')} - {moment.unix(end!).format('HH:mm A')}
        </div>
        {projectName ? (
          <div className='flex items-center px-3 py-1 gap-x-1'>
            <span
              className='w-2 h-2 rounded-full'
              style={{
                backgroundColor: eventColor.borderColor
              }}
            ></span>
            <div className='text-[14px] font-[500] leading-[16.94px]'>{projectName}</div>
          </div>
        ) : (
          <Badge className='!bg-green !text-xs !font-[500] !rounded-[5px] !px-1 !py-0.5 text-white !leading-3 !border-none ml-auto'>
            {prettyHour(end - start)}
          </Badge>
        )}
      </div>
    ) : null;
  };
  const secondContent = () => {
    return start && end ? (
      <div className='flex items-center justify-between'>
        <div className='flex items-center'>
          {icon && <img src={icon} alt='vsc-icon' className='w-5 h-5 mr-1 rounded-full' />}
          <span className={`text-xs font-semibold leading-[14.52px]`}>{title}</span>
          <span className='text-lg font-semibold leading-[21.78px]'>{name}</span>
        </div>
        <Badge className='!bg-accent-blue !text-xs !font-[500] !rounded-[5px] !px-1 !py-0.5 text-white !leading-3 !border-none'>
          {prettyHour(end - start)}
        </Badge>
      </div>
    ) : null;
  };
  return (
    <div
      className='absolute flex justify-center w-3/4 overflow-hidden -translate-x-1/2 cursor-pointer select-none left-1/2'
      style={{
        ...eventColor,
        top: `${top}px`,
        height: `${height}px`,
        borderRadius: '10px',
        zIndex: 1,
        color: eventColor.textColor
      }}
    >
      <span
        className='shrink-0 w-[5px]'
        style={{
          backgroundColor: eventColor.borderColor
        }}
      ></span>
      <div className='flex flex-col flex-1 py-1.5 px-3  gap-y-1 '>
        {'start' in event ? (
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
