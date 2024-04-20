import { formatTime } from '@/utils';
import { ChecklistIcon, ClockFillIcon } from '@primer/octicons-react';
import moment from 'moment';
import { useCallback, useEffect, useMemo, useState } from 'react';

function getCurrentTz() {
  const date = new Date();
  const offset = date.getTimezoneOffset();
  const offsetInHours = -offset / 60;

  return `GMT-${offsetInHours}`;
}

const TableRow: IComponent<{ isLastRow: boolean; title: string }> = ({ isLastRow, title }) => {
  let rowStyle = 'border-x border-light-gray h-10';
  if (!isLastRow) rowStyle += ' border-b';

  return (
    <tr id={title}>
      <td className='font-bold px-[15px] align-bottom'>
        {!isLastRow ? <div className='text-sm text-end text-gray translate-y-1/2'>{title}</div> : null}
      </td>
      <td className={rowStyle}></td>
      <td className={rowStyle}></td>
      <td className='font-bold px-[15px] align-bottom'>
        {!isLastRow ? <div className='text-sm text-start text-gray translate-y-1/2'>{title}</div> : null}
      </td>
    </tr>
  );
};

// type QueryRange = {
//   from: number;
//   to: number;
// }
const ZOOM_UNIT_PX = 2;
const NUM_SECS_PER_HOUR = 3600;
const NUM_SECS_PER_DAY = 86400;

/**
 * Implementation description is in `docs/timetable.md`
 */
export function TimeTable() {
  // const [queryRange, setQueryRange] = useState<QueryRange>();
  const [zoomFactor, setZoomFactor] = useState<number>(1);
  const [scrollHeight, setScrollHeight] = useState<number>(0);
  const [scrollTop, setScrollTop] = useState<number>(0);
  const [clientHeight, setClientHeight] = useState<number>(0);

  const timeUnit = useMemo(() => NUM_SECS_PER_HOUR / zoomFactor, [zoomFactor]);

  const queryRange = useMemo(() => {
    // console.log({
    //   scrollHeight,
    //   scrollTop,
    //   clientHeight,
    // });
    const startOfDay = moment().startOf('day').unix();
    return {
      from: startOfDay + Math.floor((NUM_SECS_PER_DAY * scrollTop) / scrollHeight),
      to: startOfDay + Math.ceil((NUM_SECS_PER_DAY * (scrollTop + clientHeight)) / scrollHeight)
    };
  }, [clientHeight, scrollHeight, scrollTop]);

  useEffect(() => {
    console.log(queryRange);
    // TODO: fetch eve
  }, [queryRange]);

  useEffect(() => {
    const table = document.getElementById('timeline-table');
    if (!table) return;
    setScrollTop(table.scrollTop);
    setScrollHeight(table.scrollHeight);
    setClientHeight(table.clientHeight);

    function handleWheel(e: WheelEvent) {
      const isPressingCtrl = e.ctrlKey;
      if (isPressingCtrl) {
        e.preventDefault();
        const diffPixel = e.deltaY;
        let diffFactor = Math.floor(Math.abs(diffPixel) / ZOOM_UNIT_PX);
        if (diffPixel < 0) diffFactor *= -1;

        setZoomFactor((prev) => Math.max(1, prev + diffFactor / 100));
      } else {
        if (!table) return;
        setScrollTop(table.scrollTop);
        setScrollHeight(table.scrollHeight);
        setClientHeight(table.clientHeight);
      }
    }

    table.addEventListener('wheel', handleWheel);

    return () => {
      table.removeEventListener('wheel', handleWheel);
    };
  }, []);

  const renderRows = useCallback(() => {
    const numOfRows = Math.ceil(NUM_SECS_PER_DAY / timeUnit);

    const rows = [];
    for (let i = 0; i < numOfRows; i++) {
      const isLastRow = i === numOfRows - 1;
      const time = formatTime(Math.floor(i * timeUnit));
      rows.push(<TableRow key={i} isLastRow={isLastRow} title={time} />);
    }
    return rows;
  }, [timeUnit]);

  return (
    <div
      className='rounded-2xl bg-white p-0 border-light-gray border mt-4 max-h-[80vh] no-scrollbar overflow-y-scroll'
      id='timeline-table'
    >
      <table className='w-full border-collapse'>
        <thead className='sticky top-0 z-10 bg-white'>
          <tr>
            <th className='px-[15px] text-end w-20 shadow-sm border-b border-light-gray'>
              <div>EST</div>
              <div>{getCurrentTz()}</div>
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
              <div>{getCurrentTz()}</div>
            </th>
          </tr>
        </thead>
        <tbody id='timeline-table-body'>{renderRows()}</tbody>
      </table>
    </div>
  );
}
