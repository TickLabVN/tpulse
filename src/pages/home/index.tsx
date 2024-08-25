import { NUM_SECS_IN_DAY, TIMETABLE_UNIT } from '@/constants';
import { Button, Tab, TabList } from '@fluentui/react-components';
import { DatePicker } from '@fluentui/react-datepicker-compat';
import { ChevronLeft16Regular, ChevronRight16Regular } from '@fluentui/react-icons';
import moment from 'moment';
import { useCallback, useMemo, useState } from 'react';
import { TimeRow } from './timeRow';

const rowArr = Array.from({ length: NUM_SECS_IN_DAY / TIMETABLE_UNIT });

function onFormatDate(date?: Date): string {
  return !date ? '' : `${date.getDate()}/${date.getMonth() + 1}/${date.getFullYear()}`;
}

export function HomePage() {
  const [selectedDate, setSelectedDate] = useState<moment.Moment>(moment());
  const goPrevious = useCallback(() => {
    setSelectedDate((prev) => moment(prev).subtract(1, 'day'));
  }, []);
  const goNext = useCallback(() => {
    setSelectedDate((prev) => moment(prev).add(1, 'day'));
  }, []);

  const beginOfDay = useMemo(() => selectedDate.startOf('day').unix(), [selectedDate]);
  const onParseDateFromString = useCallback(
    (newValue: string): Date => moment(newValue, 'DD/MM/YYYY').toDate(),
    []
  );

  return (
    <div className='w-full'>
      <div className='flex gap-2 px-8 pt-5'>
        <DatePicker
          className='flex-1'
          value={selectedDate.toDate()}
          onSelectDate={(date) => setSelectedDate(moment(date))}
          formatDate={onFormatDate}
          parseDateFromString={onParseDateFromString}
        />
        <Button icon={<ChevronLeft16Regular />} onClick={goPrevious} />
        <Button icon={<ChevronRight16Regular />} onClick={goNext} />
      </div>

      <TabList className='w-full flex px-5 mt-2' defaultSelectedValue={'afk'}>
        <Tab value='afk' className='flex-1'>
          Sessions
        </Tab>
        <Tab value='projects' className='flex-1'>
          Projects
        </Tab>
        <Tab value='event' className='flex-1'>
          Events
        </Tab>
      </TabList>

      <div className='rounded-md mt-6'>
        {rowArr.map((_, i) => {
          const startTime = moment.unix(beginOfDay + i * TIMETABLE_UNIT);
          return (
            // biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
            <TimeRow key={i} startTime={startTime} />
          );
        })}
      </div>
    </div>
  );
}
