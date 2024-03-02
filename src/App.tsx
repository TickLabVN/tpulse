// import { useState } from 'react';
import { DayView } from './DayView';
import { ThemeProvider, BaseStyles } from '@primer/react';
import './App.css';

function App() {
  return (
    <ThemeProvider>
      <BaseStyles>
        <DayView />
      </BaseStyles>
    </ThemeProvider>
  );
}

export default App;
