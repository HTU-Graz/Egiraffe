import { Link, useNavigate, useSearchParams } from "@solidjs/router";
import { For, Show, createResource, createSignal } from "solid-js";
import { Course, getCourses } from "../api/courses";
import { debounce } from "../utils/debounce";

export default function SearchBar() {
  const [searchParams] = useSearchParams<{ search: string }>();
  const navigate = useNavigate();
  const [query, setQuery] = createSignal(searchParams.search || "");
  const [courses] = createResource(query, debounce(getCourses, 100), {
    initialValue: [],
  });
  let searchInput: HTMLInputElement | undefined;

  const courseURL = (course: Course) => `/courses/${encodeURIComponent(course.id)}`;

  function submit(event: SubmitEvent) {
    event.preventDefault();
    searchInput?.blur();
    navigate(`/courses?search=${encodeURIComponent(query())}`);
  }

  return (
    <form onSubmit={submit} class="w-full max-w-lg">
      <div class="join w-full rounded-full">
        <div class="w-full relative group">
          <input
            type="search"
            placeholder="Suche nach Kursen…"
            required
            class="input input-bordered join-item w-full group"
            ref={searchInput}
            value={query()}
            onInput={(e) => setQuery(e.currentTarget.value)}
          />
          <Show when={query().length > 0 && courses().length > 0}>
            <ul class="menu absolute z-10 mt-2 rounded-box w-full bg-base-200 shadow-md hidden group-focus-within:block">
              <For each={courses()}>
                {(course) => (
                  <li>
                    <Link href={courseURL(course)} onClick={(e) => e.currentTarget.blur()}>
                      {course.name}
                    </Link>
                  </li>
                )}
              </For>
            </ul>
          </Show>
        </div>
        <button class="btn join-item">
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
