import firebase from "firebase/app";
import "firebase/auth";
import "firebase/firestore";
import "firebase/functions";
import config from './config';


const firebaseInit = () => {
  firebase.initializeApp(config.firebase);
  firebase.firestore();
  firebase.functions();
};

export default firebaseInit;
