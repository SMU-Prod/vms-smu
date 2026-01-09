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
    const [selectedCamera, setSelectedCamera] = createSignal<Camera | null>(null);

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
                                                    onDblClick={() => { setEditingCamera(camera); setShowCameraModal(true); }}
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

                            <form>
                                {/* Nome e Descrição */}
                                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-bottom: 16px;">
                                    <div class="form-group">
                                        <label style="font-weight: 500;">Nome da Câmera *</label>
                                        <input type="text" value={editingCamera()?.name || ''} style="width: 100%; padding: 10px;" />
                                    </div>
                                    <div class="form-group">
                                        <label style="font-weight: 500;">Descrição</label>
                                        <input type="text" value={editingCamera()?.description || ''} style="width: 100%; padding: 10px;" />
                                    </div>
                                </div>

                                {/* Fabricante, Modelo, Firmware */}
                                <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px; margin-bottom: 16px;">
                                    <div class="form-group">
                                        <label style="font-weight: 500;">Fabricante</label>
                                        <select style="width: 100%; padding: 10px;">
                                            <option>Selecione</option>
                                            <option>Hikvision</option>
                                            <option>Dahua</option>
                                            <option>Intelbras</option>
                                            <option>TP-Link</option>
                                        </select>
                                    </div>
                                    <div class="form-group">
                                        <label style="font-weight: 500;">Modelo</label>
                                        <select style="width: 100%; padding: 10px;">
                                            <option>Selecione</option>
                                        </select>
                                    </div>
                                    <div class="form-group">
                                        <label style="font-weight: 500;">Firmware</label>
                                        <select style="width: 100%; padding: 10px;">
                                            <option>Selecione</option>
                                        </select>
                                    </div>
                                </div>

                                {/* Credenciais de Conexão */}
                                <div style="background: var(--bg-secondary); padding: 16px; border-radius: 8px; margin-bottom: 16px;">
                                    <h4 style="margin: 0 0 12px 0; font-size: 14px; color: var(--accent-color);">🔐 Credenciais de Conexão</h4>
                                    <div style="display: grid; grid-template-columns: 2fr 1fr; gap: 12px; margin-bottom: 12px;">
                                        <div class="form-group">
                                            <label>Endereço IP *</label>
                                            <input type="text" value={editingCamera()?.ip_address || ''} style="width: 100%; padding: 10px;" />
                                        </div>
                                        <div class="form-group">
                                            <label>Porta RTSP</label>
                                            <input type="number" value="554" style="width: 100%; padding: 10px;" />
                                        </div>
                                    </div>
                                    <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px;">
                                        <div class="form-group">
                                            <label>Usuário *</label>
                                            <input type="text" style="width: 100%; padding: 10px;" />
                                        </div>
                                        <div class="form-group">
                                            <label>Senha *</label>
                                            <input type="password" style="width: 100%; padding: 10px;" />
                                        </div>
                                        <div class="form-group">
                                            <label>Transporte</label>
                                            <select style="width: 100%; padding: 10px;">
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
                                        <input type="text" style="width: 100%; padding: 10px;" />
                                    </div>
                                    <div class="form-group">
                                        <label>Latitude</label>
                                        <input type="text" value="0.000000" style="width: 100%; padding: 10px;" />
                                    </div>
                                    <div class="form-group">
                                        <label>Longitude</label>
                                        <input type="text" value="0.000000" style="width: 100%; padding: 10px;" />
                                    </div>
                                </div>

                                {/* Gravação */}
                                <div style="display: grid; grid-template-columns: 2fr 1fr; gap: 12px; margin-bottom: 16px;">
                                    <div class="form-group">
                                        <label>Diretório de Gravação</label>
                                        <div style="display: flex; gap: 8px;">
                                            <input type="text" style="flex: 1; padding: 10px;" />
                                            <button type="button" class="btn btn-secondary" style="padding: 10px;">📁</button>
                                        </div>
                                    </div>
                                    <div class="form-group">
                                        <label>Timeout (ms)</label>
                                        <input type="number" value="30000" style="width: 100%; padding: 10px;" />
                                    </div>
                                </div>

                                {/* Observações */}
                                <div class="form-group" style="margin-bottom: 16px;">
                                    <label>Observações Gerais</label>
                                    <textarea rows="3" style="width: 100%; padding: 10px; resize: vertical;"></textarea>
                                </div>

                                {/* Ativar Câmera */}
                                <div style="padding: 12px; background: var(--bg-secondary); border-radius: 8px; margin-bottom: 20px;">
                                    <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                                        <input type="checkbox" checked={editingCamera()?.enabled || false} style="width: 18px; height: 18px;" />
                                        <span style="font-weight: 500;">✅ Ativar Câmera</span>
                                    </label>
                                </div>

                                {/* Botões */}
                                <div style="display: flex; justify-content: flex-end; gap: 12px;">
                                    <button type="button" class="btn btn-secondary" onClick={() => setShowCameraModal(false)}>Cancelar</button>
                                    <button type="submit" class="btn btn-primary">OK</button>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
            </Show>
        </>
    );
}
