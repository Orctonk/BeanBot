import React from 'react';
import { HomeScreen } from './screens/HomeScreen';
import { BeanScreen } from './screens/BeanScreen';
import { NotFound } from './screens/NotFound';
import { PageHeader } from './containers/PageHeader';
import { MessageLog } from './containers/MessageLog';
import { BrowserRouter, Route, Switch } from 'react-router-dom';
import './App.css';

function App() {
  return (
    <BrowserRouter>
      <div>
        <PageHeader />
        <MessageLog />
        <Switch>
          <Route exact path='/'>
            <HomeScreen />
          </Route>
          <Route path='/beans'>
            <BeanScreen />
          </Route>
          <Route>
            <NotFound />
          </Route>
        </Switch>
      </div>
    </BrowserRouter>
    );
}

export default App;
