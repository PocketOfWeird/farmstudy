import React from 'react';
import { Provider } from 'react-redux';
import Routes from './Routes';
import firebaseInit from './firebaseInit';
import createStore from './store';

firebaseInit();
const store = createStore();

const App = () => (
  <Provider store={store}>
    <Routes />
  </Provider>
)

export default App;
