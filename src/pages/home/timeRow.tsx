import { TIMETABLE_ROW_HEIGHT } from '@/constants/timetable';
import { getCalendarEvents, getWorkSessions } from '@/db';
import { useQuery } from '@tanstack/react-query';
import moment from 'moment';
import { useEffect, useMemo, useState } from 'react';
import { CalendarEventSpan } from './timeSpan/calendarEvent';
import { CurrentTimeline } from './timeSpan/currentTimeline';
import { WorkSessionSpan } from './timeSpan/workSession';

type TimeRowProps = {
  startTime: moment.Moment;
  mode: DashboardTab;
};

export function TimeRow({ startTime, mode }: TimeRowProps) {
  const { endTime, milestone } = useMemo(() => {
    const endTime = startTime.clone().add(1, 'hour');
    const milestone = endTime.format('HH:mm');
    return {
      endTime,
      milestone: milestone === '00:00' ? null : milestone
    };
  }, [startTime]);

  const [currentTime, setCurrentTime] = useState(moment());

  useEffect(() => {
    const interval = setInterval(() => {
      setCurrentTime(moment());
    }, 5000);
    return () => clearInterval(interval);
  }, []);

  const borderStyle = useMemo(() => (milestone ? 'border-t-[1px]' : 'border-y-[1px]'), [milestone]);
  const needRefetch = useMemo(
    () => currentTime.isBetween(startTime, endTime),
    [startTime, endTime, currentTime]
  );

  const { data: workSessions } = useQuery({
    queryKey: ['work_session', startTime.unix(), endTime.unix()],
    queryFn: () => {
      const from = startTime.unix();
      const to = endTime.unix();
      return getWorkSessions(from, to);
    },
    enabled: mode === 'work_session',
    refetchInterval: needRefetch ? 1000 * 60 : false,
    initialData: []
  });

  const { data: calendarEvents } = useQuery({
    queryKey: ['calendar_event', startTime.unix(), endTime.unix()],
    queryFn: () => {
      const from = startTime.unix();
      const to = endTime.unix();
      return getCalendarEvents(from, to);
    },
    enabled: mode === 'calendar_event',
    initialData: []
  });

  return (
    <div
      style={{
        height: `${TIMETABLE_ROW_HEIGHT}px`,
        maxHeight: `${TIMETABLE_ROW_HEIGHT}px`
      }}
      className={`flex gap-2 items-end overflow-visible z-[1] ${borderStyle}`}
    >
      <div className='align-bottom min-w-14'>
        {milestone && (
          <div className='text-xs translate-y-1/2 ps-3 pe-2 text-center z-[2] bg-white text-gray-500'>
            {milestone}
          </div>
        )}
      </div>
      <div className='border-s-[1px] h-full relative flex-1'>
        {mode === 'work_session' && workSessions.map((ws) => <WorkSessionSpan key={ws.id} data={ws} />)}
        {mode === 'project' && <div>Projects</div>}
        {mode === 'calendar_event' &&
          calendarEvents.map((ce) => <CalendarEventSpan key={ce.id} event={ce} />)}
        {needRefetch && <CurrentTimeline currentTime={currentTime} />}
      </div>
    </div>
  );
}
