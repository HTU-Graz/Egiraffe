import { useNavigate } from "@solidjs/router";
import { For, Show, createEffect, createResource, createSignal } from "solid-js";
import { createStore } from "solid-js/store";
import { createCourse } from "../api/courses";
import { getUniversities } from "../api/universities";

export default function CreateCourse() {
  const navigate = useNavigate();
  const [universities] = createResource(getUniversities);
  const [form, setForm] = createStore({
    name: "",
    held_at: "",
  });
  const [error, setError] = createSignal<string | undefined>();
  const [loading, setLoading] = createSignal(false);

  createEffect(() => {
    const firstUni = universities()?.[0];
    if (firstUni && form.held_at === "") setForm("held_at", firstUni.id);
  });

  async function submit(event: SubmitEvent) {
    event.preventDefault();
    setLoading(true);
    try {
      const course = await createCourse(form);
      navigate(`/courses/${encodeURIComponent(course.id)}`);
    } catch (error) {
      if (error instanceof Error) setError(error.message);
    }
    setLoading(false);
  }

  return (
    <form onSubmit={submit} class="flex flex-col items-center">
      <Show when={error()}>
        <div class="w-full max-w-xs">
          <span class="text-error">{error()}</span>
          <div class="divider" />
        </div>
      </Show>

      <div class="form-control w-full max-w-xs">
        <label for="create-course-name" class="label">
          <span class="label-text">Name</span>
        </label>
        <input
          type="text"
          id="create-course-name"
          name="name"
          autocomplete="off"
          autofocus
          required
          disabled={loading()}
          onInput={(e) => setForm("name", e.currentTarget.value)}
          class="input input-bordered w-full max-w-xs"
        />

        <label for="create-held-at" class="label">
          <span class="label-text">Universität</span>
        </label>
        <select
          id="create-held-at"
          name="held_at"
          required
          onInput={(e) => setForm("held_at", e.currentTarget.value)}
          class="select select-bordered w-full max-w-xs"
        >
          <For each={universities()}>
            {(uni) => <option value={uni.id}>{uni.full_name}</option>}
          </For>
        </select>

        <button type="submit" class="btn btn-primary mt-8" disabled={loading()}>
          Kurs erstellen
        </button>
      </div>
    </form>
  );
}
