import { createSignal } from 'solid-js';

export default function SearchBar() {
  const [query, setQuery] = createSignal('');

  function submit(event: SubmitEvent) {
    event.preventDefault();
    alert(`Suche nach ${query()}`);
  }

  return (
    <form onSubmit={submit} class="w-full max-w-lg">
      <div class="join w-full rounded-full">
        <input
          type="search"
          placeholder="Suche nach Kursen…"
          required
          class="input input-bordered join-item w-full"
          onInput={e => setQuery(e.currentTarget.value)}
        />
        <button type="submit" class="btn join-item">
          <svg
            width="28"
            height="28"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            fill="none"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path stroke="none" d="M0 0h24v24H0z" fill="none" />
            <path d="M10 10m-7 0a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" />
            <path d="M21 21l-6 -6" />
          </svg>
        </button>
      </div>
    </form>
  );
}
