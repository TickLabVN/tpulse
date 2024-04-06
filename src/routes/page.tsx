import { createFileRoute } from '@tanstack/react-router';
import { DayView } from '@pages';

export const Route = createFileRoute('/page')({
  component: DayView
});