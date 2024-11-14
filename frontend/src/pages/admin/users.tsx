import { useRouteData } from "@solidjs/router";
import { createResource, createSignal, For, Match, Show, Switch } from "solid-js";
import { AscendingIcon, DescendingIcon } from "../../icons/Sorting";
import { createSystemTransaction, getAllUsers } from "../../api/admin";
import { authLevelToString, RedactedUser } from "../../api/users";

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
                        {user => <UserCard user={user} />}
                    </For>
                </Match>
            </Switch>
        </>
    );
}

function UserCard({ user }: { user: RedactedUser }) {
    const [ecInput, setEcInput] = createSignal("");
    const [reasonInput, setReasonInput] = createSignal("");

    const ecInputId = `ecInput-${user.id}`;
    const reasonInputId = `reasonInput-${user.id}`;

    const handleCreateTransaction = async () => {
        try {
            await createSystemTransaction({
                user_id: user.id,
                delta_ec: parseInt(ecInput(), 10),
                reason: reasonInput(),
            });
            alert('Transaction created successfully');
        } catch (error: any) {
            alert(`Failed to create transaction: ${error.message}`);
        }
    };

    return (
        <div class="card shadow-md">
            <div class="card-body">
                <div class="grid grid-cols-[auto,auto]">
                    <span>User ID</span>
                    <span>{user.id}</span>
                    <span>First Names</span>
                    <span>{user.first_names}</span>
                    <span>Last Name</span>
                    <span>{user.last_name}</span>
                    <span>2FA Enabled</span>
                    <span>{user.totp_enabled ? "Yes" : "No"}</span>
                    <span>User Role</span>
                    <span>{authLevelToString(user.user_role)} (auth level {user.user_role})</span>

                    <span>Add/remove ECs</span>
                    <div class="flex flex-row gap-2 items-baseline">
                        <div class="form-control">
                            <label class="label" for={ecInputId}>
                                <span class="label-text">EC Input</span>
                            </label>
                            <input class="input input-bordered" type="number" id={ecInputId} value={ecInput()} onInput={(e) => setEcInput(e.currentTarget.value)} />
                        </div>
                        <div class="form-control">
                            <label class="label" for={reasonInputId}>
                                <span class="label-text">Reason</span>
                            </label>
                            <input class="input input-bordered" type="text" id={reasonInputId} value={reasonInput()} onInput={(e) => setReasonInput(e.currentTarget.value)} />
                        </div>
                        <button class="btn" onClick={handleCreateTransaction}>Create system transaction</button>
                    </div>
                </div>

            </div>
        </div>
    );
}
