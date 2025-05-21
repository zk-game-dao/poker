import ReactDOM from "react-dom/client";

import { App } from "../../libraries/frontend/apps/zk-poker";

// eslint-disable-next-line no-undef
const root = document.getElementById("root") || document.createElement("div");

ReactDOM.createRoot(root).render(<App />);
