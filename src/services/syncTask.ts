import { invoke } from '@tauri-apps/api/tauri';
import { SyncTaskData } from '@/interfaces';
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
