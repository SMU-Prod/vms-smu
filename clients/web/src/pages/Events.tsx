import { Component, For, Show, onMount, createSignal } from 'solid-js';
import { Bell, Filter, Check, X, Play, Image, FileText, AlertTriangle, AlertCircle, Info, ChevronDown } from 'lucide-solid';
import { useEvents, useCameras } from '../stores';
import type { VmsEvent, EventPriority } from '../types';

const priorityConfig: Record<EventPriority, { color: string; label: string }> = {
  critical: { color: 'red', label: 'Crítico' },
  high: { color: 'orange', label: 'Alto' },
  medium: { color: 'yellow', label: 'Médio' },
  low: { color: 'green', label: 'Baixo' },
  info: { color: 'blue', label: 'Info' },
};

const Events: Component = () => {
  const { state, loadEvents, acknowledgeEvent, selectEvent, getFilteredEvents } = useEvents();
  const { loadCameras } = useCameras();
  const [filterOpen, setFilterOpen] = createSignal(false);
  const [selectedPriorities, setSelectedPriorities] = createSignal<EventPriority[]>([]);

  onMount(() => { loadEvents(); loadCameras(); });

  const togglePriority = (priority: EventPriority) => {
    const current = selectedPriorities();
    if (current.includes(priority)) {
      setSelectedPriorities(current.filter(p => p !== priority));
    } else {
      setSelectedPriorities([...current, priority]);
    }
  };

  const filteredEvents = () => {
    let events = getFilteredEvents();
    if (selectedPriorities().length > 0) {
      events = events.filter(e => selectedPriorities().includes(e.priority));
    }
    return events;
  };

  return (
    <div class="h-full flex">
      <div class="flex-1 flex flex-col">
        <div class="flex items-center justify-between p-4 bg-slate-800/50 border-b border-slate-700">
          <div class="flex items-center gap-3">
            <Bell class="w-5 h-5 text-vms-accent" />
            <h1 class="text-lg font-semibold text-white">Eventos & Alarmes</h1>
            <span class="px-2 py-0.5 bg-slate-700 rounded-full text-xs text-slate-300">{filteredEvents().length}</span>
          </div>
          <button class={`btn btn-ghost flex items-center gap-2`} onClick={() => setFilterOpen(!filterOpen())}>
            <Filter class="w-4 h-4" /> Filtros
          </button>
        </div>

        <div class="flex-1 overflow-auto">
          <table class="w-full">
            <thead class="bg-slate-800/80 sticky top-0">
              <tr class="text-left text-xs text-slate-400">
                <th class="w-8 px-4 py-3"></th>
                <th class="px-4 py-3">Hora</th>
                <th class="px-4 py-3">Evento</th>
                <th class="px-4 py-3">Câmera</th>
                <th class="px-4 py-3">Prioridade</th>
                <th class="px-4 py-3">Status</th>
                <th class="px-4 py-3 text-right">Ações</th>
              </tr>
            </thead>
            <tbody>
              <For each={filteredEvents()}>
                {(event) => (
                  <tr class="border-b border-slate-700/50 hover:bg-slate-800/50 cursor-pointer" onClick={() => selectEvent(event)}>
                    <td class="px-4 py-3"><span class={`w-3 h-3 rounded-full inline-block bg-${priorityConfig[event.priority].color}-500`}></span></td>
                    <td class="px-4 py-3 text-sm text-slate-300 font-mono">{event.timestamp.toLocaleTimeString('pt-BR')}</td>
                    <td class="px-4 py-3 text-sm text-white truncate max-w-xs">{event.description}</td>
                    <td class="px-4 py-3 text-sm text-slate-400">{event.cameraName}</td>
                    <td class="px-4 py-3"><span class={`px-2 py-0.5 rounded-full text-xs priority-${event.priority}`}>{priorityConfig[event.priority].label}</span></td>
                    <td class="px-4 py-3"><Show when={event.acknowledged} fallback={<span class="text-yellow-400 text-xs">Pendente</span>}><span class="text-green-400 text-xs">✓</span></Show></td>
                    <td class="px-4 py-3 text-right">
                      <Show when={!event.acknowledged}>
                        <button class="p-1 text-green-400 hover:bg-green-500/10 rounded" onClick={(e) => { e.stopPropagation(); acknowledgeEvent(event.id); }}><Check class="w-4 h-4" /></button>
                      </Show>
                    </td>
                  </tr>
                )}
              </For>
            </tbody>
          </table>
        </div>
      </div>

      <Show when={state().selectedEvent}>
        {(event) => (
          <div class="w-80 border-l border-slate-700 bg-slate-800/50 flex flex-col">
            <div class="flex items-center justify-between p-4 border-b border-slate-700">
              <h2 class="font-semibold text-white">Detalhes</h2>
              <button class="p-1 text-slate-400 hover:text-white rounded" onClick={() => selectEvent(null)}><X class="w-5 h-5" /></button>
            </div>
            <div class="flex-1 p-4 space-y-3">
              <div class="aspect-video bg-slate-900 rounded-lg flex items-center justify-center text-slate-600"><Image class="w-10 h-10" /></div>
              <div><label class="text-xs text-slate-500">Evento</label><p class="text-white text-sm">{event().description}</p></div>
              <div class="grid grid-cols-2 gap-3">
                <div><label class="text-xs text-slate-500">Câmera</label><p class="text-white text-sm">{event().cameraName}</p></div>
                <div><label class="text-xs text-slate-500">Prioridade</label><p class={`text-${priorityConfig[event().priority].color}-400 text-sm`}>{priorityConfig[event().priority].label}</p></div>
              </div>
              <div><label class="text-xs text-slate-500">Data/Hora</label><p class="text-white text-sm">{event().timestamp.toLocaleString('pt-BR')}</p></div>
            </div>
            <div class="p-4 border-t border-slate-700 space-y-2">
              <button class="w-full btn btn-primary text-sm"><Play class="w-4 h-4 mr-2" />Ver Vídeo</button>
              <button class="w-full btn btn-secondary text-sm"><FileText class="w-4 h-4 mr-2" />Criar Ocorrência</button>
            </div>
          </div>
        )}
      </Show>
    </div>
  );
};

export default Events;
