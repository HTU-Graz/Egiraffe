import { Link, useRouteData } from "@solidjs/router";
import { For, Show, Suspense, createResource } from "solid-js";
import { Course } from "../api/courses";
import { getUniversities } from "../api/universities";
import { AuthLevel } from "../api/users";
import { useAuthContext } from "../context/AuthContext";
import { CoursesDataType } from "./courses.data";
import { getUploads } from "../api/uploads";
import { mod_getAllFiles, mod_getAllUploads } from "../api/moderate";
import UploadCard from "../components/UploadCard";
import Mod_FileCard from "../components/ModFileCard";

export default function Moderation() {
  const { hasRole } = useAuthContext();
  // const [uploads] = createResource(mod_getAllUploads);
  const [files] = createResource(mod_getAllFiles);

  return (
    <div class="flex flex-col gap-6 items-center">
      <Show when={hasRole(AuthLevel.MODERATOR)} fallback={
        <p>
          Not authorized
        </p>
      }>
        <Show
          when={(files()?.length ?? 0) > 0}
          fallback={
            <div class="card card-compact card-side bg-base-200 shadow-md">
              <div class="card-body">
                <div class="text-center">
                  {/* HACK this looks appalling, improve font/layout */}
                  <h2 class="text-3xl font-bold">Keine files gefunden</h2>
                </div>
              </div>
            </div>
          }>
          <For each={files()}>{(file) => <Mod_FileCard {...(file[0])} />}</For>
        </Show>
      </Show>
    </div>
  );
}
