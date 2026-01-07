import { Component, createSignal } from 'solid-js';
import Header from './components/Header';
import Sidebar from './components/Sidebar';
import CameraGrid from './components/CameraGrid';
import Timeline from './components/Timeline';

export type ViewMode = 'live' | 'playback';
export type GridLayout = 1 | 4 | 9 | 16;

const App: Component = () => {
    const [viewMode, setViewMode] = createSignal<ViewMode>('live');
    const [gridLayout, setGridLayout] = createSignal<GridLayout>(4);
    const [selectedCamera, setSelectedCamera] = createSignal<string | null>(null);

    return (
        <div class="app">
            <Header
                viewMode={viewMode()}
                onViewModeChange={setViewMode}
                gridLayout={gridLayout()}
                onGridLayoutChange={setGridLayout}
            />

            <div class="main-content">
                <Sidebar
                    onCameraSelect={setSelectedCamera}
                    selectedCamera={selectedCamera()}
                />

                <div class="content-area">
                    <CameraGrid
                        layout={gridLayout()}
                        viewMode={viewMode()}
                    />

                    {viewMode() === 'playback' && (
                        <Timeline
                            cameraId={selectedCamera()}
                        />
                    )}
                </div>
            </div>
        </div>
    );
};

export default App;
