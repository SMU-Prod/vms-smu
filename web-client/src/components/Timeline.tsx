import { Component, createSignal, For, Show, createResource } from 'solid-js';

interface TimelineProps {
    cameraId: string | null;
}

interface TimelineSegment {
    start: string;
    end: string;
    duration_seconds: number;
    has_video: boolean;
}

const Timeline: Component<TimelineProps> = (props) => {
    const [currentTime, setCurrentTime] = createSignal(new Date());
    const [isPlaying, setIsPlaying] = createSignal(false);

    // Generate hour labels
    const hours = Array.from({ length: 25 }, (_, i) =>
        `${String(i).padStart(2, '0')}:00`
    );

    // Mock timeline segments
    const segments = () => [
        { start: 0, width: 35, hasVideo: true },   // 00:00 - 08:24
        { start: 40, width: 20, hasVideo: true },  // 09:36 - 14:24
        { start: 65, width: 35, hasVideo: true },  // 15:36 - 24:00
    ];

    const cursorPosition = () => {
        const now = new Date();
        const percent = (now.getHours() * 60 + now.getMinutes()) / (24 * 60) * 100;
        return percent;
    };

    return (
        <div class="timeline">
            <div class="timeline-header">
                <span class="timeline-title">
                    📅 {currentTime().toLocaleDateString('pt-BR')}
                </span>

                <div class="timeline-controls">
                    <button class="timeline-btn" title="Início">⏮️</button>
                    <button class="timeline-btn" title="Voltar">⏪</button>
                    <button
                        class="timeline-btn"
                        title={isPlaying() ? 'Pausar' : 'Play'}
                        onClick={() => setIsPlaying(!isPlaying())}
                    >
                        {isPlaying() ? '⏸️' : '▶️'}
                    </button>
                    <button class="timeline-btn" title="Avançar">⏩</button>
                    <button class="timeline-btn" title="Fim">⏭️</button>
                </div>
            </div>

            <div class="timeline-track">
                <For each={segments()}>
                    {(seg) => (
                        <div
                            class={`timeline-segment ${seg.hasVideo ? 'has-video' : ''}`}
                            style={{
                                left: `${seg.start}%`,
                                width: `${seg.width}%`,
                            }}
                        />
                    )}
                </For>

                <div
                    class="timeline-cursor"
                    style={{ left: `${cursorPosition()}%` }}
                />
            </div>

            <div class="timeline-hours">
                <For each={[0, 6, 12, 18, 24]}>
                    {(hour) => (
                        <span>{String(hour).padStart(2, '0')}:00</span>
                    )}
                </For>
            </div>
        </div>
    );
};

export default Timeline;
