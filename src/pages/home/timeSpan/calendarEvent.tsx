import { TIMETABLE_ROW_HEIGHT, TIMETABLE_UNIT } from '@/constants';
import { type CalendarEvent, categorizeActivities } from '@/db';
import { Badge } from '@fluentui/react-components';
import { useQuery } from '@tanstack/react-query';
import moment from 'moment';
import { useCallback, useMemo } from 'react';

type EventProps<T> = {
  event: T;
};

const badgeColors = ['brand', 'danger', 'important', 'informative', 'severe', 'subtle', 'success', 'warning'];
const randomColor = () => badgeColors[Math.floor(Math.random() * badgeColors.length)];

export function CalendarEventSpan({ event }: EventProps<CalendarEvent>) {
  const { height, top } = useMemo(() => {
    const startTime = event.start_time;
    const endTime = event.end_time ?? moment().unix();

    const startOfDay = moment().startOf('day').unix();
    const height = Math.floor(((endTime - startTime) / TIMETABLE_UNIT) * TIMETABLE_ROW_HEIGHT);
    let top = (startTime - startOfDay) % TIMETABLE_UNIT;
    top = Math.round((top / TIMETABLE_UNIT) * TIMETABLE_ROW_HEIGHT);

    return { height, top };
  }, [event]);

  const isSmall = useMemo(() => height <= TIMETABLE_ROW_HEIGHT / 2, [height]);

  const { data: workCategories } = useQuery({
    queryKey: ['work_categories', event.start_time, event.end_time],
    queryFn: () => categorizeActivities(event.start_time, event.end_time),
    initialData: []
  });

  const Categories = useCallback(() => {
    const body = workCategories.map(({ category, percentage }) => (
      <Badge
        key={category}
        shape='rounded'
        size='small'
        // biome-ignore lint/suspicious/noExplicitAny: <explanation>
        color={randomColor() as any}
      >
        {percentage.toFixed(0)}% {category}
      </Badge>
    ));
    return isSmall ? <>{body}</> : <div className='flex flex-wrap gap-1 mt-1'>{body}</div>;
  }, [isSmall, workCategories]);

  return (
    <div
      className={`${isSmall ? 'flex gap-1 flex-wrap' : ''} overflow-y-hidden absolute text-white rounded-md left-4 right-4 border-b-[1px] border-b-white bg-blue-400 z-[2] px-1`}
      style={{
        height: `${height}px`,
        maxHeight: `${height}px`,
        top: `${top}px`
      }}
    >
      <div className='font-semibold text-xs'>{event.name}</div>
      <Categories />
    </div>
  );
}
