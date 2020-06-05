import './styles/main.scss';

import("../bin/app/pkg")
  .catch(e => console.error("Error importing `index.js`:", e))
  .then((app) => app.run_app());