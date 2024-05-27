import * as React from 'react';
import * as ReactDOM from 'react-dom/client';

import 'react-loading-skeleton/dist/skeleton.css';

import App from './App';
import { BrowserRouter } from 'react-router-dom';

const root = ReactDOM.createRoot(document.getElementById('app') as HTMLElement);
root.render(
  <React.StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </React.StrictMode>,
);
