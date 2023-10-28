import { ErrorResponse, put } from '.';

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

export async function getCourses(): Promise<Course[]> {
  const response = await put<GetCoursesResponse>('/api/v1/get/courses');
  if (!response.success) throw new Error(response.message);
  return response.courses;
}
