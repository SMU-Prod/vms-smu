import { createSignal, createEffect, Show, For } from "solid-js";
import "./App.css";

// Types
interface User {
  id: string;
  username: string;
  name: string;
  email: string | null;
  role: "admin" | "operator" | "viewer";
  enabled: boolean;
}

interface LoginResponse {
  token: string;
  user: User;
  expires_at: string;
}

interface Camera {
  id: string;
  name: string;
  rtsp_url: string;
  enabled: boolean;
  resolution_width: number;
  resolution_height: number;
}

interface Server {
  id: string;
  name: string;
  ip: string;
  port: number;
  username: string;
  status: "online" | "offline" | "error";
  enabled: boolean;
  webrtc_url: string;
  created_at: string;
}

// Icons as SVG components
const Icons = {
  Dashboard: () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <rect x="3" y="3" width="7" height="7" rx="1" />
      <rect x="14" y="3" width="7" height="7" rx="1" />
      <rect x="3" y="14" width="7" height="7" rx="1" />
      <rect x="14" y="14" width="7" height="7" rx="1" />
    </svg>
  ),
  Camera: () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M23 7l-7 5 7 5V7z" />
      <rect x="1" y="5" width="15" height="14" rx="2" ry="2" />
    </svg>
  ),
  Users: () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
      <circle cx="9" cy="7" r="4" />
      <path d="M23 21v-2a4 4 0 0 0-3-3.87" />
      <path d="M16 3.13a4 4 0 0 1 0 7.75" />
    </svg>
  ),
  Server: () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <rect x="2" y="2" width="20" height="8" rx="2" ry="2" />
      <rect x="2" y="14" width="20" height="8" rx="2" ry="2" />
      <line x1="6" y1="6" x2="6.01" y2="6" />
      <line x1="6" y1="18" x2="6.01" y2="18" />
    </svg>
  ),
  Settings: () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <circle cx="12" cy="12" r="3" />
      <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
    </svg>
  ),
  Logout: () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
      <polyline points="16 17 21 12 16 7" />
      <line x1="21" y1="12" x2="9" y2="12" />
    </svg>
  ),
};

// API Base URL
const API_URL = "http://localhost:9095/api/v1";

function App() {
  // Auth state
  const [token, setToken] = createSignal<string | null>(localStorage.getItem("token"));
  const [user, setUser] = createSignal<User | null>(null);

  // UI state
  const [currentPage, setCurrentPage] = createSignal("dashboard");
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal("");

  // Data state
  const [cameras, setCameras] = createSignal<Camera[]>([]);
  const [users, setUsers] = createSignal<User[]>([]);
  const [servers, setServers] = createSignal<Server[]>([]);
  const [showServerModal, setShowServerModal] = createSignal(false);
  const [showCameraModal, setShowCameraModal] = createSignal(false);
  const [showUserModal, setShowUserModal] = createSignal(false);

  createEffect(() => {
    if (token()) {
      loadCameras();
      loadUsers();
      loadServers();
    }
  });

  // API calls
  async function login(username: string, password: string) {
    setLoading(true);
    setError("");
    try {
      const res = await fetch(`${API_URL}/auth/login`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username, password }),
      });

      if (!res.ok) {
        const data = await res.json();
        throw new Error(data.error || "Login failed");
      }

      const data: LoginResponse = await res.json();
      setToken(data.token);
      setUser(data.user);
      localStorage.setItem("token", data.token);
    } catch (e: any) {
      setError(e.message);
    } finally {
      setLoading(false);
    }
  }

  function logout() {
    setToken(null);
    setUser(null);
    localStorage.removeItem("token");
  }

  async function loadCameras() {
    try {
      const res = await fetch(`${API_URL}/cameras`, {
        headers: { Authorization: `Bearer ${token()}` },
      });
      if (res.ok) {
        setCameras(await res.json());
      }
    } catch (e) {
      console.error("Failed to load cameras:", e);
    }
  }

  async function loadUsers() {
    try {
      const res = await fetch(`${API_URL}/users`, {
        headers: { Authorization: `Bearer ${token()}` },
      });
      if (res.ok) {
        setUsers(await res.json());
      }
    } catch (e) {
      console.error("Failed to load users:", e);
    }
  }

  async function loadServers() {
    try {
      const res = await fetch(`${API_URL}/servers`, {
        headers: { Authorization: `Bearer ${token()}` },
      });
      if (res.ok) {
        setServers(await res.json());
      }
    } catch (e) {
      console.error("Failed to load servers:", e);
    }
  }

  async function createServer(data: { name: string; ip: string; port: number; username: string; password: string }) {
    try {
      const res = await fetch(`${API_URL}/servers`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token()}`,
        },
        body: JSON.stringify(data),
      });
      if (res.ok) {
        loadServers();
        setShowServerModal(false);
      }
    } catch (e) {
      console.error("Failed to create server:", e);
    }
  }

  async function deleteServer(id: string) {
    if (!confirm("Tem certeza que deseja excluir este servidor?")) return;
    try {
      const res = await fetch(`${API_URL}/servers/${id}`, {
        method: "DELETE",
        headers: { Authorization: `Bearer ${token()}` },
      });
      if (res.ok) {
        loadServers();
      }
    } catch (e) {
      console.error("Failed to delete server:", e);
    }
  }

  // Camera CRUD - Professional VMS
  interface CameraFormData {
    // Geral
    name: string;
    description?: string;
    manufacturer?: string;
    model?: string;
    firmware?: string;
    // Streaming
    ip_address: string;
    rtsp_port: number;
    onvif_port?: number;
    username: string;
    password: string;
    transport: string;
    timeout_ms: number;
    // Gravação
    recording_mode: string;
    recording_dir?: string;
    retention_days: number;
    // Localização
    shortcut?: string;
    latitude?: number;
    longitude?: number;
    // Vinculação
    server_id?: string;
  }

  async function createCamera(data: CameraFormData) {
    try {
      const res = await fetch(`${API_URL}/cameras`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token()}`,
        },
        body: JSON.stringify(data),
      });
      if (res.ok) {
        loadCameras();
        setShowCameraModal(false);
      } else {
        const err = await res.json();
        console.error("Camera creation failed:", err);
      }
    } catch (e) {
      console.error("Failed to create camera:", e);
    }
  }

  async function deleteCamera(id: string) {
    if (!confirm("Tem certeza que deseja excluir esta câmera?")) return;
    try {
      const res = await fetch(`${API_URL}/cameras/${id}`, {
        method: "DELETE",
        headers: { Authorization: `Bearer ${token()}` },
      });
      if (res.ok) {
        loadCameras();
      }
    } catch (e) {
      console.error("Failed to delete camera:", e);
    }
  }

  // User CRUD
  async function createUser(data: { username: string; password: string; name: string; role: string }) {
    try {
      const res = await fetch(`${API_URL}/users`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token()}`,
        },
        body: JSON.stringify(data),
      });
      if (res.ok) {
        loadUsers();
        setShowUserModal(false);
      }
    } catch (e) {
      console.error("Failed to create user:", e);
    }
  }

  async function deleteUser(id: string) {
    if (!confirm("Tem certeza que deseja excluir este usuário?")) return;
    try {
      const res = await fetch(`${API_URL}/users/${id}`, {
        method: "DELETE",
        headers: { Authorization: `Bearer ${token()}` },
      });
      if (res.ok) {
        loadUsers();
      }
    } catch (e) {
      console.error("Failed to delete user:", e);
    }
  }

  // Login Screen
  function LoginScreen() {
    const [username, setUsername] = createSignal("");
    const [password, setPassword] = createSignal("");

    const handleSubmit = (e: Event) => {
      e.preventDefault();
      login(username(), password());
    };

    return (
      <div class="login-container">
        <div class="login-card">
          <div class="login-logo">
            <div class="icon">SMU</div>
            <h1>VMS Admin</h1>
            <p>Enterprise Video Management</p>
          </div>

          <Show when={error()}>
            <div class="login-error">{error()}</div>
          </Show>

          <form onSubmit={handleSubmit}>
            <div class="form-group">
              <label>Username</label>
              <input
                type="text"
                class="input"
                placeholder="Enter username"
                value={username()}
                onInput={(e) => setUsername(e.currentTarget.value)}
                required
              />
            </div>

            <div class="form-group">
              <label>Password</label>
              <input
                type="password"
                class="input"
                placeholder="Enter password"
                value={password()}
                onInput={(e) => setPassword(e.currentTarget.value)}
                required
              />
            </div>

            <button type="submit" class="btn btn-primary login-btn" disabled={loading()}>
              <Show when={loading()} fallback="Sign In">
                <div class="spinner" />
              </Show>
            </button>
          </form>
        </div>
      </div>
    );
  }

  // Dashboard
  function Dashboard() {
    return (
      <div>
        <div class="grid grid-4" style="margin-bottom: 24px">
          <div class="stat-card">
            <div class="icon blue">📹</div>
            <div class="stat-value">{cameras().length}</div>
            <div class="stat-label">Total Cameras</div>
          </div>
          <div class="stat-card">
            <div class="icon green">✓</div>
            <div class="stat-value">{cameras().filter(c => c.enabled).length}</div>
            <div class="stat-label">Online</div>
          </div>
          <div class="stat-card">
            <div class="icon yellow">👥</div>
            <div class="stat-value">{users().length}</div>
            <div class="stat-label">Users</div>
          </div>
          <div class="stat-card">
            <div class="icon blue">💾</div>
            <div class="stat-value">--</div>
            <div class="stat-label">Storage Used</div>
          </div>
        </div>

        <div class="card">
          <div class="card-header">
            <h3 class="card-title">Recent Cameras</h3>
            <button class="btn btn-secondary" onClick={() => setCurrentPage("cameras")}>
              View All
            </button>
          </div>
          <table class="table">
            <thead>
              <tr>
                <th>Name</th>
                <th>Resolution</th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              <For each={cameras().slice(0, 5)}>
                {(camera) => (
                  <tr>
                    <td>{camera.name}</td>
                    <td>{camera.resolution_width}x{camera.resolution_height}</td>
                    <td>
                      <span class={`status ${camera.enabled ? "status-online" : "status-offline"}`}>
                        {camera.enabled ? "Online" : "Offline"}
                      </span>
                    </td>
                  </tr>
                )}
              </For>
              <Show when={cameras().length === 0}>
                <tr>
                  <td colspan="3" style="text-align: center; color: var(--text-muted)">
                    No cameras configured
                  </td>
                </tr>
              </Show>
            </tbody>
          </table>
        </div>
      </div>
    );
  }

  // Cameras Page - Professional VMS
  function CamerasPage() {
    // Tab state
    const [activeTab, setActiveTab] = createSignal("camera");

    // Form fields - Geral
    const [camName, setCamName] = createSignal("");
    const [camDesc, setCamDesc] = createSignal("");
    const [camManufacturer, setCamManufacturer] = createSignal("");
    const [camModel, setCamModel] = createSignal("");
    const [camFirmware, setCamFirmware] = createSignal("");

    // Streaming
    const [camIp, setCamIp] = createSignal("");
    const [camRtspPort, setCamRtspPort] = createSignal("554");
    const [camOnvifPort, setCamOnvifPort] = createSignal("80");
    const [camUser, setCamUser] = createSignal("");
    const [camPass, setCamPass] = createSignal("");
    const [camTransport, setCamTransport] = createSignal("auto");
    const [camTimeout, setCamTimeout] = createSignal("30000");

    // Gravação
    const [camRecMode, setCamRecMode] = createSignal("disabled");
    const [camRecDir, setCamRecDir] = createSignal("");
    const [camRetention, setCamRetention] = createSignal("30");

    // Localização
    const [camShortcut, setCamShortcut] = createSignal("");
    const [camLat, setCamLat] = createSignal("");
    const [camLng, setCamLng] = createSignal("");

    // Vinculação
    const [camServerId, setCamServerId] = createSignal("");

    const resetForm = () => {
      setCamName(""); setCamDesc(""); setCamManufacturer(""); setCamModel("");
      setCamFirmware(""); setCamIp(""); setCamRtspPort("554"); setCamOnvifPort("80");
      setCamUser(""); setCamPass(""); setCamTransport("auto"); setCamTimeout("30000");
      setCamRecMode("disabled"); setCamRecDir(""); setCamRetention("30");
      setCamShortcut(""); setCamLat(""); setCamLng(""); setCamServerId("");
      setActiveTab("camera");
      setConnectionStatus(null);
      setPreviewUrl(null);
    };

    // Connection test state
    const [connectionStatus, setConnectionStatus] = createSignal<{ success: boolean; message: string } | null>(null);
    const [previewUrl, setPreviewUrl] = createSignal<string | null>(null);
    const [testingConnection, setTestingConnection] = createSignal(false);

    const testConnection = async () => {
      if (!camIp() || !camUser() || !camPass()) {
        setConnectionStatus({ success: false, message: "Preencha IP, Usuário e Senha primeiro" });
        return;
      }
      setTestingConnection(true);
      setConnectionStatus(null);
      setPreviewUrl(null);
      try {
        const res = await fetch(`${API_URL}/cameras/test`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token()}`,
          },
          body: JSON.stringify({
            ip_address: camIp(),
            rtsp_port: parseInt(camRtspPort()) || 554,
            username: camUser(),
            password: camPass(),
          }),
        });
        const data = await res.json();
        setConnectionStatus({ success: data.success, message: data.message });
        if (data.success && data.preview_url) {
          // Build full preview URL using API base
          setPreviewUrl(`http://localhost:9095${data.preview_url}`);
        }
      } catch (e: any) {
        setConnectionStatus({ success: false, message: `Erro: ${e.message}` });
      } finally {
        setTestingConnection(false);
      }
    };

    const handleCreate = (e: Event) => {
      e.preventDefault();
      createCamera({
        name: camName(),
        description: camDesc() || undefined,
        manufacturer: camManufacturer() || undefined,
        model: camModel() || undefined,
        firmware: camFirmware() || undefined,
        ip_address: camIp(),
        rtsp_port: parseInt(camRtspPort()) || 554,
        onvif_port: camOnvifPort() ? parseInt(camOnvifPort()) : undefined,
        username: camUser(),
        password: camPass(),
        transport: camTransport(),
        timeout_ms: parseInt(camTimeout()) || 30000,
        recording_mode: camRecMode(),
        recording_dir: camRecDir() || undefined,
        retention_days: parseInt(camRetention()) || 30,
        shortcut: camShortcut() || undefined,
        latitude: camLat() ? parseFloat(camLat()) : undefined,
        longitude: camLng() ? parseFloat(camLng()) : undefined,
        server_id: camServerId() || undefined,
      });
      resetForm();
    };

    const tabs = [
      { id: "camera", label: "📹 Câmera", icon: "📹" },
      { id: "streaming", label: "🎥 Streaming", icon: "🎥" },
      { id: "recording", label: "💾 Gravação", icon: "💾" },
      { id: "location", label: "🗺️ Localização", icon: "🗺️" },
      { id: "server", label: "🔗 Servidor", icon: "🔗" },
    ];

    return (
      <>
        <div class="card">
          <div class="card-header">
            <h3 class="card-title">Câmeras</h3>
            <button class="btn btn-primary" onClick={() => setShowCameraModal(true)}>
              + Adicionar Câmera
            </button>
          </div>
          <table class="table">
            <thead>
              <tr>
                <th>Nome</th>
                <th>IP</th>
                <th>Fabricante</th>
                <th>Status</th>
                <th>Servidor</th>
                <th>Ações</th>
              </tr>
            </thead>
            <tbody>
              <For each={cameras()}>
                {(camera: any) => (
                  <tr>
                    <td>{camera.name}</td>
                    <td style="font-family: monospace">{camera.ip_address || camera.rtsp_url?.split("@")[1]?.split(":")[0] || "-"}</td>
                    <td>{camera.manufacturer || "-"}</td>
                    <td>
                      <span class={`status ${camera.enabled ? "status-online" : "status-offline"}`}>
                        {camera.enabled ? "Ativa" : "Inativa"}
                      </span>
                    </td>
                    <td>{camera.server_id ? "Vinculada" : "Sem servidor"}</td>
                    <td>
                      <button class="btn btn-danger" style="padding: 6px 12px" onClick={() => deleteCamera(camera.id)}>
                        🗑️
                      </button>
                    </td>
                  </tr>
                )}
              </For>
            </tbody>
          </table>
        </div>

        {/* Camera Modal - Professional Tabs */}
        <Show when={showCameraModal()}>
          <div class="modal-overlay" onClick={() => { setShowCameraModal(false); resetForm(); }}>
            <div class="modal-content modal-large" onClick={(e) => e.stopPropagation()} style="max-width: 800px; display: flex; flex-direction: row; padding: 0;">
              {/* Sidebar Tabs */}
              <div style="width: 180px; background: var(--bg-secondary); border-right: 1px solid var(--border); padding: 16px 0;">
                <h3 style="padding: 0 16px; margin-bottom: 16px; font-size: 14px; color: var(--text-muted);">Configurações</h3>
                <For each={tabs}>
                  {(tab) => (
                    <div
                      onClick={() => setActiveTab(tab.id)}
                      style={`padding: 12px 16px; cursor: pointer; transition: all 0.2s; ${activeTab() === tab.id ? "background: var(--accent-color); color: white;" : ""}`}
                    >
                      {tab.label}
                    </div>
                  )}
                </For>
              </div>

              {/* Form Content */}
              <div style="flex: 1; padding: 24px;">
                <div class="modal-header" style="margin-bottom: 20px;">
                  <h2>Nova Câmera</h2>
                  <button class="modal-close" onClick={() => { setShowCameraModal(false); resetForm(); }}>×</button>
                </div>

                <form onSubmit={handleCreate}>
                  {/* Tab: Camera */}
                  <Show when={activeTab() === "camera"}>
                    <div class="form-group">
                      <label>Nome da Câmera *</label>
                      <input type="text" value={camName()} onInput={(e) => setCamName(e.currentTarget.value)} placeholder="Entrada Principal" required />
                    </div>
                    <div class="form-group">
                      <label>Descrição</label>
                      <textarea value={camDesc()} onInput={(e) => setCamDesc(e.currentTarget.value)} placeholder="Observações gerais sobre a câmera" rows={3} style="width: 100%; resize: vertical;" />
                    </div>
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px;">
                      <div class="form-group">
                        <label>Fabricante</label>
                        <select value={camManufacturer()} onChange={(e) => setCamManufacturer(e.currentTarget.value)}>
                          <option value="">Selecione...</option>
                          <option value="Hikvision">Hikvision</option>
                          <option value="Dahua">Dahua</option>
                          <option value="Intelbras">Intelbras</option>
                          <option value="Axis">Axis</option>
                          <option value="TP-Link">TP-Link (Tapo)</option>
                          <option value="Outro">Outro</option>
                        </select>
                      </div>
                      <div class="form-group">
                        <label>Modelo</label>
                        <input type="text" value={camModel()} onInput={(e) => setCamModel(e.currentTarget.value)} placeholder="Ex: DS-2CD2143G2" />
                      </div>
                    </div>
                    <div class="form-group">
                      <label>Firmware</label>
                      <input type="text" value={camFirmware()} onInput={(e) => setCamFirmware(e.currentTarget.value)} placeholder="Ex: V5.6.0" />
                    </div>
                  </Show>

                  {/* Tab: Streaming */}
                  <Show when={activeTab() === "streaming"}>
                    <div style="display: grid; grid-template-columns: 2fr 1fr; gap: 12px;">
                      <div class="form-group">
                        <label>Endereço IP *</label>
                        <input type="text" value={camIp()} onInput={(e) => setCamIp(e.currentTarget.value)} placeholder="192.168.1.100" required />
                      </div>
                      <div class="form-group">
                        <label>Porta RTSP</label>
                        <input type="number" value={camRtspPort()} onInput={(e) => setCamRtspPort(e.currentTarget.value)} placeholder="554" />
                      </div>
                    </div>
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px;">
                      <div class="form-group">
                        <label>Usuário *</label>
                        <input type="text" value={camUser()} onInput={(e) => setCamUser(e.currentTarget.value)} placeholder="admin" required />
                      </div>
                      <div class="form-group">
                        <label>Senha *</label>
                        <input type="password" value={camPass()} onInput={(e) => setCamPass(e.currentTarget.value)} placeholder="••••••" required />
                      </div>
                    </div>
                    <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px;">
                      <div class="form-group">
                        <label>Porta ONVIF</label>
                        <input type="number" value={camOnvifPort()} onInput={(e) => setCamOnvifPort(e.currentTarget.value)} placeholder="80" />
                      </div>
                      <div class="form-group">
                        <label>Transporte</label>
                        <select value={camTransport()} onChange={(e) => setCamTransport(e.currentTarget.value)}>
                          <option value="auto">Auto</option>
                          <option value="tcp">TCP</option>
                          <option value="udp">UDP</option>
                        </select>
                      </div>
                      <div class="form-group">
                        <label>Timeout (ms)</label>
                        <input type="number" value={camTimeout()} onInput={(e) => setCamTimeout(e.currentTarget.value)} placeholder="30000" />
                      </div>
                    </div>

                    {/* Test Connection Button */}
                    <div style="margin-top: 16px; padding: 16px; background: var(--bg-secondary); border-radius: 8px;">
                      <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px;">
                        <button
                          type="button"
                          class="btn btn-secondary"
                          onClick={testConnection}
                          disabled={testingConnection()}
                          style="display: flex; align-items: center; gap: 8px;"
                        >
                          {testingConnection() ? "⏳ Testando..." : "🔌 Testar Conexão"}
                        </button>
                        <Show when={connectionStatus()}>
                          <span style={`font-weight: 500; color: ${connectionStatus()?.success ? "#22c55e" : "#ef4444"}`}>
                            {connectionStatus()?.success ? "✅" : "❌"} {connectionStatus()?.message}
                          </span>
                        </Show>
                      </div>

                      {/* Preview Area */}
                      <Show when={previewUrl()}>
                        <div style="margin-top: 12px;">
                          <label style="font-size: 12px; color: var(--text-muted);">Preview da Câmera:</label>
                          <div style="margin-top: 8px; background: #000; border-radius: 8px; overflow: hidden; aspect-ratio: 16/9; max-width: 400px;">
                            <img
                              src={previewUrl()!}
                              alt="Camera Preview"
                              style="width: 100%; height: 100%; object-fit: contain;"
                              onError={(e) => { (e.target as HTMLImageElement).style.display = 'none'; }}
                            />
                          </div>
                        </div>
                      </Show>
                    </div>
                  </Show>

                  {/* Tab: Recording */}
                  <Show when={activeTab() === "recording"}>
                    <div class="form-group">
                      <label>Modo de Gravação</label>
                      <select value={camRecMode()} onChange={(e) => setCamRecMode(e.currentTarget.value)}>
                        <option value="disabled">Desabilitado</option>
                        <option value="continuous">Contínuo (24h)</option>
                        <option value="motion">Detecção de Movimento</option>
                        <option value="manual">Manual</option>
                      </select>
                    </div>
                    <div class="form-group">
                      <label>Diretório de Gravação</label>
                      <input type="text" value={camRecDir()} onInput={(e) => setCamRecDir(e.currentTarget.value)} placeholder="C:\Gravacoes\Camera01" />
                    </div>
                    <div class="form-group">
                      <label>Retenção (dias)</label>
                      <input type="number" value={camRetention()} onInput={(e) => setCamRetention(e.currentTarget.value)} placeholder="30" min="1" max="365" />
                    </div>
                  </Show>

                  {/* Tab: Location */}
                  <Show when={activeTab() === "location"}>
                    <div class="form-group">
                      <label>Atalho no Cliente</label>
                      <input type="text" value={camShortcut()} onInput={(e) => setCamShortcut(e.currentTarget.value)} placeholder="CAM-01" />
                    </div>
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px;">
                      <div class="form-group">
                        <label>Latitude</label>
                        <input type="number" step="0.000001" value={camLat()} onInput={(e) => setCamLat(e.currentTarget.value)} placeholder="-23.550520" />
                      </div>
                      <div class="form-group">
                        <label>Longitude</label>
                        <input type="number" step="0.000001" value={camLng()} onInput={(e) => setCamLng(e.currentTarget.value)} placeholder="-46.633308" />
                      </div>
                    </div>
                  </Show>

                  {/* Tab: Server */}
                  <Show when={activeTab() === "server"}>
                    <div class="form-group">
                      <label>Vincular ao Servidor</label>
                      <select value={camServerId()} onChange={(e) => setCamServerId(e.currentTarget.value)}>
                        <option value="">Nenhum servidor</option>
                        <For each={servers()}>
                          {(s: any) => (
                            <option value={s.id}>{s.name} ({s.ip}:{s.port})</option>
                          )}
                        </For>
                      </select>
                      <p style="color: var(--text-muted); font-size: 12px; margin-top: 8px;">
                        Vincule esta câmera a um servidor para habilitar streaming WebRTC.
                      </p>
                    </div>
                  </Show>

                  <div class="modal-footer" style="margin-top: 24px; display: flex; justify-content: space-between;">
                    <button type="button" class="btn btn-secondary" onClick={() => { setShowCameraModal(false); resetForm(); }}>Cancelar</button>
                    <button type="submit" class="btn btn-primary">Criar Câmera</button>
                  </div>
                </form>
              </div>
            </div>
          </div>
        </Show>
      </>
    );
  }

  // Users Page
  function UsersPage() {
    const [userName, setUserName] = createSignal("");
    const [userUsername, setUserUsername] = createSignal("");
    const [userPassword, setUserPassword] = createSignal("");
    const [userRole, setUserRole] = createSignal("viewer");

    const handleCreate = (e: Event) => {
      e.preventDefault();
      createUser({
        name: userName(),
        username: userUsername(),
        password: userPassword(),
        role: userRole(),
      });
      setUserName("");
      setUserUsername("");
      setUserPassword("");
      setUserRole("viewer");
    };

    return (
      <>
        <div class="card">
          <div class="card-header">
            <h3 class="card-title">Usuários</h3>
            <button class="btn btn-primary" onClick={() => setShowUserModal(true)}>
              + Adicionar Usuário
            </button>
          </div>
          <table class="table">
            <thead>
              <tr>
                <th>Usuário</th>
                <th>Nome</th>
                <th>Perfil</th>
                <th>Status</th>
                <th>Ações</th>
              </tr>
            </thead>
            <tbody>
              <For each={users()}>
                {(u) => (
                  <tr>
                    <td>{u.username}</td>
                    <td>{u.name}</td>
                    <td style="text-transform: capitalize">{u.role}</td>
                    <td>
                      <span class={`status ${u.enabled ? "status-online" : "status-offline"}`}>
                        {u.enabled ? "Ativo" : "Inativo"}
                      </span>
                    </td>
                    <td>
                      <button
                        class="btn btn-danger"
                        style="padding: 6px 12px"
                        onClick={() => deleteUser(u.id)}
                        disabled={u.username === "admin"}
                      >
                        🗑️
                      </button>
                    </td>
                  </tr>
                )}
              </For>
            </tbody>
          </table>
        </div>

        {/* User Modal */}
        <Show when={showUserModal()}>
          <div class="modal-overlay" onClick={() => setShowUserModal(false)}>
            <div class="modal-content" onClick={(e) => e.stopPropagation()}>
              <div class="modal-header">
                <h2>Novo Usuário</h2>
                <button class="modal-close" onClick={() => setShowUserModal(false)}>×</button>
              </div>
              <form onSubmit={handleCreate}>
                <div class="form-group">
                  <label>Nome Completo</label>
                  <input type="text" value={userName()} onInput={(e) => setUserName(e.currentTarget.value)} placeholder="João Silva" required />
                </div>
                <div class="form-group">
                  <label>Usuário (login)</label>
                  <input type="text" value={userUsername()} onInput={(e) => setUserUsername(e.currentTarget.value)} placeholder="joao" required />
                </div>
                <div class="form-group">
                  <label>Senha</label>
                  <input type="password" value={userPassword()} onInput={(e) => setUserPassword(e.currentTarget.value)} placeholder="••••••" required />
                </div>
                <div class="form-group">
                  <label>Perfil</label>
                  <select value={userRole()} onChange={(e) => setUserRole(e.currentTarget.value)}>
                    <option value="admin">Administrador</option>
                    <option value="operator">Operador</option>
                    <option value="viewer">Visualizador</option>
                  </select>
                </div>
                <div class="modal-footer">
                  <button type="button" class="btn btn-secondary" onClick={() => setShowUserModal(false)}>Cancelar</button>
                  <button type="submit" class="btn btn-primary">Criar Usuário</button>
                </div>
              </form>
            </div>
          </div>
        </Show>
      </>
    );
  }

  // Servers Page
  function ServersPage() {
    const [serverName, setServerName] = createSignal("");
    const [serverIp, setServerIp] = createSignal("");
    const [serverPort, setServerPort] = createSignal("9094");
    const [serverUser, setServerUser] = createSignal("");
    const [serverPass, setServerPass] = createSignal("");

    const handleCreate = (e: Event) => {
      e.preventDefault();
      createServer({
        name: serverName(),
        ip: serverIp(),
        port: parseInt(serverPort()) || 9094,
        username: serverUser(),
        password: serverPass(),
      });
      setServerName("");
      setServerIp("");
      setServerPort("9094");
      setServerUser("");
      setServerPass("");
    };

    return (
      <>
        <div class="card">
          <div class="card-header">
            <h3 class="card-title">Servidores de Streaming</h3>
            <button class="btn btn-primary" onClick={() => setShowServerModal(true)}>
              + Novo Servidor
            </button>
          </div>
          <table class="table">
            <thead>
              <tr>
                <th>Nome</th>
                <th>IP</th>
                <th>Porta</th>
                <th>Usuário</th>
                <th>Status</th>
                <th>WebRTC URL</th>
                <th>Ações</th>
              </tr>
            </thead>
            <tbody>
              <For each={servers()}>
                {(s) => (
                  <tr>
                    <td>{s.name}</td>
                    <td style="font-family: monospace">{s.ip}</td>
                    <td>{s.port}</td>
                    <td>{s.username}</td>
                    <td>
                      <span class={`status status-${s.status}`}>
                        {s.status}
                      </span>
                    </td>
                    <td style="font-family: monospace; font-size: 11px">{s.webrtc_url}</td>
                    <td>
                      <button
                        class="btn btn-danger"
                        style="padding: 6px 12px"
                        onClick={() => deleteServer(s.id)}
                      >
                        🗑️
                      </button>
                    </td>
                  </tr>
                )}
              </For>
            </tbody>
          </table>
        </div>

        {/* Create Server Modal */}
        <Show when={showServerModal()}>
          <div class="modal-overlay" onClick={() => setShowServerModal(false)}>
            <div class="modal-content" onClick={(e) => e.stopPropagation()}>
              <div class="modal-header">
                <h2>Novo Servidor</h2>
                <button class="modal-close" onClick={() => setShowServerModal(false)}>×</button>
              </div>
              <form onSubmit={handleCreate}>
                <div class="form-group">
                  <label>Nome</label>
                  <input
                    type="text"
                    value={serverName()}
                    onInput={(e) => setServerName(e.currentTarget.value)}
                    placeholder="Servidor Principal"
                    required
                  />
                </div>
                <div class="form-group">
                  <label>IP</label>
                  <input
                    type="text"
                    value={serverIp()}
                    onInput={(e) => setServerIp(e.currentTarget.value)}
                    placeholder="192.168.1.100"
                    required
                  />
                </div>
                <div class="form-group">
                  <label>Porta</label>
                  <input
                    type="number"
                    value={serverPort()}
                    onInput={(e) => setServerPort(e.currentTarget.value)}
                    placeholder="9094"
                  />
                </div>
                <div class="form-group">
                  <label>Usuário</label>
                  <input
                    type="text"
                    value={serverUser()}
                    onInput={(e) => setServerUser(e.currentTarget.value)}
                    placeholder="admin"
                    required
                  />
                </div>
                <div class="form-group">
                  <label>Senha</label>
                  <input
                    type="password"
                    value={serverPass()}
                    onInput={(e) => setServerPass(e.currentTarget.value)}
                    placeholder="••••••"
                    required
                  />
                </div>
                <div class="modal-footer">
                  <button type="button" class="btn btn-secondary" onClick={() => setShowServerModal(false)}>
                    Cancelar
                  </button>
                  <button type="submit" class="btn btn-primary">
                    Criar Servidor
                  </button>
                </div>
              </form>
            </div>
          </div>
        </Show>
      </>
    );
  }

  // Main App Layout
  function MainLayout() {
    const renderPage = () => {
      switch (currentPage()) {
        case "cameras":
          return <CamerasPage />;
        case "users":
          return <UsersPage />;
        case "servers":
          return <ServersPage />;
        default:
          return <Dashboard />;
      }
    };

    return (
      <div class="app-container">
        {/* Sidebar */}
        <aside class="sidebar">
          <div class="sidebar-logo">
            <div class="icon">SMU</div>
            <span class="text">VMS Admin</span>
          </div>

          <nav class="sidebar-nav">
            <div class="nav-section">
              <div class="nav-section-title">Main</div>
              <div
                class={`nav-item ${currentPage() === "dashboard" ? "active" : ""}`}
                onClick={() => setCurrentPage("dashboard")}
              >
                <Icons.Dashboard />
                <span>Dashboard</span>
              </div>
              <div
                class={`nav-item ${currentPage() === "cameras" ? "active" : ""}`}
                onClick={() => setCurrentPage("cameras")}
              >
                <Icons.Camera />
                <span>Cameras</span>
              </div>
            </div>

            <div class="nav-section">
              <div class="nav-section-title">Management</div>
              <div
                class={`nav-item ${currentPage() === "users" ? "active" : ""}`}
                onClick={() => setCurrentPage("users")}
              >
                <Icons.Users />
                <span>Users</span>
              </div>
              <div
                class={`nav-item ${currentPage() === "servers" ? "active" : ""}`}
                onClick={() => setCurrentPage("servers")}
              >
                <Icons.Server />
                <span>Servidores</span>
              </div>
              <div class="nav-item">
                <Icons.Settings />
                <span>Settings</span>
              </div>
            </div>
          </nav>

          <div style="margin-top: auto; padding: 12px">
            <div class="nav-item" onClick={logout}>
              <Icons.Logout />
              <span>Logout</span>
            </div>
          </div>
        </aside>

        {/* Main Content */}
        <main class="main-content">
          <header class="header">
            <h1 class="header-title">
              {currentPage().charAt(0).toUpperCase() + currentPage().slice(1)}
            </h1>
            <div class="header-actions">
              <div class="user-badge">
                <div class="user-avatar">
                  {user()?.name.charAt(0).toUpperCase() || "A"}
                </div>
                <div class="user-info">
                  <div class="name">{user()?.name || "Admin"}</div>
                  <div class="role">{user()?.role || "admin"}</div>
                </div>
              </div>
            </div>
          </header>

          <div class="content">
            {renderPage()}
          </div>
        </main>
      </div>
    );
  }

  return (
    <Show when={token()} fallback={<LoginScreen />}>
      <MainLayout />
    </Show>
  );
}

export default App;
