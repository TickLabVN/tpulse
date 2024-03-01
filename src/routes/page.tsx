import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/page')({
  component: ExamplePage
});

function ExamplePage() {
  return (
    <div className='p-2'>
      <h3>Hello world!</h3>
    </div>
  );
}
