import { RouteDataFunc } from "@solidjs/router";
import { Resource, createResource } from "solid-js";
import { Course, getCourses } from "../api/courses";

const CoursesData: RouteDataFunc<unknown, Resource<Course[]>> = ({ location }) => {
  const [data] = createResource(() => location.query.search || "", getCourses);
  return data;
};

export default CoursesData;
export type CoursesDataType = typeof CoursesData;
