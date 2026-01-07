import { Component, For, Show } from 'solid-js';
import { A, useLocation } from '@solidjs/router';
import { 
  Video, 
  PlayCircle, 
  Bell, 
  FileText, 
  Settings, 
  ChevronLeft,
  ChevronRight,
  Camera,
  Users,
  BarChart3,
  MapPin
} from 'lucide-solid';
import { useConfig } from '../../stores';

interface NavItem {
  path: string;
  label: string;
  icon: typeof Video;
  badge?: number;
}

const navItems: NavItem[] = [
  { path: '/live', label: 'Ao Vivo', icon: Video },
  { path: '/playback', label: 'Playback', icon: PlayCircle },
  { path: '/events', label: 'Eventos', icon: Bell },
  { path: '/evidence', label: 'Ocorrências', icon: FileText },
  { path: '/config', label: 'Configurações', icon: Settings },
];

const configSubItems: NavItem[] = [
  { path: '/config/cameras', label: 'Câmeras', icon: Camera },
  { path: '/config/users', label: 'Usuários', icon: Users },
  { path: '/config/analytics', label: 'Analytics', icon: BarChart3 },
  { path: '/config/maps', label: 'Mapas', icon: MapPin },
];

export const Sidebar: Component = () => {
  const location = useLocation();
  const { state, toggleSidebar } = useConfig();
  
  const isActive = (path: string) => {
    if (path === '/config') {
      return location.pathname.startsWith('/config');
    }
    return location.pathname === path;
  };

  const isConfigOpen = () => location.pathname.startsWith('/config');

  return (
    <aside 
      class={`
        ${state().sidebarCollapsed ? 'w-16' : 'w-56'} 
        bg-slate-900 border-r border-slate-700 flex flex-col transition-all duration-300 glass
      `}
    >
      {/* Toggle Button */}
      <button 
        class="absolute -right-3 top-20 w-6 h-6 bg-slate-800 border border-slate-600 rounded-full flex items-center justify-center text-slate-400 hover:text-white hover:bg-slate-700 transition-colors z-10"
        onClick={toggleSidebar}
      >
        <Show when={state().sidebarCollapsed} fallback={<ChevronLeft class="w-4 h-4" />}>
          <ChevronRight class="w-4 h-4" />
        </Show>
      </button>

      {/* Navigation */}
      <nav class="flex-1 py-4">
        <ul class="space-y-1 px-2">
          <For each={navItems}>
            {(item) => (
              <li>
                <A
                  href={item.path}
                  class={`
                    flex items-center gap-3 px-3 py-2.5 rounded-lg transition-all duration-200
                    ${isActive(item.path) 
                      ? 'bg-vms-primary/20 text-vms-accent border-l-2 border-vms-accent' 
                      : 'text-slate-400 hover:text-white hover:bg-slate-800'
                    }
                  `}
                >
                  <item.icon class="w-5 h-5 flex-shrink-0" />
                  <Show when={!state().sidebarCollapsed}>
                    <span class="font-medium text-sm">{item.label}</span>
                  </Show>
                  <Show when={item.badge && !state().sidebarCollapsed}>
                    <span class="ml-auto px-2 py-0.5 text-xs font-bold bg-red-500 text-white rounded-full">
                      {item.badge}
                    </span>
                  </Show>
                </A>

                {/* Config Sub-items */}
                <Show when={item.path === '/config' && isConfigOpen() && !state().sidebarCollapsed}>
                  <ul class="mt-1 ml-4 pl-4 border-l border-slate-700 space-y-1">
                    <For each={configSubItems}>
                      {(subItem) => (
                        <li>
                          <A
                            href={subItem.path}
                            class={`
                              flex items-center gap-2 px-2 py-1.5 rounded-lg text-sm transition-all duration-200
                              ${location.pathname === subItem.path
                                ? 'text-vms-accent bg-vms-accent/10'
                                : 'text-slate-500 hover:text-slate-300 hover:bg-slate-800/50'
                              }
                            `}
                          >
                            <subItem.icon class="w-4 h-4" />
                            <span>{subItem.label}</span>
                          </A>
                        </li>
                      )}
                    </For>
                  </ul>
                </Show>
              </li>
            )}
          </For>
        </ul>
      </nav>

      {/* Bottom info */}
      <Show when={!state().sidebarCollapsed}>
        <div class="p-4 border-t border-slate-700">
          <div class="text-xs text-slate-500">
            <div>VMS Enterprise v0.1.0</div>
            <div class="mt-1">© 2025 VMS Systems</div>
          </div>
        </div>
      </Show>
    </aside>
  );
};
