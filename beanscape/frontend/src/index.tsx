import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
import {Auth0Provider} from '@auth0/auth0-react';
import { Provider } from 'react-redux';
import { store } from './store';

ReactDOM.render(
  <Auth0Provider
    domain={process.env.REACT_APP_AUTH0_DOMAIN ?? ''}
    clientId={process.env.REACT_APP_AUTH0_CLIENT_ID ?? ''}
    redirectUri={`${window.location.origin}?redirect=${window.location.pathname}`}
    audience={process.env.REACT_APP_AUTH0_AUDIENCE ?? ''}
    scope="write:beans"
  >
    <React.StrictMode>
      <Provider store={store}>
        <App />
      </Provider>
    </React.StrictMode>
  </Auth0Provider>,
  document.getElementById('root')
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
