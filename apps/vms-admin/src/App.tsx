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

  // Cameras Page
  function CamerasPage() {
    return (
      <div class="card">
        <div class="card-header">
          <h3 class="card-title">Cameras</h3>
          <button class="btn btn-primary">+ Add Camera</button>
        </div>
        <table class="table">
          <thead>
            <tr>
              <th>Name</th>
              <th>RTSP URL</th>
              <th>Resolution</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <For each={cameras()}>
              {(camera) => (
                <tr>
                  <td>{camera.name}</td>
                  <td style="font-family: monospace; font-size: 12px">{camera.rtsp_url}</td>
                  <td>{camera.resolution_width}x{camera.resolution_height}</td>
                  <td>
                    <span class={`status ${camera.enabled ? "status-online" : "status-offline"}`}>
                      {camera.enabled ? "Online" : "Offline"}
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
