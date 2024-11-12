import { ErrorResponse, put } from ".";
import { RedactedUser } from "./users";

export type GetAllUsersResponse =
  | ErrorResponse
  | { success: true; users: RedactedUser[] };


export async function getAllUsers(): Promise<RedactedUser[]> {
  const response = await put<GetAllUsersResponse>("/api/v1/admin/users/get-users");
  if (!response.success) throw new Error(response.message);
  return response.users;
}
