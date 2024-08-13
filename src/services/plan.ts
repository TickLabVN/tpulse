import { log } from '@/utils/log';
import moment from 'moment';
import { getDb } from './db';

export type Task = {
  id: number;
  name: string;
  status: 'todo' | 'in_progress' | 'done';
  start: number | null;
  end: number | null;
  created_at: number;
  project_id: number | null;
  color: string;
};

async function getInCurrentDay(): Promise<Task[]> {
  const db = await getDb();
  const startOfDay = moment().startOf('day').unix();
  try {
    const tasks = await db.select<Task[]>('SELECT * FROM "tasks" WHERE "start" >= $1', [startOfDay]);
    return tasks;
  } catch (error) {
    log.error(error);
    return [];
  }
}

async function getInRange(start: number, end: number): Promise<Task[]> {
  const db = await getDb();
  try {
    const tasks = await db.select<Task[]>('SELECT * FROM "tasks" WHERE "start" >= $1 AND "end" <= $2', [
      start,
      end
    ]);
    return tasks;
  } catch (error) {
    log.error(error);
    return [];
  }
}

export const taskSvc = {
  getInCurrentDay,
  getInRange
};
