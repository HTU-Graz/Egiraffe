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
