import type { BasicTarget } from '@/interfaces';
import { getTargetElement } from '@/utils';
import { useState } from 'react';

import { useEventListener } from './useEventListener';

export interface CursorState {
  screenX: number;
  screenY: number;
  clientX: number;
  clientY: number;
  pageX: number;
  pageY: number;
  elementX: number;
  elementY: number;
  elementH: number;
  elementW: number;
  elementPosX: number;
  elementPosY: number;
}

export function useMouse(target?: BasicTarget) {
  const [state, setState] = useState<CursorState>({
    screenX: NaN,
    screenY: NaN,
    clientX: NaN,
    clientY: NaN,
    pageX: NaN,
    pageY: NaN,
    elementX: NaN,
    elementY: NaN,
    elementH: NaN,
    elementW: NaN,
    elementPosX: NaN,
    elementPosY: NaN
  });

  useEventListener('mousemove', (event: MouseEvent) => {
    const { screenX, screenY, clientX, clientY, pageX, pageY } = event;
    const newState: CursorState = {
      screenX,
      screenY,
      clientX,
      clientY,
      pageX,
      pageY,
      elementX: NaN,
      elementY: NaN,
      elementH: NaN,
      elementW: NaN,
      elementPosX: NaN,
      elementPosY: NaN
    };
    const targetElement = getTargetElement(target);
    if (targetElement) {
      const { left, top, width, height } = targetElement.getBoundingClientRect();
      newState.elementPosX = left + window.scrollX;
      newState.elementPosY = top + window.scrollY;
      newState.elementX = pageX - newState.elementPosX;
      newState.elementY = pageY - newState.elementPosY;
      newState.elementW = width;
      newState.elementH = height;
    }
    setState(newState);
  });

  return state;
}
