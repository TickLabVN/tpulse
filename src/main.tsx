import React from 'react';
import ReactDOM from 'react-dom/client';
import { ThemeProvider, BaseStyles } from '@primer/react';
import { RouterProvider, createRouter } from '@tanstack/react-router';
import { ToastContainer } from 'react-toastify';
import { routeTree } from './routeTree.gen';
import 'react-toastify/dist/ReactToastify.css';
import './index.css';

const router = createRouter({
  routeTree,
  defaultPreload: 'intent'
});

declare module '@tanstack/react-router' {
  // eslint-disable-next-line no-unused-vars
  interface Register {
    router: typeof router;
  }
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <ThemeProvider>
      <BaseStyles>
        <ToastContainer limit={1} />
        <RouterProvider router={router} />
      </BaseStyles>
    </ThemeProvider>
  </React.StrictMode>
);
