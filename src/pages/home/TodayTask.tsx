import { Badge } from '@/components';
import { Checkbox } from '@/components/ui/checkbox';
import { openDialog } from '@/hooks';
import { taskSvc } from '@/services';
import { prettyHour } from '@/utils';
import { ChevronRightIcon, FeedPlusIcon } from '@primer/octicons-react';
import { useQuery } from '@tanstack/react-query';
import moment from 'moment';

export function TodayTask() {
  const { data: tasks } = useQuery({
    queryKey: ['tasks'],
    queryFn: taskSvc.getInCurrentDay
  });

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
        <div className='flex items-center gap-3'>
          <FeedPlusIcon size={32} className='cursor-pointer stroke-1 text-green' />
        </div>
      </div>
      <div className='flex flex-col w-full gap-3 mt-3'>
        {tasks?.map((item, index) => {
          if (!item.start || !item.end) return null;
          return (
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
                      {prettyHour(item.end - item.start)}
                    </Badge>
                  </div>
                  <div className='flex items-center text-sm font-semibold text-background'>
                    <span className='mr-5'>{moment.unix(item.start).format('HH:mm')}</span>
                  </div>
                </div>
              </div>
              <div
                onClick={() => {
                  showDialog(index);
                }}
                className='flex items-center gap-3'
              >
                {' '}
                <ChevronRightIcon
                  size={30}
                  className='cursor-pointer text-dark-gray !stroke-[5] !font-bold'
                />
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
