import { create } from 'zustand';
import { EventStore } from '@/interfaces';
import { getEvents } from '@/services';

export const useEventStore = create<EventStore>((set) => ({
  eventList: [],
  setEventList: (events) => set({ eventList: events }),
  fetchEvents: async () => {
    const events = await getEvents();
    set({ eventList: events });
  }
}));
