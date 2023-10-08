import { Show, createSignal } from 'solid-js';
import { createStore } from 'solid-js/store';
import { useAuthContext } from '../context/AuthContext';

export default function Register() {
  const { register } = useAuthContext();
  const [form, setForm] = createStore({
    first_names: '',
    last_name: '',
    email: '',
    password: '',
  });
  const [error, setError] = createSignal<string | undefined>();
  const [loading, setLoading] = createSignal(false);

  async function submit(event: SubmitEvent) {
    event.preventDefault();
    setLoading(true);
    try {
      await register(form);
    } catch (error) {
      if (error instanceof Error) setError(error.message);
    }
    setLoading(false);
  }

  return (
    <form onSubmit={submit} class="flex flex-col items-center">
      <h3 class="font-bold text-lg text-center mb-6">
        Bei Egiraffe Registrieren
      </h3>

      <Show when={error()}>
        <div class="w-full max-w-xs">
          <span class="text-error">{error()}</span>
          <div class="divider" />
        </div>
      </Show>

      <div class="form-control w-full max-w-xs">
        <label for="register-first_names" class="label">
          <span class="label-text">Vorname</span>
        </label>
        <input
          type="text"
          id="register-first_names"
          autocomplete="given-name"
          autofocus
          required
          disabled={loading()}
          onInput={e => setForm('first_names', e.currentTarget.value)}
          class="input input-bordered w-full max-w-xs"
        />

        <label for="register-last_name" class="label">
          <span class="label-text">Nachname</span>
        </label>
        <input
          type="text"
          id="register-last_name"
          autocomplete="family-name"
          required
          disabled={loading()}
          onInput={e => setForm('last_name', e.currentTarget.value)}
          class="input input-bordered w-full max-w-xs"
        />

        <label for="register-email" class="label">
          <span class="label-text">Email</span>
        </label>
        <input
          type="email"
          id="register-email"
          autocomplete="email"
          required
          disabled={loading()}
          onInput={e => setForm('email', e.currentTarget.value)}
          class="input input-bordered w-full max-w-xs"
        />

        <label for="register-password" class="label">
          <span class="label-text">Passwort</span>
        </label>
        <input
          type="password"
          id="register-password"
          autocomplete="new-password"
          minlength="8"
          required
          disabled={loading()}
          onInput={e => setForm('password', e.currentTarget.value)}
          class="input input-bordered w-full max-w-xs"
        />

        <button type="submit" class="btn btn-primary mt-8" disabled={loading()}>
          Registrieren
        </button>
      </div>
    </form>
  );
}
