import { ChevronRightIcon, FeedPlusIcon } from '@primer/octicons-react';
import { prettyHour } from '@/utils';
import { Badge } from '@/components';
import { Checkbox } from '@/components/ui/checkbox';
import { TaskDialog } from '@/pages/home/task/TaskDialog';
import { useState } from 'react';
import { useTaskData } from '@/hooks';
import { openDialog } from '@/hooks';
import moment from 'moment';

export function TodayTask() {
  const { tasks } = useTaskData();
  const [openTaskDialog, setOpenTaskDialog] = useState(false);
  const showDialog = (index: number) => {
    if (index === -1 || !tasks) {
      openDialog('mutate-task', null);
    } else {
      openDialog('mutate-task', tasks[index]);
    }
  };
  return (
    <div className='mt-[30px]'>
      <div className='relative flex items-center justify-between'>
        <span className='font-bold text-navy text-[26px] leading-8'>Today Tasks</span>
        <div
          onClick={() => {
            showDialog(-1), setOpenTaskDialog(true);
          }}
          className='flex items-center gap-3'
        >
          <FeedPlusIcon size={32} className='cursor-pointer stroke-1 text-green' />
        </div>
        <TaskDialog open={openTaskDialog} onClose={() => setOpenTaskDialog(false)} />
      </div>
      <div className='flex flex-col w-full gap-3 mt-3'>
        {tasks?.map((item, index) => (
          <div
            key={item.id}
            className='p-[18px] rounded-2xl bg-white border border-light-gray flex justify-between items-center'
          >
            <div className='flex items-center gap-4 w-fit'>
              <Checkbox className='bg-white checked:!bg-green !w-6 !h-6 !border-1 !border-light-gray !rounded-full !p-2' />
              <div className='flex flex-col justify-between gap-3'>
                <div className='flex items-center gap-3'>
                  <span className='font-semibold text-[18px] leading-5'>{item.name}</span>
                  <Badge className='!bg-accent-blue !text-sm !font-bold !rounded-[5px] !px-2 !py-1 text-white !leading-5 !border-none'>
                    {prettyHour(item.to - item.from)}
                  </Badge>
                </div>
                <div className='flex items-center text-sm font-semibold text-background'>
                  <span className='mr-5'>{moment.unix(item.from).format('HH:mm')}</span>
                  {/* <div className='w-2 h-2 mr-2 rounded-full bg-accent-purple'></div> */}
                  {/* <span>{item.projectName}</span> */}
                </div>
              </div>
            </div>
            <div
              onClick={() => {
                setOpenTaskDialog(true);
                showDialog(index);
              }}
              className='flex items-center gap-3'
            >
              {' '}
              <ChevronRightIcon size={30} className='cursor-pointer text-dark-gray !stroke-[5] !font-bold' />
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
