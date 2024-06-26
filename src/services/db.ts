import Database from '@tauri-apps/plugin-sql';

let db: Promise<Database> | null = null;

export function getDb(): Promise<Database> {
  if (!db) db = Database.load('sqlite:tpulse.sqlite3');
  return db;
}
