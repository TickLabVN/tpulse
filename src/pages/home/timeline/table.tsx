import { Button } from '@/components/ui/button';
import { TIME_TABLE_ROW_HEIGHT } from '@/constants';
import moment from 'moment';
import { useCallback, useRef } from 'react';
import { FixedSizeList } from 'react-window';
import { TableRow } from './row';
import { TimeTableHeader } from './tableHeader';

const TIME_UNIT = 60;
const NUM_SECS_IN_DAY = 86400;

export function TimeTable() {
  const list = useRef<FixedSizeList>(null);

  const jumpToPresent = useCallback(() => {
    const currentTime = moment().unix();
    const startOfDay = moment().startOf('day').unix();
    const currentItem = Math.floor((currentTime - startOfDay) / TIME_UNIT);
    list.current?.scrollToItem(currentItem);
  }, []);

  return (
    <div className='rounded-2xl bg-white p-0 border-light-gray border mt-4'>
      <Button onClick={jumpToPresent}>Jump to present</Button>
      <TimeTableHeader />
      <FixedSizeList
        height={600}
        itemCount={NUM_SECS_IN_DAY / TIME_UNIT}
        itemSize={TIME_TABLE_ROW_HEIGHT}
        width={'100%'}
        className='no-scrollbar'
        itemData={{ timeUnit: TIME_UNIT }}
        ref={list}
      >
        {TableRow}
      </FixedSizeList>
    </div>
  );
}
