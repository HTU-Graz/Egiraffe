import { useRouteData } from "@solidjs/router";
import { createResource, createSignal, Match, Show, Switch } from "solid-js";
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
                    <div>{JSON.stringify(users())}</div>
                </Match>
            </Switch>
        </>
    );
}
