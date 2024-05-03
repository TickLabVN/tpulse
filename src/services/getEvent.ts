import { invoke } from '@tauri-apps/api/tauri';
import Database from 'tauri-plugin-sql-api';
import { EventData, DatabaseItem } from '@/interfaces';

export const getEvents = async (): Promise<EventData[]> => {
  try {
    const homedirectory = await invoke('get_home_dir');
    const dbPath = `${homedirectory}/.ticklabvn.tpulse/tpulse.sqlite3`;
    const db = await Database.load(`sqlite:${dbPath}`);
    const result: DatabaseItem[] = await db.select('SELECT name, start_time, end_time FROM activity_log');
    return result.map((item: DatabaseItem, index: number) => ({
      id: index.toString(),
      title: item.name,
      start: item.start_time,
      end: item.end_time,
      icon: '/icons/youtube-icon.jpg'
    })) as EventData[];
  } catch (error) {
    return [];
  }
};
