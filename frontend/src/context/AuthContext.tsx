import {
  ParentProps,
  Show,
  createContext,
  createSignal,
  useContext,
} from 'solid-js';
import { put } from '../api';
import {
  LoginRequest,
  LoginResponse,
  RegisterRequest,
  RegisterResponse,
} from '../api/auth';
import LoginModal from '../components/LoginModal';

interface User {
  email: string;
}

function useProviderValue() {
  // TODO: fetch own user data when already logged in (session cookie)
  const [user, setUser] = createSignal<User>();
  const [loginModal, setLoginModal] = createSignal(false);

  return {
    user,
    loginModal,
    setLoginModal,
    async login(req: LoginRequest) {
      const data = await put<LoginResponse>('/api/v1/auth/login', req);
      setUser({ email: data.email });
    },
    async register(req: RegisterRequest) {
      await put<RegisterResponse>('/api/v1/auth/register', req);
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
    <AuthContext.Provider value={value}>
      {props.children}
      <Show when={value.loginModal()}>
        <LoginModal onClose={() => value.setLoginModal(false)} />
      </Show>
    </AuthContext.Provider>
  );
}

export function useAuthContext() {
  return useContext(AuthContext)!;
}
