import { useRouteData } from "@solidjs/router";
import { createResource, createSignal, For, Match, Show, Switch } from "solid-js";
import { AscendingIcon, DescendingIcon } from "../../icons/Sorting";
import { getAllUsers } from "../../api/admin";

/**
 * @file This page displays and manages all users in the system. It is only accessible to administrators.
 */
export default function Users() {
    const [users] = createResource(getAllUsers);

    return (
        <>
            <h1 class="text-lg">Users</h1>

            <Show when={users.loading}>
                <div class="skeleton"></div>
            </Show>

            <Switch>
                <Match when={users.error}>
                    <span>Error: {users.error}</span>
                </Match>
                <Match when={users()}>
                    <For each={users()!!}>
                        {(user) => (
                            <div class="card shadow-md">
                                <div class="card-body">
                                    <div class="grid grid-cols-2">
                                        <span>User ID</span>
                                        <span>{user.id}</span>
                                        <span>First Names</span>
                                        <span>{user.first_names}</span>
                                        <span>Last Name</span>
                                        <span>{user.last_name}</span>
                                        <span>2FA Enabled</span>
                                        <span>{user.totp_enabled ? "Yes" : "No"}</span>
                                        <span>User Role</span>
                                        <span>{user.user_role}</span>

                                        <span>Add/remove ECs</span>
                                        <div>
                                            <input></input>
                                            <button class="btn">Todo</button>
                                        </div>
                                    </div>

                                </div>
                            </div>
                        )}
                    </For>
                </Match>
            </Switch>
        </>
    );
}
