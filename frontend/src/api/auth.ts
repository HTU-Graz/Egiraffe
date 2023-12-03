import { put } from ".";

export interface LoginRequest {
  email: string;
  password: string;
  totp?: string;
}

export interface LoginResponse {
  success: boolean;
  email: string;
}

export interface RegisterRequest {
  first_names: string;
  last_name: string;
  email: string;
  password: string;
}

export interface RegisterResponse {
  success: boolean;
  // email: string;
}

export async function login(req: LoginRequest): Promise<string> {
  const response = await put<LoginResponse>("/api/v1/auth/login", req);
  if (!response.success) throw new Error("Email oder Passwort falsch");
  return response.email;
}

export async function register(req: RegisterRequest): Promise<void> {
  const response = await put<RegisterResponse>("/api/v1/auth/register", req);
  if (!response.success) throw new Error("Registrierung fehlgeschlagen");
}

export async function logout(): Promise<void> {
  await put("/api/v1/auth/logout");
}
