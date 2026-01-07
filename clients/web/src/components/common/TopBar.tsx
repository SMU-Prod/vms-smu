import { Component, Show } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { Bell, Settings, LogOut, User, ChevronDown, Shield } from 'lucide-solid';
import { useAuth, useEvents, useConfig } from '../../stores';

export const TopBar: Component = () => {
  const { state: authState, logout } = useAuth();
  const { state: eventsState } = useEvents();
  const { state: configState } = useConfig();
  const navigate = useNavigate();

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  return (
    <header class="h-14 bg-slate-900 border-b border-slate-700 flex items-center justify-between px-4 glass">
      {/* Logo */}
      <div class="flex items-center gap-3">
        <div class="flex items-center gap-2">
          <div class="w-8 h-8 bg-gradient-to-br from-vms-primary to-vms-accent rounded-lg flex items-center justify-center">
            <Shield class="w-5 h-5 text-white" />
          </div>
          <span class="text-lg font-bold bg-gradient-to-r from-vms-primary to-vms-accent bg-clip-text text-transparent">
            VMS Enterprise
          </span>
        </div>
      </div>

      {/* Right side */}
      <div class="flex items-center gap-4">
        {/* System Status Pills */}
        <div class="hidden md:flex items-center gap-2 text-xs">
          <div class="px-2 py-1 rounded-full bg-green-500/20 text-green-400 border border-green-500/30">
            <span class="font-medium">{configState().systemStatus.camerasOnline}</span>
            <span class="text-green-300/70">/{configState().systemStatus.camerasTotal} câmeras</span>
          </div>
          <Show when={configState().systemStatus.recordingActive}>
            <div class="px-2 py-1 rounded-full bg-red-500/20 text-red-400 border border-red-500/30 flex items-center gap-1">
              <span class="w-2 h-2 bg-red-500 rounded-full animate-pulse"></span>
              <span class="font-medium">REC</span>
            </div>
          </Show>
        </div>

        {/* Notifications */}
        <button class="relative p-2 text-slate-400 hover:text-white hover:bg-slate-700/50 rounded-lg transition-colors">
          <Bell class="w-5 h-5" />
          <Show when={eventsState().unacknowledgedCount > 0}>
            <span class="absolute -top-0.5 -right-0.5 w-5 h-5 bg-red-500 text-white text-xs font-bold rounded-full flex items-center justify-center">
              {eventsState().unacknowledgedCount > 9 ? '9+' : eventsState().unacknowledgedCount}
            </span>
          </Show>
        </button>

        {/* Settings */}
        <button 
          class="p-2 text-slate-400 hover:text-white hover:bg-slate-700/50 rounded-lg transition-colors"
          onClick={() => navigate('/config')}
        >
          <Settings class="w-5 h-5" />
        </button>

        {/* User Menu */}
        <div class="flex items-center gap-2 pl-3 border-l border-slate-700">
          <div class="w-8 h-8 bg-gradient-to-br from-vms-primary to-vms-secondary rounded-full flex items-center justify-center">
            <User class="w-4 h-4 text-white" />
          </div>
          <div class="hidden sm:block">
            <div class="text-sm font-medium text-white">{authState().user?.name}</div>
            <div class="text-xs text-slate-400 capitalize">{authState().user?.role}</div>
          </div>
          <ChevronDown class="w-4 h-4 text-slate-400 hidden sm:block" />
          
          <button 
            class="ml-2 p-2 text-slate-400 hover:text-red-400 hover:bg-red-500/10 rounded-lg transition-colors"
            onClick={handleLogout}
            title="Sair"
          >
            <LogOut class="w-4 h-4" />
          </button>
        </div>
      </div>
    </header>
  );
};
