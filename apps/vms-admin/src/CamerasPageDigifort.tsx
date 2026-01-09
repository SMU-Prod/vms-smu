import { createSignal, For, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

// Types
interface Server {
    id: string;
    name: string;
    ip: string;
    port: number;
    status?: string;
}

interface CameraFolder {
    id: string;
    name: string;
    isProtected: boolean;
}

interface Camera {
    id: string;
    name: string;
    description?: string;
    ip_address?: string;
    rtsp_port?: number;
    username?: string;
    password?: string;
    transport?: string;
    manufacturer?: string;
    model?: string;
    firmware?: string;
    enabled: boolean;
    audio_enabled?: boolean;
    server_id?: string;
    folder_id?: string;
    shortcut?: string;
    latitude?: number;
    longitude?: number;
    recording_dir?: string;
    timeout_ms?: number;
}

interface Props {
    servers: () => Server[];
    cameras: () => Camera[];
    API_URL: string;
    onRefresh: () => void;
}

// Camera manufacturer/model/firmware data for cascading dropdowns
const CAMERA_DATA: Record<string, Record<string, string[]>> = {
    "Hikvision": {
        "DS-2CD2085FWD-I": ["V5.5.6", "V5.5.7", "V5.6.0"],
        "DS-2CD2145FWD-IS": ["V5.5.6", "V5.6.0"],
        "DS-2CD2385FWD-I": ["V5.5.7", "V5.6.1"],
    },
    "Dahua": {
        "DH-IPC-HDW4431C-A": ["V2.800", "V2.830"],
        "DH-IPC-HFW4431EP-SE": ["V2.800", "V2.820"],
    },
    "Intelbras": {
        "VIP 1230 D G4": ["V1.0.0", "V1.0.1"],
        "VIP 3260 Z G2": ["V2.0.0", "V2.1.0"],
        "VIP 1020 B G4": ["V1.0.0"],
    },
    "TP-Link": {
        "Tapo C100": ["V1.0", "V1.1", "V1.2"],
        "Tapo C200": ["V1.0", "V1.1"],
        "Tapo C310": ["V1.0", "V2.0"],
    },
    "Outro": {
        "Genérico RTSP": ["N/A"],
        "ONVIF Compatível": ["N/A"],
    },
};

const getModels = (manufacturer: string) => Object.keys(CAMERA_DATA[manufacturer] || {});
const getFirmwares = (manufacturer: string, model: string) => CAMERA_DATA[manufacturer]?.[model] || [];

// Digifort-style 3-column layout
export function CamerasPageDigifort(props: Props) {
    // Navigation state
    const [selectedServer, setSelectedServer] = createSignal<Server | null>(null);
    const [selectedNavItem, setSelectedNavItem] = createSignal<string>("cameras");
    const [expandedServers, setExpandedServers] = createSignal<Set<string>>(new Set());

    // Folders state
    const [folders, setFolders] = createSignal<CameraFolder[]>([
        { id: "all", name: "Todos os objetos", isProtected: true },
        { id: "ungrouped", name: "Não agrupados", isProtected: true },
    ]);
    const [selectedFolder, setSelectedFolder] = createSignal<string>("all");
    const [newFolderName, setNewFolderName] = createSignal("");

    // Camera modal
    const [showCameraModal, setShowCameraModal] = createSignal(false);
    const [editingCamera, setEditingCamera] = createSignal<Camera | null>(null);
    const [selectedCamera, setSelectedCamera] = createSignal<Camera | null>(null);

    // Form state
    const [formName, setFormName] = createSignal("");
    const [formDesc, setFormDesc] = createSignal("");
    const [formIp, setFormIp] = createSignal("");
    const [formPort, setFormPort] = createSignal("554");
    const [formUser, setFormUser] = createSignal("");
    const [formPass, setFormPass] = createSignal("");
    const [formTransport, setFormTransport] = createSignal("auto");
    const [formManufacturer, setFormManufacturer] = createSignal("");
    const [formModel, setFormModel] = createSignal("");
    const [formFirmware, setFormFirmware] = createSignal("");
    const [formShortcut, setFormShortcut] = createSignal("");
    const [formLat, setFormLat] = createSignal("0.000000");
    const [formLng, setFormLng] = createSignal("0.000000");
    const [formRecDir, setFormRecDir] = createSignal("");
    const [formTimeout, setFormTimeout] = createSignal("30000");
    const [formEnabled, setFormEnabled] = createSignal(false);
    const [formAudioEnabled, setFormAudioEnabled] = createSignal(false);

    // Streaming configuration
    const [formCodec, setFormCodec] = createSignal("h264");
    const [formStreamType, setFormStreamType] = createSignal("main");


    // Folder picker for recording directory
    const [showFolderPicker, setShowFolderPicker] = createSignal(false);
    const [folderPath, setFolderPath] = createSignal("C:\\");
    const [folderEntries, setFolderEntries] = createSignal<{ name: string; path: string }[]>([]);
    const [folderDrives, setFolderDrives] = createSignal<string[]>([]);
    const [newRecFolderName, setNewRecFolderName] = createSignal("");

    // Load form when editing camera
    const openCameraForm = (camera: Camera | null) => {
        if (camera) {
            setFormName(camera.name || "");
            setFormDesc(camera.description || "");
            setFormIp(camera.ip_address || "");
            setFormPort(String(camera.rtsp_port || 554));
            setFormUser(camera.username || "");
            setFormPass(camera.password || "");
            setFormTransport(camera.transport || "auto");
            setFormManufacturer(camera.manufacturer || "");
            setFormModel(camera.model || "");
            setFormFirmware(camera.firmware || "");
            setFormShortcut(camera.shortcut || "");
            setFormLat(String(camera.latitude || 0));
            setFormLng(String(camera.longitude || 0));
            setFormRecDir(camera.recording_dir || "");
            setFormTimeout(String(camera.timeout_ms || 30000));
            setFormEnabled(camera.enabled || false);
            setFormAudioEnabled(camera.audio_enabled || false);
            setFormCodec((camera as any).codec || "h264");
            setFormStreamType((camera as any).stream_type || "main");
        } else {
            // Reset for new camera
            setFormName(""); setFormDesc(""); setFormIp(""); setFormPort("554");
            setFormUser(""); setFormPass(""); setFormTransport("auto");
            setFormManufacturer(""); setFormModel(""); setFormFirmware("");
            setFormShortcut(""); setFormLat("0.000000"); setFormLng("0.000000");
            setFormRecDir(""); setFormTimeout("30000");
            setFormEnabled(false); setFormAudioEnabled(false);
            setFormCodec("h264"); setFormStreamType("main");
        }
        setEditingCamera(camera);
        setShowCameraModal(true);
    };


    // Folder picker functions
    const loadDirectory = async (path: string) => {
        try {
            const res = await fetch(`${props.API_URL}/filesystem/list?path=${encodeURIComponent(path)}`, {
                headers: { "Authorization": `Bearer ${localStorage.getItem("token")}` }
            });
            if (res.ok) {
                const data = await res.json();
                setFolderPath(data.current_path);
                setFolderEntries(data.entries || []);
                setFolderDrives(data.drives || []);
            }
        } catch (e) { console.error(e); }
    };

    const createRecFolder = async () => {
        if (!newRecFolderName()) return;
        try {
            const res = await fetch(`${props.API_URL}/filesystem/create`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    "Authorization": `Bearer ${localStorage.getItem("token")}`
                },
                body: JSON.stringify({ path: folderPath(), name: newRecFolderName() })
            });
            if (res.ok) {
                setNewRecFolderName("");
                loadDirectory(folderPath());
            }
        } catch (e) { console.error(e); }
    };

    const goParentFolder = () => {
        const parts = folderPath().split("\\").filter(Boolean);
        if (parts.length > 1) {
            loadDirectory(parts.slice(0, -1).join("\\") + "\\");
        }
    };

    // Save camera to backend
    const handleSaveCamera = async (e: Event) => {
        e.preventDefault();

        const cameraData = {
            name: formName(),
            description: formDesc() || null,
            manufacturer: formManufacturer() || null,
            model: formModel() || null,
            firmware: formFirmware() || null,
            enabled: formEnabled(),
            ip_address: formIp(),
            rtsp_port: parseInt(formPort()) || 554,
            username: formUser(),
            password: formPass() || undefined, // Don't send empty password on edit
            transport: formTransport(),
            timeout_ms: parseInt(formTimeout()) || 30000,
            recording_dir: formRecDir() || null,
            audio_enabled: formAudioEnabled(),
            shortcut: formShortcut() || null,
            latitude: parseFloat(formLat()) || 0,
            longitude: parseFloat(formLng()) || 0,
        };

        try {
            const isEdit = !!editingCamera();
            const url = isEdit
                ? `${props.API_URL}/cameras/${editingCamera()!.id}`
                : `${props.API_URL}/cameras`;

            // For edit, only send password if it was changed (not empty)
            const bodyData = isEdit && !formPass()
                ? { ...cameraData, password: undefined }
                : cameraData;

            const res = await fetch(url, {
                method: isEdit ? "PUT" : "POST",
                headers: {
                    "Content-Type": "application/json",
                    "Authorization": `Bearer ${localStorage.getItem("token")}`
                },
                body: JSON.stringify(bodyData)
            });

            if (res.ok) {
                setShowCameraModal(false);
                props.onRefresh();
            } else {
                const err = await res.text();
                console.error("Erro ao salvar câmera:", err);
                alert("Erro ao salvar câmera: " + err);
            }
        } catch (e) {
            console.error(e);
            alert("Erro de conexão");
        }
    };

    // Toggle server expansion
    const toggleServer = (serverId: string) => {
        const current = expandedServers();
        const newSet = new Set(current);
        if (newSet.has(serverId)) {
            newSet.delete(serverId);
        } else {
            newSet.add(serverId);
        }
        setExpandedServers(newSet);
    };

    // Add folder
    const addFolder = () => {
        if (!newFolderName()) return;
        const newFolder: CameraFolder = {
            id: `folder_${Date.now()}`,
            name: newFolderName(),
            isProtected: false,
        };
        setFolders([...folders(), newFolder]);
        setNewFolderName("");
    };

    // Delete folder
    const deleteFolder = () => {
        const selected = selectedFolder();
        if (!selected) return;
        const folder = folders().find(f => f.id === selected);
        if (folder?.isProtected) return;
        setFolders(folders().filter(f => f.id !== selected));
        setSelectedFolder("all");
    };

    // Get cameras for selected folder
    const filteredCameras = () => {
        const folderId = selectedFolder();
        if (folderId === "all") return props.cameras();
        if (folderId === "ungrouped") return props.cameras().filter(c => !c.folder_id);
        return props.cameras().filter(c => c.folder_id === folderId);
    };

    // Stats for dashboard
    const stats = () => ({
        total: props.cameras().length,
        enabled: props.cameras().filter(c => c.enabled).length,
        disabled: props.cameras().filter(c => !c.enabled).length,
    });

    return (
        <>
            <div style="display: flex; flex-direction: column; height: calc(100vh - 100px);">
                {/* Main Content - 3 Columns */}
                <div style="display: flex; flex: 1; overflow: hidden;">

                    {/* Column 1: Navigation Tree */}
                    <div style="width: 220px; background: var(--bg-secondary); border-right: 1px solid var(--border); display: flex; flex-direction: column;">
                        <div style="padding: 12px; border-bottom: 1px solid var(--border);">
                            <h4 style="margin: 0; font-size: 12px; color: var(--text-muted); text-transform: uppercase;">Navegação</h4>
                        </div>

                        <div style="flex: 1; overflow-y: auto; padding: 8px;">
                            {/* SMU Root */}
                            <div style="font-weight: 600; padding: 8px; color: var(--accent-color);">
                                🖥️ SMU
                            </div>

                            {/* Servers */}
                            <For each={props.servers()}>
                                {(server) => (
                                    <>
                                        <div
                                            style={`padding: 8px 8px 8px 16px; cursor: pointer; display: flex; align-items: center; gap: 6px; border-radius: 4px; ${selectedServer()?.id === server.id ? 'background: var(--accent-color); color: white;' : ''}`}
                                            onClick={() => { setSelectedServer(server); toggleServer(server.id); }}
                                        >
                                            <span>{expandedServers().has(server.id) ? '▼' : '▶'}</span>
                                            <span>📟</span>
                                            <span style="font-size: 13px;">{server.name}</span>
                                        </div>

                                        <Show when={expandedServers().has(server.id)}>
                                            <div style="padding-left: 32px;">
                                                <div
                                                    style={`padding: 6px 8px; cursor: pointer; border-radius: 4px; font-size: 12px; ${selectedNavItem() === 'status' ? 'background: var(--bg-tertiary);' : ''}`}
                                                    onClick={() => setSelectedNavItem('status')}
                                                >
                                                    📊 Status
                                                </div>
                                                <div
                                                    style={`padding: 6px 8px; cursor: pointer; border-radius: 4px; font-size: 12px; ${selectedNavItem() === 'cameras' ? 'background: var(--bg-tertiary);' : ''}`}
                                                    onClick={() => setSelectedNavItem('cameras')}
                                                >
                                                    📹 Câmeras
                                                </div>
                                                <div
                                                    style={`padding: 6px 8px; cursor: pointer; border-radius: 4px; font-size: 12px; ${selectedNavItem() === 'recording' ? 'background: var(--bg-tertiary);' : ''}`}
                                                    onClick={() => setSelectedNavItem('recording')}
                                                >
                                                    💾 Gravação
                                                </div>
                                            </div>
                                        </Show>
                                    </>
                                )}
                            </For>
                        </div>

                        {/* Server Toolbar */}
                        <div style="padding: 8px; border-top: 1px solid var(--border); display: flex; gap: 4px;">
                            <button class="btn btn-secondary" style="padding: 6px 10px; font-size: 14px;" title="Adicionar Servidor">➕</button>
                            <button class="btn btn-secondary" style="padding: 6px 10px; font-size: 14px;" title="Atualizar" onClick={props.onRefresh}>🔄</button>
                            <button class="btn btn-secondary" style="padding: 6px 10px; font-size: 14px;" title="Remover">➖</button>
                            <button class="btn btn-secondary" style="padding: 6px 10px; font-size: 14px;" title="Parar">✕</button>
                        </div>
                    </div>

                    {/* Column 2: Folders */}
                    <div style="width: 180px; background: var(--bg-primary); border-right: 1px solid var(--border); display: flex; flex-direction: column;">
                        <div style="padding: 12px; border-bottom: 1px solid var(--border);">
                            <h4 style="margin: 0; font-size: 12px; color: var(--text-muted); text-transform: uppercase;">Pastas</h4>
                        </div>

                        <div style="flex: 1; overflow-y: auto;">
                            <For each={folders()}>
                                {(folder) => (
                                    <div
                                        style={`padding: 10px 12px; cursor: pointer; display: flex; align-items: center; gap: 8px; border-bottom: 1px solid var(--border); ${selectedFolder() === folder.id ? 'background: var(--accent-color); color: white;' : ''}`}
                                        onClick={() => setSelectedFolder(folder.id)}
                                    >
                                        <span>{folder.isProtected ? '📁' : '📂'}</span>
                                        <span style="font-size: 13px;">{folder.name}</span>
                                    </div>
                                )}
                            </For>
                        </div>

                        {/* Folder Toolbar */}
                        <div style="padding: 8px; border-top: 1px solid var(--border);">
                            <div style="display: flex; gap: 4px; margin-bottom: 8px;">
                                <input
                                    type="text"
                                    value={newFolderName()}
                                    onInput={(e) => setNewFolderName(e.currentTarget.value)}
                                    placeholder="Nova pasta"
                                    style="flex: 1; padding: 6px; font-size: 12px;"
                                />
                            </div>
                            <div style="display: flex; gap: 4px;">
                                <button class="btn btn-secondary" style="flex: 1; padding: 6px; font-size: 12px;" onClick={addFolder}>+ Adicionar</button>
                                <button class="btn btn-danger" style="flex: 1; padding: 6px; font-size: 12px;" onClick={deleteFolder}>- Excluir</button>
                            </div>
                        </div>
                    </div>

                    {/* Column 3: Content (Camera List or Dashboard) */}
                    <div style="flex: 1; display: flex; flex-direction: column; overflow: hidden;">

                        <Show when={selectedNavItem() === 'status'}>
                            {/* Dashboard */}
                            <div style="padding: 24px;">
                                <h2 style="margin-bottom: 24px;">📊 Monitoramento do Servidor</h2>
                                <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;">
                                    <div class="card" style="text-align: center; padding: 24px;">
                                        <div style="font-size: 48px; font-weight: 700; color: var(--accent-color);">{stats().total}</div>
                                        <div style="color: var(--text-muted);">Total</div>
                                    </div>
                                    <div class="card" style="text-align: center; padding: 24px;">
                                        <div style="font-size: 48px; font-weight: 700; color: #10b981;">{stats().enabled}</div>
                                        <div style="color: var(--text-muted);">Ativadas</div>
                                    </div>
                                    <div class="card" style="text-align: center; padding: 24px;">
                                        <div style="font-size: 48px; font-weight: 700; color: #ef4444;">{stats().disabled}</div>
                                        <div style="color: var(--text-muted);">Desativadas</div>
                                    </div>
                                </div>
                            </div>
                        </Show>

                        <Show when={selectedNavItem() === 'cameras'}>
                            {/* Camera List */}
                            <div style="padding: 12px; border-bottom: 1px solid var(--border); display: flex; align-items: center; gap: 12px;">
                                <input type="text" placeholder="🔍 Pesquisar" style="padding: 8px 12px; width: 200px;" />
                            </div>

                            <div style="flex: 1; overflow-y: auto;">
                                <table class="table">
                                    <thead>
                                        <tr>
                                            <th>Nome</th>
                                            <th>Descrição</th>
                                            <th>Em Funcionamento</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <For each={filteredCameras()}>
                                            {(camera) => (
                                                <tr
                                                    style={`cursor: pointer; ${selectedCamera()?.id === camera.id ? 'background: var(--accent-color); color: white;' : ''}`}
                                                    onClick={() => setSelectedCamera(camera)}
                                                    onDblClick={() => openCameraForm(camera)}
                                                >
                                                    <td>
                                                        <span style="display: flex; align-items: center; gap: 8px;">
                                                            📹 {camera.name}
                                                        </span>
                                                    </td>
                                                    <td>{camera.description || '-'}</td>
                                                    <td>
                                                        <span class={`status ${camera.enabled ? 'status-online' : 'status-offline'}`}>
                                                            {camera.enabled ? '✅' : '❌'}
                                                        </span>
                                                    </td>
                                                </tr>
                                            )}
                                        </For>
                                        <Show when={filteredCameras().length === 0}>
                                            <tr>
                                                <td colspan="3" style="text-align: center; color: var(--text-muted); padding: 24px;">
                                                    Nenhuma câmera nesta pasta
                                                </td>
                                            </tr>
                                        </Show>
                                    </tbody>
                                </table>
                            </div>

                            {/* Camera Toolbar */}
                            <div style="padding: 12px; border-top: 1px solid var(--border); display: flex; gap: 8px; justify-content: flex-end;">
                                <button class="btn btn-primary" onClick={() => { setEditingCamera(null); setShowCameraModal(true); }}>Adicionar</button>
                                <button class="btn btn-secondary">Alterar</button>
                                <button class="btn btn-danger">Excluir</button>
                                <button class="btn btn-secondary">Importar</button>
                                <button class="btn btn-secondary">Procurar</button>
                                <button class="btn btn-secondary">Exportar</button>
                            </div>
                        </Show>

                        <Show when={selectedNavItem() === 'recording'}>
                            <div style="padding: 24px;">
                                <h2>💾 Configuração de Gravação</h2>
                                <p style="color: var(--text-muted);">Configurações de gravação do servidor selecionado.</p>
                            </div>
                        </Show>
                    </div>
                </div>
            </div>

            {/* Camera Form Modal */}
            <Show when={showCameraModal()}>
                <div class="modal-overlay" onClick={() => setShowCameraModal(false)}>
                    <div class="modal-content" onClick={(e) => e.stopPropagation()} style="max-width: 900px; display: flex; padding: 0;">
                        {/* Sidebar - Digifort style */}
                        <div style="width: 180px; background: var(--bg-secondary); border-right: 1px solid var(--border); padding: 16px 0;">
                            <div style="padding: 8px 16px; font-weight: 600; color: var(--accent-color);">📹 Câmera</div>
                            <div style="padding: 6px 16px 6px 24px; font-size: 13px; background: var(--bg-tertiary);">Geral</div>
                            <div style="padding: 6px 16px 6px 24px; font-size: 13px; color: var(--text-muted);">Lentes</div>
                            <div style="padding: 6px 16px 6px 24px; font-size: 13px; color: var(--text-muted);">Detecção</div>

                            <div style="padding: 8px 16px; font-weight: 600; color: var(--accent-color); margin-top: 8px;">🎥 Streaming</div>
                            <div style="padding: 6px 16px 6px 24px; font-size: 13px; color: var(--text-muted);">Perfis</div>
                            <div style="padding: 6px 16px 6px 24px; font-size: 13px; color: var(--text-muted);">Visualização</div>

                            <div style="padding: 8px 16px; font-weight: 600; color: var(--accent-color); margin-top: 8px;">💾 Gravação</div>
                            <div style="padding: 6px 16px 6px 24px; font-size: 13px; color: var(--text-muted);">Configurações</div>
                            <div style="padding: 6px 16px 6px 24px; font-size: 13px; color: var(--text-muted);">Arquivamento</div>

                            <div style="padding: 8px 16px; font-weight: 600; color: var(--accent-color); margin-top: 8px;">🔒 Direitos</div>
                            <div style="padding: 6px 16px 6px 24px; font-size: 13px; color: var(--text-muted);">Usuários</div>
                        </div>

                        {/* Form Content */}
                        <div style="flex: 1; padding: 24px; overflow-y: auto; max-height: 80vh;">
                            <div class="modal-header" style="margin-bottom: 20px;">
                                <h2>{editingCamera() ? 'Alterar Câmera' : 'Nova Câmera'}</h2>
                                <button class="modal-close" onClick={() => setShowCameraModal(false)}>×</button>
                            </div>

                            <form onSubmit={handleSaveCamera}>
                                {/* Nome e Descrição */}
                                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-bottom: 16px;">
                                    <div class="form-group">
                                        <label style="font-weight: 500;">Nome da Câmera *</label>
                                        <input type="text" value={formName()} onInput={(e) => setFormName(e.currentTarget.value)} style="width: 100%; padding: 10px;" />
                                    </div>
                                    <div class="form-group">
                                        <label style="font-weight: 500;">Descrição</label>
                                        <input type="text" value={formDesc()} onInput={(e) => setFormDesc(e.currentTarget.value)} style="width: 100%; padding: 10px;" />
                                    </div>
                                </div>

                                {/* Fabricante, Modelo, Firmware */}
                                <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px; margin-bottom: 16px;">
                                    <div class="form-group">
                                        <label style="font-weight: 500;">Fabricante</label>
                                        <select value={formManufacturer()} onChange={(e) => setFormManufacturer(e.currentTarget.value)} style="width: 100%; padding: 10px;">
                                            <option value="">Selecione</option>
                                            <option>Hikvision</option>
                                            <option>Dahua</option>
                                            <option>Intelbras</option>
                                            <option>TP-Link</option>
                                            <option>Outro</option>
                                        </select>
                                    </div>
                                    <div class="form-group">
                                        <label style="font-weight: 500;">Modelo</label>
                                        <select value={formModel()} onChange={(e) => setFormModel(e.currentTarget.value)} style="width: 100%; padding: 10px;">
                                            <option value="">Selecione</option>
                                            <For each={getModels(formManufacturer())}>
                                                {(model) => <option value={model}>{model}</option>}
                                            </For>
                                        </select>
                                    </div>
                                    <div class="form-group">
                                        <label style="font-weight: 500;">Firmware</label>
                                        <select value={formFirmware()} onChange={(e) => setFormFirmware(e.currentTarget.value)} style="width: 100%; padding: 10px;">
                                            <option value="">Selecione</option>
                                            <For each={getFirmwares(formManufacturer(), formModel())}>
                                                {(fw) => <option value={fw}>{fw}</option>}
                                            </For>
                                        </select>
                                    </div>
                                </div>

                                {/* Credenciais de Conexão */}
                                <div style="background: var(--bg-secondary); padding: 16px; border-radius: 8px; margin-bottom: 16px;">
                                    <h4 style="margin: 0 0 12px 0; font-size: 14px; color: var(--accent-color);">🔐 Credenciais de Conexão</h4>
                                    <div style="display: grid; grid-template-columns: 2fr 1fr; gap: 12px; margin-bottom: 12px;">
                                        <div class="form-group">
                                            <label>Endereço IP *</label>
                                            <input type="text" value={formIp()} onInput={(e) => setFormIp(e.currentTarget.value)} style="width: 100%; padding: 10px;" />
                                        </div>
                                        <div class="form-group">
                                            <label>Porta RTSP</label>
                                            <input type="number" value={formPort()} onInput={(e) => setFormPort(e.currentTarget.value)} style="width: 100%; padding: 10px;" />
                                        </div>
                                    </div>
                                    <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px;">
                                        <div class="form-group">
                                            <label>Usuário *</label>
                                            <input type="text" value={formUser()} onInput={(e) => setFormUser(e.currentTarget.value)} style="width: 100%; padding: 10px;" />
                                        </div>
                                        <div class="form-group">
                                            <label>Senha *</label>
                                            <input type="password" value={formPass()} onInput={(e) => setFormPass(e.currentTarget.value)} style="width: 100%; padding: 10px;" />
                                        </div>
                                        <div class="form-group">
                                            <label>Transporte</label>
                                            <select value={formTransport()} onChange={(e) => setFormTransport(e.currentTarget.value)} style="width: 100%; padding: 10px;">
                                                <option value="auto">Auto</option>
                                                <option value="tcp">TCP</option>
                                                <option value="udp">UDP</option>
                                            </select>
                                        </div>
                                    </div>
                                </div>

                                {/* Localização */}
                                <div style="display: grid; grid-template-columns: 2fr 1fr 1fr; gap: 12px; margin-bottom: 16px;">
                                    <div class="form-group">
                                        <label>Atalho no Cliente</label>
                                        <input type="text" value={formShortcut()} onInput={(e) => setFormShortcut(e.currentTarget.value)} style="width: 100%; padding: 10px;" />
                                    </div>
                                    <div class="form-group">
                                        <label>Latitude</label>
                                        <input type="text" value={formLat()} onInput={(e) => setFormLat(e.currentTarget.value)} style="width: 100%; padding: 10px;" />
                                    </div>
                                    <div class="form-group">
                                        <label>Longitude</label>
                                        <input type="text" value={formLng()} onInput={(e) => setFormLng(e.currentTarget.value)} style="width: 100%; padding: 10px;" />
                                    </div>
                                </div>

                                {/* Gravação */}
                                <div style="display: grid; grid-template-columns: 2fr 1fr; gap: 12px; margin-bottom: 16px;">
                                    <div class="form-group">
                                        <label>Diretório de Gravação</label>
                                        <div style="display: flex; gap: 8px;">
                                            <input type="text" value={formRecDir()} onInput={(e) => setFormRecDir(e.currentTarget.value)} style="flex: 1; padding: 10px;" />
                                            <button type="button" class="btn btn-secondary" style="padding: 10px;" onClick={() => { loadDirectory(formRecDir() || "C:\\"); setShowFolderPicker(true); }}>📁</button>
                                        </div>
                                    </div>
                                    <div class="form-group">
                                        <label>Timeout (ms)</label>
                                        <input type="number" value={formTimeout()} onInput={(e) => setFormTimeout(e.currentTarget.value)} style="width: 100%; padding: 10px;" />
                                    </div>
                                </div>

                                {/* Observações */}
                                <div class="form-group" style="margin-bottom: 16px;">
                                    <label>Observações Gerais</label>
                                    <textarea rows="2" style="width: 100%; padding: 10px; resize: vertical;"></textarea>
                                </div>

                                {/* Streaming Configuration */}
                                <div style="background: var(--bg-secondary); padding: 16px; border-radius: 8px; margin-bottom: 16px;">
                                    <h4 style="margin: 0 0 12px 0; font-size: 14px; color: var(--accent-color);">🎥 Configurações de Streaming</h4>
                                    <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px; margin-bottom: 12px;">
                                        <div class="form-group">
                                            <label>Compressão de Vídeo</label>
                                            <select value={formCodec()} onChange={(e) => setFormCodec(e.currentTarget.value)} style="width: 100%; padding: 10px;">
                                                <option value="h264">H.264 (AVC)</option>
                                                <option value="h265">H.265 (HEVC)</option>
                                                <option value="mjpeg">MJPEG</option>
                                            </select>
                                        </div>
                                        <div class="form-group">
                                            <label>Stream</label>
                                            <select value={formStreamType()} onChange={(e) => setFormStreamType(e.currentTarget.value)} style="width: 100%; padding: 10px;">
                                                <option value="main">Stream Principal (4K)</option>
                                                <option value="sub">Stream Secundário</option>
                                            </select>
                                        </div>

                                    </div>
                                    <div style="padding: 10px; background: var(--bg-tertiary); border-radius: 6px; font-size: 12px; color: var(--text-muted);">
                                        <strong>Informações do Stream:</strong> Resolução até 4K • Latência alvo: 80ms • Codec: {formCodec().toUpperCase()}
                                    </div>
                                </div>

                                {/* Ativar Câmera e Áudio */}
                                <div style="padding: 12px; background: var(--bg-secondary); border-radius: 8px; margin-bottom: 20px; display: flex; flex-direction: column; gap: 12px;">
                                    <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                                        <input type="checkbox" checked={formEnabled()} onChange={(e) => setFormEnabled(e.currentTarget.checked)} style="width: 18px; height: 18px;" />
                                        <span style="font-weight: 500;">Ativar Câmera</span>
                                        <span style="font-size: 12px; color: var(--text-muted);">(habilita streaming e gravação)</span>
                                    </label>
                                    <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                                        <input type="checkbox" checked={formAudioEnabled()} onChange={(e) => setFormAudioEnabled(e.currentTarget.checked)} style="width: 18px; height: 18px;" />
                                        <span style="font-weight: 500;">Habilitar Áudio</span>
                                        <span style="font-size: 12px; color: var(--text-muted);">(grava MP4 com áudio)</span>
                                    </label>
                                </div>

                                {/* Botões */}
                                <div style="display: flex; justify-content: flex-end; gap: 12px;">
                                    <button
                                        type="button"
                                        class="btn btn-success"
                                        style="background: #22c55e; margin-right: auto;"
                                        onClick={async () => {
                                            const rtspUrl = `rtsp://${formIp()}:${formPort() || 554}/stream1`;
                                            try {
                                                const result = await invoke('open_player', {
                                                    rtspUrl: rtspUrl,
                                                    username: formUser(),
                                                    password: formPass(),
                                                    width: 960,
                                                    height: 540
                                                });
                                                console.log('Player started:', result);
                                            } catch (e) {
                                                console.error('Failed to start player:', e);
                                                alert('Erro ao abrir player: ' + e);
                                            }
                                        }}
                                    >
                                        🎬 Visualizar
                                    </button>
                                    <button type="button" class="btn btn-secondary" onClick={() => setShowCameraModal(false)}>Cancelar</button>
                                    <button type="submit" class="btn btn-primary">OK</button>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
            </Show>

            {/* Folder Picker Modal */}
            <Show when={showFolderPicker()}>
                <div class="modal-overlay" onClick={() => setShowFolderPicker(false)}>
                    <div class="modal-content" onClick={(e) => e.stopPropagation()} style="max-width: 550px;">
                        <div class="modal-header">
                            <h3>📁 Selecionar Diretório de Gravação</h3>
                            <button class="modal-close" onClick={() => setShowFolderPicker(false)}>×</button>
                        </div>

                        <div style="padding: 12px; background: var(--bg-secondary); border-radius: 6px; font-family: monospace; margin-bottom: 12px;">
                            {folderPath()}
                        </div>

                        <div style="display: flex; gap: 6px; margin-bottom: 12px; flex-wrap: wrap;">
                            <For each={folderDrives()}>
                                {(drive) => (
                                    <button
                                        class={`btn ${folderPath().startsWith(drive) ? "btn-primary" : "btn-secondary"}`}
                                        style="padding: 6px 12px; font-size: 12px;"
                                        onClick={() => loadDirectory(drive)}
                                    >
                                        💾 {drive}
                                    </button>
                                )}
                            </For>
                        </div>

                        <button class="btn btn-secondary" onClick={goParentFolder} style="margin-bottom: 12px;">⬆️ Voltar</button>

                        <div style="max-height: 250px; overflow-y: auto; border: 1px solid var(--border); border-radius: 6px; margin-bottom: 12px;">
                            <For each={folderEntries()}>
                                {(entry) => (
                                    <div style="padding: 10px; cursor: pointer; border-bottom: 1px solid var(--border); display: flex; align-items: center; gap: 8px;"
                                        onClick={() => loadDirectory(entry.path)}
                                        onMouseOver={(e) => (e.currentTarget.style.background = 'var(--bg-secondary)')}
                                        onMouseOut={(e) => (e.currentTarget.style.background = '')}
                                    >
                                        📁 {entry.name}
                                    </div>
                                )}
                            </For>
                            <Show when={folderEntries().length === 0}>
                                <div style="padding: 20px; text-align: center; color: var(--text-muted);">Nenhuma pasta</div>
                            </Show>
                        </div>

                        <div style="display: flex; gap: 6px; margin-bottom: 16px;">
                            <input type="text" value={newRecFolderName()} onInput={(e) => setNewRecFolderName(e.currentTarget.value)} placeholder="Nova pasta" style="flex: 1; padding: 8px;" />
                            <button class="btn btn-secondary" onClick={createRecFolder}>➕ Criar</button>
                        </div>

                        <div style="display: flex; justify-content: flex-end; gap: 8px;">
                            <button class="btn btn-secondary" onClick={() => setShowFolderPicker(false)}>Cancelar</button>
                            <button class="btn btn-primary" onClick={() => { setFormRecDir(folderPath()); setShowFolderPicker(false); }}>Selecionar</button>
                        </div>
                    </div>
                </div>
            </Show>

            {/* Preview removido - usar botão Visualizar (GStreamer) */}
        </>
    );
}
