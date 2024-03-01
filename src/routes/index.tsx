import { createFileRoute } from '@tanstack/react-router';
import { PageLayout } from '@primer/react';

export const Route = createFileRoute('/')({
  component: HomePage
});

function HomePage() {
  return (
    <PageLayout>
      <PageLayout.Header>
        <p className='text-red-500'>Header</p>
      </PageLayout.Header>
      <PageLayout.Content>Content</PageLayout.Content>
      <PageLayout.Pane>Pane</PageLayout.Pane>
      <PageLayout.Footer>Footer</PageLayout.Footer>
    </PageLayout>
  );
}
