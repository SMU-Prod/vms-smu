import { Component, For } from 'solid-js';
import { A } from '@solidjs/router';
import { Settings, Camera, Users, BarChart3, MapPin, Bell, HardDrive, Shield } from 'lucide-solid';

const configItems = [
  { path: '/config/cameras', label: 'Câmeras', icon: Camera, desc: 'Gerenciar câmeras IP e ONVIF' },
  { path: '/config/users', label: 'Usuários', icon: Users, desc: 'Usuários e permissões' },
  { path: '/config/analytics', label: 'Analytics', icon: BarChart3, desc: 'Regras de detecção e zonas' },
  { path: '/config/alarms', label: 'Alarmes', icon: Bell, desc: 'Configurar alertas' },
  { path: '/config/storage', label: 'Armazenamento', icon: HardDrive, desc: 'Discos e retenção' },
  { path: '/config/security', label: 'Segurança', icon: Shield, desc: 'Autenticação e logs' },
];

const Config: Component = () => {
  return (
    <div class="h-full flex flex-col">
      <div class="flex items-center gap-3 p-4 bg-slate-800/50 border-b border-slate-700">
        <Settings class="w-5 h-5 text-vms-accent" />
        <h1 class="text-lg font-semibold text-white">Configurações</h1>
      </div>

      <div class="flex-1 overflow-auto p-6">
        <div class="max-w-4xl mx-auto">
          <p class="text-slate-400 mb-6">Gerencie as configurações do sistema VMS Enterprise.</p>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <For each={configItems}>
              {(item) => (
                <A href={item.path} class="card hover:border-vms-accent/50 transition-all group">
                  <div class="flex items-start gap-4">
                    <div class="p-3 bg-slate-700 rounded-lg group-hover:bg-vms-primary/20 transition-colors">
                      <item.icon class="w-6 h-6 text-slate-400 group-hover:text-vms-accent transition-colors" />
                    </div>
                    <div>
                      <h3 class="text-white font-medium group-hover:text-vms-accent transition-colors">{item.label}</h3>
                      <p class="text-slate-500 text-sm mt-1">{item.desc}</p>
                    </div>
                  </div>
                </A>
              )}
            </For>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Config;
