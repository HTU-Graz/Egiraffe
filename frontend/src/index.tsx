/* @refresh reload */
import './index.css';

import { Router } from '@solidjs/router';
import 'solid-devtools';
import { render } from 'solid-js/web';
import App from './app';

render(
  () => (
    <Router>
      <App />
    </Router>
  ),
  document.getElementById('root')!
);
