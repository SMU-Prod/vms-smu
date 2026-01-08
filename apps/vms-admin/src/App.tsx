import { createSignal, createEffect, Show, For } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
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
  access_token: string;
  refresh_token: string;
  user: User;
  expires_in: number;
}

interface Camera {
  id: string;
  name: string;
  description?: string;
  manufacturer: string;
  model: string;
  firmware?: string;
  rtsp_url: string;
  onvif_url?: string;
  username: string;
  password: string;
  shortcut?: string;
  recording_dir?: string;
  notes?: string;
  enabled: boolean;
  resolution_width: number;
  resolution_height: number;
}

// Camera form data (based on Digifort model)
interface CameraFormData {
  name: string;
  description: string;
  manufacturer: string;
  model: string;
  firmware: string;
  ip: string;
  rtsp_port: number;
  onvif_port: number;
  username: string;
  password: string;
  stream_path: string;
  recording_path: string;       // Diretório de gravação
  connection_timeout_ms: number; // Timeout da conexão
  latitude: number;             // Coordenadas
  longitude: number;
  notes: string;
  transport: string;            // Auto/TCP/UDP
  enabled: boolean;
}

// Manufacturer/Model/Firmware options
const MANUFACTURERS = ["TP-Link", "Hikvision", "Dahua", "Intelbras", "Axis", "Samsung", "Generic"];
const MODELS: Record<string, string[]> = {
  "TP-Link": ["Tapo C100", "Tapo C200", "Tapo C310", "Tapo C320WS"],
  "Hikvision": ["DS-2CD2032", "DS-2CD2042WD", "DS-2CD2143G0-I"],
  "Dahua": ["IPC-HFW2431S", "IPC-HDBW2431E"],
  "Intelbras": ["VIP 1130 B", "VIP 3230 B", "VIP 1020 B"],
  "Axis": ["P3245-V", "M3057-PLVE"],
  "Samsung": ["SNV-6084R", "XNV-8080R"],
  "Generic": ["IP Camera"]
};
const FIRMWARE_VERSIONS = ["Automático", "1.0.0", "1.1.0", "2.0.0", "Personalizado"];

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

  // Camera form modal state
  const [showCameraModal, setShowCameraModal] = createSignal(false);
  const [cameraForm, setCameraForm] = createSignal<CameraFormData>({
    name: "",
    description: "",
    manufacturer: "Generic",
    model: "IP Camera",
    firmware: "Automático",
    ip: "",
    rtsp_port: 554,
    onvif_port: 2020,
    username: "",
    password: "",
    stream_path: "stream1",
    recording_path: "",
    connection_timeout_ms: 30000,
    latitude: 0,
    longitude: 0,
    notes: "",
    transport: "auto",
    enabled: true
  });

  // Create camera API call
  async function createCamera() {
    setLoading(true);
    const form = cameraForm();
    try {
      // If editing, use PUT to update existing camera
      if (editingCamera()) {
        const res = await fetch(`${API_URL}/cameras/${editingCamera()!.id}`, {
          method: "PUT",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token()}`
          },
          body: JSON.stringify({
            name: form.name,
            description: form.description || null,
            manufacturer: form.manufacturer,
            model: form.model,
            firmware: form.firmware || null,
            ip: form.ip,
            rtsp_port: form.rtsp_port,
            onvif_port: form.onvif_port || null,
            username: form.username,
            password: form.password,
            stream_path: form.stream_path,
            enabled: form.enabled
          }),
        });
        if (res.ok) {
          setShowCameraModal(false);
          setEditingCamera(null);
          loadCameras();
          alert('Câmera atualizada com sucesso!');
        } else {
          const err = await res.json();
          alert(err.error || "Erro ao atualizar câmera");
        }
        setLoading(false);
        return;
      }

      // Creating new camera - get node first
      const nodesRes = await fetch(`${API_URL}/nodes`, {
        headers: { Authorization: `Bearer ${token()}` }
      });
      const nodes = await nodesRes.json();
      const nodeId = nodes[0]?.id;
      if (!nodeId) {
        alert("Nenhum node registrado! Inicie o vms_node primeiro.");
        setLoading(false);
        return;
      }

      const res = await fetch(`${API_URL}/cameras`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token()}`
        },
        body: JSON.stringify({
          node_id: nodeId,
          name: form.name,
          description: form.description || null,
          manufacturer: form.manufacturer,
          model: form.model,
          firmware: form.firmware || null,
          ip: form.ip,
          rtsp_port: form.rtsp_port,
          onvif_port: form.onvif_port || null,
          username: form.username,
          password: form.password,
          stream_path: form.stream_path,
          enabled: form.enabled
        }),
      });
      if (res.ok) {
        setShowCameraModal(false);
        loadCameras();
        // Reset form
        setCameraForm({
          name: "", description: "", manufacturer: "Generic", model: "IP Camera",
          firmware: "Automático", ip: "", rtsp_port: 554, onvif_port: 2020,
          username: "", password: "", stream_path: "stream1", recording_path: "",
          connection_timeout_ms: 30000, latitude: 0, longitude: 0, notes: "", transport: "auto", enabled: true
        });
      } else {
        const err = await res.json();
        alert(err.error || "Erro ao criar câmera");
      }
    } catch (e: any) {
      alert(e.message);
    } finally {
      setLoading(false);
    }
  }

  // Load data on auth
  createEffect(() => {
    if (token()) {
      loadCameras();
      loadUsers();
    }
  });

  // API calls
  async function login(email: string, password: string) {
    setLoading(true);
    setError("");
    try {
      const res = await fetch(`${API_URL}/auth/login`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email, password }),
      });

      if (!res.ok) {
        const data = await res.json();
        throw new Error(data.error || "Login failed");
      }

      const data: LoginResponse = await res.json();
      setToken(data.access_token);
      setUser(data.user);
      localStorage.setItem("token", data.access_token);
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

  // Signal for editing camera
  const [editingCamera, setEditingCamera] = createSignal<Camera | null>(null);

  // Open edit modal with camera data
  function openEditModal(camera: Camera) {
    // Parse RTSP URL to extract IP and port
    const rtspMatch = camera.rtsp_url.match(/rtsp:\/\/([^:\/]+):?(\d+)?(.*)$/);
    const ip = rtspMatch ? rtspMatch[1] : '';
    const port = rtspMatch && rtspMatch[2] ? parseInt(rtspMatch[2]) : 554;
    const path = rtspMatch ? rtspMatch[3].replace(/^\//, '') : 'stream1';

    setCameraForm({
      name: camera.name,
      description: camera.description || '',
      manufacturer: camera.manufacturer,
      model: camera.model,
      firmware: camera.firmware || 'Automático',
      ip: ip,
      rtsp_port: port,
      onvif_port: 2020,
      username: camera.username || '',
      password: camera.password || '',
      stream_path: path,
      recording_path: camera.recording_dir || '',
      connection_timeout_ms: 30000,
      latitude: 0,
      longitude: 0,
      notes: camera.notes || '',
      transport: 'auto',
      enabled: camera.enabled
    });
    setEditingCamera(camera);
    setShowCameraModal(true);
  }

  // Delete camera
  async function deleteCamera(cameraId: string, cameraName: string) {
    if (!confirm(`Tem certeza que deseja excluir a câmera "${cameraName}"?`)) {
      return;
    }

    try {
      const res = await fetch(`${API_URL}/cameras/${cameraId}`, {
        method: 'DELETE',
        headers: { Authorization: `Bearer ${token()}` }
      });

      if (res.ok) {
        alert(`Câmera "${cameraName}" excluída com sucesso!`);
        loadCameras();
      } else {
        const err = await res.json();
        alert(err.error || 'Erro ao excluir câmera');
      }
    } catch (e: any) {
      alert(`Erro: ${e.message}`);
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
              <label>Usuário</label>
              <input
                type="text"
                class="input"
                placeholder="admin"
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

  // Cameras Page with Modal
  function CamerasPage() {
    const updateField = (field: keyof CameraFormData, value: any) => {
      setCameraForm({ ...cameraForm(), [field]: value });
    };

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
                <th>Fabricante</th>
                <th>Modelo</th>
                <th>RTSP URL</th>
                <th>Status</th>
                <th>Ações</th>
              </tr>
            </thead>
            <tbody>
              <For each={cameras()}>
                {(camera) => (
                  <tr>
                    <td>{camera.name}</td>
                    <td>{camera.manufacturer}</td>
                    <td>{camera.model}</td>
                    <td style="font-family: monospace; font-size: 11px">{camera.rtsp_url}</td>
                    <td>
                      <span class={`status ${camera.enabled ? "status-online" : "status-offline"}`}>
                        {camera.enabled ? "Ativa" : "Inativa"}
                      </span>
                    </td>
                    <td style="display: flex; gap: 8px">
                      <button
                        class="btn btn-secondary"
                        style="padding: 6px 12px"
                        onClick={() => openEditModal(camera)}
                      >
                        ✏️ Editar
                      </button>
                      <button
                        class="btn"
                        style="padding: 6px 12px; background: #dc3545; color: white"
                        onClick={() => deleteCamera(camera.id, camera.name)}
                      >
                        🗑️ Excluir
                      </button>
                    </td>
                  </tr>
                )}
              </For>
              <Show when={cameras().length === 0}>
                <tr>
                  <td colspan="6" style="text-align: center; color: var(--text-muted)">
                    Nenhuma câmera configurada
                  </td>
                </tr>
              </Show>
            </tbody>
          </table>
        </div>

        {/* Camera Form Modal */}
        <Show when={showCameraModal()}>
          <div class="modal-overlay" onClick={() => { setShowCameraModal(false); setEditingCamera(null); }}>
            <div class="modal" onClick={(e) => e.stopPropagation()} style="width: 700px; max-height: 85vh; overflow-y: auto">
              <div class="modal-header">
                <h2>{editingCamera() ? `Editar Câmera: ${editingCamera()!.name}` : 'Nova Câmera'}</h2>
                <button class="modal-close" onClick={() => { setShowCameraModal(false); setEditingCamera(null); }}>&times;</button>
              </div>
              <div class="modal-body">
                {/* Row 1: Nome e Descrição */}
                <div class="form-row" style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px">
                  <div class="form-group">
                    <label>Nome da Câmera *</label>
                    <input
                      type="text" class="input" placeholder="Ex: Entrada Principal"
                      value={cameraForm().name}
                      onInput={(e) => updateField("name", e.currentTarget.value)}
                      required
                    />
                  </div>
                  <div class="form-group">
                    <label>Descrição</label>
                    <input
                      type="text" class="input" placeholder="Ex: Câmera de vigilância"
                      value={cameraForm().description}
                      onInput={(e) => updateField("description", e.currentTarget.value)}
                    />
                  </div>
                </div>

                {/* Row 2: Fabricante, Modelo, Firmware */}
                <div class="form-row" style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 16px">
                  <div class="form-group">
                    <label>Fabricante *</label>
                    <select
                      class="input"
                      value={cameraForm().manufacturer}
                      onChange={(e) => {
                        const mfr = e.currentTarget.value;
                        updateField("manufacturer", mfr);
                        updateField("model", MODELS[mfr]?.[0] || "IP Camera");
                      }}
                    >
                      <For each={MANUFACTURERS}>
                        {(mfr) => <option value={mfr}>{mfr}</option>}
                      </For>
                    </select>
                  </div>
                  <div class="form-group">
                    <label>Modelo *</label>
                    <select
                      class="input"
                      value={cameraForm().model}
                      onChange={(e) => updateField("model", e.currentTarget.value)}
                    >
                      <For each={MODELS[cameraForm().manufacturer] || ["IP Camera"]}>
                        {(model) => <option value={model}>{model}</option>}
                      </For>
                    </select>
                  </div>
                  <div class="form-group">
                    <label>Firmware</label>
                    <select
                      class="input"
                      value={cameraForm().firmware}
                      onChange={(e) => updateField("firmware", e.currentTarget.value)}
                    >
                      <For each={FIRMWARE_VERSIONS}>
                        {(fw) => <option value={fw}>{fw}</option>}
                      </For>
                    </select>
                  </div>
                </div>

                {/* Row 3: IP, Porta RTSP, Porta Firmware */}
                <div class="form-row" style="display: grid; grid-template-columns: 2fr 1fr 1fr; gap: 16px">
                  <div class="form-group">
                    <label>IP da Câmera *</label>
                    <input
                      type="text" class="input" placeholder="192.168.1.100"
                      value={cameraForm().ip}
                      onInput={(e) => updateField("ip", e.currentTarget.value)}
                      required
                    />
                  </div>
                  <div class="form-group">
                    <label>Porta RTSP</label>
                    <input
                      type="number" class="input"
                      value={cameraForm().rtsp_port}
                      onInput={(e) => updateField("rtsp_port", parseInt(e.currentTarget.value) || 554)}
                    />
                  </div>
                  <div class="form-group">
                    <label>Porta (Firmware)</label>
                    <input
                      type="number" class="input"
                      value={cameraForm().onvif_port}
                      onInput={(e) => updateField("onvif_port", parseInt(e.currentTarget.value) || 80)}
                    />
                  </div>
                </div>

                {/* Row 4: Usuário, Senha, Stream */}
                <div class="form-row" style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 16px">
                  <div class="form-group">
                    <label>Usuário *</label>
                    <input
                      type="text" class="input" placeholder="admin"
                      value={cameraForm().username}
                      onInput={(e) => updateField("username", e.currentTarget.value)}
                      required
                    />
                  </div>
                  <div class="form-group">
                    <label>Senha *</label>
                    <input
                      type="password" class="input" placeholder="********"
                      value={cameraForm().password}
                      onInput={(e) => updateField("password", e.currentTarget.value)}
                      required
                    />
                  </div>
                  <div class="form-group">
                    <label>Stream Path</label>
                    <input
                      type="text" class="input" placeholder="stream1"
                      value={cameraForm().stream_path}
                      onInput={(e) => updateField("stream_path", e.currentTarget.value)}
                    />
                  </div>
                </div>

                {/* Row 5: Diretório de Gravação e Timeout */}
                <div class="form-row" style="display: grid; grid-template-columns: 2fr 1fr; gap: 16px">
                  <div class="form-group">
                    <label>Diretório de Gravação</label>
                    <div style="display: flex; gap: 8px">
                      <input
                        type="text" class="input" placeholder="D:\Recordings\Camera1"
                        value={cameraForm().recording_path}
                        onInput={(e) => updateField("recording_path", e.currentTarget.value)}
                        style="flex: 1"
                      />
                      <button
                        class="btn btn-secondary"
                        style="padding: 8px 12px"
                        onClick={async () => {
                          try {
                            const { open } = await import("@tauri-apps/plugin-dialog");
                            const selected = await open({ directory: true });
                            if (selected) updateField("recording_path", selected);
                          } catch (e) { console.error(e); }
                        }}
                      >
                        📁
                      </button>
                    </div>
                  </div>
                  <div class="form-group">
                    <label>Timeout Conexão (ms)</label>
                    <input
                      type="number" class="input" placeholder="30000"
                      value={cameraForm().connection_timeout_ms}
                      onInput={(e) => updateField("connection_timeout_ms", parseInt(e.currentTarget.value))}
                    />
                  </div>
                </div>

                {/* Row 6: Observações */}
                <div class="form-group">
                  <label>Observações</label>
                  <textarea
                    class="input"
                    rows="3"
                    placeholder="Anotações sobre esta câmera..."
                    value={cameraForm().notes}
                    onInput={(e) => updateField("notes", e.currentTarget.value)}
                    style="resize: vertical"
                  />
                </div>

                {/* Row 7: Ativar Câmera */}
                <div class="form-group" style="display: flex; align-items: center; gap: 12px; margin-top: 12px">
                  <input
                    type="checkbox"
                    id="enabled-checkbox"
                    checked={cameraForm().enabled}
                    onChange={(e) => updateField("enabled", e.currentTarget.checked)}
                    style="width: 20px; height: 20px"
                  />
                  <label for="enabled-checkbox" style="margin: 0; font-weight: 600">
                    Ativar Câmera (liga gravação e visualização)
                  </label>
                </div>
              </div>

              {/* Modal Footer */}
              <div class="modal-footer" style="display: flex; justify-content: flex-end; gap: 12px; padding: 16px 24px; border-top: 1px solid var(--border)">
                <button class="btn btn-secondary" onClick={() => setShowCameraModal(false)}>
                  Cancelar
                </button>
                <button class="btn btn-primary" onClick={createCamera} disabled={loading()}>
                  {loading() ? "Salvando..." : "OK - Salvar"}
                </button>
              </div>
            </div>
          </div>
        </Show>
      </>
    );
  }

  // Users Page
  function UsersPage() {
    return (
      <div class="card">
        <div class="card-header">
          <h3 class="card-title">Users</h3>
          <button class="btn btn-primary">+ Add User</button>
        </div>
        <table class="table">
          <thead>
            <tr>
              <th>Username</th>
              <th>Name</th>
              <th>Role</th>
              <th>Status</th>
              <th>Actions</th>
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
                      {u.enabled ? "Active" : "Disabled"}
                    </span>
                  </td>
                  <td>
                    <button class="btn btn-secondary" style="padding: 6px 12px">Edit</button>
                  </td>
                </tr>
              )}
            </For>
          </tbody>
        </table>
      </div>
    );
  }

  // Servers Page - CRUD de servidores
  function ServersPage() {
    const [servers, setServers] = createSignal<any[]>([]);
    const [showModal, setShowModal] = createSignal(false);
    const [serverForm, setServerForm] = createSignal({ name: "", ip: "", port: 9095 });

    // Load servers on mount
    createEffect(() => {
      loadServers();
    });

    async function loadServers() {
      try {
        const res = await fetch(`${API_URL}/nodes`, {
          headers: { Authorization: `Bearer ${token()}` }
        });
        if (res.ok) {
          setServers(await res.json());
        }
      } catch (e) {
        console.error("Failed to load servers:", e);
      }
    }

    async function createServer() {
      const form = serverForm();
      if (!form.name || !form.ip) {
        alert("Nome e IP são obrigatórios");
        return;
      }
      try {
        const res = await fetch(`${API_URL}/nodes/register`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token()}`
          },
          body: JSON.stringify(form)
        });
        if (res.ok) {
          setShowModal(false);
          setServerForm({ name: "", ip: "", port: 9095 });
          loadServers();
          alert("Servidor criado com sucesso!");
        } else {
          const err = await res.json();
          alert(err.error || "Erro ao criar servidor");
        }
      } catch (e: any) {
        alert(e.message);
      }
    }

    async function deleteServer(id: string, name: string) {
      if (!confirm(`Excluir servidor "${name}"? Todas as câmeras vinculadas serão excluídas.`)) return;
      try {
        const res = await fetch(`${API_URL}/nodes/${id}`, {
          method: "DELETE",
          headers: { Authorization: `Bearer ${token()}` }
        });
        if (res.ok) {
          loadServers();
          alert("Servidor excluído!");
        }
      } catch (e) {
        console.error(e);
      }
    }

    return (
      <>
        <div class="card">
          <div class="card-header">
            <h3 class="card-title">Servidores</h3>
            <button class="btn btn-primary" onClick={() => setShowModal(true)}>+ Novo Servidor</button>
          </div>
          <table class="table">
            <thead>
              <tr>
                <th>Nome</th>
                <th>IP</th>
                <th>Porta</th>
                <th>Status</th>
                <th>Ações</th>
              </tr>
            </thead>
            <tbody>
              <For each={servers()}>
                {(s) => (
                  <tr>
                    <td>{s.name}</td>
                    <td>{s.ip}</td>
                    <td>{s.port}</td>
                    <td>
                      <span class={`status ${s.status === "online" ? "status-online" : "status-offline"}`}>
                        {s.status === "online" ? "Online" : "Offline"}
                      </span>
                    </td>
                    <td>
                      <button class="btn btn-danger" style="padding: 6px 12px" onClick={() => deleteServer(s.id, s.name)}>Excluir</button>
                    </td>
                  </tr>
                )}
              </For>
            </tbody>
          </table>
        </div>

        {/* Modal Novo Servidor */}
        <Show when={showModal()}>
          <div class="modal-overlay" onClick={() => setShowModal(false)}>
            <div class="modal" onClick={(e) => e.stopPropagation()}>
              <div class="modal-header">
                <h2>Novo Servidor</h2>
                <button class="close-btn" onClick={() => setShowModal(false)}>×</button>
              </div>
              <div class="modal-body">
                <div class="form-group">
                  <label>Nome do Servidor *</label>
                  <input type="text" class="input" placeholder="Servidor Principal"
                    value={serverForm().name}
                    onInput={(e) => setServerForm({ ...serverForm(), name: e.currentTarget.value })}
                  />
                </div>
                <div class="form-group">
                  <label>IP da Máquina *</label>
                  <input type="text" class="input" placeholder="192.168.1.100"
                    value={serverForm().ip}
                    onInput={(e) => setServerForm({ ...serverForm(), ip: e.currentTarget.value })}
                  />
                </div>
                <div class="form-group">
                  <label>Porta</label>
                  <input type="number" class="input"
                    value={serverForm().port}
                    onInput={(e) => setServerForm({ ...serverForm(), port: parseInt(e.currentTarget.value) || 9095 })}
                  />
                </div>
              </div>
              <div class="modal-footer">
                <button class="btn btn-secondary" onClick={() => setShowModal(false)}>Cancelar</button>
                <button class="btn btn-primary" onClick={createServer}>Criar Servidor</button>
              </div>
            </div>
          </div>
        </Show>
      </>
    );
  }

  // Main App Layout
  function MainLayout() {
    const pages: Record<string, () => any> = {
      dashboard: Dashboard,
      cameras: CamerasPage,
      users: UsersPage,
      servers: ServersPage,
      settings: () => <div class="card"><h2>Configurações</h2><p>Em desenvolvimento...</p></div>,
    };

    const PageComponent = () => {
      const Component = pages[currentPage()];
      return Component ? <Component /> : <Dashboard />;
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
                <span>Servers</span>
              </div>
              <div
                class={`nav-item ${currentPage() === "settings" ? "active" : ""}`}
                onClick={() => setCurrentPage("settings")}
              >
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
            <PageComponent />
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
