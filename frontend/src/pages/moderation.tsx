import { Link, useRouteData } from "@solidjs/router";
import { For, Show, Suspense, createResource } from "solid-js";
import { Course } from "../api/courses";
import { getUniversities } from "../api/universities";
import { AuthLevel } from "../api/users";
import { useAuthContext } from "../context/AuthContext";
import { CoursesDataType } from "./courses.data";
import { getUploads } from "../api/uploads";

export default function Courses() {
  const { hasRole } = useAuthContext();
  const [uploads] = createResource(getUploads);

  return (
    <div class="flex flex-col gap-6 items-center">
      <Show when={hasRole(AuthLevel.MODERATOR)} fallback={
        <p>
          Not authorized
        </p>
      }>
        <Show
          when={(uploads()?.length ?? 0) > 0}
          fallback={
            <div class="card card-compact card-side bg-base-200 shadow-md">
              <div class="card-body">
                <div class="text-center">
                  {/* HACK this looks appalling, improve font/layout */}
                  <h2 class="text-3xl font-bold">Keine Uploads gefunden</h2>
                </div>
              </div>
            </div>
          }>
          <For each={uploads()}>{(upload) => <UploadCard {...upload} />}</For>
        </Show>
      </Show>
    </div>
  );
}
