import React from 'react';
import { BrowserRouter as Router, Route } from 'react-router-dom';
import Home from './components/Home';
import Login from './components/Login';
import Dashboard from './components/Dashboard';


const Routes = () => (
  <Router>
    <Route exact path="/" component={Home} />
    <Route path="/login" component={Login} />
    <Route path="/app" component={Dashboard} />
  </Router>
)

export default Routes;
