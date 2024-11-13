import { ErrorResponse, put } from ".";
import { RedactedUser } from "./users";

export type GetAllUsersResponse =
  | ErrorResponse
  | { success: true; users: RedactedUser[] };

export type UserBalanceResponse =
  | ErrorResponse
  | { success: true; balance: number };

export interface CreateSystemTransactionRequest {
  user_id: string,
  delta_ec: number,
  reason?: string,
};

export type CreateSystemTransactionResponse =
  | ErrorResponse
  | { success: true; };

export async function getAllUsers(): Promise<RedactedUser[]> {
  const response = await put<GetAllUsersResponse>("/api/v1/admin/users/get-users");
  if (!response.success) throw new Error(response.message);
  return response.users;
}

export async function getUserBalance(user_id: string): Promise<number> {
  const response = await put<UserBalanceResponse>("/api/v1/admin/ecs/get-user-balance", { user_id });
  if (!response.success) throw new Error(response.message);
  return response.balance;
}

export async function createSystemTransaction(
  request: CreateSystemTransactionRequest
): Promise<void> {
  const response = await put<GetAllUsersResponse>("/api/v1/admin/ecs/create-system-transaction", request);
  if (!response.success) throw new Error(response.message);
}
