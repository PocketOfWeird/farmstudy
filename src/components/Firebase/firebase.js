import { firebaseConfig } from './secrets';

class Firebase {
  constructor(app) {
    app.initializeApp(firebaseConfig);

    /* Helper */

    this.emailAuthProvider = app.auth.EmailAuthProvider;

    /* Firebase APIs */

    this.auth = app.auth();
    this.db = app.firestore();

    app.firestore().enablePersistence()
      .catch(function(err) {
          if (err.code === 'failed-precondition') {
              // Multiple tabs open, persistence can only be enabled
              // in one tab at a a time.
              // ...
          } else if (err.code === 'unimplemented') {
              // The current browser does not support all of the
              // features required to enable persistence
              // ...
          }
      });

    /* Social Sign In Method Provider */

    this.googleProvider = new app.auth.GoogleAuthProvider();
    this.facebookProvider = new app.auth.FacebookAuthProvider();
    this.twitterProvider = new app.auth.TwitterAuthProvider();
  }

  // *** Auth API ***

  doCreateUserWithEmailAndPassword = (email, password) =>
    this.auth.createUserWithEmailAndPassword(email, password);

  doSignInWithEmailAndPassword = (email, password) =>
    this.auth.signInWithEmailAndPassword(email, password);

  doSignInWithGoogle = () =>
    this.auth.signInWithPopup(this.googleProvider);

  doSignInWithFacebook = () =>
    this.auth.signInWithPopup(this.facebookProvider);

  doSignInWithTwitter = () =>
    this.auth.signInWithPopup(this.twitterProvider);

  doSignOut = () => this.auth.signOut();

  doPasswordReset = email => this.auth.sendPasswordResetEmail(email);

  doSendEmailVerification = () =>
    this.auth.currentUser.sendEmailVerification({
      url: process.env.GATSBY_CONFIRMATION_EMAIL_REDIRECT,
    });

  doPasswordUpdate = password =>
    this.auth.currentUser.updatePassword(password);

  // *** Merge Auth and DB User API *** //

  onAuthUserListener = (next, fallback) =>
    this.auth.onAuthStateChanged(authUser => {
      if (authUser) {
        this.user(authUser.uid).get().then(function(user) {
            if (user.exists) {
                let dbUser = user.data();

                // default empty roles
                if (!dbUser.roles) {
                  dbUser.roles = {};
                }

                // merge auth and db user
                authUser = {
                  uid: authUser.uid,
                  email: authUser.email,
                  emailVerified: authUser.emailVerified,
                  providerData: authUser.providerData,
                  ...dbUser,
                };

                next(authUser);
            } else {
                this.db.collection("users").doc(authUser.uid).set({
                    uid: authUser.uid,
                    email: authUser.email,
                    emailVerified: authUser.emailVerified,
                    providerData: authUser.providerData,
                })
            }
        }).catch(function(error) {
                console.log("Error getting user:", error);
        });
      } else {
        fallback();
      }
    });

  // *** User API ***

  user = uid => this.db.collection('users').doc(uid);

  users = () => this.db.collection('users');

  // *** Message API ***

  message = uid => this.db.collection('messages').doc(uid);

  messages = () => this.db.collection('messages');
}

let firebase;

function getFirebase(app, auth, database) {
  if (!firebase) {
    firebase = new Firebase(app, auth, database);
  }

  return firebase;
}

export default getFirebase;
