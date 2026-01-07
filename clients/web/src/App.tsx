import { Component, Show, lazy, Suspense, onMount } from 'solid-js';
import { A, useNavigate, useLocation } from '@solidjs/router';
import { TopBar, Sidebar, StatusBar } from './components/common';
import { AuthProvider, CameraProvider, EventsProvider, ConfigProvider, useAuth } from './stores';

// Lazy load pages
const Live = lazy(() => import('./pages/Live'));
const Playback = lazy(() => import('./pages/Playback'));
const Events = lazy(() => import('./pages/Events'));
const Evidence = lazy(() => import('./pages/Evidence'));
const Config = lazy(() => import('./pages/Config'));

// Loading spinner
const PageLoader: Component = () => (
  <div class="h-full flex items-center justify-center">
    <div class="w-8 h-8 border-2 border-vms-accent border-t-transparent rounded-full animate-spin"></div>
  </div>
);

// Main layout component with auth check
const MainLayout: Component<{ children?: any }> = (props) => {
  const { state, checkAuth } = useAuth();
  const navigate = useNavigate();

  onMount(() => {
    if (!checkAuth()) {
      navigate('/login');
    }
  });

  return (
    <Show when={state().isAuthenticated} fallback={<PageLoader />}>
      <div class="h-screen flex flex-col bg-vms-darker">
        <TopBar />
        <div class="flex-1 flex overflow-hidden relative">
          <Sidebar />
          <main class="flex-1 overflow-hidden bg-slate-900/50">
            <Suspense fallback={<PageLoader />}>
              {props.children}
            </Suspense>
          </main>
        </div>
        <StatusBar />
      </div>
    </Show>
  );
};

// Individual page wrappers
export const LivePage: Component = () => (
  <MainLayout><Live /></MainLayout>
);

export const PlaybackPage: Component = () => (
  <MainLayout><Playback /></MainLayout>
);

export const EventsPage: Component = () => (
  <MainLayout><Events /></MainLayout>
);

export const EvidencePage: Component = () => (
  <MainLayout><Evidence /></MainLayout>
);

export const ConfigPage: Component = () => (
  <MainLayout><Config /></MainLayout>
);

// Login page is loaded directly without lazy for faster initial load
import Login from './pages/Login';
export const LoginPage: Component = () => <Login />;

const App: Component = () => {
  return (
    <AuthProvider>
      <ConfigProvider>
        <CameraProvider>
          <EventsProvider>
            {/* This is the root component, routes are defined in index.tsx */}
          </EventsProvider>
        </CameraProvider>
      </ConfigProvider>
    </AuthProvider>
  );
};

export default App;
