import { ChecklistIcon, ClockFillIcon } from '@primer/octicons-react';

function getCurrentTz() {
  const date = new Date();
  const offset = date.getTimezoneOffset();
  const offsetInHours = -offset / 60;

  return `GMT-${offsetInHours}`;
}

export function TimeTableHeader() {
  const currentTz = getCurrentTz();

  return (
    <div className='flex w-full justify-between items-center border-b-[1px] border-light-gray shadow-sm sticky top-0'>
      <div className='text-center text-sm leading-4 font-bold w-20'>
        <div>EST</div>
        <div>{currentTz}</div>
      </div>
      <div className='py-4 flex-1 px-8 border-s-[1px] border-light-gray'>
        <div className='flex items-center gap-4'>
          <div className='p-3 bg-[#D3FFD1] rounded-xl w-fit'>
            <ClockFillIcon size={21} />
          </div>
          <div className='flex flex-col items-start gap-1'>
            <span className='text-xl font-bold text-green'>ACTIVITY</span>
            <span className='text-sm font-bold text-navy'>Automatic Tracking</span>
          </div>
        </div>
      </div>
      <div className='py-4 flex-1 px-8 border-x-[1px] border-light-gray'>
        <div className='flex items-center gap-4'>
          <div className='p-3 bg-accent-light rounded-xl w-fit'>
            <ChecklistIcon size={21} />
          </div>
          <div className='flex flex-col items-start gap-1'>
            <span className='text-xl font-bold text-accent-blue'>PLANNING</span>
            <span className='text-sm font-bold text-navy'>Manual Tracking</span>
          </div>
        </div>
      </div>
      <div className='text-center text-sm leading-4 font-bold w-20'>
        <div>EST</div>
        <div>{currentTz}</div>
      </div>
    </div>
  );
}
