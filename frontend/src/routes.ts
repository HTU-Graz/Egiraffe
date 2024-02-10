import type { RouteDefinition } from "@solidjs/router";
import { lazy } from "solid-js";

import AboutData from "./pages/about.data";
import CoursesData from "./pages/courses.data";
import Home from "./pages/home";
import UploadsData from "./pages/uploads.data";

export const routes: RouteDefinition[] = [
  {
    path: "/",
    component: Home,
  },
  {
    path: "/register",
    component: lazy(() => import("./pages/register")),
  },
  {
    path: "/courses",
    component: lazy(() => import("./pages/courses")),
    data: CoursesData,
  },
  {
    path: "/courses/:id",
    component: lazy(() => import("./pages/uploads")),
    data: UploadsData,
  },
  {
    path: "/create-course",
    component: lazy(() => import("./pages/create-course")),
  },
  {
    path: "/upload",
    component: lazy(() => import("./pages/create-upload")),
  },
  {
    path: "/about",
    component: lazy(() => import("./pages/about")),
    data: AboutData,
  },
  {
    path: "/moderation",
    component: lazy(() => import("./pages/moderation")),
  },
  {
    path: "/debug",
    component: lazy(() => import("./pages/debug")),
  },
  {
    path: "**",
    component: lazy(() => import("./errors/404")),
  },
];
