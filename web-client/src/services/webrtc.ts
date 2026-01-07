/**
 * WebRTC Service for Low-Latency Video Streaming
 * 
 * Connects to vms-stream for H264 video with minimal latency
 * Target: 50-150ms latency with hardware decode
 */

export interface WebRTCConfig {
    streamServerUrl: string;
    iceServers?: RTCIceServer[];
}

export interface StreamInfo {
    cameraId: string;
    streamId: string;
}

export class WebRTCService {
    private config: WebRTCConfig;
    private connections: Map<string, RTCPeerConnection> = new Map();
    private streams: Map<string, MediaStream> = new Map();

    constructor(config: WebRTCConfig) {
        this.config = {
            streamServerUrl: config.streamServerUrl || 'http://localhost:9094',
            iceServers: config.iceServers || [
                { urls: 'stun:stun.l.google.com:19302' },
            ],
        };
    }

    /**
     * Connect to a camera stream via WebRTC
     */
    async connect(cameraId: string): Promise<MediaStream> {
        console.log(`[WebRTC] Connecting to camera: ${cameraId}`);

        // Create peer connection
        const pc = new RTCPeerConnection({
            iceServers: this.config.iceServers,
        });

        this.connections.set(cameraId, pc);

        // Handle incoming tracks
        const streamPromise = new Promise<MediaStream>((resolve, reject) => {
            pc.ontrack = (event) => {
                console.log(`[WebRTC] Received track:`, event.track.kind);
                const stream = event.streams[0];
                this.streams.set(cameraId, stream);
                resolve(stream);
            };

            // Timeout after 10 seconds
            setTimeout(() => {
                if (!this.streams.has(cameraId)) {
                    reject(new Error('Connection timeout'));
                }
            }, 10000);
        });

        // Handle ICE candidates
        pc.onicecandidate = async (event) => {
            if (event.candidate) {
                await this.sendIceCandidate(cameraId, event.candidate);
            }
        };

        pc.oniceconnectionstatechange = () => {
            console.log(`[WebRTC] ICE state: ${pc.iceConnectionState}`);
        };

        pc.onconnectionstatechange = () => {
            console.log(`[WebRTC] Connection state: ${pc.connectionState}`);
        };

        // Add transceivers for receiving video/audio
        pc.addTransceiver('video', { direction: 'recvonly' });
        pc.addTransceiver('audio', { direction: 'recvonly' });

        // Create offer
        const offer = await pc.createOffer();
        await pc.setLocalDescription(offer);

        // Send offer to signaling server
        const answer = await this.sendOffer(cameraId, offer);
        await pc.setRemoteDescription(answer);

        return streamPromise;
    }

    /**
     * Send SDP offer to signaling server
     */
    private async sendOffer(cameraId: string, offer: RTCSessionDescriptionInit): Promise<RTCSessionDescriptionInit> {
        const response = await fetch(`${this.config.streamServerUrl}/api/v1/webrtc/offer`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                camera_id: cameraId,
                sdp: offer.sdp,
                type: offer.type,
            }),
        });

        if (!response.ok) {
            throw new Error(`Failed to send offer: ${response.statusText}`);
        }

        const data = await response.json();
        return {
            type: 'answer',
            sdp: data.sdp,
        };
    }

    /**
     * Send ICE candidate to signaling server
     */
    private async sendIceCandidate(cameraId: string, candidate: RTCIceCandidate): Promise<void> {
        await fetch(`${this.config.streamServerUrl}/api/v1/webrtc/ice`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                camera_id: cameraId,
                candidate: candidate.candidate,
                sdpMid: candidate.sdpMid,
                sdpMLineIndex: candidate.sdpMLineIndex,
            }),
        });
    }

    /**
     * Disconnect from a camera stream
     */
    disconnect(cameraId: string): void {
        const pc = this.connections.get(cameraId);
        if (pc) {
            pc.close();
            this.connections.delete(cameraId);
        }

        const stream = this.streams.get(cameraId);
        if (stream) {
            stream.getTracks().forEach(track => track.stop());
            this.streams.delete(cameraId);
        }

        console.log(`[WebRTC] Disconnected from camera: ${cameraId}`);
    }

    /**
     * Get current stream for a camera
     */
    getStream(cameraId: string): MediaStream | undefined {
        return this.streams.get(cameraId);
    }

    /**
     * Disconnect all cameras
     */
    disconnectAll(): void {
        for (const cameraId of this.connections.keys()) {
            this.disconnect(cameraId);
        }
    }
}

// Singleton instance
let webrtcServiceInstance: WebRTCService | null = null;

export function getWebRTCService(): WebRTCService {
    if (!webrtcServiceInstance) {
        webrtcServiceInstance = new WebRTCService({
            streamServerUrl: 'http://localhost:9094',
        });
    }
    return webrtcServiceInstance;
}
