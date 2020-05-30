import * as React from "react";
import * as ReactDOM from "react-dom";

import App from './App';

document.getElementById('load-error')?.remove();

ReactDOM.render(
    <App />,
    document.getElementById('root') as HTMLElement
);

