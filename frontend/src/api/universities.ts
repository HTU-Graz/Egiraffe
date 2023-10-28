import { ErrorResponse, put } from '.';

export interface University {
  id: string;
  full_name: string;
  mid_name: string;
  short_name: string;
  domain_names: string[];
}

export type GetUniversitiesResponse =
  | ErrorResponse
  | { success: true; universities: University[] };

export async function getUniversities(): Promise<University[]> {
  const response = await put<GetUniversitiesResponse>(
    '/api/v1/get/universities'
  );
  if (!response.success) throw new Error(response.message);
  return response.universities;
}
