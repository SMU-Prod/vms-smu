import { createSignal, createEffect, For, Show, onCleanup, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

// Types
interface Camera {
  id: string;
  name: string;
  rtsp_url: string;
  username: string;
  password: string;
  enabled: boolean;
  resolution_width: number;
  resolution_height: number;
  framerate: number;
}

interface CameraConfig {
  id: string;
  name: string;
  rtsp_url: string;
  username: string;
  password: string;
  width: number;
  height: number;
}

interface PlayerResult {
  success: boolean;
  message: string;
  camera_id: string | null;
}

// API Base URL
const API_URL = "http://localhost:9095/api/v1";

function App() {
  // State
  const [cameras, setCameras] = createSignal<Camera[]>([]);
  const [gridSize, setGridSize] = createSignal<"1x1" | "2x2" | "3x3" | "4x4">("2x2");
  const [activePlayers, setActivePlayers] = createSignal<string[]>([]);
  const [currentTime, setCurrentTime] = createSignal(new Date().toLocaleTimeString());
  const [statusMessage, setStatusMessage] = createSignal("");
  const [gstreamerAvailable, setGstreamerAvailable] = createSignal(false);
  const [useNativePlayer, setUseNativePlayer] = createSignal(true);

  // Load cameras and check GStreamer on mount
  onMount(async () => {
    await loadCameras();
    await checkGStreamer();
  });

  // Update time every second
  createEffect(() => {
    const interval = setInterval(() => {
      setCurrentTime(new Date().toLocaleTimeString());
    }, 1000);
    onCleanup(() => clearInterval(interval));
  });

  // Cleanup players on unmount
  onCleanup(async () => {
    await stopAllPlayers();
  });

  async function loadCameras() {
    try {
      const res = await fetch(`${API_URL}/cameras`);
      if (res.ok) {
        const data = await res.json();
        setCameras(data);
      }
    } catch (e) {
      console.error("Failed to load cameras:", e);
    }
  }

  async function checkGStreamer() {
    try {
      const result = await invoke<PlayerResult>("check_gstreamer");
      setGstreamerAvailable(result.success);
      if (!result.success) {
        showStatus("⚠️ GStreamer não encontrado - usando MJPEG");
        setUseNativePlayer(false);
      } else {
        showStatus("✅ GStreamer disponível - latência ultra-baixa");
      }
    } catch (e) {
      console.error("GStreamer check failed:", e);
      setGstreamerAvailable(false);
      setUseNativePlayer(false);
    }
  }

  function showStatus(msg: string) {
    setStatusMessage(msg);
    setTimeout(() => setStatusMessage(""), 4000);
  }

  async function startPlayer(camera: Camera) {
    if (!useNativePlayer()) {
      showStatus("Modo MJPEG (baixa performance)");
      return;
    }

    const config: CameraConfig = {
      id: camera.id,
      name: camera.name,
      rtsp_url: camera.rtsp_url,
      username: camera.username,
      password: camera.password,
      width: camera.resolution_width,
      height: camera.resolution_height,
    };

    try {
      const result = await invoke<PlayerResult>("start_player", { camera: config });
      if (result.success) {
        setActivePlayers([...activePlayers(), camera.id]);
        showStatus(`▶️ ${camera.name} - Latência: 30-80ms`);
      } else {
        showStatus(`❌ ${result.message}`);
      }
    } catch (e) {
      console.error("Failed to start player:", e);
      showStatus(`❌ Erro ao iniciar player: ${e}`);
    }
  }

  async function stopPlayer(cameraId: string) {
    try {
      const result = await invoke<PlayerResult>("stop_player", { cameraId });
      if (result.success) {
        setActivePlayers(activePlayers().filter((id) => id !== cameraId));
        showStatus(`⏹️ Player parado`);
      }
    } catch (e) {
      console.error("Failed to stop player:", e);
    }
  }

  async function stopAllPlayers() {
    try {
      await invoke<PlayerResult>("stop_all_players");
      setActivePlayers([]);
    } catch (e) {
      console.error("Failed to stop all players:", e);
    }
  }

  function isPlaying(cameraId: string): boolean {
    return activePlayers().includes(cameraId);
  }

  function getMjpegUrl(cameraId: string): string {
    return `${API_URL}/mjpeg/${cameraId}`;
  }

  // Camera Tile Component
  function CameraTile(props: { camera?: Camera; isEmpty?: boolean; index: number }) {
    if (props.isEmpty || !props.camera) {
      return <div class="camera-tile empty" />;
    }

    const camera = props.camera;
    const playing = () => isPlaying(camera.id);

    return (
      <div class="camera-tile">
        {/* Show MJPEG fallback if not using native player or player not started */}
        <Show when={!useNativePlayer()}>
          <img
            src={getMjpegUrl(camera.id)}
            alt={camera.name}
            onError={(e) => {
              (e.target as HTMLImageElement).style.display = "none";
            }}
          />
        </Show>

        {/* Native player placeholder when using GStreamer */}
        <Show when={useNativePlayer()}>
          <div class="native-player-area">
            <Show when={!playing()}>
              <div class="player-prompt">
                <div class="camera-icon">📹</div>
                <div class="camera-title">{camera.name}</div>
                <div class="camera-resolution">
                  {camera.resolution_width}x{camera.resolution_height} @ {camera.framerate}fps
                </div>
                <button
                  class="start-player-btn"
                  onClick={() => startPlayer(camera)}
                >
                  ▶️ Iniciar (30-80ms)
                </button>
              </div>
            </Show>
            <Show when={playing()}>
              <div class="player-running">
                <div class="latency-badge">⚡ 30-80ms</div>
                <div class="player-info">
                  <span class="camera-name">{camera.name}</span>
                  <span class="native-label">GStreamer Nativo</span>
                </div>
                <button
                  class="stop-player-btn"
                  onClick={() => stopPlayer(camera.id)}
                >
                  ⏹️ Parar
                </button>
              </div>
            </Show>
          </div>
        </Show>

        {/* Status Indicator */}
        <div class={`status-indicator ${playing() ? "playing" : ""}`} />

        {/* Camera Info Overlay */}
        <div class="camera-overlay">
          <span class="name">{camera.name}</span>
          <div class="stats">
            <span class="stat">{camera.resolution_width}x{camera.resolution_height}</span>
            <Show when={playing()}>
              <span class="stat latency">⚡30-80ms</span>
            </Show>
          </div>
        </div>
      </div>
    );
  }

  // Get cameras for grid slots
  function getGridCameras(): (Camera | null)[] {
    const count = getGridCount(gridSize());
    const cams = cameras();
    const result: (Camera | null)[] = [];

    for (let i = 0; i < count; i++) {
      result.push(cams[i] || null);
    }

    return result;
  }

  function getGridCount(size: string): number {
    switch (size) {
      case "1x1": return 1;
      case "2x2": return 4;
      case "3x3": return 9;
      case "4x4": return 16;
      default: return 4;
    }
  }

  // Start all visible cameras
  async function startAllPlayers() {
    const visibleCameras = getGridCameras().filter((c) => c !== null) as Camera[];
    for (const camera of visibleCameras) {
      if (!isPlaying(camera.id)) {
        await startPlayer(camera);
      }
    }
  }

  return (
    <div class="viewer-container">
      {/* Status Toast */}
      <Show when={statusMessage()}>
        <div class="toast">{statusMessage()}</div>
      </Show>

      {/* Sidebar */}
      <aside class="sidebar">
        <div class="sidebar-header">
          <div class="icon">SMU</div>
          <span class="title">VMS Viewer</span>
        </div>

        {/* GStreamer Status */}
        <div class={`gstreamer-status ${gstreamerAvailable() ? "available" : "unavailable"}`}>
          <Show when={gstreamerAvailable()} fallback={<span>⚠️ MJPEG Mode</span>}>
            <span>✅ GStreamer NVDEC</span>
          </Show>
        </div>

        {/* Camera List */}
        <div class="camera-list">
          <div class="camera-list-title">Câmeras ({cameras().length})</div>
          <For each={cameras()}>
            {(camera) => (
              <div
                class={`camera-item ${isPlaying(camera.id) ? "playing" : ""}`}
                onClick={() => isPlaying(camera.id) ? stopPlayer(camera.id) : startPlayer(camera)}
              >
                <div class={`status-dot ${camera.enabled ? "online" : "offline"}`} />
                <div class="camera-info">
                  <div class="camera-name">{camera.name}</div>
                  <div class="camera-meta">
                    {camera.resolution_width}x{camera.resolution_height}
                  </div>
                </div>
                <Show when={isPlaying(camera.id)}>
                  <span class="playing-badge">▶️</span>
                </Show>
              </div>
            )}
          </For>
        </div>

        {/* Grid Controls */}
        <div class="grid-controls">
          <button
            class={`grid-btn ${gridSize() === "1x1" ? "active" : ""}`}
            onClick={() => setGridSize("1x1")}
          >
            1×1
          </button>
          <button
            class={`grid-btn ${gridSize() === "2x2" ? "active" : ""}`}
            onClick={() => setGridSize("2x2")}
          >
            2×2
          </button>
          <button
            class={`grid-btn ${gridSize() === "3x3" ? "active" : ""}`}
            onClick={() => setGridSize("3x3")}
          >
            3×3
          </button>
          <button
            class={`grid-btn ${gridSize() === "4x4" ? "active" : ""}`}
            onClick={() => setGridSize("4x4")}
          >
            4×4
          </button>
        </div>

        {/* Player Controls */}
        <div class="player-controls">
          <button class="control-btn start" onClick={startAllPlayers}>
            ▶️ Iniciar Todos
          </button>
          <button class="control-btn stop" onClick={stopAllPlayers}>
            ⏹️ Parar Todos
          </button>
        </div>

        {/* Latency Mode Toggle */}
        <div class="mode-toggle">
          <label>
            <input
              type="checkbox"
              checked={useNativePlayer()}
              onChange={(e) => setUseNativePlayer(e.currentTarget.checked)}
              disabled={!gstreamerAvailable()}
            />
            <span>Modo Ultra Baixa Latência</span>
          </label>
        </div>
      </aside>

      {/* Main Content */}
      <main class="main-content">
        <header class="header">
          <span class="header-title">
            Live View - {gridSize()}
            <Show when={useNativePlayer()}>
              <span class="latency-indicator">⚡ 30-80ms</span>
            </Show>
          </span>
          <span class="header-time">{currentTime()}</span>
        </header>

        {/* Camera Grid */}
        <div class={`camera-grid grid-${gridSize()}`}>
          <For each={getGridCameras()}>
            {(camera, index) => (
              <CameraTile camera={camera || undefined} isEmpty={!camera} index={index()} />
            )}
          </For>
        </div>
      </main>
    </div>
  );
}

export default App;
