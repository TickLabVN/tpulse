import { NUM_SECS_IN_DAY, TIMETABLE_UNIT } from '@/constants';
import { log } from '@/utils/log';
import {
  Button,
  Image,
  Menu,
  MenuButton,
  MenuItem,
  MenuList,
  MenuPopover,
  MenuTrigger,
  type SelectTabEventHandler,
  Tab,
  TabList
} from '@fluentui/react-components';
import { DatePicker } from '@fluentui/react-datepicker-compat';
import { CalendarSync24Regular, ChevronLeft16Regular, ChevronRight16Regular } from '@fluentui/react-icons';
import { useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import moment from 'moment';
import { useCallback, useMemo, useState } from 'react';
import { TimeRow } from './timeRow';

const rowArr = Array.from({ length: NUM_SECS_IN_DAY / TIMETABLE_UNIT });

function onFormatDate(date?: Date): string {
  return !date ? '' : `${date.getDate()}/${date.getMonth() + 1}/${date.getFullYear()}`;
}

export function HomePage() {
  const [selectedDate, setSelectedDate] = useState<moment.Moment>(moment());
  const goPrevious = useCallback(() => {
    setSelectedDate((prev) => moment(prev).subtract(1, 'day'));
  }, []);
  const goNext = useCallback(() => {
    setSelectedDate((prev) => moment(prev).add(1, 'day'));
  }, []);

  const beginOfDay = useMemo(() => selectedDate.startOf('day').unix(), [selectedDate]);
  const onParseDateFromString = useCallback(
    (newValue: string): Date => moment(newValue, 'DD/MM/YYYY').toDate(),
    []
  );

  const [mode, setMode] = useState<DashboardTab>('work_session');
  const onTabSelect: SelectTabEventHandler = useCallback(
    (_, data) => setMode(data.value as DashboardTab),
    []
  );

  const queryClient = useQueryClient();

  const fetchGoogleEvents = useCallback(async () => {
    const params = {
      fromDate: selectedDate.startOf('day').toISOString(),
      toDate: selectedDate.endOf('day').toISOString()
    };
    try {
      const success = await invoke<boolean>('sync_google_calendar', params);
      if (!success) {
        await invoke('connect_google_account');
        invoke<boolean>('sync_google_calendar', params);
      }

      await queryClient.invalidateQueries({
        queryKey: ['calendar_event'],
        refetchType: 'all'
      });
    } catch (error) {
      log.error(error);
    }
  }, [queryClient, selectedDate]);

  return (
    <div className='w-full overflow-y-scroll max-h-screen no-scrollbar'>
      <div className='w-full sticky top-0 z-20 bg-white shadow-mdd pb-6'>
        <div className='flex gap-2 px-8 pt-5'>
          <DatePicker
            className='flex-1'
            value={selectedDate.toDate()}
            onSelectDate={(date) => setSelectedDate(moment(date))}
            formatDate={onFormatDate}
            parseDateFromString={onParseDateFromString}
            showGoToToday={true}
          />
          <Button icon={<ChevronLeft16Regular />} onClick={goPrevious} />
          <Button icon={<ChevronRight16Regular />} onClick={goNext} />
        </div>

        <TabList className='w-full flex px-5 mt-2' defaultSelectedValue={mode} onTabSelect={onTabSelect}>
          <Tab value='work_session' className='flex-1'>
            Sessions
          </Tab>
          <Tab value='project' className='flex-1'>
            {/* TODO: display projects instead */}
            Activities
          </Tab>
          <Tab value='calendar_event' className='flex-1'>
            Events
          </Tab>
        </TabList>
      </div>
      <div className='rounded-md'>
        {rowArr.map((_, i) => {
          const startTime = moment.unix(beginOfDay + i * TIMETABLE_UNIT);
          return (
            // biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
            <TimeRow key={i} startTime={startTime} mode={mode} />
          );
        })}
      </div>

      {mode === 'calendar_event' && (
        <Menu>
          <MenuTrigger disableButtonEnhancement>
            <MenuButton
              className='fixed bottom-4 right-4 !min-w-12 !h-12 z-10'
              icon={<CalendarSync24Regular />}
              shape='circular'
              appearance='primary'
            />
          </MenuTrigger>
          <MenuPopover>
            <MenuList>
              <MenuItem className='flex items-center gap-2' onClick={() => fetchGoogleEvents()}>
                <span className='me-2'>Sync with Google Calendar</span>
                <Image src='/icons/google_calendar.svg' alt='Google' width={24} height={24} />
              </MenuItem>
            </MenuList>
          </MenuPopover>
        </Menu>
      )}
    </div>
  );
}
