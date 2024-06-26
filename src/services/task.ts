import { TaskData } from '@/interfaces';
import { log } from '@/utils/log';
import moment from 'moment';

import { getDb } from './db';

async function getInCurrentDay(): Promise<TaskData[]> {
  const db = await getDb();
  const startOfDay = moment().startOf('day').unix();
  try {
    const tasks = await db.select<TaskData[]>('SELECT * FROM "tasks" WHERE "from" >= $1', [startOfDay]);
    return tasks;
  } catch (error) {
    log.error(error);
    return [];
  }
}

async function addTask(task: TaskData): Promise<void> {
  const db = await getDb();

  await db.execute('INSERT INTO "tasks" ("name", "from", "to") VALUES ($1, $2, $3)', [
    task.name,
    task.from,
    task.to
  ]);
}
export const taskSvc = {
  getInCurrentDay,
  addTask
};
