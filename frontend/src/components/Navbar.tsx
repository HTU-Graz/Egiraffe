import { Link } from '@solidjs/router';
import { createSignal, Show } from 'solid-js';
import { useAuthContext } from '../context/AuthContext';
import LoginModal from './LoginModal';

export default function Navbar() {
  const { user, logout } = useAuthContext();
  const [loginModal, setLoginModal] = createSignal(false);

  return (
    <div class="navbar bg-base-200">
      <Show when={loginModal()}>
        <LoginModal onClose={() => setLoginModal(false)} />
      </Show>

      <div class="flex-none">
        <button class="btn btn-square btn-ghost">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            class="inline-block w-5 h-5 stroke-current"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M4 6h16M4 12h16M4 18h16"
            ></path>
          </svg>
        </button>
      </div>

      <div class="flex-1">
        <Link href="/" class="btn btn-ghost normal-case text-xl">
          Egiraffe
        </Link>
      </div>
      <div class="flex-none gap-2">
        <div class="dropdown dropdown-end">
          <Show
            when={user()}
            fallback={
              <button
                onClick={() => setLoginModal(true)}
                class="btn btn-outline"
              >
                Anmelden
              </button>
            }
          >
            <label tabIndex={0} class="btn btn-ghost btn-circle avatar">
              <div class="w-10 p-1 bg-base-300 rounded-full">
                <svg viewBox="0 0 12 12">
                  <g fill="none">
                    <path
                      d="M6 1a2 2 0 1 0 0 4a2 2 0 0 0 0-4zM5 3a1 1 0 1 1 2 0a1 1 0 0 1-2 0zm3.5 3h-5A1.5 1.5 0 0 0 2 7.5c0 1.116.459 2.01 1.212 2.615C3.953 10.71 4.947 11 6 11c1.053 0 2.047-.29 2.788-.885C9.54 9.51 10 8.616 10 7.5A1.5 1.5 0 0 0 8.5 6zm-5 1h5a.5.5 0 0 1 .5.5c0 .817-.325 1.423-.838 1.835C7.636 9.757 6.88 10 6 10c-.88 0-1.636-.243-2.162-.665C3.325 8.923 3 8.317 3 7.5a.5.5 0 0 1 .5-.5z"
                      fill="currentColor"
                    ></path>
                  </g>
                </svg>
              </div>
            </label>
            <ul
              tabIndex={0}
              class="mt-3 z-[1] p-2 shadow menu menu-sm dropdown-content bg-base-100 rounded-box w-52"
            >
              <li>
                <span class="font-semibold">{user()!.email}</span>
              </li>
              <div class="divider my-0" />
              <li>
                <Link href="/settings">Einstellungen</Link>
              </li>
              <li>
                <a onClick={logout}>Abmelden</a>
              </li>
            </ul>
          </Show>
        </div>
      </div>
    </div>
  );
}
