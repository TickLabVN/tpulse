import { createFileRoute } from '@tanstack/react-router';
import FullCalendar from '@fullcalendar/react';
import timeGridPlugin from '@fullcalendar/timegrid';

export const Route = createFileRoute('/')({
  component: HomePage
});

function HomePage() {
  return (
    <FullCalendar plugins={[timeGridPlugin]} initialView='timeGridDay' headerToolbar={{ left: 'title' }} />
  );
}
