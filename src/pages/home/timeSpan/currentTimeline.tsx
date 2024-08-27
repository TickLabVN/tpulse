import { TIMETABLE_ROW_HEIGHT, TIMETABLE_UNIT } from '@/constants';
import moment from 'moment';
import { useMemo } from 'react';

type CurrentTimelineProps = {
  currentTime: moment.Moment;
};
export function CurrentTimeline({ currentTime }: CurrentTimelineProps) {
  const top = useMemo(() => {
    const startOfDay = moment().startOf('day').unix();
    let top = (currentTime.unix() - startOfDay) % TIMETABLE_UNIT;
    top = Math.round((top / TIMETABLE_UNIT) * TIMETABLE_ROW_HEIGHT);

    return top;
  }, [currentTime]);

  return <hr className='absolute w-full h-[3px] z-10 bg-red-500' style={{ top: `${top}px` }} />;
}
