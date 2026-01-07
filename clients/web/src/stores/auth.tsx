import { createSignal, createContext, useContext, ParentComponent } from 'solid-js';
import type { User } from '../types';

interface AuthState {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
}

interface AuthContextValue {
  state: () => AuthState;
  login: (username: string, password: string) => Promise<boolean>;
  logout: () => void;
  checkAuth: () => boolean;
}

const AuthContext = createContext<AuthContextValue>();

// Mock user for development
const mockUser: User = {
  id: '1',
  username: 'admin',
  email: 'admin@vms.local',
  name: 'Administrador',
  role: 'admin',
  permissions: ['*'],
  lastLogin: new Date(),
  createdAt: new Date('2024-01-01'),
};

export const AuthProvider: ParentComponent = (props) => {
  const [state, setState] = createSignal<AuthState>({
    user: null,
    token: null,
    isAuthenticated: false,
  });

  const login = async (username: string, password: string): Promise<boolean> => {
    // Mock login - replace with real API call
    if (username === 'admin' && password === 'admin') {
      const token = 'mock-jwt-token-' + Date.now();
      localStorage.setItem('vms_token', token);
      setState({
        user: mockUser,
        token,
        isAuthenticated: true,
      });
      return true;
    }
    return false;
  };

  const logout = () => {
    localStorage.removeItem('vms_token');
    setState({
      user: null,
      token: null,
      isAuthenticated: false,
    });
  };

  const checkAuth = (): boolean => {
    const token = localStorage.getItem('vms_token');
    if (token) {
      // In real app, validate token with API
      setState({
        user: mockUser,
        token,
        isAuthenticated: true,
      });
      return true;
    }
    return false;
  };

  const value: AuthContextValue = {
    state,
    login,
    logout,
    checkAuth,
  };

  return (
    <AuthContext.Provider value={value}>
      {props.children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within AuthProvider');
  }
  return context;
};
