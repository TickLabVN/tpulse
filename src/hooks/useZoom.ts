import { useState, useEffect } from 'react';
import { ResizeObserver } from '@juggle/resize-observer';
import * as d3 from 'd3';

interface Dimension {
  width: number;
  height: number;
}

export function useZoom(ref: React.MutableRefObject<SVGSVGElement>) {
  const [dimensions, setDimensions] = useState<Dimension>({
    width: 0,
    height: 0
  });

  useEffect(() => {
    const observeTarget = ref.current;
    const resizeObserver = new ResizeObserver((entries) => {
      entries.forEach((entry) => {
        setDimensions(entry.contentRect);
      });
    });
    resizeObserver.observe(observeTarget);
    return () => resizeObserver.unobserve(observeTarget);
  }, [ref]);

  useEffect(() => {
    const element = d3.select(ref.current);
    const g = element.select('g');

    element.call(
      d3
        .zoom<SVGSVGElement, unknown>()
        .extent([
          [0, 0],
          [dimensions.width, dimensions.height]
        ])
        .scaleExtent([0.5, 8])
        .on('zoom', (event: d3.D3ZoomEvent<SVGSVGElement, unknown>) => {
          g.attr('transform', event.transform.toString());
        })
    );

    element.attr('width', dimensions.width).attr('height', dimensions.height);
  }, [dimensions, ref]);

  return dimensions;
}
