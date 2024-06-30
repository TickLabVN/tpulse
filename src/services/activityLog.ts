import { log } from '@/utils/log';

import { getDb } from './db';

export type ActivityLog = {
  name: string;
  start_time: number;
  end_time: number;
  category_tag: string;
  task_id: number | null;
};

async function getLogs(start: number, end: number): Promise<ActivityLog[]> {
  const db = await getDb();
  try {
    const activities = await db.select<ActivityLog[]>(
      'SELECT * FROM "activity_log" WHERE "start_time" >= $1 AND "end_time" <= $2',
      [start, end]
    );
    return activities;
  } catch (error) {
    log.error(error);
    return [];
  }
}

export const activityLogSvc = {
  getLogs
};
