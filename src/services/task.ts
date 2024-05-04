import moment from 'moment';
import { getDb } from './db';

export type Task = {
  id: number;
  name: string;
  from: number;
  to: number;
  project_id: number | null;
};

// TODO: get project together with task
async function getInCurrentDay(): Promise<Task[]> {
  const db = await getDb();
  const startOfDay = moment().startOf('day').unix();
  try {
    const tasks = await db.select<Task[]>('SELECT * FROM "tasks" WHERE "from" >= $1', [startOfDay]);
    return tasks;
  } catch (error) {
    console.error(error);
    return [];
  }
}

export const taskSvc = {
  getInCurrentDay
};
