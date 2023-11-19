import { Link, useRouteData } from '@solidjs/router';
import { For, Suspense } from 'solid-js';
import { Course } from '../api/courses';
import { CoursesDataType } from './courses.data';

export default function Courses() {
  const courses = useRouteData<CoursesDataType>();
  const courseURL = (course: Course) =>
    `/courses/${encodeURIComponent(course.id)}`;

  return (
    <Suspense fallback={<span>...</span>}>
      <div class="flex flex-col gap-6 items-center">
        <For each={courses()}>
          {course => (
            <Link
              class="rounded-lg bg-base-200 shadow-xl p-4 w-full max-w-2xl"
              href={courseURL(course)}
            >
              <h2 class="text-lg font-bold">{course.name}</h2>
              <p>{course.held_at}</p>
            </Link>
          )}
        </For>
      </div>
    </Suspense>
  );
}
