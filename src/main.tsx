import React from 'react';
import ReactDOM from 'react-dom/client';
import { ThemeProvider, BaseStyles } from '@primer/react';
import { RouterProvider, createRouter } from '@tanstack/react-router';
import { routeTree } from './routeTree.gen';
import './index.css';

const router = createRouter({
  routeTree,
  defaultPreload: 'intent'
});

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router;
  }
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <ThemeProvider>
      <BaseStyles>
        <RouterProvider router={router} />
      </BaseStyles>
    </ThemeProvider>
  </React.StrictMode>
);
