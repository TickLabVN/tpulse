import { invoke } from '@tauri-apps/api/tauri';
import Database from 'tauri-plugin-sql-api';
let db: Promise<Database> | null = null;

export async function getDb() {
  if (db === null) {
    const homedir: string = await invoke('get_home_dir');
    const dbPath = `${homedir}/.ticklabvn.tpulse/tpulse.sqlite3`;
    db = Database.load(`sqlite:${dbPath}`);
  }
  return db;
}
