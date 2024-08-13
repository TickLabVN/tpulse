import { NUM_SECS_IN_DAY, TIMETABLE_ROW_HEIGHT, TIMETABLE_UNIT } from '@/constants';
import { planSvc } from '@/services';
import { useQuery } from '@tanstack/react-query';
import moment from 'moment';
import { useMemo } from 'react';
import { PlanSpan } from './timeSpan';

export function TableRow({ index }: { index: number }) {
  const { milestone, isLastRow, rowStyle, startTime, endTime } = useMemo(() => {
    const startOfDay = moment().startOf('day');
    const startTime = startOfDay.clone().add(index * TIMETABLE_UNIT, 'seconds');
    const endTime = startOfDay.clone().add((index + 1) * TIMETABLE_UNIT, 'seconds');

    const milestone = endTime.format('HH:mm');
    const isLastRow = index === NUM_SECS_IN_DAY / TIMETABLE_UNIT - 1;

    let rowStyle = 'px-4 border-light-gray flex-1 h-full';
    if (!isLastRow) rowStyle += ' border-b-[1px]';
    return {
      milestone,
      isLastRow,
      rowStyle,
      startTime: startTime.unix(),
      endTime: endTime.unix() - 1
    };
  }, [index]);

  const { data: plans } = useQuery({
    queryKey: ['plans', startTime, endTime],
    queryFn: () => planSvc.getPlans(startTime, endTime)
  });

  return (
    <div className={`flex justify-between items-end h-[${TIMETABLE_ROW_HEIGHT}px]`}>
      <div className='font-bold align-bottom w-20'>
        {!isLastRow && <div className='text-sm translate-y-1/2 text-center text-gray'>{milestone}</div>}
      </div>
      <div className={`${rowStyle} border-x-[1px]`}>Activities</div>
      <div className={`${rowStyle} border-e-[1px]`}>
        {plans?.map((p) => (
          <PlanSpan key={p.id} data={p} />
        ))}
      </div>
      <div className='font-bold align-bottom w-20'>
        {!isLastRow && <div className='text-sm translate-y-1/2 text-center text-gray'>{milestone}</div>}
      </div>
    </div>
  );
}
