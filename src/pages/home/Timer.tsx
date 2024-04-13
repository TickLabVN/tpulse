import { HourglassIcon, StopwatchIcon } from '@primer/octicons-react';
import { useState } from 'react';
import { CircularProgressbarWithChildren } from 'react-circular-progressbar';

const tabHeaderStyle =
  'cursor-pointer py-[14px] rounded-lg flex-1 font-[500] leading-4 flex items-center justify-center gap-2 text-dark-gray';

export function Timer() {
  const [tab, setTab] = useState<'focus' | 'elapse'>('focus');

  return (
    <div className='p-[10px] border-[1px] rounded-[15px] border-light-gray w-full shadow-sm bg-white'>
      <div className='flex gap-0 justify-between items-center rounded-[10px] bg-[#EFEFEF] p-[6px]'>
        <div
          className={`${tabHeaderStyle}${tab === 'focus' ? ' bg-white' : ''}`}
          onClick={() => setTab('focus')}
        >
          <HourglassIcon size={16} className='text-accent-purple' /> Focus timer
        </div>
        <div
          className={`${tabHeaderStyle}${tab === 'elapse' ? ' bg-white' : ''}`}
          onClick={() => setTab('elapse')}
        >
          <StopwatchIcon size={16} className='text-green' /> Elapse timer
        </div>
      </div>
      <div className='flex justify-center mt-[14px]'>
        <CircularProgressbarWithChildren
          value={45}
          strokeWidth={9}
          className='w-48 h-48'
          styles={{
            path: {
              stroke: tab === 'focus' ? '#907BFD' : '#2A9665',
              transition: 'stroke-dashoffset 0.5s ease 0s'
            },
            trail: { stroke: '#D0D7DE33' }
          }}
        >
          <div className='bg-[#F2E7FF] w-[139px] h-[139px] rounded-full'>
            <img
              src='/timer_play.svg'
              alt='play'
              className='rounded-full cursor-pointer relative top-1/2 left-1/2 -translate-y-1/2 -translate-x-1/2'
            />
          </div>
        </CircularProgressbarWithChildren>
      </div>

      <div className='p-[14px] rounded-2xl bg-[#F6F7F8] flex flex-col items-center gap-1 mt-10 h-fit'>
        <div className='rounded-full text-green border-green border-[1px] py-[2px] px-1 font-bold text-[12px] leading-5'>
          WORKING
        </div>
        <div className='text-[#1F2328] font-bold text-[36px] leading-[43px]'>01:20:02</div>
      </div>
    </div>
  );
}
