import { Badge } from '@/components';
import { TIMETABLE_ROW_HEIGHT, TIMETABLE_UNIT } from '@/constants';
import { type CalendarEvent, activityLogSvc } from '@/services';
import { prettyTime } from '@/utils';
import { useQuery } from '@tanstack/react-query';
import moment from 'moment';
import { useMemo } from 'react';

type EventProps<T> = {
  data: T;
};

function FlexBadge({
  children,
  className,
  isSm
}: { children: React.ReactNode; className?: string; isSm?: boolean }) {
  return isSm ? (
    <div className={`${className} text-[10px] px-2`}>{children}</div>
  ) : (
    <Badge className={className}>{children}</Badge>
  );
}

export function CalendarEventSpan({ data: event }: EventProps<CalendarEvent>) {
  const { duration, height, top } = useMemo(() => {
    const startOfDay = moment().startOf('day').unix();
    const duration = prettyTime(event.end_time - event.start_time);

    const height = Math.floor(((event.end_time - event.start_time) / TIMETABLE_UNIT) * TIMETABLE_ROW_HEIGHT);
    const top =
      (Math.floor((event.start_time - startOfDay) % TIMETABLE_UNIT) / TIMETABLE_UNIT) * TIMETABLE_ROW_HEIGHT;
    return { duration, height, top };
  }, [event]);

  const { spanStyle, titleStyle, isSm } = useMemo(() => {
    let spanStyle = '';
    let titleStyle = '';

    let isSm = true;
    if (height <= TIMETABLE_ROW_HEIGHT / 4) {
      spanStyle = 'px-1 py-0 rounded-sm border-l-[3px]';
      titleStyle = 'text-xs';
    } else if (height <= TIMETABLE_ROW_HEIGHT / 2) {
      spanStyle = 'px-1 py-0 rounded-sm border-l-[3px]';
      titleStyle = 'text-md';
      isSm = false;
    } else {
      isSm = false;
      spanStyle = 'px-2 py-1 rounded-md border-l-4';
      titleStyle = 'text-lg';
    }
    return { spanStyle, titleStyle, isSm };
  }, [height]);

  const { data: workCategories } = useQuery({
    queryKey: ['workCategories', event.start_time, event.end_time],
    queryFn: () => activityLogSvc.categorizeActivities(event.start_time, event.end_time),
    initialData: []
  });

  return (
    <div
      style={{
        height: `${height}px`,
        top: `${top}px`,
        left: '16px',
        right: '16px'
      }}
      className={`${spanStyle} absolute border-[1px] border-l-google border-light-gray bg-white z-[2] overflow-hidden`}
    >
      <div className='flex justify-between items-center'>
        <div className={`${titleStyle} text-background font-semibold`}>{event.name}</div>
        <div className='flex gap-[1px]'>
          {event.source && (
            <FlexBadge isSm={isSm} className='font-semibold bg-google text-white rounded-md'>
              {event.source[0].toUpperCase() + event.source.slice(1)}
            </FlexBadge>
          )}
          <FlexBadge isSm={isSm} className='font-semibold bg-green text-white text-xs rounded-md px-1'>
            {duration}
          </FlexBadge>
        </div>
      </div>

      {!isSm && (
        <div>
          {workCategories.map((wc) => (
            <FlexBadge key={wc.category} isSm={isSm} className='bg-accent-blue text-white font-semibold'>
              {wc.category} {Math.round(wc.percentage)}%
            </FlexBadge>
          ))}
        </div>
      )}
    </div>
  );
}
