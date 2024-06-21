import { BriefcaseIcon, GearIcon, GraphIcon, HomeIcon, InfoIcon } from '@primer/octicons-react';
import { Link, useRouterState } from '@tanstack/react-router';

const mainRoutes = [
  {
    path: '/',
    name: 'Home',
    icon: <HomeIcon size={24} />
  },
  {
    path: '/page',
    name: 'Statistic',
    icon: <GraphIcon size={24} />
  },
  {
    path: '/tasks',
    name: 'Projects & Tasks',
    icon: <BriefcaseIcon size={24} />
  },
  {
    path: '/setting',
    name: 'Setting',
    icon: <GearIcon size={24} />
  }
];

export function SideBar() {
  const {
    location: { pathname }
  } = useRouterState();

  return (
    <div
      style={{ boxShadow: '0px 0px 30px 0px #0000000D' }}
      className='w-full h-[calc(100%-40px)] my-[20px] rounded-tr-[50px] rounded-br-[50px] flex flex-col items-center justify-between bg-white border border-s-none border-light-gray p-5'
    >
      <div className='flex flex-col items-center w-[68px] h-fit'>
        <Link to='/'>
          <img src='/logo.svg' alt='logo' className='w-full h-[68px] rounded-full mb-16' />
        </Link>
        <div className='flex flex-col w-full gap-0'>
          {mainRoutes.map(({ path, icon }) => (
            <Link
              key={path}
              to={path}
              activeProps={{ className: 'font-bold' }}
              activeOptions={{ exact: true }}
              className={`text-[#1F2328] font-bold text-center w-full h-[68px] flex flex-col items-center justify-center rounded-full ${pathname === path ? 'bg-[#DCECE3]' : 'cursor-pointer'}`}
            >
              {icon}
            </Link>
          ))}
        </div>
      </div>
      <div
        onClick={() => window.open('https://github.com/TickLabVN')}
        className='font-bold text-[#1F2328] text-center w-full h-[68px] flex flex-col items-center justify-center rounded-full cursor-pointer'
      >
        <InfoIcon size={24} className='font-bold stroke-2' />
      </div>
    </div>
  );
}
