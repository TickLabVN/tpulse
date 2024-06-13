import { useEffect } from 'react';

type DialogName = 'mutate-task';
type EventKey = `dialog:open:${DialogName}`;

export function useListenEvent<T = unknown>(event: EventKey, handler: (params: T) => void) {
  useEffect(() => {
    const baseHandler = (e: CustomEvent<T>) => handler(e.detail);
    // @ts-ignore
    window.addEventListener(event, baseHandler);
    return () => {
      // @ts-ignore
      window.removeEventListener(event, baseHandler);
    };
  }, [event, handler]);
}

export function emitEvent<T = unknown>(event: EventKey, payload?: T) {
  return window.dispatchEvent(new CustomEvent(event, { detail: payload }));
}

export function openDialog<T = unknown>(dialog: DialogName, payload?: T) {
  emitEvent(`dialog:open:${dialog}`, payload);
}
