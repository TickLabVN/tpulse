import { PageLayout } from '@primer/react';

export function Page() {
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
