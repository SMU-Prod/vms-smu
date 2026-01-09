import { createSignal, For, Show } from "solid-js";

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
    manufacturer?: string;
    enabled: boolean;
    server_id?: string;
    folder_id?: string;
}

interface Props {
    servers: () => Server[];
    cameras: () => Camera[];
    API_URL: string;
    onRefresh: () => void;
}

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
                                                style="cursor: pointer;"
                                                onClick={() => { setEditingCamera(camera); setShowCameraModal(true); }}
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
    );
}
