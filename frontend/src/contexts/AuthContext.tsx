import {
  createContext,
  useContext,
  useState,
  useCallback,
  useEffect,
} from 'react';

export interface User {
  id: number;
  username: string;
  created_at: string;
}

interface AuthContextType {
  user: User | null;
  isLoading: boolean;
  login: (username: string, password: string) => Promise<void>;
  register: (username: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
  isAuthenticated: boolean;
}

const AuthContext = createContext<AuthContextType | null>(null);

export function useAuth(): AuthContextType {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}

function fetchUser(): Promise<User | null> {
  return fetch('/api/auth/me', {
    credentials: 'include',
  })
    .then((res) => {
      if (res.ok) return res.json();
      return null;
    })
    .catch(() => null);
}

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    fetchUser().then(setUser);
  }, []);

  const login = useCallback(async (username: string, password: string) => {
    setIsLoading(true);
    const res = await fetch('/api/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ username, password }),
    });
    if (!res.ok) {
      const data = await res.json().catch(() => ({}));
      throw new Error(data.message || 'Login failed');
    }
    const data = await res.json();
    setUser(data);
    setIsLoading(false);
  }, []);

  const register = useCallback(async (username: string, password: string) => {
    setIsLoading(true);
    const res = await fetch('/api/auth/register', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ username, password }),
    });
    if (!res.ok) {
      const data = await res.json().catch(() => ({}));
      throw new Error(data.message || 'Registration failed');
    }
    const data = await res.json();
    setUser(data);
    setIsLoading(false);
  }, []);

  const logout = useCallback(async () => {
    await fetch('/api/auth/logout', {
      method: 'DELETE',
      credentials: 'include',
    });
    setUser(null);
  }, []);

  const value = {
    user,
    isLoading,
    login,
    register,
    logout,
    isAuthenticated: user !== null,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}
