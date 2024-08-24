import { TIMETABLE_ROW_HEIGHT } from '@/constants/timetable';

type TimeRowProps = {
  milestone?: string;
};

export function TimeRow({ milestone }: TimeRowProps) {
  return (
    <div
      style={{
        height: `${TIMETABLE_ROW_HEIGHT}px`,
        maxHeight: `${TIMETABLE_ROW_HEIGHT}px`
      }}
      className='flex justify-between items-end overflow-visible z-[1] border-[1px]'
    >
      <div className='align-bottom'>
        {milestone && (
          <div className='text-xs translate-y-1/2 pe-2 text-center z-[2] bg-white'>{milestone}</div>
        )}
      </div>
    </div>
  );
}
