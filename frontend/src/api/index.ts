export async function put<Response>(
  path: string,
  body?: unknown
): Promise<Response> {
  const response = await fetch(path, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  return response.json();
}

export interface ErrorResponse {
  success: false;
  message: string;
}