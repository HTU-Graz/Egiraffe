import { Link } from '@solidjs/router';
import { Show } from 'solid-js';
import { useAuthContext } from '../context/AuthContext';

export default function Sidebar() {
  const { user, logout, setLoginModal } = useAuthContext();

  return (
    <div class="drawer-side z-10">
      <label for="navbar-drawer" class="drawer-overlay" />
      <ul class="menu p-4 w-80 min-h-full bg-base-200">
        <Show
          when={user()}
          fallback={
            <button onClick={() => setLoginModal(true)} class="btn btn-outline">
              Anmelden
            </button>
          }
        >
          <label class="avatar">
            <div class="w-24 rounded-full">
              <svg
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                fill="none"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                <path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" />
                <path d="M12 10m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" />
                <path d="M6.168 18.849a4 4 0 0 1 3.832 -2.849h4a4 4 0 0 1 3.834 2.855" />
              </svg>
            </div>
          </label>
          <li>
            <p class="font-semibold">
              {user()!.first_names} {user()!.last_name}
            </p>
          </li>
          <div class="divider"></div>
          <li>
            <Link href="/settings">Einstellungen</Link>
          </li>
          <li>
            <a onClick={logout}>Abmelden</a>
          </li>
        </Show>
      </ul>
    </div>
  );
}
