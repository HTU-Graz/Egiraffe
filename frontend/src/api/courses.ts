import { ErrorResponse, put } from ".";

export type GetCoursesResponse =
  | ErrorResponse
  | {
      success: true;
      courses: Course[];
    };

export interface Course {
  id: string;
  name: string;
  held_at: string;
}

export interface CreateCourseRequest {
  name: string;
  held_at: string;
}

export type CreateCourseResponse = ErrorResponse | { success: true; course: Course };

export async function getCourses(query = ""): Promise<Course[]> {
  const response = await put<GetCoursesResponse>(
    query.length > 0
      ? "/api/v1/get/courses?query=" + encodeURIComponent(query)
      : "/api/v1/get/courses",
  );
  if (!response.success) throw new Error(response.message);

  // TODO: filter in backend
  return response.courses.filter((course) =>
    course.name.toLowerCase().includes(query.toLowerCase()),
  );
}

export async function createCourse(req: CreateCourseRequest): Promise<Course> {
  const response = await put<CreateCourseResponse>("/api/v1/mod/courses/create", req);
  if (!response.success) throw new Error(response.message);
  return response.course;
}
