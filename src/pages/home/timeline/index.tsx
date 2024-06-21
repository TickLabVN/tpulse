import { useTaskStore } from '@/states';
import { ChevronLeftIcon, ChevronRightIcon } from '@primer/octicons-react';
import moment from 'moment';
import { useRef, useState } from 'react';

import { TimeTable } from './table';

export function Timeline() {
  const [currentTime, setCurrentTime] = useState<number>(moment().startOf('day').unix());
  const dateInputRef = useRef<HTMLInputElement>(null);
  const { syncTask } = useTaskStore();
  return (
    <>
      <div className='flex items-end justify-between'>
        <div className='font-semibold text-navy text-[28px] leading-8'>Time Tracking</div>
        <div className='flex gap-3'>
          <div
            className='border-[2px] border-light-gray px-6 py-3 rounded-[10px] text-navy text-[16px] leading-5 font-[500] bg-white cursor-pointer'
            onClick={() => syncTask(moment.unix(currentTime).format('YYYY-MM-DD'))}
          >
            Sync with Google Calendar
          </div>
          <div
            className='border-[2px] border-light-gray px-6 py-3 rounded-[10px] text-navy text-[16px] leading-5 font-[500] bg-white cursor-pointer'
            onClick={() => setCurrentTime(moment().startOf('day').unix())}
          >
            Today
          </div>
          <div className='border-[2px] border-light-gray px-6 py-3 rounded-[10px] text-navy text-[16px] leading-5 font-[500] bg-white flex items-center gap-[29px]'>
            <div onClick={() => setCurrentTime(currentTime - 86400)}>
              <ChevronLeftIcon size={20} className='cursor-pointer text-dark-gray' />
            </div>
            <span
              className='text-navy font-semibold text-[16px]'
              onClick={() => {
                dateInputRef.current?.showPicker();
              }}
            >
              {moment.unix(currentTime).format('ddd, DD/MM/YYYY')}
            </span>
            <input type='date' className='hidden' ref={dateInputRef} />
            <div onClick={() => setCurrentTime(currentTime + 86400)}>
              <ChevronRightIcon size={20} className='cursor-pointer text-dark-gray' />
            </div>
          </div>
        </div>
      </div>
      <TimeTable />
    </>
  );
}
