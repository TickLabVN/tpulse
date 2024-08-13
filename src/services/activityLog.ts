import { log } from '@/utils/log';
import { db } from './db';

export type ActivityLog = {
  name: string;
  start_time: number;
  end_time: number;
  category_tag: string;
  task_id: number | null;
};

async function getActivities(from: number, to: number): Promise<ActivityLog[]> {
  try {
    const activities = await db.select<ActivityLog[]>(
      'SELECT * FROM "activity_log" WHERE "start_time" >= $1 AND "end_time" <= $2',
      [from, to]
    );
    return activities;
  } catch (error) {
    log.error(error);
    return [];
  }
}

export const activityLogSvc = {
  getActivities
};
