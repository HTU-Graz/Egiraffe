import { ParentProps, createContext, createSignal, useContext } from 'solid-js';
import { put } from '../api';
import {
  LoginRequest,
  LoginResponse,
  RegisterRequest,
  RegisterResponse,
} from '../api/auth';

interface User {
  email: string;
}

function useProviderValue() {
  // TODO: fetch own user data when already logged in (session cookie)
  const [user, setUser] = createSignal<User | undefined>();
  const [loginModal, setLoginModal] = createSignal(false);

  return {
    user,
    loginModal,
    setLoginModal,
    async login(req: LoginRequest) {
      const response = await put<LoginResponse>('/api/v1/auth/login', req);
      if (!response.success) throw new Error('Email oder Passwort falsch');
      setUser({ email: response.email });
    },
    async register(req: RegisterRequest) {
      const response = await put<RegisterResponse>(
        '/api/v1/auth/register',
        req
      );
      if (!response.success) throw new Error('Registrierung fehlgeschlagen');
    },
    async logout() {
      setUser(undefined);
      await put('/api/v1/auth/logout');
    },
  };
}

const AuthContext = createContext<ReturnType<typeof useProviderValue>>();

export function AuthContextProvider(props: ParentProps) {
  const value = useProviderValue();
  return (
    <AuthContext.Provider value={value}>{props.children}</AuthContext.Provider>
  );
}

export function useAuthContext() {
  return useContext(AuthContext)!;
}
