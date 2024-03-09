import { MutableRefObject, useRef } from 'react';
import { createFileRoute } from '@tanstack/react-router';
import { useZoom } from '@hooks';

export const Route = createFileRoute('/')({
  component: HomePage
});

function HomePage() {
  const svgRef = useRef<HTMLDivElement>(null);
  const dimensions = useZoom(svgRef as MutableRefObject<HTMLElement>);

  return (
    <div ref={svgRef} style={{ width: '100vw', height: '100vh', border: '1px solid red' }}>
      <svg style={{ width: '100%', height: '100%' }}>
        <g>
          <rect x={dimensions.width / 2} y={dimensions.height / 2} width='3em' height='3em' fill='gold' />
        </g>
      </svg>
    </div>
  );
}
