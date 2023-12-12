import { ParentProps, createContext, createResource, createSignal, useContext } from "solid-js";
import { LoginRequest, RegisterRequest, login, logout, register } from "../api/auth";
import { AuthLevel, getMe } from "../api/users";

function useProviderValue() {
  const [user, { mutate: mutateUser, refetch: refetchUser }] = createResource(() =>
    getMe().catch(console.warn),
  );
  const [loginModal, setLoginModal] = createSignal(false);

  return {
    user,
    loginModal,
    setLoginModal,
    async login(req: LoginRequest) {
      await login(req);
      await refetchUser();
    },
    async register(req: RegisterRequest) {
      await register(req);
    },
    async logout() {
      mutateUser(undefined);
      await logout();
    },
    hasRole(role: AuthLevel) {
      return (user()?.user_role ?? AuthLevel.ANYONE) >= role;
    },
  };
}

const AuthContext = createContext<ReturnType<typeof useProviderValue>>();

export function AuthContextProvider(props: ParentProps) {
  const value = useProviderValue();
  return <AuthContext.Provider value={value}>{props.children}</AuthContext.Provider>;
}

export function useAuthContext() {
  return useContext(AuthContext)!;
}
