import { SyncTaskData } from '@/interfaces';
import { log } from '@/utils/log';
import { invoke } from '@tauri-apps/api';

export const syncTask = async (date: string) => {
  try {
    const response: string = await invoke('handle_google_calendar', {
      date
    });
    log.info(response);
    const data: SyncTaskData[] = JSON.parse(response);
    return data;
  } catch (error) {
    log.error(error);
  }
};
