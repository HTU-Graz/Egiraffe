import { ErrorResponse, put } from ".";

export enum AuthLevel {
  ANYONE = 0,
  REGULARUSER = 1,
  MODERATOR = 2,
  ADMIN = 3,
}

export interface RedactedUser {
  success: boolean;
  id: string;
  first_names: string;
  last_name: string;
  totp_enabled: boolean;
  user_role: AuthLevel;
}

export type GetMeResponse =
  | ErrorResponse
  | {
      success: true;
      user: RedactedUser;
    };

export interface UpdateMeRequest {
  first_names?: string;
  last_name?: string;
  password?: string;
}

export type UpdateMeResponse =
  | ErrorResponse
  | { success: true; message: string; user: RedactedUser };

export async function getMe(): Promise<RedactedUser> {
  const response = await put<GetMeResponse>("/api/v1/get/me");
  if (!response.success) throw new Error(response.message);
  return response.user;
}

export async function updateMe(options: UpdateMeRequest): Promise<RedactedUser> {
  const response = await put<UpdateMeResponse>("/api/v1/do/me", options);
  if (!response.success) throw new Error(response.message);
  return response.user;
}
