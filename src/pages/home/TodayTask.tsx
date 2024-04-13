import { ChevronRightIcon, FeedPlusIcon } from '@primer/octicons-react';
import { Checkbox, Label } from '@primer/react';
import { prettyHour } from '@utils';
import moment from 'moment';

type TaskItem = {
  id: string;
  name: string;
  start: number;
  stop: number;
  project: string;
  status: 'done' | 'todo';
};

const items: TaskItem[] = [
  {
    id: '1',
    name: 'Architecture Design',
    start: moment().startOf('day').add(8, 'hours').unix(),
    stop: moment().startOf('day').add(10, 'hours').unix(),
    project: 'TPulse',
    status: 'done'
  },
  {
    id: '2',
    name: 'Developer meeting',
    start: moment().startOf('day').add(8, 'hours').unix(),
    stop: moment().startOf('day').add(10, 'hours').unix(),
    project: 'TPulse',
    status: 'todo'
  }
];

export function TodayTask() {
  return (
    <div className='mt-[30px]'>
      <div className='flex items-center justify-between'>
        <span className='font-bold text-navy text-[26px] leading-8'>Today Tasks</span>
        <FeedPlusIcon size={32} className='cursor-pointer text-green stroke-1' />
      </div>
      <div className='mt-3 w-full flex flex-col gap-3'>
        {items.map((item) => (
          <div
            key={item.id}
            className='p-[18px] rounded-2xl bg-white border border-light-gray flex justify-between items-center'
          >
            <div className='w-fit flex items-center gap-4'>
              <Checkbox className='bg-white checked:!bg-green !w-6 !h-6 !border-1 !border-light-gray !rounded-full !p-2' />
              <div className='flex flex-col justify-between gap-3'>
                <div className='flex items-center gap-3'>
                  <span className='font-semibold text-[18px] leading-5'>{item.name}</span>
                  <Label className='!bg-accent-blue !text-sm !font-bold !rounded-[5px] !px-2 !py-1 text-white !leading-5 !border-none'>
                    {prettyHour(item.stop - item.start)}
                  </Label>
                </div>
                <div className='flex items-center text-background font-semibold text-sm'>
                  <span className='mr-5'>Today,&nbsp;{moment.unix(item.start).format('HH:mm')}</span>
                  <div className='w-2 h-2 bg-accent-purple rounded-full mr-2'></div>
                  <span>{item.project}</span>
                </div>
              </div>
            </div>
            <ChevronRightIcon size={30} className='cursor-pointer text-dark-gray !stroke-[5] !font-bold' />
          </div>
        ))}
      </div>
    </div>
  );
}
