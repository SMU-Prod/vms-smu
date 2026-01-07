import { Component, For, createResource, createSignal, onMount } from 'solid-js';
import type { GridLayout, ViewMode } from '../App';
import CameraTile from './CameraTile';

interface Camera {
    id: string;
    name: string;
    rtsp_url: string;
    enabled: boolean;
    username?: string;
    password?: string;
}

interface CameraGridProps {
    layout: GridLayout;
    viewMode: ViewMode;
}

const CameraGrid: Component<CameraGridProps> = (props) => {
    const [cameras, setCameras] = createSignal<Camera[]>([]);
    const [loading, setLoading] = createSignal(true);

    // Fetch real cameras from API
    onMount(async () => {
        try {
            const response = await fetch('http://localhost:9095/api/v1/cameras');
            if (response.ok) {
                const data = await response.json();
                setCameras(data.filter((c: Camera) => c.enabled));
                console.log('[CameraGrid] Loaded cameras:', data);
            }
        } catch (error) {
            console.error('[CameraGrid] Failed to fetch cameras:', error);
        } finally {
            setLoading(false);
        }
    });

    // Get cameras to display based on layout
    const displayCameras = () => {
        const allCameras = cameras();
        const layoutCount = props.layout;

        // If we have real cameras, only show each camera ONCE (don't repeat!)
        if (allCameras.length > 0) {
            // Return only unique cameras up to layoutCount
            return allCameras.slice(0, layoutCount);
        }

        // Fallback to empty placeholders
        return Array.from({ length: layoutCount }, (_, i) => ({
            id: `placeholder-${i}`,
            name: `Camera ${i + 1}`,
            rtsp_url: '',
            enabled: false,
        }));
    };

    return (
        <div class={`camera-grid layout-${props.layout}`}>
            {loading() ? (
                <div style={{
                    display: 'flex',
                    'align-items': 'center',
                    'justify-content': 'center',
                    color: 'var(--text-secondary)',
                    'grid-column': '1 / -1',
                    'grid-row': '1 / -1'
                }}>
                    Carregando câmeras...
                </div>
            ) : (
                <For each={displayCameras()}>
                    {(camera, index) => (
                        <CameraTile
                            id={camera.id}
                            name={camera.name}
                            rtspUrl={camera.rtsp_url}
                            username={camera.username || 'adminsmu'}
                            password={camera.password || 'Naotemsenha1@'}
                            isLive={props.viewMode === 'live' && camera.enabled}
                        />
                    )}
                </For>
            )}
        </div>
    );
};

export default CameraGrid;
