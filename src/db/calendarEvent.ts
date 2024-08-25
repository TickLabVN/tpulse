import { log } from '@/utils/log';
import { db } from './db';

export type CalendarEvent = {
  id: number;
  name: string;
  description?: string;
  start_time: number;
  end_time: number;
  source?: string;
  external_id?: string;
};

export type WorkSession = {
  id: number;
  start_time: number;
  end_time?: number;
  status: 'open' | 'closed';
};

export async function getCalendarEvents(from: number, to: number): Promise<CalendarEvent[]> {
  try {
    return await db.select<CalendarEvent[]>(
      'SELECT * FROM "plan" WHERE "start_time" >= $1 AND "start_time" <= $2',
      [from, to]
    );
  } catch (error) {
    log.error(error);
    return [];
  }
}

export async function getWorkSessions(from: number, to: number): Promise<WorkSession[]> {
  try {
    return await db.select<WorkSession[]>(
      'SELECT * FROM "work_session" WHERE "start_time" >= $1 AND "start_time" <= $2',
      [from, to]
    );
  } catch (error) {
    log.error(error);
    return [];
  }
}
