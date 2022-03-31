import { log, request } from "@suborbital/runnable";

import React from 'react';
import ReactDOMServer from 'react-dom/server';

import App from '../../webapp/src/App.jsx';

export const run = (_) => {
  log.info("rendering page")

  const index = request.state("index")

  const app = ReactDOMServer.renderToString(React.createElement(App));

  const html = index.replace('<div id="root"></div>', `<div id="root">${app}</div>`)

  return html
};
