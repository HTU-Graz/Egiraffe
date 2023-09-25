import { useLocation } from '@solidjs/router';
import { createStore } from 'solid-js/store';
import { useAuthContext } from '../context/AuthContext';

export default function Register() {
  const { register } = useAuthContext();
  const location = useLocation<{ email: string; password: string }>();
  const [form, setForm] = createStore({
    first_names: '',
    last_name: '',
    email: location.state?.email ?? '',
    password: location.state?.password ?? '',
  });

  async function submit(event: SubmitEvent) {
    event.preventDefault();
    await register(form);
  }

  return (
    <form onSubmit={submit} class="flex flex-col items-center">
      <h3 class="font-bold text-lg text-center mb-6">
        Bei Egiraffe Registrieren
      </h3>

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
          value={form.email}
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
          minlength={8}
          required
          value={form.password}
          onInput={e => setForm('password', e.currentTarget.value)}
          class="input input-bordered w-full max-w-xs"
        />

        <button type="submit" class="btn btn-primary mt-8">
          Registrieren
        </button>
      </div>
    </form>
  );
}
