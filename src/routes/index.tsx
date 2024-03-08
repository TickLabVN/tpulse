import { MutableRefObject, useRef } from 'react';
import { createFileRoute } from '@tanstack/react-router';
import { useZoom } from '@hooks';

export const Route = createFileRoute('/')({
  component: HomePage
});

function HomePage() {
  const svgRef = useRef<SVGSVGElement>(null);
  // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-call
  const dimensions = useZoom(svgRef as MutableRefObject<SVGSVGElement>);

  return (
    <div style={{ width: '100vw', height: '100vh', border: '1px solid red' }}>
      <svg ref={svgRef} style={{ width: '100%', height: '100%' }}>
        <g>
          {/* eslint-disable-next-line @typescript-eslint/no-unsafe-member-access */}
          <rect x={dimensions.width / 2} y={dimensions.height / 2} width='3em' height='3em' fill='gold' />
        </g>
      </svg>
    </div>
  );
}
