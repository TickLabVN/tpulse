import Database from '@tauri-apps/plugin-sql';

export const db = await Database.load('sqlite:tpulse.db');
