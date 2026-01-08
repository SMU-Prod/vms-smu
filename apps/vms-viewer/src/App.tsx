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

// WebRTC connection storage
const webrtcConnections = new Map<string, RTCPeerConnection>();
const videoElements = new Map<string, HTMLVideoElement>();

function App() {
  // State
  const [cameras, setCameras] = createSignal<Camera[]>([]);
  const [gridSize, setGridSize] = createSignal<"1x1" | "2x2" | "3x3" | "4x4">("2x2");
  const [activePlayers, setActivePlayers] = createSignal<string[]>([]);
  const [currentTime, setCurrentTime] = createSignal(new Date().toLocaleTimeString());
  const [statusMessage, setStatusMessage] = createSignal("");
  const [gstreamerAvailable, setGstreamerAvailable] = createSignal(false);
  // WebRTC mode - low latency inline video
  const [useNativePlayer, setUseNativePlayer] = createSignal(false); // MJPEG (funciona)
  const [isLoggedIn, setIsLoggedIn] = createSignal(false);

  // Auto-login and load cameras on mount
  onMount(async () => {
    await autoLogin();
    await loadCameras();
    await checkGStreamer();
  });

  // Auto-login with default credentials
  async function autoLogin() {
    try {
      const result = await invoke<{ success: boolean; message: string }>("api_login", {
        email: "admin@vms.local",
        password: "admin123"
      });
      console.log("Login result:", result);
      setIsLoggedIn(result.success);
    } catch (e) {
      console.log("Tauri login failed, will use HTTP:", e);
      // For HTTP mode, we don't need auth for cameras list
      setIsLoggedIn(true);
    }
  }

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
      // Try Tauri invoke first (works in native app)
      const data = await invoke<Camera[]>("api_list_cameras");
      setCameras(data);
      console.log("Loaded cameras via Tauri:", data.length);

    } catch (e) {
      console.log("Tauri invoke failed, trying HTTP fetch:", e);
      // Fallback to HTTP fetch (works in browser dev mode)
      try {
        const res = await fetch(`${API_URL}/cameras`);
        if (res.ok) {
          const data = await res.json();
          setCameras(data);
          console.log("Loaded cameras via HTTP:", data.length);
        }
      } catch (httpError) {
        console.error("Failed to load cameras:", httpError);
      }
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

  // Start WebRTC connection for low-latency inline video
  async function startWebRTC(camera: Camera, videoEl: HTMLVideoElement) {
    showStatus(`🔗 Conectando WebRTC para ${camera.name}...`);

    try {
      // Create peer connection with STUN server
      const pc = new RTCPeerConnection({
        iceServers: [{ urls: 'stun:stun.l.google.com:19302' }]
      });

      webrtcConnections.set(camera.id, pc);
      videoElements.set(camera.id, videoEl);

      // Handle incoming video stream
      pc.ontrack = (event) => {
        console.log('[WebRTC] Track received:', event.track.kind);
        if (event.streams[0]) {
          videoEl.srcObject = event.streams[0];
          videoEl.play().catch(e => console.error('Autoplay failed:', e));
        }
      };

      // Log ICE connection state
      pc.oniceconnectionstatechange = () => {
        console.log('[WebRTC] ICE state:', pc.iceConnectionState);
        if (pc.iceConnectionState === 'connected') {
          showStatus(`✅ ${camera.name} - WebRTC conectado!`);
        } else if (pc.iceConnectionState === 'failed') {
          showStatus(`❌ ${camera.name} - Conexão falhou`);
        }
      };

      // Collect and send ICE candidates
      pc.onicecandidate = async (event) => {
        if (event.candidate) {
          try {
            await fetch(`${API_URL}/webrtc/ice/${camera.id}`, {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({
                candidate: event.candidate.candidate,
                sdpMid: event.candidate.sdpMid,
                sdpMLineIndex: event.candidate.sdpMLineIndex
              })
            });
          } catch (e) {
            console.error('[WebRTC] ICE candidate error:', e);
          }
        }
      };

      // Add receive-only transceivers for video and audio
      pc.addTransceiver('video', { direction: 'recvonly' });
      pc.addTransceiver('audio', { direction: 'recvonly' });

      // Create and send SDP offer
      const offer = await pc.createOffer();
      await pc.setLocalDescription(offer);

      const response = await fetch(`${API_URL}/webrtc/offer/${camera.id}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          sdp: offer.sdp,
          type: offer.type
        })
      });

      if (!response.ok) {
        throw new Error(`Server error: ${response.status}`);
      }

      const answer = await response.json();
      await pc.setRemoteDescription(new RTCSessionDescription({
        type: answer.type,
        sdp: answer.sdp
      }));

      setActivePlayers([...activePlayers(), camera.id]);
      console.log('[WebRTC] Connection established for', camera.name);

    } catch (e) {
      console.error('[WebRTC] Connection failed:', e);
      showStatus(`❌ WebRTC falhou: ${e}`);
      stopWebRTC(camera.id);
    }
  }

  // Stop WebRTC connection
  function stopWebRTC(cameraId: string) {
    const pc = webrtcConnections.get(cameraId);
    if (pc) {
      pc.close();
      webrtcConnections.delete(cameraId);
    }

    const video = videoElements.get(cameraId);
    if (video) {
      video.srcObject = null;
      videoElements.delete(cameraId);
    }

    setActivePlayers(activePlayers().filter(id => id !== cameraId));
  }

  // Stream URL response type
  interface StreamUrlResponse {
    rtsp_url: string;
    camera_name: string;
    resolution: [number, number];
  }

  async function startPlayer(camera: Camera) {
    if (!useNativePlayer()) {
      showStatus("Modo MJPEG (baixa performance)");
      return;
    }

    try {
      // First fetch authenticated RTSP URL from server
      showStatus(`🔄 Obtendo URL para ${camera.name}...`);
      const streamInfo = await invoke<StreamUrlResponse>("api_get_stream_url", { cameraId: camera.id });

      // Build camera config with authenticated URL
      const config: CameraConfig = {
        id: camera.id,
        name: camera.name,
        rtsp_url: streamInfo.rtsp_url, // Already has credentials embedded
        username: "", // Not needed - URL has credentials
        password: "", // Not needed - URL has credentials
        width: streamInfo.resolution[0] || 1920,
        height: streamInfo.resolution[1] || 1080,
      };

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

  // Camera Tile Component with WebRTC inline video
  function CameraTile(props: { camera?: Camera; isEmpty?: boolean; index: number }) {
    // Each tile has its own video ref - critical for WebRTC to work correctly!
    let tileVideoRef: HTMLVideoElement | null = null;

    if (props.isEmpty || !props.camera) {
      return <div class="camera-tile empty" />;
    }

    const camera = props.camera;
    const playing = () => isPlaying(camera.id);

    // Handle start button click
    const handleStart = () => {
      if (tileVideoRef) {
        console.log('[WebRTC] Starting for', camera.name, 'video element:', tileVideoRef);
        startWebRTC(camera, tileVideoRef);
      } else {
        console.error('[WebRTC] No video element for', camera.name);
      }
    };

    return (
      <div class="camera-tile">
        {/* MJPEG fallback mode */}
        <Show when={!useNativePlayer()}>
          <img
            src={getMjpegUrl(camera.id)}
            alt={camera.name}
            style="width: 100%; height: 100%; object-fit: cover;"
            onError={(e) => {
              (e.target as HTMLImageElement).style.display = "none";
            }}
          />
        </Show>

        {/* WebRTC inline video mode */}
        <Show when={useNativePlayer()}>
          <div class="native-player-area" style="position: relative; width: 100%; height: 100%;">
            {/* Video element for WebRTC stream */}
            <video
              ref={(el) => tileVideoRef = el}
              autoplay
              playsinline
              muted
              style="width: 100%; height: 100%; object-fit: cover; background: #111;"
            />

            {/* Start button overlay when not playing */}
            <Show when={!playing()}>
              <div class="player-prompt" style="position: absolute; inset: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; background: rgba(0,0,0,0.7);">
                <div class="camera-icon">📹</div>
                <div class="camera-title">{camera.name}</div>
                <button
                  class="start-player-btn"
                  onClick={handleStart}
                >
                  ▶️ WebRTC (50-150ms)
                </button>
              </div>
            </Show>

            {/* Overlay when playing */}
            <Show when={playing()}>
              <div style="position: absolute; bottom: 0; left: 0; right: 0; padding: 8px; background: linear-gradient(transparent, rgba(0,0,0,0.8));">
                <div style="display: flex; justify-content: space-between; align-items: center;">
                  <span style="color: #0f0; font-size: 12px;">⚡ WebRTC Live</span>
                  <button
                    style="background: #f44; color: white; border: none; padding: 4px 8px; border-radius: 4px; cursor: pointer;"
                    onClick={() => stopWebRTC(camera.id)}
                  >
                    ⏹️
                  </button>
                </div>
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
