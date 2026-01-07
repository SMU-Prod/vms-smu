/**
 * CamerasConfig Page
 * Gerenciamento de câmeras com discovery ONVIF
 */

import { Component, createSignal, For, Show } from 'solid-js';
import { Search, Plus, Trash2, Settings, Video, Circle } from 'lucide-solid';
import { useCamera } from '../../stores/camera.tsx';
import { CameraDiscovery } from '../../components/camera';

const CamerasConfig: Component = () => {
  const { state, actions } = useCamera();
  const [showDiscovery, setShowDiscovery] = createSignal(false);
  const [deletingId, setDeletingId] = createSignal<string | null>(null);

  const handleDelete = async (id: string) => {
    if (confirm('Tem certeza que deseja remover esta câmera?')) {
      setDeletingId(id);
      try {
        await actions.deleteCamera(id);
      } finally {
        setDeletingId(null);
      }
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'online':
      case 'recording':
        return 'text-green-500';
      case 'offline':
        return 'text-slate-500';
      case 'error':
        return 'text-red-500';
      default:
        return 'text-slate-500';
    }
  };

  return (
    <div class="h-full flex flex-col">
      {/* Header */}
      <div class="flex items-center justify-between p-4 bg-slate-800/50 border-b border-slate-700">
        <div class="flex items-center gap-3">
          <Video class="w-5 h-5 text-vms-accent" />
          <h1 class="text-lg font-semibold text-white">Gerenciar Câmeras</h1>
        </div>
        <button
          onClick={() => setShowDiscovery(true)}
          class="px-4 py-2 bg-vms-primary hover:bg-vms-secondary rounded-lg font-medium transition-colors flex items-center gap-2"
        >
          <Search class="w-4 h-4" />
          Descobrir Câmeras ONVIF
        </button>
      </div>

      {/* Content */}
      <div class="flex-1 overflow-auto p-6">
        <Show when={state.loading}>
          <div class="flex items-center justify-center py-12">
            <div class="w-8 h-8 border-2 border-vms-accent border-t-transparent rounded-full animate-spin" />
          </div>
        </Show>

        <Show when={state.error}>
          <div class="p-4 bg-red-500/20 border border-red-500/50 rounded-lg text-red-400 mb-4">
            {state.error}
          </div>
        </Show>

        <Show when={!state.loading && state.cameras.length === 0}>
          <div class="text-center py-12">
            <Video class="w-16 h-16 text-slate-600 mx-auto mb-4" />
            <h2 class="text-xl font-semibold text-white mb-2">Nenhuma câmera configurada</h2>
            <p class="text-slate-400 mb-6">
              Clique em "Descobrir Câmeras ONVIF" para buscar câmeras na rede local.
            </p>
            <button
              onClick={() => setShowDiscovery(true)}
              class="px-6 py-3 bg-vms-primary hover:bg-vms-secondary rounded-lg font-medium transition-colors inline-flex items-center gap-2"
            >
              <Plus class="w-5 h-5" />
              Adicionar Primeira Câmera
            </button>
          </div>
        </Show>

        <Show when={state.cameras.length > 0}>
          <div class="space-y-3">
            <For each={state.cameras}>
              {(camera) => (
                <div class="bg-slate-800 rounded-lg border border-slate-700 p-4 flex items-center gap-4">
                  {/* Status Indicator */}
                  <Circle 
                    class={`w-3 h-3 fill-current ${getStatusColor(camera.status)}`} 
                  />

                  {/* Camera Info */}
                  <div class="flex-1">
                    <div class="text-white font-medium">{camera.name}</div>
                    <div class="text-sm text-slate-400">
                      {camera.resolution} • {camera.fps}fps • {camera.codec?.toUpperCase()}
                    </div>
                  </div>

                  {/* Status Badge */}
                  <div class={`px-2 py-1 rounded text-xs font-medium ${
                    camera.status === 'online' || camera.status === 'recording'
                      ? 'bg-green-500/20 text-green-400'
                      : camera.status === 'error'
                      ? 'bg-red-500/20 text-red-400'
                      : 'bg-slate-700 text-slate-400'
                  }`}>
                    {camera.status === 'recording' ? '● REC' : camera.status.toUpperCase()}
                  </div>

                  {/* Actions */}
                  <div class="flex items-center gap-2">
                    <button
                      class="p-2 hover:bg-slate-700 rounded-lg transition-colors"
                      title="Configurações"
                    >
                      <Settings class="w-4 h-4 text-slate-400" />
                    </button>
                    <button
                      onClick={() => handleDelete(camera.id)}
                      disabled={deletingId() === camera.id}
                      class="p-2 hover:bg-red-500/20 rounded-lg transition-colors group"
                      title="Remover"
                    >
                      <Trash2 class="w-4 h-4 text-slate-400 group-hover:text-red-400" />
                    </button>
                  </div>
                </div>
              )}
            </For>
          </div>
        </Show>
      </div>

      {/* Discovery Modal */}
      <Show when={showDiscovery()}>
        <CameraDiscovery onClose={() => setShowDiscovery(false)} />
      </Show>
    </div>
  );
};

export default CamerasConfig;
