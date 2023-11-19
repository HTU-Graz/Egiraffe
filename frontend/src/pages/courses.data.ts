import { RouteDataFunc } from '@solidjs/router';
import { Resource, createResource } from 'solid-js';
import { Course } from '../api/courses';

const CoursesData: RouteDataFunc<unknown, Resource<Course[]>> = () => {
  // TODO: use API
  // const [data] = createResource(getCourses);
  const [data] = createResource<Course[]>(async () => {
    const sleep = (ms: number) =>
      new Promise(resolve => setTimeout(resolve, ms));
    await sleep(500);
    return Array.from({ length: 20 }, (_, i) => ({
      id: `${i}`,
      name: `Course ${i}`,
      held_at: `${i % 3}`,
    }));
  });
  return data;
};

export default CoursesData;
export type CoursesDataType = typeof CoursesData;
