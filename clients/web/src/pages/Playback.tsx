import { Component, createSignal, For, Show, onMount } from 'solid-js';
import { 
  Play, 
  Pause, 
  SkipBack, 
  SkipForward, 
  Rewind, 
  FastForward,
  Download,
  Camera as CameraIcon,
  Calendar,
  Clock,
  ChevronDown
} from 'lucide-solid';
import { useCameras, useEvents } from '../stores';
import type { VmsEvent } from '../types';

const Playback: Component = () => {
  const { state: cameraState, loadCameras } = useCameras();
  const { state: eventsState, loadEvents } = useEvents();
  
  const [selectedCameraId, setSelectedCameraId] = createSignal<string>('cam-001');
  const [selectedDate, setSelectedDate] = createSignal(new Date().toISOString().split('T')[0]);
  const [isPlaying, setIsPlaying] = createSignal(false);
  const [currentTime, setCurrentTime] = createSignal(0); // seconds from midnight
  const [playbackSpeed, setPlaybackSpeed] = createSignal(1);
  const [showCameraDropdown, setShowCameraDropdown] = createSignal(false);

  onMount(() => {
    loadCameras();
    loadEvents();
  });

  const formatTime = (seconds: number): string => {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    return `${String(h).padStart(2, '0')}:${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
  };

  const selectedCamera = () => cameraState().cameras.find(c => c.id === selectedCameraId());

  const timelineEvents = () => eventsState().events.filter(e => e.cameraId === selectedCameraId()).slice(0, 10);

  const speeds = [0.25, 0.5, 1, 2, 4, 8, 16];

  return (
    <div class="h-full flex flex-col">
      {/* Header Controls */}
      <div class="flex items-center gap-4 p-4 bg-slate-800/50 border-b border-slate-700">
        {/* Camera Selector */}
        <div class="relative">
          <button 
            class="flex items-center gap-2 px-4 py-2 bg-slate-700 hover:bg-slate-600 rounded-lg transition-colors min-w-[200px]"
            onClick={() => setShowCameraDropdown(!showCameraDropdown())}
          >
            <CameraIcon class="w-4 h-4 text-slate-400" />
            <span class="flex-1 text-left truncate">{selectedCamera()?.name || 'Selecione'}</span>
            <ChevronDown class="w-4 h-4 text-slate-400" />
          </button>
          
          <Show when={showCameraDropdown()}>
            <div class="absolute top-full left-0 mt-1 w-full bg-slate-800 border border-slate-600 rounded-lg shadow-xl z-20 max-h-60 overflow-auto">
              <For each={cameraState().cameras}>
                {(camera) => (
                  <button
                    class={`
                      w-full px-4 py-2 text-left hover:bg-slate-700 transition-colors flex items-center gap-2
                      ${camera.id === selectedCameraId() ? 'bg-vms-primary/20 text-vms-accent' : 'text-slate-300'}
                    `}
                    onClick={() => {
                      setSelectedCameraId(camera.id);
                      setShowCameraDropdown(false);
                    }}
                  >
                    <span class={`w-2 h-2 rounded-full ${camera.status === 'online' ? 'bg-green-500' : 'bg-red-500'}`}></span>
                    {camera.name}
                  </button>
                )}
              </For>
            </div>
          </Show>
        </div>

        {/* Date Picker */}
        <div class="flex items-center gap-2">
          <Calendar class="w-4 h-4 text-slate-400" />
          <input
            type="date"
            value={selectedDate()}
            onInput={(e) => setSelectedDate(e.currentTarget.value)}
            class="px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white"
          />
        </div>

        {/* Time Jump */}
        <div class="flex items-center gap-2">
          <Clock class="w-4 h-4 text-slate-400" />
          <input
            type="time"
            class="px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white"
            step="1"
          />
        </div>

        <div class="flex-1"></div>

        {/* Export */}
        <button class="btn btn-primary flex items-center gap-2">
          <Download class="w-4 h-4" />
          Exportar Clipe
        </button>
      </div>

      {/* Main Content */}
      <div class="flex-1 flex overflow-hidden">
        {/* Video Player Area */}
        <div class="flex-1 flex flex-col">
          {/* Video Container */}
          <div class="flex-1 p-4">
            <div class="h-full bg-slate-900 rounded-xl border border-slate-700 overflow-hidden relative">
              {/* Placeholder Video */}
              <div class="absolute inset-0 flex items-center justify-center bg-gradient-to-br from-slate-800 to-slate-900">
                <div class="text-center">
                  <CameraIcon class="w-24 h-24 text-slate-700 mx-auto mb-4" />
                  <p class="text-slate-500 text-lg">Reprodução de vídeo</p>
                  <p class="text-slate-600 text-sm mt-1">{selectedCamera()?.name}</p>
                  <p class="text-slate-600 text-sm">{selectedDate()}</p>
                </div>
              </div>

              {/* Current Time Display */}
              <div class="absolute top-4 left-4 px-3 py-1.5 bg-black/70 rounded-lg">
                <span class="text-white font-mono text-lg">{formatTime(currentTime())}</span>
              </div>

              {/* Recording Indicator */}
              <div class="absolute top-4 right-4 flex items-center gap-4">
                <div class="px-2 py-1 bg-black/70 rounded text-xs text-slate-300">
                  {selectedCamera()?.resolution} @ {selectedCamera()?.fps}fps
                </div>
              </div>
            </div>
          </div>

          {/* Player Controls */}
          <div class="px-4 pb-4">
            <div class="bg-slate-800 rounded-xl border border-slate-700 p-4">
              {/* Timeline */}
              <div class="mb-4">
                <div class="relative h-8 bg-slate-900 rounded-lg overflow-hidden">
                  {/* Timeline Background */}
                  <div class="absolute inset-0 flex">
                    <For each={Array.from({ length: 24 })}>
                      {(_, i) => (
                        <div class="flex-1 border-r border-slate-700/50 relative">
                          <span class="absolute bottom-0 left-0 text-[10px] text-slate-600 px-0.5">
                            {String(i()).padStart(2, '0')}
                          </span>
                        </div>
                      )}
                    </For>
                  </div>

                  {/* Recording Bars */}
                  <div class="absolute inset-y-1 left-[10%] right-[60%] bg-green-500/30 rounded"></div>
                  <div class="absolute inset-y-1 left-[50%] right-[20%] bg-green-500/30 rounded"></div>

                  {/* Event Markers */}
                  <For each={timelineEvents()}>
                    {(event) => {
                      const position = Math.random() * 80 + 10; // Random for demo
                      return (
                        <div 
                          class={`absolute top-0 bottom-0 w-0.5 ${
                            event.priority === 'critical' ? 'bg-red-500' :
                            event.priority === 'high' ? 'bg-orange-500' :
                            'bg-yellow-500'
                          }`}
                          style={{ left: `${position}%` }}
                          title={event.description}
                        ></div>
                      );
                    }}
                  </For>

                  {/* Playhead */}
                  <div 
                    class="absolute top-0 bottom-0 w-0.5 bg-vms-accent"
                    style={{ left: `${(currentTime() / 86400) * 100}%` }}
                  >
                    <div class="absolute -top-1 left-1/2 -translate-x-1/2 w-3 h-3 bg-vms-accent rounded-full"></div>
                  </div>

                  {/* Clickable Timeline */}
                  <input
                    type="range"
                    min="0"
                    max="86400"
                    value={currentTime()}
                    onInput={(e) => setCurrentTime(parseInt(e.currentTarget.value))}
                    class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
                  />
                </div>
              </div>

              {/* Controls Row */}
              <div class="flex items-center justify-between">
                {/* Left - Time Display */}
                <div class="flex items-center gap-4 text-sm">
                  <span class="font-mono text-white">{formatTime(currentTime())}</span>
                  <span class="text-slate-500">/ 23:59:59</span>
                </div>

                {/* Center - Playback Controls */}
                <div class="flex items-center gap-2">
                  <button class="p-2 text-slate-400 hover:text-white hover:bg-slate-700 rounded-lg transition-colors">
                    <Rewind class="w-5 h-5" />
                  </button>
                  <button 
                    class="p-2 text-slate-400 hover:text-white hover:bg-slate-700 rounded-lg transition-colors"
                    onClick={() => setCurrentTime(Math.max(0, currentTime() - 1))}
                  >
                    <SkipBack class="w-5 h-5" />
                  </button>
                  <button 
                    class="p-3 bg-vms-primary hover:bg-vms-secondary text-white rounded-full transition-colors"
                    onClick={() => setIsPlaying(!isPlaying())}
                  >
                    <Show when={isPlaying()} fallback={<Play class="w-6 h-6" />}>
                      <Pause class="w-6 h-6" />
                    </Show>
                  </button>
                  <button 
                    class="p-2 text-slate-400 hover:text-white hover:bg-slate-700 rounded-lg transition-colors"
                    onClick={() => setCurrentTime(Math.min(86400, currentTime() + 1))}
                  >
                    <SkipForward class="w-5 h-5" />
                  </button>
                  <button class="p-2 text-slate-400 hover:text-white hover:bg-slate-700 rounded-lg transition-colors">
                    <FastForward class="w-5 h-5" />
                  </button>
                </div>

                {/* Right - Speed Control */}
                <div class="flex items-center gap-2">
                  <span class="text-sm text-slate-400">Velocidade:</span>
                  <select
                    value={playbackSpeed()}
                    onChange={(e) => setPlaybackSpeed(parseFloat(e.currentTarget.value))}
                    class="px-3 py-1.5 bg-slate-700 border border-slate-600 rounded-lg text-white text-sm"
                  >
                    <For each={speeds}>
                      {(speed) => (
                        <option value={speed}>{speed}x</option>
                      )}
                    </For>
                  </select>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Events Sidebar */}
        <div class="w-80 border-l border-slate-700 flex flex-col bg-slate-800/50">
          <div class="p-4 border-b border-slate-700">
            <h3 class="font-semibold text-white">Eventos do Dia</h3>
          </div>
          <div class="flex-1 overflow-auto p-2">
            <Show when={timelineEvents().length > 0} fallback={
              <p class="text-slate-500 text-sm text-center py-8">Nenhum evento encontrado</p>
            }>
              <div class="space-y-2">
                <For each={timelineEvents()}>
                  {(event) => (
                    <button class="w-full p-3 bg-slate-700/50 hover:bg-slate-700 rounded-lg text-left transition-colors">
                      <div class="flex items-start gap-2">
                        <span class={`
                          w-2 h-2 rounded-full mt-1.5 flex-shrink-0
                          ${event.priority === 'critical' ? 'bg-red-500' :
                            event.priority === 'high' ? 'bg-orange-500' :
                            event.priority === 'medium' ? 'bg-yellow-500' :
                            'bg-green-500'}
                        `}></span>
                        <div class="flex-1 min-w-0">
                          <p class="text-sm font-medium text-white truncate">{event.description}</p>
                          <p class="text-xs text-slate-400 mt-0.5">
                            {event.timestamp.toLocaleTimeString('pt-BR')}
                          </p>
                        </div>
                      </div>
                    </button>
                  )}
                </For>
              </div>
            </Show>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Playback;
