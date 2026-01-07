/**
 * VMS API Service
 * Cliente HTTP para comunicação com o backend Rust
 */

// Configuração da API
const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:9095';

/**
 * Função helper para fazer requests
 */
async function request<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<T> {
  const url = `${API_BASE_URL}${endpoint}`;
  
  const config: RequestInit = {
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
    ...options,
  };

  // Adiciona token de autenticação se existir
  const token = localStorage.getItem('vms_token');
  if (token) {
    config.headers = {
      ...config.headers,
      'Authorization': `Bearer ${token}`,
    };
  }

  const response = await fetch(url, config);

  if (!response.ok) {
    const error = await response.text();
    throw new Error(error || `HTTP ${response.status}`);
  }

  return response.json();
}

/**
 * Câmera descoberta
 */
export interface DiscoveredCamera {
  name: string;
  ip: string;
  port: number;
  url: string;
}

/**
 * Resposta do discovery
 */
export interface DiscoverResponse {
  cameras: DiscoveredCamera[];
  count: number;
}

/**
 * Profile de câmera
 */
export interface CameraProfile {
  token: string;
  name: string;
  resolution: string | null;
  fps: number | null;
  codec: string | null;
  rtsp_url: string | null;
}

/**
 * Câmera registrada no sistema
 */
export interface Camera {
  id: string;
  name: string;
  status: 'online' | 'offline' | 'error' | 'recording';
  current_fps: number;
  current_bitrate: number;
}

/**
 * Request para conectar a uma câmera
 */
export interface ConnectCameraRequest {
  ip: string;
  port?: number;
  username: string;
  password: string;
}

/**
 * Request para adicionar uma câmera
 */
export interface AddCameraRequest {
  name: string;
  url: string;
  username?: string;
  password?: string;
  resolution?: { width: number; height: number };
  fps?: number;
}

// ============ ONVIF Discovery ============

/**
 * Descobre câmeras ONVIF na rede local
 */
export async function discoverCameras(timeoutSecs = 5): Promise<DiscoveredCamera[]> {
  const response = await request<DiscoverResponse>('/api/v1/cameras/discover', {
    method: 'POST',
    body: JSON.stringify({ timeout_secs: timeoutSecs }),
  });
  return response.cameras;
}

/**
 * Obtém profiles de uma câmera ONVIF
 */
export async function getCameraProfiles(params: ConnectCameraRequest): Promise<CameraProfile[]> {
  return request<CameraProfile[]>('/api/v1/cameras/profiles', {
    method: 'POST',
    body: JSON.stringify(params),
  });
}

/**
 * Envia comando PTZ para uma câmera
 */
export async function sendPtzCommand(params: {
  ip: string;
  port?: number;
  username: string;
  password: string;
  profile_token: string;
  pan: number;
  tilt: number;
  zoom: number;
}): Promise<void> {
  await request('/api/v1/cameras/ptz', {
    method: 'POST',
    body: JSON.stringify(params),
  });
}

// ============ Cameras CRUD ============

/**
 * Lista todas as câmeras registradas
 */
export async function listCameras(): Promise<Camera[]> {
  return request<Camera[]>('/api/v1/cameras');
}

/**
 * Obtém uma câmera específica
 */
export async function getCamera(id: string): Promise<Camera> {
  return request<Camera>(`/api/v1/cameras/${id}`);
}

/**
 * Adiciona uma nova câmera
 */
export async function addCamera(data: AddCameraRequest): Promise<Camera> {
  return request<Camera>('/api/v1/cameras', {
    method: 'POST',
    body: JSON.stringify(data),
  });
}

/**
 * Remove uma câmera
 */
export async function deleteCamera(id: string): Promise<void> {
  await request(`/api/v1/cameras/${id}`, {
    method: 'DELETE',
  });
}

// ============ Streams ============

export interface StreamResponse {
  stream_id: string;
  url: string;
  protocol: string;
}

/**
 * Inicia um stream de câmera
 */
export async function startStream(cameraId: string, protocol: 'webrtc' | 'srt' = 'webrtc'): Promise<StreamResponse> {
  return request<StreamResponse>('/api/v1/streams', {
    method: 'POST',
    body: JSON.stringify({ camera_id: cameraId, protocol }),
  });
}

/**
 * Para um stream
 */
export async function stopStream(streamId: string): Promise<void> {
  await request(`/api/v1/streams/${streamId}`, {
    method: 'DELETE',
  });
}

// ============ Health ============

export interface HealthResponse {
  status: string;
  version: string;
}

/**
 * Verifica saúde da API
 */
export async function healthCheck(): Promise<HealthResponse> {
  return request<HealthResponse>('/health');
}
