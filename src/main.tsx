import React from 'react';
import ReactDOM from 'react-dom/client';
import { ThemeProvider } from '@primer/react';
import App from './App.tsx';
import './index.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <ThemeProvider>
      <App />
    </ThemeProvider>
  </React.StrictMode>
);
