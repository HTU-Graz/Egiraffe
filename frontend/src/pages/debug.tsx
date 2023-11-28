import { Link, useRouteData } from '@solidjs/router';
import { For, Suspense } from 'solid-js';
import { Course } from '../api/courses';
import { CoursesDataType } from './courses.data';

export default function Debug() {

  return (
    <Suspense fallback={<span>...</span>}>
      <div>
        Hello world
      </div>
    </Suspense>
  );
}
