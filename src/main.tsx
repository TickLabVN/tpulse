import { AdapterMoment } from '@mui/x-date-pickers/AdapterMoment';
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { RouterProvider, createRouter } from '@tanstack/react-router';
import { attachConsole } from '@tauri-apps/plugin-log';
import 'react-circular-progressbar/dist/styles.css';
import ReactDOM from 'react-dom/client';
import './index.css';
import { StrictMode } from 'react';
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

// biome-ignore lint/style/noNonNullAssertion: <explanation>
ReactDOM.createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <LocalizationProvider dateAdapter={AdapterMoment}>
      <QueryClientProvider client={queryClient}>
        <RouterProvider router={router} />
      </QueryClientProvider>
    </LocalizationProvider>
  </StrictMode>
);
