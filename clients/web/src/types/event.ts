// Event types
export type EventPriority = 'critical' | 'high' | 'medium' | 'low' | 'info';

export type EventType = 
  | 'motion'
  | 'person'
  | 'vehicle'
  | 'face_match'
  | 'face_blocklist'
  | 'plate_match'
  | 'plate_blocklist'
  | 'line_crossing'
  | 'intrusion'
  | 'loitering'
  | 'camera_offline'
  | 'recording_error'
  | 'storage_warning'
  | 'system';

export interface VmsEvent {
  id: string;
  type: EventType;
  priority: EventPriority;
  cameraId: string;
  cameraName: string;
  timestamp: Date;
  description: string;
  acknowledged: boolean;
  acknowledgedBy?: string;
  acknowledgedAt?: Date;
  metadata?: Record<string, unknown>;
  thumbnailUrl?: string;
  videoClipUrl?: string;
}

export interface EventFilter {
  types?: EventType[];
  priorities?: EventPriority[];
  cameraIds?: string[];
  startDate?: Date;
  endDate?: Date;
  acknowledged?: boolean;
}

export interface AlarmRule {
  id: string;
  name: string;
  eventTypes: EventType[];
  cameraIds: string[];
  priority: EventPriority;
  actions: AlarmAction[];
  enabled: boolean;
  schedule?: AlarmSchedule;
}

export interface AlarmAction {
  type: 'email' | 'sms' | 'webhook' | 'popup' | 'sound';
  config: Record<string, unknown>;
}

export interface AlarmSchedule {
  days: number[]; // 0-6 (Sunday-Saturday)
  startTime: string; // HH:mm
  endTime: string; // HH:mm
}
