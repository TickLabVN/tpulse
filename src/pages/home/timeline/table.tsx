import { ActivityLog, Task, activityLogSvc, taskSvc } from '@/services';
import { log } from '@/utils/log';
import { ChecklistIcon, ClockFillIcon } from '@primer/octicons-react';
import { useQuery } from '@tanstack/react-query';
import moment from 'moment';
import { useCallback, useEffect, useMemo, useState } from 'react';
import { useDebounceValue } from 'usehooks-ts';

import { TimelineRow } from './row';

function getCurrentTz() {
  const date = new Date();
  const offset = date.getTimezoneOffset();
  const offsetInHours = -offset / 60;

  return `GMT-${offsetInHours}`;
}

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
  const [scrollHeight, setScrollHeight] = useState<number>(0);
  const [scrollTop, setScrollTop] = useState<number>(0);
  const [clientHeight, setClientHeight] = useState<number>(0);

  const timeUnit = useMemo(() => {
    const idx = Math.floor(zoomFactor / ZOOM_SCALE);
    return TIME_UNITS[idx];
  }, [zoomFactor]);

  const [queryRange, setQueryRange] = useDebounceValue(
    {
      from: moment().startOf('day').unix(),
      to: moment().endOf('day').unix()
    },
    1000
  );

  useEffect(() => {
    const startOfDay = moment().startOf('day').unix();
    const range = {
      from: startOfDay + Math.floor((NUM_SECS_PER_DAY * scrollTop) / scrollHeight),
      to: startOfDay + Math.ceil((NUM_SECS_PER_DAY * (scrollTop + clientHeight)) / scrollHeight)
    };
    log.info(range, 'range');
    setQueryRange(range);
  }, [clientHeight, scrollHeight, scrollTop, setQueryRange]);

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

  const { data: tasks } = useQuery({
    queryKey: ['tasks'],
    queryFn: () => taskSvc.getInRange(queryRange.from, queryRange.to)
  });

  const { data: activityLogs } = useQuery({
    queryKey: ['activities'],
    queryFn: () => activityLogSvc.getLogs(queryRange.from, queryRange.to)
  });

  const renderRows = useCallback(() => {
    const numOfRows = Math.ceil(NUM_SECS_PER_DAY / timeUnit);
    const timelineRows = [];

    let rowIdx = 0;
    let activityIdx = 0;

    for (let i = 0; i < numOfRows; i++) {
      const isLastRow = i === numOfRows - 1;
      const rowStartTime = moment()
        .startOf('day')
        .add((i + 1) * timeUnit, 'seconds')
        .unix();

      const rowEndTime = moment()
        .startOf('day')
        .add((i + 2) * timeUnit, 'seconds')
        .unix();

      const rowActivityLogs: ActivityLog[] = [];
      if (activityLogs) {
        while (activityIdx < activityLogs.length) {
          if (
            rowStartTime <= activityLogs[activityIdx].start_time &&
            activityLogs[activityIdx].start_time <= rowEndTime
          ) {
            rowActivityLogs.push(activityLogs[activityIdx]);
          } else if (activityLogs[activityIdx].start_time > rowEndTime) {
            break;
          }
          activityIdx++;
        }
      }

      const rowTasks: Task[] = [];
      if (tasks) {
        while (rowIdx < tasks.length) {
          const taskStartTime = tasks[rowIdx].start;
          if (!taskStartTime) continue;

          if (rowStartTime <= taskStartTime && taskStartTime <= rowEndTime) {
            rowTasks.push(tasks[rowIdx]);
          } else if (taskStartTime > rowEndTime) {
            break;
          }
          rowIdx++;
        }
      }

      timelineRows.push(
        <TimelineRow
          key={i}
          displayTime={moment.unix(rowStartTime).format('HH:mm')}
          isLastRow={isLastRow}
          timeUnit={timeUnit}
          tasks={rowTasks}
          activities={rowActivityLogs}
        />
      );
    }
    return timelineRows;
  }, [timeUnit, tasks, activityLogs]);

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
