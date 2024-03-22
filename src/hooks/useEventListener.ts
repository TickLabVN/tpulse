import { useEffect } from 'react';

type EventKey = 'mousemove';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function useEventListener(event: EventKey, listener: (...args: any[]) => void) {
  useEffect(() => {
    window.addEventListener(event, listener);
    return () => {
      window.removeEventListener(event, listener);
    };
  }, [event, listener]);
}
