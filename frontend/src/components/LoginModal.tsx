import { Link } from '@solidjs/router';
import { createSignal, onMount, Show } from 'solid-js';
import { createStore } from 'solid-js/store';
import { useAuthContext } from '../context/AuthContext';

interface Props {
  onClose: () => void;
}

export default function LoginModal(props: Props) {
  const { login } = useAuthContext();
  const [form, setForm] = createStore({ email: '', password: '' });
  const [error, setError] = createSignal<string | undefined>();
  const [loading, setLoading] = createSignal(false);

  let dialog: HTMLDialogElement | undefined;

  onMount(() => {
    dialog!.showModal();
  });

  function close() {
    dialog!.close();
    props.onClose();
  }

  async function submit(event: SubmitEvent) {
    event.preventDefault();
    setLoading(true);
    try {
      await login(form);
      close();
    } catch (error) {
      if (error instanceof Error) setError(error.message);
    }
    setLoading(false);
  }

  return (
    <dialog
      ref={dialog}
      onClose={close}
      class="modal modal-top sm:modal-middle"
    >
      <div class="modal-box">
        <form method="dialog">
          <button
            onClick={close}
            class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
          >
            ✕
          </button>
        </form>

        <form onSubmit={submit} class="flex flex-col items-center">
          <h3 class="font-bold text-lg text-center mb-6">
            Bei Egiraffe Anmelden
          </h3>

          <Show when={error()}>
            <div class="w-full max-w-xs">
              <span class="text-error">{error()}</span>
              <div class="divider" />
            </div>
          </Show>

          <div class="form-control w-full max-w-xs">
            <label for="login-email" class="label">
              <span class="label-text">Email</span>
            </label>
            <input
              type="email"
              id="login-email"
              autocomplete="email"
              autofocus
              required
              disabled={loading()}
              onInput={e => setForm('email', e.currentTarget.value)}
              class="input input-bordered w-full max-w-xs"
            />

            <label for="login-password" class="label">
              <span class="label-text">Passwort</span>
            </label>
            <input
              type="password"
              id="login-password"
              autocomplete="current-password"
              required
              disabled={loading()}
              onInput={e => setForm('password', e.currentTarget.value)}
              class="input input-bordered w-full max-w-xs"
            />
            <label class="label">
              <Link
                href="/forgot-password"
                onClick={close}
                class="label-text-alt link"
              >
                Passwort vergessen?
              </Link>
            </label>
          </div>

          <div class="modal-action w-full max-w-xs justify-between">
            <button
              type="submit"
              class="btn btn-primary flex-grow"
              disabled={loading()}
            >
              Anmelden
            </button>
            <Link href="/register" onClick={close} class="btn">
              Account erstellen
            </Link>
          </div>
        </form>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button>close</button>
      </form>
    </dialog>
  );
}
