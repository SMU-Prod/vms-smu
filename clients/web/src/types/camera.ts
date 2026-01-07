// Camera types
export interface Camera {
  id: string;
  name: string;
  ip: string;
  port: number;
  status: 'online' | 'offline' | 'error';
  recording: boolean;
  ptzEnabled: boolean;
  resolution: string;
  fps: number;
  codec: 'h264' | 'h265' | 'av1';
  onvifProfile?: string;
  group?: string;
  location?: string;
  createdAt: Date;
  updatedAt: Date;
}

export interface CameraStream {
  cameraId: string;
  streamUrl: string;
  protocol: 'webrtc' | 'hls' | 'rtsp';
  quality: 'main' | 'sub';
  bitrate: number;
  latency: number;
}

export interface CameraSnapshot {
  cameraId: string;
  timestamp: Date;
  imageUrl: string;
  width: number;
  height: number;
}

export type LayoutType = '1x1' | '2x2' | '3x3' | '4x4' | '1+5' | '1+7' | '3+4';

export interface GridLayout {
  type: LayoutType;
  cells: GridCell[];
}

export interface GridCell {
  index: number;
  cameraId: string | null;
  colspan?: number;
  rowspan?: number;
}
