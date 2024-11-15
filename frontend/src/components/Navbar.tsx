import { Link } from "@solidjs/router";
import { Show, createResource, Switch, Match } from "solid-js";
import { useAuthContext } from "../context/AuthContext";
import UploadIcon from "../icons/UploadIcon";
import SearchBar from "./SearchBar";
import Sidebar from "./Sidebar";
import { AuthLevel, getMyEcsBalance } from "../api/users";

export default function Navbar() {
  const { user, logout, setLoginModal } = useAuthContext();
  const [myEcsBalance] = createResource(getMyEcsBalance);

  return (
    <div class="drawer">
      <input id="navbar-drawer" type="checkbox" class="drawer-toggle" />
      <div class="drawer-content flex flex-col">
        <div class="navbar border-b-2 border-base-300">
          <div class="justify-start md:w-1/2">
            <label for="navbar-drawer" class="btn btn-square btn-ghost md:hidden">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                class="inline-block w-6 h-6 stroke-current"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M4 6h16M4 12h16M4 18h16"
                ></path>
              </svg>
            </label>
            <Link href="/" class="btn btn-ghost text-xl hidden md:flex" aria-label="Startseite">
              Egiraffe
            </Link>
          </div>

          <div class="basis-3/4 grow justify-center">
            <SearchBar />
          </div>

          <div class="navbar-end hidden md:inline-flex">
            <div class="navbar-end hidden md:inline-flex">
              <Link href="/upload" class="btn btn-outline">
                <UploadIcon />
                Hochladen
              </Link>
            </div>

            <div class="navbar-end hidden md:inline-flex">
              <Link href="/debug">
                <button type="button" class="btn btn-ghost btn-sm rounded-btn">
                  Debug menu
                </button>
              </Link>
            </div>

            <div class="flex-none gap-2">
              <div class="dropdown dropdown-end">
                <Show
                  when={user()}
                  fallback={
                    <button onClick={() => setLoginModal(true)} class="btn btn-outline">
                      Anmelden
                    </button>
                  }
                >
                  <label tabIndex={0} class="btn btn-ghost btn-circle avatar">
                    <div class="w-10 rounded-full">
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
                  <ul
                    tabIndex={0}
                    class="mt-3 z-[1] p-2 shadow menu menu-sm dropdown-content bg-base-100 rounded-box w-52"
                  >
                    <li>
                      <span class="font-semibold">
                        {user()!.first_names} {user()!.last_name}
                      </span>
                      <br />
                      <span class="text-xs text-base-content">
                        <Switch>
                          <Match when={myEcsBalance.loading}>
                            ...
                          </Match>
                          <Match when={myEcsBalance.error}>
                            Error getting ECs balance
                          </Match>
                          <Match when={typeof myEcsBalance() === "number"}>
                            {myEcsBalance()} ECs
                          </Match>
                        </Switch>
                      </span>
                    </li>
                    <div class="divider my-0" />
                    <li>
                      <Link href="/settings">Einstellungen</Link>
                    </li>
                    <Show when={user()!.user_role >= AuthLevel.MODERATOR}>
                      <li>
                        <Link href="/moderation">Moderation</Link>
                      </li>
                    </Show>
                    <Show when={user()!.user_role >= AuthLevel.ADMIN}>
                      <li>
                        <Link href="/admin">Admin</Link>
                      </li>
                    </Show>
                    <div class="divider my-0" />
                    <li>
                      <a onClick={logout}>Abmelden</a>
                    </li>
                  </ul>
                </Show>
              </div>
            </div>
          </div>
        </div>
      </div>
      <Sidebar />
    </div>
  );
}
