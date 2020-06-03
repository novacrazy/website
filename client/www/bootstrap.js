import './styles/main.scss';

import("../pkg")
  .catch(e => console.error("Error importing `index.js`:", e))
  .then((app) => app.run_app());