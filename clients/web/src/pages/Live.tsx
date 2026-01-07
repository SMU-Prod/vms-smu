import { Component, For, Show, onMount, createSignal } from 'solid-js';
import {
  Grid2x2,
  Grid3x3,
  Maximize2,
  MoreVertical,
  Camera,
  Mic,
  MicOff,
  Volume2,
  VolumeX,
  Circle,
  Expand
} from 'lucide-solid';
import { useCameras, useConfig } from '../stores';
import { WebRTCPlayer } from '../components/camera';
import type { LayoutType, Camera as CameraType } from '../types';

const layoutOptions: { type: LayoutType; label: string; cols: number }[] = [
  { type: '1x1', label: '1', cols: 1 },
  { type: '2x2', label: '4', cols: 2 },
  { type: '3x3', label: '9', cols: 3 },
  { type: '4x4', label: '16', cols: 4 },
];

// Camera Cell Component
const CameraCell: Component<{ camera: CameraType | undefined; index: number }> = (props) => {
  const [isHovered, setIsHovered] = createSignal(false);
  const [isMuted, setIsMuted] = createSignal(true);

  const getRandomColor = () => {
    const colors = [
      'from-blue-900/40 to-slate-900',
      'from-purple-900/40 to-slate-900',
      'from-cyan-900/40 to-slate-900',
      'from-emerald-900/40 to-slate-900',
    ];
    return colors[props.index % colors.length];
  };

  return (
    <div 
      class={`
        relative bg-gradient-to-br ${getRandomColor()} 
        border border-slate-700 rounded-lg overflow-hidden 
        camera-cell cursor-pointer group
      `}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <Show when={props.camera} fallback={
        <div class="absolute inset-0 flex flex-col items-center justify-center text-slate-600">
          <Camera class="w-12 h-12 mb-2" />
          <span class="text-sm">Sem câmera</span>
        </div>
      }>
        {(cam) => (
          <>
            {/* WebRTC Video Stream */}
            <div class="absolute inset-0">
              <WebRTCPlayer
                cameraId={cam().id}
                onError={(err) => console.error('Stream error:', cam().name, err)}
                onConnected={() => console.log('Stream connected:', cam().name)}
              />
            </div>

            {/* Top Info Bar */}
            <div class="absolute top-0 left-0 right-0 p-2 bg-gradient-to-b from-black/70 to-transparent">
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <span class={`w-2 h-2 rounded-full ${cam().status === 'online' ? 'bg-green-500' : 'bg-red-500'}`}></span>
                  <span class="text-white text-sm font-medium truncate">{cam().name}</span>
                </div>
                <Show when={cam().recording}>
                  <div class="flex items-center gap-1 text-red-400 text-xs">
                    <Circle class="w-2 h-2 fill-current animate-pulse" />
                    <span>REC</span>
                  </div>
                </Show>
              </div>
            </div>

            {/* Bottom Info Bar */}
            <div class="absolute bottom-0 left-0 right-0 p-2 bg-gradient-to-t from-black/70 to-transparent">
              <div class="flex items-center justify-between text-xs text-slate-300">
                <span>{cam().resolution} @ {cam().fps}fps</span>
                <span>{cam().codec.toUpperCase()}</span>
              </div>
            </div>

            {/* Hover Controls */}
            <Show when={isHovered()}>
              <div class="absolute inset-0 bg-black/40 flex items-center justify-center gap-3 animate-fadeIn">
                <button class="p-2 bg-white/20 hover:bg-white/30 rounded-full transition-colors" title="Tela cheia">
                  <Expand class="w-5 h-5 text-white" />
                </button>
                <button 
                  class="p-2 bg-white/20 hover:bg-white/30 rounded-full transition-colors"
                  onClick={() => setIsMuted(!isMuted())}
                  title={isMuted() ? 'Ativar áudio' : 'Desativar áudio'}
                >
                  <Show when={isMuted()} fallback={<Volume2 class="w-5 h-5 text-white" />}>
                    <VolumeX class="w-5 h-5 text-white" />
                  </Show>
                </button>
                <button class="p-2 bg-white/20 hover:bg-white/30 rounded-full transition-colors" title="Mais opções">
                  <MoreVertical class="w-5 h-5 text-white" />
                </button>
              </div>
            </Show>
          </>
        )}
      </Show>
    </div>
  );
};

const Live: Component = () => {
  const { state, actions } = useCameras();
  const { loadCameras, setLayout, getCameraById } = actions;

  onMount(() => {
    loadCameras();
  });

  const getGridClasses = () => {
    const layout = state.layout;
    switch (layout) {
      case '1x1': return 'grid-cols-1';
      case '2x2': return 'grid-cols-2';
      case '3x3': return 'grid-cols-3';
      case '4x4': return 'grid-cols-4';
      default: return 'grid-cols-2';
    }
  };

  return (
    <div class="h-full flex flex-col">
      {/* Toolbar */}
      <div class="flex items-center justify-between p-3 bg-slate-800/50 border-b border-slate-700">
        {/* Layout Selector */}
        <div class="flex items-center gap-2">
          <span class="text-sm text-slate-400 mr-2">Layout:</span>
          <div class="flex items-center bg-slate-900 rounded-lg p-1">
            <For each={layoutOptions}>
              {(option) => (
                <button
                  class={`
                    px-3 py-1.5 rounded-md text-sm font-medium transition-all
                    ${state.layout === option.type
                      ? 'bg-vms-primary text-white'
                      : 'text-slate-400 hover:text-white hover:bg-slate-700'
                    }
                  `}
                  onClick={() => setLayout(option.type)}
                >
                  {option.label}
                </button>
              )}
            </For>
          </div>
        </div>

        {/* Right Controls */}
        <div class="flex items-center gap-2">
          <button class="btn btn-ghost flex items-center gap-2 text-sm">
            <Maximize2 class="w-4 h-4" />
            Tela Cheia
          </button>
        </div>
      </div>

      {/* Camera Grid */}
      <div class="flex-1 p-3 overflow-auto">
        <div class={`grid ${getGridClasses()} gap-3 h-full`}>
          <For each={state.gridCells}>
            {(cell, index) => (
              <CameraCell
                camera={cell.cameraId ? getCameraById(cell.cameraId) : undefined}
                index={index()}
              />
            )}
          </For>
        </div>
      </div>
    </div>
  );
};

export default Live;
