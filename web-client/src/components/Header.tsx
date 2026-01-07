import { Component } from 'solid-js';
import type { ViewMode, GridLayout } from '../App';

interface HeaderProps {
    viewMode: ViewMode;
    onViewModeChange: (mode: ViewMode) => void;
    gridLayout: GridLayout;
    onGridLayoutChange: (layout: GridLayout) => void;
}

const Header: Component<HeaderProps> = (props) => {
    return (
        <header class="header">
            <div class="header-left">
                <div class="logo">
                    <svg class="logo-icon" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z" />
                    </svg>
                    <span>VMS Pro</span>
                </div>

                <div class="view-toggle">
                    <button
                        class={`view-btn ${props.viewMode === 'live' ? 'active' : ''}`}
                        onClick={() => props.onViewModeChange('live')}
                    >
                        🔴 Ao Vivo
                    </button>
                    <button
                        class={`view-btn ${props.viewMode === 'playback' ? 'active' : ''}`}
                        onClick={() => props.onViewModeChange('playback')}
                    >
                        ⏪ Playback
                    </button>
                </div>
            </div>

            <div class="header-right">
                <div class="grid-selector">
                    {([1, 4, 9, 16] as GridLayout[]).map((layout) => (
                        <button
                            class={`grid-btn ${props.gridLayout === layout ? 'active' : ''}`}
                            onClick={() => props.onGridLayoutChange(layout)}
                            title={`${layout} câmera${layout > 1 ? 's' : ''}`}
                        >
                            {layout === 1 && '1'}
                            {layout === 4 && '2×2'}
                            {layout === 9 && '3×3'}
                            {layout === 16 && '4×4'}
                        </button>
                    ))}
                </div>

                <button class="view-btn" title="Configurações">
                    ⚙️
                </button>
            </div>
        </header>
    );
};

export default Header;
