import { Link, useRouteData } from "@solidjs/router";
import { For, Show, Suspense } from "solid-js";
import { Course } from "../api/courses";
import { CoursesDataType } from "./courses.data";

export default function Courses() {
  const courses = useRouteData<CoursesDataType>();
  const courseURL = (course: Course) =>
    `/courses/${encodeURIComponent(course.id)}`;

  return (
    <div class="flex flex-col gap-6 items-center">
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
              <p>{course.held_at}</p>
            </Link>
          )}
        </For>
      </Suspense>
    </div>
  );
}
