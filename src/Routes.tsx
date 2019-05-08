import React from 'react';
import { BrowserRouter as Router, Route } from 'react-router-dom';
import { withFirebase, isLoaded, isEmpty } from 'react-redux-firebase';
import { compose } from 'redux';
import { connect } from 'react-redux';
import Home from './components/Home';
import Loading from './components/Loading';
import Login from './components/Login';
import Signup from './components/Signup';
import Dashboard from './components/Dashboard';


const Routes = (props: any) => (
  <Router>
    <Route exact path="/" component={Home} />
    {
      !isLoaded(props.auth) ?
        <Loading />
      : isEmpty(props.auth) ?
        <React.Fragment>
          <Route path="/login" component={Login} />
          <Route path="/signup" component={Signup} />
        </React.Fragment>
      :
        <React.Fragment>
          <Route path="/app" component={Dashboard} />
        </React.Fragment>
    }
  </Router>
)

export default compose<any>(
  withFirebase,
  connect(({ firebase: { auth } }: any) => ({ auth }))
)(Routes);
