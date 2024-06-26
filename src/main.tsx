import { AdapterMoment } from '@mui/x-date-pickers/AdapterMoment';
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { RouterProvider, createRouter } from '@tanstack/react-router';
import { attachConsole } from '@tauri-apps/plugin-log';
import React from 'react';
import 'react-circular-progressbar/dist/styles.css';
import ReactDOM from 'react-dom/client';
import { ToastContainer } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

import './index.css';
import { routeTree } from './routeTree.gen';
import { log } from './utils/log';

const router = createRouter({
  routeTree,
  defaultPreload: 'intent'
});
const queryClient = new QueryClient();

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router;
  }
}

// with TargetKind::Webview enabled this function will print logs to the browser console
attachConsole().then(() => log.info('attached console'));

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <LocalizationProvider dateAdapter={AdapterMoment}>
      <QueryClientProvider client={queryClient}>
        <ToastContainer limit={1} />
        <RouterProvider router={router} />
      </QueryClientProvider>
    </LocalizationProvider>
  </React.StrictMode>
);
