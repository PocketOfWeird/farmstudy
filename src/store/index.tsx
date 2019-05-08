import { compose, createStore } from "redux";
import firebase from "firebase";
import { reactReduxFirebase } from 'react-redux-firebase';
import { reduxFirestore } from 'redux-firestore';
import rootReducer from './reducers';
import { initialState } from './middleware';


// react-redux-firebase config
const rrfConfig: object = {
  userProfile: 'users',
  useFirestoreForProfile: true
};

const createStoreWithFirebase = compose<any>(
  reactReduxFirebase(firebase, rrfConfig),
  reduxFirestore(firebase)
)(createStore);

export default () => createStoreWithFirebase(rootReducer, initialState);
