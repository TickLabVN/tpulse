import { log } from '@/utils/log';
import moment from 'moment';
import { db } from './db';

export type ActivityLog = {
  name: string;
  start_time: number;
  end_time?: number;
  category?: string;
};

export async function categorizeActivities(
  from: number,
  to: number
): Promise<{ category: string; percentage: number }[]> {
  const percentage: Record<string, number> = {};
  try {
    const activities = await db.select<ActivityLog[]>(
      'SELECT * FROM "activity_log" WHERE "start_time" >= $1 AND "start_time" < $2',
      [from, to]
    );
    for (const activity of activities) {
      if (!activity.end_time) activity.end_time = moment().unix();
      if (activity.end_time > to) activity.end_time = to;

      if (!activity.category) activity.category = 'Uncategorized';
      const count = percentage[activity.category] ?? 0;
      percentage[activity.category] = count + 1;
    }

    for (const category in percentage) {
      percentage[category] = (percentage[category] / activities.length) * 100;
    }

    const result = Object.entries(percentage).map(([category, percentage]) => ({ category, percentage }));
    result.sort((a, b) => b.percentage - a.percentage);
    result.splice(4);

    if (result.length >= 4) {
      const otherPercentage = 100 - result.reduce((acc, { percentage }) => acc + percentage, 0);
      result.push({ category: 'Other', percentage: otherPercentage });
    }

    return result;
  } catch (error) {
    log.error(error);
    return [];
  }
}
