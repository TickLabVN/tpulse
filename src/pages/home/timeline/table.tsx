import moment from 'moment';
import { useMemo } from 'react';
import { FixedSizeList, type ListChildComponentProps } from 'react-window';
import { TimeTableHeader } from './tableHeader';

const Row = ({ index, style }: ListChildComponentProps) => {
  const { milestone, isLastRow, rowStyle } = useMemo(() => {
    const startOfDay = moment().startOf('day');
    const milestone = startOfDay.add((index + 1) * TIME_UNIT, 'seconds').format('HH:mm:ss');
    const isLastRow = index === NUM_SECS_IN_DAY / TIME_UNIT - 1;

    let rowStyle = 'px-4 border-light-gray overflow-visible relative flex-1 h-full';
    if (!isLastRow) rowStyle += ' border-b-[1px]';
    return { milestone, isLastRow, rowStyle };
  }, [index]);

  return (
    <div style={style} className='flex justify-between items-end'>
      <div className='font-bold align-bottom w-20'>
        {!isLastRow && <div className='text-sm translate-y-1/2 text-center text-gray'>{milestone}</div>}
      </div>
      <div className={`${rowStyle} border-x-[1px]`}>Row {index}</div>
      <div className={`${rowStyle} border-e-[1px]`}>Row {index}</div>
      <div className='font-bold align-bottom w-20'>
        {!isLastRow && <div className='text-sm translate-y-1/2 text-center text-gray'>{milestone}</div>}
      </div>
    </div>
  );
};

const TIME_UNIT = 4;
const NUM_SECS_IN_DAY = 86400;

export function TimeTable() {
  return (
    <div className='rounded-2xl bg-white p-0 border-light-gray border mt-4'>
      <TimeTableHeader />
      <FixedSizeList
        height={600}
        itemCount={NUM_SECS_IN_DAY / TIME_UNIT}
        itemSize={60}
        width={'100%'}
        className='no-scrollbar'
      >
        {Row}
      </FixedSizeList>
    </div>
  );
}
