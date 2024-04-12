import { SideBar } from '@components';
import { Outlet, createRootRoute } from '@tanstack/react-router';

export const Route = createRootRoute({
  component: RootComponent
});

function RootComponent() {
  return (
    <main className='w-full h-full'>
      <div className='fixed left-0 w-[108px] h-full z-50'>
        <SideBar />
      </div>
      <div className='h-full ms-[108px] p-[30px]'>
        <Outlet />
      </div>
    </main>
  );
}
