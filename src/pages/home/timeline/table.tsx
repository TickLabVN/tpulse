import { useTaskData } from '@/hooks';
import { EventData } from '@/interfaces';
import { TaskData } from '@/interfaces';
import { filterEvent } from '@/utils';
import { ChecklistIcon, ClockFillIcon } from '@primer/octicons-react';
import moment from 'moment';
import { useCallback, useEffect, useMemo, useState } from 'react';

import { Event } from './event';

function getCurrentTz() {
  const date = new Date();
  const offset = date.getTimezoneOffset();
  const offsetInHours = -offset / 60;

  return `GMT-${offsetInHours}`;
}
const ActivityData: EventData[] = [
  {
    id: '1',
    title: 'Youtube',
    start: moment().startOf('day').add(12, 'hours').unix(),
    end: moment().startOf('day').add(14, 'hours').unix(),
    icon: '/icons/youtube-icon.jpg'
  },
  {
    id: '2',
    title: 'VsCode',
    start: moment().startOf('day').hours(14).minutes(30).unix(),
    end: moment().startOf('day').hours(20).minutes(15).unix(),
    icon: '/icons/vsc-icon.jpg'
  }
];
const TableRow: IComponent<{
  isLastRow: boolean;
  title: string;
  timeUnit: number;
  taskList: TaskData[] | undefined;
}> = ({ isLastRow, title, timeUnit, taskList }) => {
  let rowStyle = 'relative border-x border-light-gray h-10';
  if (!isLastRow) rowStyle += ' border-b';
  const filteredEventData = filterEvent(ActivityData, title, timeUnit);
  const filteredTaskData = filterEvent(taskList, title, timeUnit);
  return (
    <tr id={title}>
      <td className='font-bold px-[15px] align-bottom'>
        {!isLastRow ? <div className='text-sm translate-y-1/2 text-end text-gray'>{title}</div> : null}
      </td>
      <td className={rowStyle}>
        {filteredEventData.map((data, index) => (
          <Event
            key={index}
            event={data}
            timeUnit={timeUnit}
            top={
              'start' in data
                ? title === moment.unix(data.start).format('HH:mm')
                  ? 40
                  : (Math.abs(data.start - moment(title, 'HH:mm').unix()) / timeUnit) * 40
                : title === moment.unix(data.from).format('HH:mm')
                  ? 40
                  : (Math.abs(data.from - moment(title, 'HH:mm').unix()) / timeUnit) * 40
            }
          />
        ))}
      </td>
      <td className={rowStyle}>
        {filteredTaskData.map((data, index) => (
          <Event
            key={index}
            event={data}
            timeUnit={timeUnit}
            top={
              'from' in data
                ? title === moment.unix(data.from).format('HH:mm')
                  ? 40
                  : (Math.abs(data.from - moment(title, 'HH:mm').unix()) / timeUnit) * 40
                : title === moment.unix(data.start).format('HH:mm')
                  ? 40
                  : (Math.abs(data.start - moment(title, 'HH:mm').unix()) / timeUnit) * 40
            }
          />
        ))}
      </td>
      <td className='font-bold px-[15px] align-bottom'>
        {!isLastRow ? <div className='text-sm translate-y-1/2 text-start text-gray'>{title}</div> : null}
      </td>
    </tr>
  );
};

const ZOOM_SCALE = 100;
const NUM_SECS_PER_DAY = 86400;
const TIME_UNITS = Object.freeze(
  [1, 2, 5, 6, 10, 12, 15, 20, 25, 30, 40, 45, 50, 60].map((unit) => unit * 60)
);
const MAX_ZOOM_FACTOR = (TIME_UNITS.length - 1) * ZOOM_SCALE;

/**
 * Implementation description is in `docs/timetable.md`
 */
export function TimeTable() {
  const [zoomFactor, setZoomFactor] = useState<number>(MAX_ZOOM_FACTOR);
  // const [scrollHeight, setScrollHeight] = useState<number>(0);
  // const [scrollTop, setScrollTop] = useState<number>(0);
  // const [clientHeight, setClientHeight] = useState<number>(0);

  const timeUnit = useMemo(() => {
    const idx = Math.floor(zoomFactor / ZOOM_SCALE);
    return TIME_UNITS[idx];
  }, [zoomFactor]);

  // const queryRange = useMemo(() => {
  //   const startOfDay = moment().startOf('day').unix();
  //   return {
  //     from: startOfDay + Math.floor((NUM_SECS_PER_DAY * scrollTop) / scrollHeight),
  //     to: startOfDay + Math.ceil((NUM_SECS_PER_DAY * (scrollTop + clientHeight)) / scrollHeight)
  //   };
  // }, [clientHeight, scrollHeight, scrollTop]);
  // useEffect(() => {
  //   console.log(queryRange);
  // }, [queryRange]);

  useEffect(() => {
    const table = document.getElementById('timeline-table');
    if (!table) return;
    // setScrollTop(table.scrollTop);
    // setScrollHeight(table.scrollHeight);
    // setClientHeight(table.clientHeight);
    function handleWheel(e: WheelEvent) {
      const isPressingCtrl = e.ctrlKey;
      if (isPressingCtrl) {
        e.preventDefault();
        const diffPixel = e.deltaY;
        let diffFactor = Math.floor(Math.abs(diffPixel));
        if (diffPixel < 0) diffFactor *= -1;

        setZoomFactor((prev) => {
          const newZoomFactor = prev + diffFactor;
          if (newZoomFactor < 0) return 0;
          if (newZoomFactor > MAX_ZOOM_FACTOR) return MAX_ZOOM_FACTOR;

          return newZoomFactor;
        });
      } else {
        if (!table) return;
        // setScrollTop(table.scrollTop);
        // setScrollHeight(table.scrollHeight);
        // setClientHeight(table.clientHeight);
      }
    }

    table.addEventListener('wheel', handleWheel);
    return () => {
      table.removeEventListener('wheel', handleWheel);
    };
  }, []);
  const { tasks: taskList } = useTaskData();
  const renderRows = useCallback(() => {
    const numOfRows = Math.ceil(NUM_SECS_PER_DAY / timeUnit);

    const rows = [];
    for (let i = 0; i < numOfRows; i++) {
      const isLastRow = i === numOfRows - 1;
      const unixTs = moment()
        .startOf('day')
        .add((i + 1) * timeUnit, 'seconds');

      rows.push(
        <TableRow
          key={i}
          isLastRow={isLastRow}
          title={unixTs.format('HH:mm')}
          timeUnit={timeUnit}
          taskList={taskList}
        />
      );
    }
    return rows;
  }, [timeUnit, taskList]);

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
            <th className='py-5 border-b shadow-sm ps-8 border-x border-light-gray'>
              <div className='flex items-center gap-[14px]'>
                <div className='p-[10px] bg-[#D3FFD1] rounded-[10px] w-fit'>
                  <ClockFillIcon size={21} />
                </div>
                <div className='flex flex-col items-start gap-1'>
                  <span className='text-xl font-bold text-green'>ACTIVITY</span>
                  <span className='text-sm font-bold text-navy'>Automatic Tracking</span>
                </div>
              </div>
            </th>
            <th className='py-5 border-b shadow-sm ps-8 border-x border-light-gray'>
              <div className='flex items-center gap-[14px]'>
                <div className='p-[10px] bg-accent-light rounded-[10px] w-fit'>
                  <ChecklistIcon size={21} />
                </div>
                <div className='flex flex-col items-start gap-1'>
                  <span className='text-xl font-bold text-accent-blue'>PLANNING</span>
                  <span className='text-sm font-bold text-navy'>Manual Tracking</span>
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
