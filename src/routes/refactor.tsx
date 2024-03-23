import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/refactor')({
  component: () => <div>Hello /refactor!</div>
});
