import { log } from '@/utils/log';
import { db } from './db';

export type Event = {
  id: number;
  name: string;
  description?: string;
  start_time: number;
  end_time: number;
  source?: string;
  external_id?: string;
};

async function getPlans(from: number, to: number): Promise<Event[]> {
  try {
    return await db.select<Event[]>('SELECT * FROM "plan" WHERE "start_time" >= $1 AND "start_time" <= $2', [
      from,
      to
    ]);
  } catch (error) {
    log.error(error);
    return [];
  }
}

export const planSvc = {
  getPlans
};
