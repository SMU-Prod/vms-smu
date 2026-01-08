import { Component, createSignal, onMount, onCleanup } from 'solid-js';

interface CameraTileProps {
    id: string;
    name: string;
    rtspUrl: string;
    username?: string;
    password?: string;
    isLive: boolean;
}

const CameraTile: Component<CameraTileProps> = (props) => {
    let videoRef: HTMLVideoElement | undefined;
    const [isConnected, setIsConnected] = createSignal(false);
    const [error, setError] = createSignal<string | null>(null);
    const [connectionState, setConnectionState] = createSignal('connecting');

    let peerConnection: RTCPeerConnection | null = null;

    const VMS_STREAM_URL = 'http://localhost:9095';

    const startWebRTC = async () => {
        try {
            console.log(`[CameraTile ${props.id}] Starting WebRTC connection...`);
            setConnectionState('connecting');

            // Create peer connection
            peerConnection = new RTCPeerConnection({
                iceServers: [
                    { urls: 'stun:stun.l.google.com:19302' }
                ]
            });

            // Handle incoming video track
            peerConnection.ontrack = (event) => {
                console.log(`[CameraTile ${props.id}] Received video track!`);
                if (videoRef && event.streams[0]) {
                    videoRef.srcObject = event.streams[0];
                    setIsConnected(true);
                    setConnectionState('connected');
                    setError(null);
                }
            };

            // Handle connection state changes
            peerConnection.onconnectionstatechange = () => {
                const state = peerConnection?.connectionState;
                console.log(`[CameraTile ${props.id}] Connection state: ${state}`);
                setConnectionState(state || 'unknown');

                if (state === 'connected') {
                    setIsConnected(true);
                } else if (state === 'failed' || state === 'disconnected') {
                    setIsConnected(false);
                    setError('Conexão perdida');
                }
            };

            // Handle ICE candidates
            peerConnection.onicecandidate = async (event) => {
                if (event.candidate) {
                    try {
                        await fetch(`${VMS_STREAM_URL}/api/v1/webrtc/ice/${props.id}`, {
                            method: 'POST',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify({
                                camera_id: props.id,
                                candidate: event.candidate.candidate,
                                sdpMid: event.candidate.sdpMid,
                                sdpMLineIndex: event.candidate.sdpMLineIndex
                            })
                        });
                    } catch (e) {
                        console.warn(`[CameraTile ${props.id}] ICE candidate send error:`, e);
                    }
                }
            };

            // Add transceiver for video receiving
            peerConnection.addTransceiver('video', { direction: 'recvonly' });

            // Create offer
            const offer = await peerConnection.createOffer();
            await peerConnection.setLocalDescription(offer);

            console.log(`[CameraTile ${props.id}] Sending SDP offer to vms-stream...`);

            // Send offer to vms-stream
            const response = await fetch(`${VMS_STREAM_URL}/api/v1/webrtc/offer/${props.id}`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    camera_id: props.id,
                    rtsp_url: props.rtspUrl,
                    username: props.username || 'admin',
                    password: props.password || 'admin',
                    sdp: offer.sdp,
                    type: offer.type
                })
            });

            if (!response.ok) {
                // HTTP 409 means session already exists - wait for it and retry
                if (response.status === 409) {
                    console.log(`[CameraTile ${props.id}] Session exists, waiting for server to be ready...`);
                    await new Promise(r => setTimeout(r, 2000)); // Wait 2s
                    // Retry once to get the SDP answer
                    const retryResponse = await fetch(`${VMS_STREAM_URL}/api/v1/webrtc/offer`, {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({
                            camera_id: props.id,
                            rtsp_url: props.rtspUrl,
                            username: props.username || 'admin',
                            password: props.password || 'admin',
                            sdp: offer.sdp,
                            type: offer.type
                        })
                    });
                    if (retryResponse.ok) {
                        const retryAnswer = await retryResponse.json();
                        console.log(`[CameraTile ${props.id}] Got SDP answer on retry`);
                        await peerConnection!.setRemoteDescription({
                            type: 'answer',
                            sdp: retryAnswer.sdp
                        });
                        setConnectionState('connected');
                        return;
                    }
                    // Still 409, session is stuck
                    console.log(`[CameraTile ${props.id}] Session stuck, trying to delete and recreate`);
                    await fetch(`${VMS_STREAM_URL}/api/v1/webrtc/${props.id}`, { method: 'DELETE' });
                    throw new Error('Session stuck, please refresh page');
                }
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }

            const answer = await response.json();
            console.log(`[CameraTile ${props.id}] Received SDP answer from vms-stream`);

            // Set remote description
            await peerConnection.setRemoteDescription(new RTCSessionDescription({
                type: answer.type,
                sdp: answer.sdp
            }));

            console.log(`[CameraTile ${props.id}] WebRTC signaling complete!`);

        } catch (err) {
            console.error(`[CameraTile ${props.id}] WebRTC error:`, err);
            setError(`WebRTC: ${err}`);
            setConnectionState('failed');

            // Fallback to MJPEG after 5 seconds
            setTimeout(() => startMJPEG(), 5000);
        }
    };

    const startMJPEG = () => {
        console.log(`[CameraTile ${props.id}] Falling back to MJPEG...`);
        // If WebRTC fails, we could fallback to MJPEG
        // For now, just show the error
    };

    const cleanup = () => {
        if (peerConnection) {
            peerConnection.close();
            peerConnection = null;
        }
        if (videoRef) {
            videoRef.srcObject = null;
        }
    };

    onMount(() => {
        if (!props.isLive) return;
        startWebRTC();
    });

    onCleanup(() => {
        cleanup();
    });

    const handleDoubleClick = () => {
        if (videoRef) {
            if (document.fullscreenElement) {
                document.exitFullscreen();
            } else {
                videoRef.requestFullscreen();
            }
        }
    };

    return (
        <div class="camera-tile" onDblClick={handleDoubleClick}>
            <video
                ref={videoRef}
                autoplay
                playsinline
                muted
                style={{
                    width: '100%',
                    height: '100%',
                    'object-fit': 'cover',
                    display: isConnected() ? 'block' : 'none',
                    'background-color': '#000'
                }}
            />

            {!isConnected() && (
                <div class="camera-tile-placeholder">
                    <svg viewBox="0 0 24 24" fill="currentColor">
                        <path d="M17 10.5V7c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h12c.55 0 1-.45 1-1v-3.5l4 4v-11l-4 4z" />
                    </svg>
                    <span>{props.name}</span>
                    <span style={{ color: 'var(--accent)' }}>
                        {connectionState() === 'connecting' ? 'Conectando WebRTC...' :
                            connectionState() === 'failed' ? 'Reconectando...' : connectionState()}
                    </span>
                    {error() && <span style={{ color: 'var(--danger)', 'font-size': '0.75rem' }}>{error()}</span>}
                </div>
            )}

            <div class="camera-tile-overlay">
                <span class="camera-tile-name">{props.name}</span>
                <span class="camera-tile-status">
                    {isConnected() ? '● LIVE' : '◌ WebRTC'}
                </span>
            </div>
        </div>
    );
};

export default CameraTile;
