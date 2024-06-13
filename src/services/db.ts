import { invoke } from '@tauri-apps/api/tauri';
import Database from 'tauri-plugin-sql-api';
let db: Promise<Database> | null = null;

export async function getDb() {
  if (db === null) {
    const dataDir: string = await invoke('get_data_dir');
    const dbPath = `${dataDir}/tpulse.sqlite3`;
    db = Database.load(`sqlite:${dbPath}`);
  }
  return db;
}
