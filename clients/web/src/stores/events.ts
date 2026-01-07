import { createSignal, createContext, useContext, ParentComponent } from 'solid-js';
import type { VmsEvent, EventFilter, EventPriority, EventType } from '../types';

interface EventsState {
  events: VmsEvent[];
  unacknowledgedCount: number;
  filter: EventFilter;
  loading: boolean;
  selectedEvent: VmsEvent | null;
}

interface EventsContextValue {
  state: () => EventsState;
  loadEvents: () => Promise<void>;
  acknowledgeEvent: (eventId: string) => void;
  setFilter: (filter: Partial<EventFilter>) => void;
  selectEvent: (event: VmsEvent | null) => void;
  getFilteredEvents: () => VmsEvent[];
}

const EventsContext = createContext<EventsContextValue>();

// Mock events for development
const generateMockEvents = (): VmsEvent[] => {
  const types: EventType[] = ['motion', 'person', 'face_match', 'line_crossing', 'intrusion'];
  const priorities: EventPriority[] = ['critical', 'high', 'medium', 'low', 'info'];
  const cameras = [
    { id: 'cam-001', name: 'Entrada Principal' },
    { id: 'cam-002', name: 'Recepção' },
    { id: 'cam-003', name: 'Corredor Principal' },
    { id: 'cam-004', name: 'Estacionamento' },
  ];

  const events: VmsEvent[] = [];
  const now = new Date();

  for (let i = 0; i < 20; i++) {
    const type = types[Math.floor(Math.random() * types.length)];
    const priority = priorities[Math.floor(Math.random() * priorities.length)];
    const camera = cameras[Math.floor(Math.random() * cameras.length)];
    const timestamp = new Date(now.getTime() - Math.random() * 24 * 60 * 60 * 1000);

    events.push({
      id: `evt-${String(i + 1).padStart(4, '0')}`,
      type,
      priority,
      cameraId: camera.id,
      cameraName: camera.name,
      timestamp,
      description: getEventDescription(type),
      acknowledged: Math.random() > 0.7,
    });
  }

  return events.sort((a, b) => b.timestamp.getTime() - a.timestamp.getTime());
};

const getEventDescription = (type: EventType): string => {
  const descriptions: Record<EventType, string> = {
    motion: 'Movimento detectado na área monitorada',
    person: 'Pessoa detectada',
    vehicle: 'Veículo detectado',
    face_match: 'Face reconhecida - pessoa identificada',
    face_blocklist: 'ALERTA: Face na lista de bloqueio',
    plate_match: 'Placa reconhecida',
    plate_blocklist: 'ALERTA: Placa na lista de bloqueio',
    line_crossing: 'Cruzamento de linha detectado',
    intrusion: 'Intrusão em zona restrita',
    loitering: 'Permanência prolongada detectada',
    camera_offline: 'Câmera ficou offline',
    recording_error: 'Erro na gravação',
    storage_warning: 'Aviso de armazenamento',
    system: 'Evento do sistema',
  };
  return descriptions[type];
};

export const EventsProvider: ParentComponent = (props) => {
  const [state, setState] = createSignal<EventsState>({
    events: [],
    unacknowledgedCount: 0,
    filter: {},
    loading: false,
    selectedEvent: null,
  });

  const loadEvents = async () => {
    setState(s => ({ ...s, loading: true }));
    await new Promise(resolve => setTimeout(resolve, 300));
    const events = generateMockEvents();
    setState(s => ({
      ...s,
      events,
      unacknowledgedCount: events.filter(e => !e.acknowledged).length,
      loading: false,
    }));
  };

  const acknowledgeEvent = (eventId: string) => {
    setState(s => ({
      ...s,
      events: s.events.map(e =>
        e.id === eventId
          ? { ...e, acknowledged: true, acknowledgedAt: new Date(), acknowledgedBy: 'admin' }
          : e
      ),
      unacknowledgedCount: s.events.filter(e => !e.acknowledged && e.id !== eventId).length,
    }));
  };

  const setFilter = (filter: Partial<EventFilter>) => {
    setState(s => ({
      ...s,
      filter: { ...s.filter, ...filter },
    }));
  };

  const selectEvent = (event: VmsEvent | null) => {
    setState(s => ({ ...s, selectedEvent: event }));
  };

  const getFilteredEvents = (): VmsEvent[] => {
    const s = state();
    let filtered = [...s.events];

    if (s.filter.types?.length) {
      filtered = filtered.filter(e => s.filter.types!.includes(e.type));
    }
    if (s.filter.priorities?.length) {
      filtered = filtered.filter(e => s.filter.priorities!.includes(e.priority));
    }
    if (s.filter.cameraIds?.length) {
      filtered = filtered.filter(e => s.filter.cameraIds!.includes(e.cameraId));
    }
    if (s.filter.acknowledged !== undefined) {
      filtered = filtered.filter(e => e.acknowledged === s.filter.acknowledged);
    }

    return filtered;
  };

  const value: EventsContextValue = {
    state,
    loadEvents,
    acknowledgeEvent,
    setFilter,
    selectEvent,
    getFilteredEvents,
  };

  return (
    <EventsContext.Provider value={value}>
      {props.children}
    </EventsContext.Provider>
  );
};

export const useEvents = () => {
  const context = useContext(EventsContext);
  if (!context) {
    throw new Error('useEvents must be used within EventsProvider');
  }
  return context;
};
