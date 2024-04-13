import { ChecklistIcon, ClockFillIcon } from '@primer/octicons-react';

const maxCol = 12;
const arr = Array.from({ length: maxCol }, (_, i) => i);

export function TimeTable() {
  return (
    <div className='rounded-2xl bg-white p-0 border-light-gray border mt-4 overflow-hidden'>
      <table className='w-full border-collapse'>
        <thead>
          <tr>
            <th className='px-[15px] text-end w-20 shadow-sm border-b border-light-gray'>
              <div>EST</div>
              <div>GMT-7</div>
            </th>
            <th className='py-5 ps-8 border-x border-b border-light-gray shadow-sm'>
              <div className='flex items-center gap-[14px]'>
                <div className='p-[10px] bg-[#D3FFD1] rounded-[10px] w-fit'>
                  <ClockFillIcon size={21} />
                </div>
                <div className='flex flex-col gap-1 items-start'>
                  <span className='text-green font-bold text-xl'>ACTIVITY</span>
                  <span className='text-navy font-bold text-sm'>Automatic Tracking</span>
                </div>
              </div>
            </th>
            <th className='py-5 ps-8 border-x border-b border-light-gray shadow-sm'>
              <div className='flex items-center gap-[14px]'>
                <div className='p-[10px] bg-accent-light rounded-[10px] w-fit'>
                  <ChecklistIcon size={21} />
                </div>
                <div className='flex flex-col gap-1 items-start'>
                  <span className='text-accent-blue font-bold text-xl'>PLANNING</span>
                  <span className='text-navy font-bold text-sm'>Manual Tracking</span>
                </div>
              </div>
            </th>
            <th className='px-[15px] text-start w-20 shadow-sm border-b border-light-gray'>
              <div>EST</div>
              <div>GMT-7</div>
            </th>
          </tr>
        </thead>
        <tbody>
          {arr.map((_, i) => {
            const isLastRow = i === arr.length - 1;
            let rowStyle = 'border-x border-light-gray h-10';
            if (!isLastRow) rowStyle += ' border-b';

            return (
              <tr key={i}>
                <td className='font-bold px-[15px] align-bottom'>
                  {!isLastRow ? (
                    <div className='text-sm text-end text-gray translate-y-1/2'>{i} AM</div>
                  ) : (
                    <td></td>
                  )}
                </td>
                <td className={rowStyle}></td>
                <td className={rowStyle}></td>
                <td className='font-bold px-[15px] align-bottom'>
                  {!isLastRow ? (
                    <div className='text-sm text-start text-gray translate-y-1/2'>{i} AM</div>
                  ) : (
                    <td></td>
                  )}
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
