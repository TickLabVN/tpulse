import { SyncTaskData } from '@/interfaces';
import { invoke } from '@tauri-apps/api';

export const syncTask = async (date: string) => {
  try {
    const response: string = await invoke('handle_google_calendar', {
      date
    });
    console.log(response);
    const data: SyncTaskData[] = JSON.parse(response);
    return data;
  } catch (error) {
    console.error(error);
  }
};
