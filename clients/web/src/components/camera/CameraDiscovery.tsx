/**
 * CameraDiscovery Component
 * UI para descobrir e adicionar câmeras ONVIF
 */

import { Component, createSignal, For, Show } from 'solid-js';
import { Search, Plus, Camera, Check, X, Loader2, Wifi } from 'lucide-solid';
import { useCamera, DiscoveredCamera, CameraProfile } from '../../stores/camera.tsx';

interface DiscoveryProps {
  onClose?: () => void;
}

export const CameraDiscovery: Component<DiscoveryProps> = (props) => {
  const { actions } = useCamera();
  
  // Estado
  const [isSearching, setIsSearching] = createSignal(false);
  const [discoveredCameras, setDiscoveredCameras] = createSignal<DiscoveredCamera[]>([]);
  const [selectedCamera, setSelectedCamera] = createSignal<DiscoveredCamera | null>(null);
  const [profiles, setProfiles] = createSignal<CameraProfile[]>([]);
  const [loadingProfiles, setLoadingProfiles] = createSignal(false);
  
  // Credenciais
  const [username, setUsername] = createSignal('admin');
  const [password, setPassword] = createSignal('');
  const [error, setError] = createSignal<string | null>(null);

  // Descobrir câmeras
  const handleDiscover = async () => {
    setIsSearching(true);
    setError(null);
    setDiscoveredCameras([]);
    
    try {
      const cameras = await actions.discoverCameras(5);
      setDiscoveredCameras(cameras);
      
      if (cameras.length === 0) {
        setError('Nenhuma câmera ONVIF encontrada na rede.');
      }
    } catch (err: any) {
      setError(err.message || 'Erro ao descobrir câmeras');
    } finally {
      setIsSearching(false);
    }
  };

  // Selecionar câmera e buscar profiles
  const handleSelectCamera = async (camera: DiscoveredCamera) => {
    setSelectedCamera(camera);
    setProfiles([]);
    setLoadingProfiles(true);
    setError(null);
    
    try {
      const cameraProfiles = await actions.getCameraProfiles({
        ip: camera.ip,
        port: camera.port,
        username: username(),
        password: password(),
      });
      setProfiles(cameraProfiles);
    } catch (err: any) {
      setError('Falha ao conectar. Verifique usuário e senha.');
    } finally {
      setLoadingProfiles(false);
    }
  };

  // Adicionar câmera
  const handleAddCamera = async (profile: CameraProfile) => {
    const camera = selectedCamera();
    if (!camera || !profile.rtsp_url) return;
    
    try {
      await actions.addCamera({
        name: `${camera.name} - ${profile.name}`,
        url: profile.rtsp_url,
        username: username(),
        password: password(),
      });
      
      // Fecha modal após adicionar
      props.onClose?.();
    } catch (err: any) {
      setError(err.message || 'Erro ao adicionar câmera');
    }
  };

  return (
    <div class="fixed inset-0 bg-black/70 flex items-center justify-center z-50 animate-fadeIn">
      <div class="bg-slate-800 rounded-xl border border-slate-700 w-full max-w-2xl max-h-[80vh] overflow-hidden shadow-2xl">
        {/* Header */}
        <div class="flex items-center justify-between p-4 border-b border-slate-700">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-vms-primary/20 flex items-center justify-center">
              <Wifi class="w-5 h-5 text-vms-primary" />
            </div>
            <div>
              <h2 class="text-lg font-semibold text-white">Descobrir Câmeras ONVIF</h2>
              <p class="text-sm text-slate-400">Busque câmeras na rede local</p>
            </div>
          </div>
          <button 
            onClick={props.onClose}
            class="p-2 hover:bg-slate-700 rounded-lg transition-colors"
          >
            <X class="w-5 h-5 text-slate-400" />
          </button>
        </div>

        {/* Content */}
        <div class="p-4 overflow-y-auto max-h-[calc(80vh-140px)]">
          {/* Credenciais */}
          <div class="grid grid-cols-2 gap-4 mb-4">
            <div>
              <label class="block text-sm text-slate-400 mb-1">Usuário ONVIF</label>
              <input
                type="text"
                value={username()}
                onInput={(e) => setUsername(e.currentTarget.value)}
                class="w-full px-3 py-2 bg-slate-900 border border-slate-600 rounded-lg text-white"
                placeholder="admin"
              />
            </div>
            <div>
              <label class="block text-sm text-slate-400 mb-1">Senha</label>
              <input
                type="password"
                value={password()}
                onInput={(e) => setPassword(e.currentTarget.value)}
                class="w-full px-3 py-2 bg-slate-900 border border-slate-600 rounded-lg text-white"
                placeholder="••••••••"
              />
            </div>
          </div>

          {/* Botão Descobrir */}
          <button
            onClick={handleDiscover}
            disabled={isSearching()}
            class="w-full mb-4 px-4 py-3 bg-vms-primary hover:bg-vms-secondary disabled:bg-slate-700 rounded-lg font-medium transition-colors flex items-center justify-center gap-2"
          >
            <Show when={isSearching()} fallback={<Search class="w-5 h-5" />}>
              <Loader2 class="w-5 h-5 animate-spin" />
            </Show>
            {isSearching() ? 'Buscando...' : 'Buscar Câmeras na Rede'}
          </button>

          {/* Erro */}
          <Show when={error()}>
            <div class="mb-4 p-3 bg-red-500/20 border border-red-500/50 rounded-lg text-red-400 text-sm">
              {error()}
            </div>
          </Show>

          {/* Lista de câmeras descobertas */}
          <Show when={discoveredCameras().length > 0}>
            <div class="mb-4">
              <h3 class="text-sm font-medium text-slate-300 mb-2">
                Câmeras encontradas ({discoveredCameras().length})
              </h3>
              <div class="space-y-2">
                <For each={discoveredCameras()}>
                  {(camera) => (
                    <div 
                      onClick={() => handleSelectCamera(camera)}
                      class={`p-3 rounded-lg border cursor-pointer transition-colors ${
                        selectedCamera()?.ip === camera.ip
                          ? 'bg-vms-primary/20 border-vms-primary'
                          : 'bg-slate-900 border-slate-700 hover:border-slate-500'
                      }`}
                    >
                      <div class="flex items-center gap-3">
                        <Camera class="w-5 h-5 text-slate-400" />
                        <div class="flex-1">
                          <div class="text-white font-medium">{camera.name}</div>
                          <div class="text-sm text-slate-400">
                            {camera.ip}:{camera.port}
                          </div>
                        </div>
                        <Show when={selectedCamera()?.ip === camera.ip}>
                          <Check class="w-5 h-5 text-vms-primary" />
                        </Show>
                      </div>
                    </div>
                  )}
                </For>
              </div>
            </div>
          </Show>

          {/* Profiles da câmera selecionada */}
          <Show when={selectedCamera()}>
            <div>
              <h3 class="text-sm font-medium text-slate-300 mb-2">
                Profiles de Stream
              </h3>
              
              <Show when={loadingProfiles()}>
                <div class="flex items-center justify-center py-8">
                  <Loader2 class="w-6 h-6 text-vms-primary animate-spin" />
                </div>
              </Show>

              <Show when={!loadingProfiles() && profiles().length > 0}>
                <div class="space-y-2">
                  <For each={profiles()}>
                    {(profile) => (
                      <div class="p-3 bg-slate-900 rounded-lg border border-slate-700">
                        <div class="flex items-center justify-between">
                          <div>
                            <div class="text-white font-medium">{profile.name}</div>
                            <div class="text-sm text-slate-400">
                              {profile.resolution} • {profile.fps}fps • {profile.codec}
                            </div>
                            <div class="text-xs text-slate-500 mt-1 font-mono truncate max-w-md">
                              {profile.rtsp_url}
                            </div>
                          </div>
                          <button
                            onClick={() => handleAddCamera(profile)}
                            class="px-3 py-1.5 bg-vms-success hover:bg-green-600 rounded-lg text-sm font-medium transition-colors flex items-center gap-1"
                          >
                            <Plus class="w-4 h-4" />
                            Adicionar
                          </button>
                        </div>
                      </div>
                    )}
                  </For>
                </div>
              </Show>
            </div>
          </Show>
        </div>

        {/* Footer */}
        <div class="p-4 border-t border-slate-700 flex justify-end gap-3">
          <button
            onClick={props.onClose}
            class="px-4 py-2 bg-slate-700 hover:bg-slate-600 rounded-lg font-medium transition-colors"
          >
            Fechar
          </button>
        </div>
      </div>
    </div>
  );
};

export default CameraDiscovery;
