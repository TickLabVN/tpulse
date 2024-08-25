import { TIMETABLE_ROW_HEIGHT } from '@/constants/timetable';
import { getWorkSessions } from '@/db';
import { useQuery } from '@tanstack/react-query';
import moment from 'moment';
import { useMemo } from 'react';
import { WorkSessionSpan } from './timeSpan/workSession';

type TimeRowProps = {
  startTime: moment.Moment;
};

export function TimeRow({ startTime }: TimeRowProps) {
  const { endTime, milestone } = useMemo(() => {
    const endTime = startTime.clone().add(1, 'hour');
    const milestone = endTime.format('HH:mm');
    return {
      endTime,
      milestone: milestone === '00:00' ? null : milestone
    };
  }, [startTime]);

  const borderStyle = useMemo(() => (milestone ? 'border-t-[1px]' : 'border-y-[1px]'), [milestone]);
  const needRefetch = useMemo(() => moment().isBetween(startTime, endTime), [startTime, endTime]);

  const { data: workSessions } = useQuery({
    queryKey: ['workSessions', startTime.unix(), endTime.unix()],
    queryFn: () => {
      const from = startTime.unix();
      const to = endTime.unix();
      console.log('fetching workSessions', from, to);
      return getWorkSessions(from, to);
    },
    refetchInterval: needRefetch ? 1000 * 5 : false,
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
      <div className='align-bottom min-w-[50px]'>
        {milestone && (
          <div className='text-xs translate-y-1/2 ps-3 pe-2 text-center z-[2] bg-white'>{milestone}</div>
        )}
      </div>
      <div className='border-s-[1px] h-full relative flex-1'>
        {workSessions.map((ws) => (
          <WorkSessionSpan key={ws.id} data={ws} />
        ))}
      </div>
    </div>
  );
}
