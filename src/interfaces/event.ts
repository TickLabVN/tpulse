interface EventData {
  id: string;
  title: string;
  start: number;
  end: number;
  icon: string;
}

interface EventStore {
  eventList: EventData[];
  setEventList: (events: EventData[]) => void;
  fetchEvents: () => Promise<void>;
}

interface DatabaseItem {
  name: string;
  start_time: number;
  end_time: number;
}
export type { EventData, EventStore, DatabaseItem };
