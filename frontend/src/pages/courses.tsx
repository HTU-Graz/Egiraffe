import { Link, useRouteData } from "@solidjs/router";
import { For, Show, Suspense, createResource } from "solid-js";
import { Course } from "../api/courses";
import { getUniversities } from "../api/universities";
import { AuthLevel } from "../api/users";
import { useAuthContext } from "../context/AuthContext";
import { CoursesDataType } from "./courses.data";

export default function Courses() {
  const { hasRole } = useAuthContext();
  const courses = useRouteData<CoursesDataType>();
  const [universities] = createResource(getUniversities);
  const courseURL = (course: Course) => `/courses/${encodeURIComponent(course.id)}`;

  return (
    <div class="flex flex-col gap-6 items-center">
      <Show when={hasRole(AuthLevel.MODERATOR)}>
        <Link href="/create-course" class="btn btn-primary">
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
            <path d="M12 5l0 14" />
            <path d="M5 12l14 0" />
          </svg>
          Kurs erstellen
        </Link>
      </Show>
      <Suspense
        fallback={
          <For each={Array(5)}>
            {() => (
              <div class="rounded-lg bg-base-200 shadow-xl p-4 w-full max-w-2xl">
                <div class="skeleton h-4 w-80 max-w-full"></div>
                <div class="skeleton h-4 w-64 mt-4"></div>
              </div>
            )}
          </For>
        }
      >
        <Show when={courses()?.length === 0}>
          <p class="text-center">Keine Kurse gefunden…</p>
        </Show>
        <For each={courses()}>
          {(course) => (
            <Link
              class="rounded-lg bg-base-200 shadow-xl p-4 w-full max-w-2xl"
              href={courseURL(course)}
            >
              <h2 class="text-lg font-bold">{course.name}</h2>
              <p>
                {universities()?.find((u) => u.id === course.held_at)?.full_name ?? course.held_at}
              </p>
            </Link>
          )}
        </For>
      </Suspense>
    </div>
  );
}
