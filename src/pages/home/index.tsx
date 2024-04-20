import { ChevronDownIcon, FileDirectoryIcon } from '@primer/octicons-react';
import { Timer } from './Timer';
import { TodayTask } from './TodayTask';
import { Timeline } from './timeline';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';

// TODO: Replace this with user's name
const userName = 'Nguyen Phuc Vinh';

export function HomePage() {
  return (
    <div className='w-full h-full'>
      <div className='w-full flex justify-between'>
        <div>
          <div className='text-[#738D95] text-sm font-[400] leading-4 flex items-center gap-[6px]'>
            <FileDirectoryIcon size={18} />
            <span>Dashboard</span>
          </div>
          <div className='font-bold text-navy text-[40px] leading-[48px] mt-2'>Welcome back, Vinh 👋🏻</div>
        </div>
        <div className='flex items-center rounded-full bg-[#DBEDEC] gap-2 p-[6px] pe-[12px] text-dark-gray h-fit font-bold cursor-pointer'>
          <Avatar>
            <AvatarFallback>PV</AvatarFallback>
            <AvatarImage src='/home/sample_avatar.png' alt='avatar' sizes='38px' />
          </Avatar>
          <span className='text-[16px] leading-[20px]'>{userName}</span>
          <ChevronDownIcon size={24} />
        </div>
      </div>
      <div className='w-full mt-[30px] grid grid-cols-4 gap-[30px]'>
        <div className='col-span-3'>
          <Timeline />
        </div>
        <div className='col-span-1'>
          <Timer />
          <TodayTask />
        </div>
      </div>
    </div>
  );
}
