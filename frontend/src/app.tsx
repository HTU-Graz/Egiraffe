import { useRoutes } from "@solidjs/router";
import { Show, type Component } from "solid-js";
import Navbar from "./components/Navbar";
import { routes } from "./routes";
import { useAuthContext } from "./context/AuthContext";
import LoginModal from "./components/LoginModal";
import Breadcrumbs from "./components/Breadcrumbs";

const App: Component = () => {
  const Route = useRoutes(routes);
  const { loginModal, setLoginModal } = useAuthContext();

  return (
    <>
      <Navbar />

      <Breadcrumbs />

      <Show when={loginModal()}>
        <LoginModal onClose={() => setLoginModal(false)} />
      </Show>

      <main class="p-6">
        <Route />
      </main>
    </>
  );
};

export default App;
