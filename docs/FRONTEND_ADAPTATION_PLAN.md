# Plano de Adaptação do Frontend Existente

**Data:** 13/12/2025
**Versão:** 1.0

---

## 📊 ANÁLISE DO ESTADO ATUAL

### ✅ O QUE JÁ ESTÁ PRONTO

1. **Infraestrutura Completa**
   - ✅ SolidJS + Vite + TypeScript
   - ✅ TailwindCSS com design system customizado
   - ✅ API client (api.ts)
   - ✅ WebSocket service (websocket.ts)
   - ✅ Componentes comuns (TopBar, Sidebar, StatusBar, Modal)
   - ✅ Tipos TypeScript completos

2. **Discovery ONVIF** (100% Funcional)
   - ✅ UI completa (CameraDiscovery component)
   - ✅ Integração com backend (vms-api)
   - ✅ Funciona em produção

3. **UI de Páginas** (100% Design)
   - ✅ Live View (grid de câmeras)
   - ✅ Playback (timeline, controles)
   - ✅ Events (tabela, filtros, detalhes)
   - ✅ Evidence (lista, detalhes, custódia)
   - ✅ Config (dashboard, câmeras)

---

## 🔴 PROBLEMAS CRÍTICOS A CORRIGIR

### Problema 1: Live View Quebrado

**Arquivo:** `src/pages/Live.tsx`

**Erro:**
```typescript
// Live.tsx espera:
const { getCameraById } = useCameras();  // ❌ NÃO EXISTE
<For each={cameraState().gridCells}>     // ❌ NÃO EXISTE
```

**Causa:** Store `camera.tsx` não tem `gridCells` nem `getCameraById()`

**Solução:** Adicionar ao store (ver seção de correção abaixo)

---

### Problema 2: Streaming Não Implementado

**Sintoma:** Placeholder de vídeo estático

**Falta:**
- WebRTC player para live view
- HLS/DASH player para playback
- Integração com vms-stream

---

### Problema 3: Mocks Extensivos

**Páginas com dados mock:**
- ⚠️ Events (eventos aleatórios)
- ⚠️ Evidence (3 casos hardcoded)
- ⚠️ Playback (timeline fake)
- ⚠️ Auth (admin/admin hardcoded)
- ⚠️ System Status (valores fixos)

**Necessário:** Integrar APIs reais

---

## 🎯 PLANO DE ADAPTAÇÃO - FASE 1

### ETAPA 1: CORREÇÕES CRÍTICAS (1-2 dias)

#### 1.1 Corrigir Camera Store

**Arquivo:** `src/stores/camera.tsx`

**Mudanças necessárias:**

```typescript
// ADICIONAR ao CameraState
interface CameraState {
  cameras: Camera[];
  selectedCamera: string | null;
  layout: LayoutType;
  loading: boolean;
  error: string | null;
  // NOVO ⬇️
  gridCells: GridCell[];  // Células do grid
}

// ADICIONAR tipos
interface GridCell {
  index: number;
  cameraId: string | null;
}

type LayoutType = '1x1' | '2x2' | '3x3' | '4x4';

const layoutSizeMap: Record<LayoutType, number> = {
  '1x1': 1,
  '2x2': 4,
  '3x3': 9,
  '4x4': 16,
};

// ADICIONAR método computed
const getCameraById = (id: string) => {
  return state.cameras.find(c => c.id === id);
};

// ADICIONAR effect para atualizar gridCells quando layout muda
createEffect(() => {
  const layoutType = state.layout;
  const cellCount = layoutSizeMap[layoutType];

  const cells: GridCell[] = Array.from({ length: cellCount }, (_, i) => ({
    index: i,
    cameraId: state.cameras[i]?.id || null,
  }));

  setState('gridCells', cells);
});

// ADICIONAR método para atribuir câmera a célula
const assignCameraToCell = (cellIndex: number, cameraId: string | null) => {
  setState('gridCells', cellIndex, 'cameraId', cameraId);
};

// EXPORTAR no context
return {
  state,
  loadCameras,
  discoverCameras,
  getCameraProfiles,
  addCamera,
  deleteCamera,
  selectCamera,
  setLayout,
  sendPtz,
  // NOVO ⬇️
  getCameraById,
  assignCameraToCell,
};
```

**Resultado:** Live.tsx funcionará corretamente

---

#### 1.2 Implementar WebRTC Player Básico

**Novo arquivo:** `src/components/camera/WebRTCPlayer.tsx`

```typescript
import { onMount, onCleanup, createSignal } from 'solid-js';

interface Props {
  cameraId: string;
  streamUrl?: string;
  onError?: (error: Error) => void;
}

export default function WebRTCPlayer(props: Props) {
  let videoRef: HTMLVideoElement;
  const [status, setStatus] = createSignal<'connecting' | 'connected' | 'error'>('connecting');
  let peerConnection: RTCPeerConnection;

  onMount(async () => {
    try {
      // 1. Criar PeerConnection
      peerConnection = new RTCPeerConnection({
        iceServers: [{ urls: 'stun:stun.l.google.com:19302' }]
      });

      // 2. Handler de track (recebe stream)
      peerConnection.addEventListener('track', (event) => {
        if (videoRef) {
          videoRef.srcObject = event.streams[0];
          setStatus('connected');
        }
      });

      // 3. Criar offer
      const offer = await peerConnection.createOffer({
        offerToReceiveVideo: true,
        offerToReceiveAudio: true,
      });
      await peerConnection.setLocalDescription(offer);

      // 4. Enviar offer para servidor
      const response = await fetch(`http://localhost:9094/stream`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          camera_id: props.cameraId,
          sdp: offer.sdp,
        }),
      });

      if (!response.ok) throw new Error('Failed to create stream');

      const data = await response.json();
      const { stream_id, answer } = data;

      // 5. Configurar answer
      await peerConnection.setRemoteDescription(
        new RTCSessionDescription({ type: 'answer', sdp: answer })
      );

      console.log('WebRTC connected:', stream_id);

    } catch (err) {
      console.error('WebRTC error:', err);
      setStatus('error');
      props.onError?.(err as Error);
    }
  });

  onCleanup(() => {
    if (peerConnection) {
      peerConnection.close();
    }
  });

  return (
    <video
      ref={videoRef!}
      autoplay
      muted
      playsinline
      class="w-full h-full object-contain bg-black"
    />
  );
}
```

**Uso em Live.tsx:**

```typescript
// Substituir o placeholder por:
import WebRTCPlayer from '../components/camera/WebRTCPlayer';

// No CameraCell component:
<Show when={camera()} fallback={<EmptyCell />}>
  <WebRTCPlayer
    cameraId={camera()!.id}
    onError={(err) => console.error('Stream error:', err)}
  />
</Show>
```

---

### ETAPA 2: BACKEND PLAYBACK (3-5 dias)

#### 2.1 Implementar Timeline API

**Arquivo:** `vms-storage/src/playback/timeline.rs` (criar)

Ver detalhes no PHASE1_IMPLEMENTATION_PLAN.md seção 1.1

#### 2.2 Implementar Streaming de Gravações

**Arquivo:** `vms-storage/src/playback/streamer.rs` (criar)

Ver detalhes no PHASE1_IMPLEMENTATION_PLAN.md seção 1.2

#### 2.3 Adicionar Rotas no vms-storage

**Arquivo:** `vms-storage/src/routes/playback.rs` (criar)

```rust
use axum::{
    routing::{get, post},
    Router,
};

pub fn playback_routes() -> Router {
    Router::new()
        .route("/api/v1/recordings/:camera_id/timeline", get(get_timeline))
        .route("/api/v1/recordings/:camera_id/stream", get(stream_recording))
        .route("/api/v1/recordings/export", post(export_video))
        .route("/api/v1/bookmarks", get(list_bookmarks).post(create_bookmark))
        .route("/api/v1/bookmarks/:id", get(get_bookmark).delete(delete_bookmark))
}
```

**Integrar em main.rs:**

```rust
mod playback;
use playback::playback_routes;

let app = Router::new()
    .merge(playback_routes())
    // ... outras rotas
```

---

### ETAPA 3: ADAPTAR PLAYBACK.TSX (2-3 dias)

#### 3.1 Criar Service de Playback

**Novo arquivo:** `src/services/playback.ts`

```typescript
import { apiClient } from './api';

export interface TimelineSegment {
  start: string;
  end: string;
  file_path: string;
  size_bytes: number;
  has_motion: boolean;
  has_events: string[];
}

export interface MotionZone {
  timestamp: string;
  duration_ms: number;
  confidence: number;
}

export interface TimelineEvent {
  id: string;
  timestamp: string;
  event_type: string;
  priority: string;
}

export interface Bookmark {
  id: string;
  timestamp: string;
  user: string;
  note: string;
}

export interface Timeline {
  camera_id: string;
  start: string;
  end: string;
  resolution: string;
  segments: TimelineSegment[];
  motion_zones: MotionZone[];
  events: TimelineEvent[];
  bookmarks: Bookmark[];
}

export const playbackService = {
  async getTimeline(
    cameraId: string,
    start: Date,
    end: Date,
    resolution: string = '1m'
  ): Promise<Timeline> {
    const params = new URLSearchParams({
      start: start.toISOString(),
      end: end.toISOString(),
      resolution,
    });

    const response = await apiClient.get(
      `/api/v1/recordings/${cameraId}/timeline?${params}`
    );
    return response.data;
  },

  getStreamUrl(cameraId: string, start: Date, end?: Date, speed: number = 1.0): string {
    const params = new URLSearchParams({
      start: start.toISOString(),
      speed: speed.toString(),
    });
    if (end) params.append('end', end.toISOString());

    return `${import.meta.env.VITE_API_URL}/api/v1/recordings/${cameraId}/stream?${params}`;
  },

  async exportVideo(params: {
    camera_id: string;
    start: Date;
    end: Date;
    format: 'mp4' | 'avi';
    include_watermark: boolean;
  }): Promise<{ export_id: string }> {
    const response = await apiClient.post('/api/v1/recordings/export', {
      ...params,
      start: params.start.toISOString(),
      end: params.end.toISOString(),
    });
    return response.data;
  },

  async createBookmark(params: {
    camera_id: string;
    timestamp: Date;
    note: string;
  }): Promise<Bookmark> {
    const response = await apiClient.post('/api/v1/bookmarks', {
      ...params,
      timestamp: params.timestamp.toISOString(),
    });
    return response.data;
  },
};
```

---

#### 3.2 Criar Store de Playback

**Novo arquivo:** `src/stores/playback.tsx`

```typescript
import { createContext, useContext, ParentComponent } from 'solid-js';
import { createStore } from 'solid-js/store';
import { playbackService, Timeline } from '../services/playback';

interface PlaybackState {
  timeline: Timeline | null;
  currentTime: Date | null;
  isPlaying: boolean;
  speed: number;
  loading: boolean;
  error: string | null;
}

const PlaybackContext = createContext<{
  state: PlaybackState;
  loadTimeline: (cameraId: string, start: Date, end: Date) => Promise<void>;
  play: () => void;
  pause: () => void;
  seek: (time: Date) => void;
  setSpeed: (speed: number) => void;
  exportClip: (start: Date, end: Date) => Promise<void>;
  createBookmark: (note: string) => Promise<void>;
}>();

export const PlaybackProvider: ParentComponent = (props) => {
  const [state, setState] = createStore<PlaybackState>({
    timeline: null,
    currentTime: null,
    isPlaying: false,
    speed: 1.0,
    loading: false,
    error: null,
  });

  const loadTimeline = async (cameraId: string, start: Date, end: Date) => {
    setState({ loading: true, error: null });
    try {
      const timeline = await playbackService.getTimeline(cameraId, start, end);
      setState({ timeline, loading: false });
    } catch (err) {
      setState({ error: (err as Error).message, loading: false });
    }
  };

  const play = () => setState({ isPlaying: true });
  const pause = () => setState({ isPlaying: false });
  const seek = (time: Date) => setState({ currentTime: time });
  const setSpeed = (speed: number) => setState({ speed });

  const exportClip = async (start: Date, end: Date) => {
    if (!state.timeline) return;
    try {
      const { export_id } = await playbackService.exportVideo({
        camera_id: state.timeline.camera_id,
        start,
        end,
        format: 'mp4',
        include_watermark: true,
      });
      console.log('Export started:', export_id);
      // TODO: mostrar progress
    } catch (err) {
      console.error('Export failed:', err);
    }
  };

  const createBookmark = async (note: string) => {
    if (!state.timeline || !state.currentTime) return;
    try {
      await playbackService.createBookmark({
        camera_id: state.timeline.camera_id,
        timestamp: state.currentTime,
        note,
      });
      // TODO: atualizar timeline
    } catch (err) {
      console.error('Bookmark failed:', err);
    }
  };

  const value = {
    state,
    loadTimeline,
    play,
    pause,
    seek,
    setSpeed,
    exportClip,
    createBookmark,
  };

  return (
    <PlaybackContext.Provider value={value}>
      {props.children}
    </PlaybackContext.Provider>
  );
};

export const usePlayback = () => {
  const context = useContext(PlaybackContext);
  if (!context) throw new Error('usePlayback must be used within PlaybackProvider');
  return context;
};
```

---

#### 3.3 Adaptar Playback.tsx

**Arquivo:** `src/pages/Playback.tsx`

**Mudanças:**

1. Adicionar provider:

```typescript
// Em App.tsx ou index.tsx:
import { PlaybackProvider } from './stores/playback';

<PlaybackProvider>
  <Router>...</Router>
</PlaybackProvider>
```

2. Substituir mock por API real:

```typescript
import { usePlayback } from '../stores/playback';

export default function PlaybackPage() {
  const { state, loadTimeline, play, pause, seek, setSpeed, exportClip } = usePlayback();

  onMount(() => {
    // Carregar timeline para câmera e data selecionada
    const start = new Date(selectedDate);
    start.setHours(0, 0, 0, 0);
    const end = new Date(selectedDate);
    end.setHours(23, 59, 59, 999);

    loadTimeline(selectedCamera()!, start, end);
  });

  // Substituir eventos mock por state.timeline?.events
  const events = () => state.timeline?.events || [];

  // Renderizar timeline real
  <For each={state.timeline?.segments}>
    {(segment) => (
      <div class="recording-segment" />
    )}
  </For>

  // Player com HLS
  <video src={playbackService.getStreamUrl(selectedCamera()!, currentTime())} />
}
```

---

### ETAPA 4: INTEGRAR EVENTS COM API REAL (1-2 dias)

#### 4.1 Atualizar Events Store

**Arquivo:** `src/stores/events.tsx`

**Substituir mock por WebSocket:**

```typescript
import { websocketService } from '../services/websocket';

export const EventsProvider: ParentComponent = (props) => {
  const [state, setState] = createStore<EventsState>({
    events: [],
    // ...
  });

  onMount(() => {
    // Conectar WebSocket
    websocketService.connect();

    // Escutar eventos
    websocketService.on('event.motion', handleEvent);
    websocketService.on('event.person', handleEvent);
    websocketService.on('event.vehicle', handleEvent);
    websocketService.on('event.alarm', handleEvent);

    // Carregar eventos históricos via API
    loadEventsFromAPI();
  });

  const handleEvent = (event: VmsEvent) => {
    setState('events', (events) => [event, ...events]);
    setState('unacknowledgedCount', (c) => c + 1);
  };

  const loadEventsFromAPI = async () => {
    try {
      const response = await apiClient.get('/api/v1/events');
      setState({ events: response.data, loading: false });
    } catch (err) {
      setState({ error: (err as Error).message, loading: false });
    }
  };

  const acknowledgeEvent = async (eventId: string) => {
    try {
      await apiClient.post(`/api/v1/alarms/${eventId}/acknowledge`);
      setState('events', (e) => e.id === eventId, 'acknowledged', true);
      setState('unacknowledgedCount', (c) => Math.max(0, c - 1));
    } catch (err) {
      console.error('Acknowledge failed:', err);
    }
  };

  // ...
};
```

**Resultado:** Events.tsx mostrará eventos reais do vms-events

---

### ETAPA 5: MULTI-STREAMING (3-4 dias)

Ver PHASE1_IMPLEMENTATION_PLAN.md seção 2 para implementação completa.

**Resumo:**
- Backend: vms-ingest com múltiplos perfis
- Frontend: Seletor de qualidade no player

---

## 📊 CRONOGRAMA DE ADAPTAÇÃO

| Semana | Etapa | Componente | Status |
|--------|-------|------------|--------|
| 1 | 1.1 | Corrigir camera store | ⏳ Prioritário |
| 1 | 1.2 | WebRTC Player | ⏳ Prioritário |
| 2 | 2.1-2.3 | Backend Playback | ⏳ Backend |
| 3 | 3.1-3.3 | Adaptar Playback.tsx | ⏳ Frontend |
| 4 | 4.1 | Integrar Events | ⏳ WebSocket |
| 5 | 5 | Multi-Streaming | ⏳ Backend |

---

## ✅ CHECKLIST DE ADAPTAÇÃO

### Imediato (Semana 1)
- [ ] Corrigir `src/stores/camera.tsx` (adicionar gridCells, getCameraById)
- [ ] Criar `src/components/camera/WebRTCPlayer.tsx`
- [ ] Testar Live View com câmeras reais
- [ ] Deletar `src/stores/config.ts` (arquivo vazio)

### Backend (Semanas 2-3)
- [ ] Criar `vms-storage/src/playback/timeline.rs`
- [ ] Criar `vms-storage/src/playback/streamer.rs`
- [ ] Criar `vms-storage/src/playback/export.rs`
- [ ] Criar `vms-storage/src/playback/bookmark.rs`
- [ ] Adicionar rotas em `vms-storage/src/routes/playback.rs`
- [ ] Testar APIs com Postman/curl

### Frontend (Semana 3)
- [ ] Criar `src/services/playback.ts`
- [ ] Criar `src/stores/playback.tsx`
- [ ] Adaptar `src/pages/Playback.tsx` (remover mock)
- [ ] Criar componente HLS/DASH player
- [ ] Testar playback completo

### Integração (Semana 4)
- [ ] Atualizar `src/stores/events.tsx` (WebSocket)
- [ ] Atualizar `src/stores/auth.tsx` (API real)
- [ ] Atualizar `src/stores/config.tsx` (API real para status)
- [ ] Testar fluxo completo

### Multi-Streaming (Semana 5)
- [ ] Implementar perfis em vms-ingest
- [ ] Adicionar seletor de qualidade no player
- [ ] Testar troca de perfil dinâmica

---

## 🎯 RESULTADO ESPERADO

Ao final da adaptação, teremos:

✅ **Live View funcional** com WebRTC
✅ **Playback completo** com timeline real
✅ **Events em tempo real** via WebSocket
✅ **Exportação de vídeo** funcionando
✅ **Bookmarks** salvos no backend
✅ **Multi-streaming** com perfis de qualidade

**Sem criar código duplicado** - apenas adaptando o que já existe!

---

## 📝 NOTAS IMPORTANTES

1. **NÃO criar páginas novas** - todas já existem
2. **NÃO reescrever componentes** - apenas adicionar funcionalidade
3. **Manter design atual** - UI está ótima
4. **Focar em integração** - conectar frontend mock com backend real
5. **Testar incrementalmente** - uma feature por vez

---

**Próximo passo:** Começar pela correção crítica do camera store!
