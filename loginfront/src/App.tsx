import { type Component } from 'solid-js';

import styles from './App.module.css';
import Login from './components/Login';

const App: Component = () => {
  return (
    <div class={styles.App}>
      <Login />
    </div>
  );
};

export default App;
