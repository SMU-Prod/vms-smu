/**
 * WebRTC Player Component
 * Conecta com vms-stream para receber stream de vídeo em tempo real
 */

import { Component, onMount, onCleanup, createSignal, Show } from 'solid-js';
import { Loader, WifiOff, AlertCircle } from 'lucide-solid';

interface Props {
  cameraId: string;
  streamUrl?: string;
  onError?: (error: Error) => void;
  onConnected?: () => void;
}

type ConnectionStatus = 'connecting' | 'connected' | 'disconnected' | 'error';

const WebRTCPlayer: Component<Props> = (props) => {
  let videoRef: HTMLVideoElement;
  let peerConnection: RTCPeerConnection | null = null;
  let streamId: string | null = null;

  const [status, setStatus] = createSignal<ConnectionStatus>('connecting');
  const [errorMessage, setErrorMessage] = createSignal<string>('');

  const cleanup = () => {
    if (peerConnection) {
      peerConnection.close();
      peerConnection = null;
    }

    // Fechar stream no servidor
    if (streamId) {
      fetch(`${import.meta.env.VITE_API_URL || 'http://localhost:9094'}/stream/${streamId}`, {
        method: 'DELETE',
      }).catch(console.error);
      streamId = null;
    }
  };

  const connect = async () => {
    try {
      setStatus('connecting');
      setErrorMessage('');

      // 1. Criar PeerConnection
      peerConnection = new RTCPeerConnection({
        iceServers: [
          { urls: 'stun:stun.l.google.com:19302' },
          { urls: 'stun:stun1.l.google.com:19302' },
        ],
      });

      // 2. Handler quando receber track (stream de vídeo)
      peerConnection.ontrack = (event) => {
        console.log('WebRTC: Track received', event.streams[0]);
        if (videoRef && event.streams[0]) {
          videoRef.srcObject = event.streams[0];
          setStatus('connected');
          props.onConnected?.();
        }
      };

      // 3. Handler de mudança de estado da conexão
      peerConnection.onconnectionstatechange = () => {
        const state = peerConnection?.connectionState;
        console.log('WebRTC: Connection state:', state);

        if (state === 'connected') {
          setStatus('connected');
        } else if (state === 'disconnected' || state === 'closed') {
          setStatus('disconnected');
        } else if (state === 'failed') {
          setStatus('error');
          setErrorMessage('Conexão falhou');
        }
      };

      // 4. Handler de ICE candidate
      peerConnection.oniceconnectionstatechange = () => {
        const iceState = peerConnection?.iceConnectionState;
        console.log('WebRTC: ICE state:', iceState);
      };

      // 5. Criar offer
      const offer = await peerConnection.createOffer({
        offerToReceiveVideo: true,
        offerToReceiveAudio: true,
      });

      await peerConnection.setLocalDescription(offer);

      // 6. Enviar offer para servidor vms-stream
      const response = await fetch(`${import.meta.env.VITE_API_URL || 'http://localhost:9094'}/stream`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          camera_id: props.cameraId,
          sdp: offer.sdp,
          type: offer.type,
        }),
      });

      if (!response.ok) {
        const error = await response.text();
        throw new Error(`Falha ao criar stream: ${error}`);
      }

      const data = await response.json();
      streamId = data.stream_id;
      const { answer } = data;

      // 7. Configurar answer do servidor
      await peerConnection.setRemoteDescription(
        new RTCSessionDescription({
          type: 'answer',
          sdp: answer,
        })
      );

      console.log('WebRTC: Connection established, stream_id:', streamId);

    } catch (err) {
      console.error('WebRTC: Connection error:', err);
      setStatus('error');
      setErrorMessage((err as Error).message);
      props.onError?.(err as Error);
    }
  };

  onMount(() => {
    // Conectar após montar o componente
    connect();
  });

  onCleanup(() => {
    // Limpar conexões ao desmontar
    cleanup();
  });

  return (
    <div class="relative w-full h-full bg-black">
      {/* Video Element */}
      <video
        ref={videoRef!}
        autoplay
        muted
        playsinline
        class="w-full h-full object-contain"
      />

      {/* Loading/Error Overlays */}
      <Show when={status() === 'connecting'}>
        <div class="absolute inset-0 flex flex-col items-center justify-center bg-slate-900/90">
          <Loader class="w-12 h-12 text-vms-accent animate-spin mb-3" />
          <span class="text-sm text-slate-400">Conectando stream...</span>
        </div>
      </Show>

      <Show when={status() === 'disconnected'}>
        <div class="absolute inset-0 flex flex-col items-center justify-center bg-slate-900/90">
          <WifiOff class="w-12 h-12 text-slate-500 mb-3" />
          <span class="text-sm text-slate-400">Stream desconectado</span>
          <button
            class="mt-3 px-4 py-2 bg-vms-primary rounded-lg text-sm hover:bg-vms-primary/80 transition-colors"
            onClick={connect}
          >
            Reconectar
          </button>
        </div>
      </Show>

      <Show when={status() === 'error'}>
        <div class="absolute inset-0 flex flex-col items-center justify-center bg-slate-900/90">
          <AlertCircle class="w-12 h-12 text-red-500 mb-3" />
          <span class="text-sm text-red-400 mb-1">Erro de conexão</span>
          <span class="text-xs text-slate-500 max-w-xs text-center">{errorMessage()}</span>
          <button
            class="mt-3 px-4 py-2 bg-vms-primary rounded-lg text-sm hover:bg-vms-primary/80 transition-colors"
            onClick={connect}
          >
            Tentar Novamente
          </button>
        </div>
      </Show>

      {/* Connected Indicator */}
      <Show when={status() === 'connected'}>
        <div class="absolute top-2 right-2">
          <div class="flex items-center gap-1.5 px-2 py-1 bg-green-500/20 rounded-full border border-green-500/30">
            <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
            <span class="text-xs text-green-400 font-medium">LIVE</span>
          </div>
        </div>
      </Show>
    </div>
  );
};

export default WebRTCPlayer;
