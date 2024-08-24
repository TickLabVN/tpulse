import { NUM_SECS_IN_DAY, TIMETABLE_UNIT } from '@/constants';
import { Button, Tab, TabList } from '@fluentui/react-components';
import { DatePicker } from '@fluentui/react-datepicker-compat';
import { ChevronLeft16Regular, ChevronRight16Regular } from '@fluentui/react-icons';
import moment from 'moment';
import { TimeRow } from './timeRow';

const rowArr = Array.from({ length: NUM_SECS_IN_DAY / TIMETABLE_UNIT });

export function HomePage() {
  return (
    <div className='w-full'>
      <div className='flex gap-2 px-8 pt-5'>
        <DatePicker className='flex-1' />
        <Button icon={<ChevronLeft16Regular />} />
        <Button icon={<ChevronRight16Regular />} />
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

      <div className='rounded-md'>
        {rowArr.map((_, i) => {
          const beginOfDay = moment().startOf('day').unix();
          const rowStartTime = beginOfDay + i * TIMETABLE_UNIT;
          const rowEndTime = rowStartTime + TIMETABLE_UNIT;
          const milestone = i < rowArr.length - 1 ? moment.unix(rowEndTime).format('HH:mm') : undefined;
          return (
            // biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
            <TimeRow key={i} milestone={milestone} />
          );
        })}
      </div>
    </div>
  );
}
