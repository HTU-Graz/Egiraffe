/* @refresh reload */
import "./index.css";

import { Router } from "@solidjs/router";
import "solid-devtools";
import { render } from "solid-js/web";
import App from "./app";
import { AuthContextProvider } from "./context/AuthContext";

render(
  () => (
    <AuthContextProvider>
      <Router>
        <App />
      </Router>
    </AuthContextProvider>
  ),
  document.getElementById("root")!,
);
