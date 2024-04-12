import { ChevronDownIcon, FileDirectoryIcon } from '@primer/octicons-react';
import { Timer } from './Timer';
import { Avatar } from '@primer/react';

// TODO: Replace this with user's name
const userName = 'Nguyen Phuc Vinh';

export function HomePage() {
  return (
    <div className='w-full h-full'>
      <div className='w-full flex justify-between'>
        <div>
          <div className='text-[#738D95] text-[14px] font-[400] leading-4 flex items-center gap-[6px]'>
            <FileDirectoryIcon size={18} />
            <span>Dashboard</span>
          </div>
          <div className='font-bold text-navy text-[40px] leading-[48px] mt-2'>Welcome back, Amyra 👋🏻</div>
        </div>
        <div className='flex items-center rounded-full bg-[#DBEDEC] gap-2 p-[6px] pe-[12px] text-[#404040] h-fit font-bold cursor-pointer'>
          <Avatar src='/sample_avatar.png' alt='avatar' size={38} />
          <span className='text-[16px] leading-[20px]'>{userName}</span>
          <ChevronDownIcon size={24} />
        </div>
      </div>
      <div className='w-full mt-[30px] grid grid-cols-4'>
        <div className='col-span-3'>Hello</div>
        <div className='col-span-1'>
          <Timer />
        </div>
      </div>
    </div>
  );
}
