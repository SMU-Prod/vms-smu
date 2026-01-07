import { createSignal, createContext, useContext, ParentComponent } from 'solid-js';
import type { SystemStatus } from '../types';

interface ConfigState {
  systemStatus: SystemStatus;
  sidebarCollapsed: boolean;
  theme: 'dark' | 'light';
  language: 'pt-BR' | 'en-US';
  notifications: boolean;
  soundAlerts: boolean;
}

interface ConfigContextValue {
  state: () => ConfigState;
  toggleSidebar: () => void;
  setTheme: (theme: 'dark' | 'light') => void;
  setLanguage: (lang: 'pt-BR' | 'en-US') => void;
  toggleNotifications: () => void;
  toggleSoundAlerts: () => void;
  updateSystemStatus: (status: Partial<SystemStatus>) => void;
}

const ConfigContext = createContext<ConfigContextValue>();

const defaultSystemStatus: SystemStatus = {
  camerasOnline: 7,
  camerasTotal: 8,
  eventsToday: 45,
  eventsUnacknowledged: 3,
  storageUsed: 2.4 * 1024 * 1024 * 1024 * 1024, // 2.4 TB
  storageTotal: 5 * 1024 * 1024 * 1024 * 1024, // 5 TB
  recordingActive: true,
  cpuUsage: 35,
  memoryUsage: 62,
  gpuUsage: 28,
};

export const ConfigProvider: ParentComponent = (props) => {
  const [state, setState] = createSignal<ConfigState>({
    systemStatus: defaultSystemStatus,
    sidebarCollapsed: false,
    theme: 'dark',
    language: 'pt-BR',
    notifications: true,
    soundAlerts: true,
  });

  const toggleSidebar = () => {
    setState(s => ({ ...s, sidebarCollapsed: !s.sidebarCollapsed }));
  };

  const setTheme = (theme: 'dark' | 'light') => {
    setState(s => ({ ...s, theme }));
    document.documentElement.classList.toggle('dark', theme === 'dark');
  };

  const setLanguage = (language: 'pt-BR' | 'en-US') => {
    setState(s => ({ ...s, language }));
  };

  const toggleNotifications = () => {
    setState(s => ({ ...s, notifications: !s.notifications }));
  };

  const toggleSoundAlerts = () => {
    setState(s => ({ ...s, soundAlerts: !s.soundAlerts }));
  };

  const updateSystemStatus = (status: Partial<SystemStatus>) => {
    setState(s => ({
      ...s,
      systemStatus: { ...s.systemStatus, ...status },
    }));
  };

  const value: ConfigContextValue = {
    state,
    toggleSidebar,
    setTheme,
    setLanguage,
    toggleNotifications,
    toggleSoundAlerts,
    updateSystemStatus,
  };

  return (
    <ConfigContext.Provider value={value}>
      {props.children}
    </ConfigContext.Provider>
  );
};

export const useConfig = () => {
  const context = useContext(ConfigContext);
  if (!context) {
    throw new Error('useConfig must be used within ConfigProvider');
  }
  return context;
};
