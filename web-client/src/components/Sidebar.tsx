import { Component, createResource, For } from 'solid-js';

interface Camera {
    id: string;
    name: string;
    location?: string;
    status: 'online' | 'offline';
}

interface SidebarProps {
    selectedCamera: string | null;
    onCameraSelect: (id: string | null) => void;
}

const fetchCameras = async (): Promise<Camera[]> => {
    try {
        const response = await fetch('/api/v1/cameras');
        if (!response.ok) throw new Error('Failed to fetch cameras');
        return response.json();
    } catch (error) {
        console.error('Error fetching cameras:', error);
        // Return mock data for now
        return [
            { id: '1', name: 'Entrada Principal', location: 'Portaria', status: 'online' },
            { id: '2', name: 'Estacionamento', location: 'Área Externa', status: 'online' },
            { id: '3', name: 'Recepção', location: 'Térreo', status: 'online' },
            { id: '4', name: 'Corredor A', location: '1º Andar', status: 'offline' },
            { id: '5', name: 'Sala de Reunião', location: '2º Andar', status: 'online' },
            { id: '6', name: 'CPD', location: 'Subsolo', status: 'online' },
        ];
    }
};

const Sidebar: Component<SidebarProps> = (props) => {
    const [cameras] = createResource(fetchCameras);

    return (
        <aside class="sidebar">
            <div class="sidebar-header">
                <h2 class="sidebar-title">Câmeras ({cameras()?.length || 0})</h2>
            </div>

            <div class="camera-list">
                <For each={cameras()}>
                    {(camera) => (
                        <div
                            class={`camera-item ${props.selectedCamera === camera.id ? 'selected' : ''}`}
                            onClick={() => props.onCameraSelect(camera.id)}
                        >
                            <div class={`camera-status ${camera.status}`} />
                            <div class="camera-info">
                                <div class="camera-name">{camera.name}</div>
                                {camera.location && (
                                    <div class="camera-location">{camera.location}</div>
                                )}
                            </div>
                        </div>
                    )}
                </For>
            </div>
        </aside>
    );
};

export default Sidebar;
