import { useNavigate } from "@solidjs/router";
import { createResource, createSignal, For, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { getCourses } from "../api/courses";
import { upload, uploadFile } from "../api/uploads";

export default function Upload() {
  const navigate = useNavigate();
  const [courses] = createResource(() => getCourses());
  const [file, setFile] = createSignal<File>();
  const [form, setForm] = createStore({
    name: "",
    description: "",
    price: 0,
    belongs_to: "",
  });
  const [error, setError] = createSignal<string>();
  const [loading, setLoading] = createSignal(false);

  // function onDrop(event: DragEvent) {
  //   event.preventDefault();

  //   if (event.dataTransfer?.items) {
  //     for (const item of event.dataTransfer.items) {
  //       const file = item.getAsFile();
  //       if (!file) continue;
  //       console.log(`… file.name = ${file.name}`);
  //     }
  //   } else if (event.dataTransfer?.files) {
  //     for (const file of event.dataTransfer.files) {
  //       console.log(`… file.name = ${file.name}`);
  //     }
  //   }
  // }

  async function submit(event: SubmitEvent) {
    event.preventDefault();
    setLoading(true);
    try {
      const result = await upload(form);
      await uploadFile(result.id, file()!);
      navigate(`/courses/${encodeURIComponent(form.belongs_to)}`);
    } catch (error) {
      if (error instanceof Error) setError(error.message);
    }
    setLoading(false);
  }

  return (
    <form onSubmit={submit} class="flex flex-col items-center">
      <Show when={error()}>
        <div class="w-full max-w-md">
          <span class="text-error">{error()}</span>
          <div class="divider" />
        </div>
      </Show>

      {/* <label
        for="dropzone-file"
        class="flex flex-col w-full max-w-md items-center justify-center h-64 p-6 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer transition bg-base-200 hover:bg-base-300"
        onDrop={onDrop}
        onDragOver={(event) => event.preventDefault()}
      >
        <div class="flex flex-col items-center justify-center">
          <UploadIcon />
          <p class="mt-4 mb-2 text-sm">
            <span class="font-semibold">Klicken zum Hochladen</span> oder per Drag & Drop
          </p>
          <p class="text-xs text-gray-500 dark:text-gray-400">
            BMP/JPG, GIF, DOC, XLS, PPT, ZIP, RAR, TXT, PDF, TEX oder ODT
          </p>
        </div>
        <input id="dropzone-file" type="file" class="hidden" />
      </label> */}

      <div class="form-control w-full max-w-md">
        <label class="label" for="upload-file">
          <span class="label-text">Datei</span>
        </label>
        <input
          id="upload-file"
          type="file"
          required
          onInput={(event) => setFile(event.currentTarget.files?.[0])}
          class="file-input file-input-bordered w-full"
        />

        <label class="label" for="upload-course">
          <span class="label-text">Kurs</span>
        </label>
        <select
          id="upload-course"
          required
          onInput={(e) => setForm("belongs_to", e.currentTarget.value)}
          class="select select-bordered w-full"
        >
          <option value="" disabled selected hidden>
            Kurs auswählen
          </option>
          <For each={courses()}>{(course) => <option value={course.id}>{course.name}</option>}</For>
        </select>

        <label class="label" for="upload-description">
          <span class="label-text">Beschreibung</span>
        </label>
        <textarea
          id="upload-description"
          rows={3}
          onInput={(e) => setForm("description", e.currentTarget.value)}
          class="textarea textarea-bordered"
        />

        <label class="label" for="upload-category">
          <span class="label-text">Kategorie</span>
        </label>
        <select
          id="upload-category"
          required
          // onInput={(e) => setForm("category", e.currentTarget.value)}
          class="select select-bordered w-full"
        >
          <option>Kein Typ</option>
          <option>Klausurangabe</option>
          <option>Prüfungsfragenausarbeitung</option>
          <option>Stoffzusammenfassung</option>
          <option>Hausübung</option>
          <option>Mitschrift</option>
          <option>Fragensammlung</option>
          <option>Protokoll</option>
          <option>Sonstiges</option>
          <option>Skriptum</option>
          <option>Präsentation</option>
        </select>

        <label class="label" for="upload-date">
          <span class="label-text">Datum</span>
        </label>
        <input
          id="upload-date"
          type="date"
          // onInput={(e) => setForm("date", e.currentTarget.value)}
          class="input input-bordered w-full"
        />

        <label class="label" for="upload-price">
          <span class="label-text">Preis</span>
        </label>
        <div class="join">
          <input
            id="upload-price"
            type="number"
            required
            min={0}
            max={50}
            value={0}
            pattern="[0-9]+"
            onInput={(e) => setForm("price", parseInt(e.currentTarget.value))}
            class="input input-bordered join-item w-full"
          />
          <div class="join-item flex items-center px-4 bg-base-200 cursor-default">EC</div>
        </div>

        <button type="submit" class="btn btn-primary mt-8" disabled={loading()}>
          Hochladen
        </button>
      </div>
    </form>
  );
}
