import { ErrorResponse, put } from ".";

/**
 * Describes the different levels of authentication
 * that a user can have.
 *
 * The higher the number, the more permissions the user has.
 */
export enum AuthLevel {
  /** Describes anonymous users who are not logged in */
  ANYONE = 0,

  /** Describes users without any special permissions */
  REGULAR_USER = 1,

  /** Describes users with moderator permissions */
  MODERATOR = 2,

  /** Describes users with admin permissions */
  ADMIN = 3,
}

export function authLevelToString(level: AuthLevel): string {
  switch (level) {
    case AuthLevel.ANYONE:
      return "Guest";
    case AuthLevel.REGULAR_USER:
      return "User";
    case AuthLevel.MODERATOR:
      return "Moderator";
    case AuthLevel.ADMIN:
      return "Admin";
  }
}

/**
 * Contains most of the information the server knows about a user.
 * 
 * This is the type of object that is returned by the `/api/v1/get/me` endpoint.
 * 
 * Information like the password hash and the user's TOTP secret are withheld by the server.
 */
export interface RedactedUser {
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

export type MyEcsBalanceResponse =
  | ErrorResponse
  | { success: true; ecs_balance: number };

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


export async function getMyEcsBalance(): Promise<number> {
  const response = await put<MyEcsBalanceResponse>("/api/v1/get/my-ecs-balance");
  if (!response.success) throw new Error(response.message);
  return response.ecs_balance;
}
