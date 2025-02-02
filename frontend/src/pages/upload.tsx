import { useParams, useRouteData } from "@solidjs/router";
import { For, Match, Show, Suspense, Switch, createResource, createSignal } from "solid-js";
import UploadCard from "../components/UploadCard";
import { getFiles } from "../api/files";
import { purchaseUpload } from "../api/uploads";
import { format_date } from "../utils/format";

export default function Upload() {
  const { id } = useParams();
  const [upload_and_files] = createResource(() => id, getFiles);

  const upload = () => upload_and_files()?.upload;
  const files = () => upload_and_files()?.files;
  const uploader_name = () => upload_and_files()?.uploader_name;
  const total_files_count = () => upload_and_files()?.total_files_count;

  const most_recent_available_file = () => {
    const _files = files();

    if (_files == null || _files[0] == null) return null;

    let best_so_far = null;

    for (const file of _files) {
      // Only look at approved files
      if (file.approval_mod && file.approval_uploader) {
        if (best_so_far == null || file.revision_at > best_so_far.revision_at) {
          best_so_far = file;
        }
      }
    }

    return best_so_far;
  }

  return (
    <>
      <Suspense fallback={
        <div class="flex w-52 flex-col gap-4">
          <div class="skeleton h-32 w-full"></div>
          <div class="skeleton h-4 w-28"></div>
          <div class="skeleton h-4 w-full"></div>
          <div class="skeleton h-4 w-full"></div>
        </div>
      }>
        <h1 class="text-2xl font-bold">
          Upload-type: {upload()?.name}
        </h1>

        <div class="grid grid-cols-2 gap-4 mt-4">
          <span>Preis</span>
          <span>{upload()?.price}</span>

          <span>Hochladende Person</span>
          <span>{`${uploader_name() ?? ""} (${upload()?.uploader})`}</span>

          <span>Upload-Datum</span>
          <span>{format_date(upload()?.upload_date)}</span>

          <span>Letzte Änderung</span>
          <span>{format_date(upload()?.last_modified_date)}</span>

          <Show when={upload()?.held_by != null}>
            <span>Von Prof</span>
            <span>{upload()?.held_by}</span>
          </Show>
        </div>

        <span>Beschreibung</span>
        <p>{upload()?.description}</p>

        <h2 class="text-xl font-bold mt-8">Dateien</h2>
        <p>Gesamt {total_files_count()} Dateien</p>

        <Show when={most_recent_available_file() != null} fallback={
          <p>
            Es ist aktuell keine Revision verfügbar. <br />
            Das kann daran liegen, dass die Person, die das hochgeladen hat, die Datei zurückgezogen hat, oder,
            weil das Moderations-Team die Datei noch nicht freigegeben hat, oder diese sperren musste.
          </p>
        }>
          <div class="grid grid-cols-2 gap-4 mt-4">
            <span>Dateiname</span>
            <span>{most_recent_available_file()?.name}</span>

            <span>Datentyp (MIME type)</span>
            <span>{most_recent_available_file()?.mime_type}</span>

            <span>Dateigröße</span>
            {/* TODO format this in human-readable units */}
            <span>{most_recent_available_file()?.size + " byte"}</span>

            <span>Letzte Änderung</span>
            <span>{most_recent_available_file()?.revision_at}</span>
          </div>

          <button class="btn btn-primary" onClick={_ => purchaseUpload({ upload_id: id })}>
            <Switch>
              <Match when={upload()?.price === 0}>
                Jetzt herunterladen (gratis)
              </Match>
              <Match when={upload()?.price !== 0}>
                Für {upload()?.price} ECs kaufen
              </Match>
            </Switch>
          </button>

        </Show>
      </Suspense >
    </>
  );
}
