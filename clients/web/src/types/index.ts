export * from './camera';
export * from './event';
export * from './evidence';

// User types
export interface User {
  id: string;
  username: string;
  email: string;
  name: string;
  role: 'admin' | 'operator' | 'viewer';
  permissions: string[];
  lastLogin?: Date;
  createdAt: Date;
}

// System status types
export interface SystemStatus {
  camerasOnline: number;
  camerasTotal: number;
  eventsToday: number;
  eventsUnacknowledged: number;
  storageUsed: number;
  storageTotal: number;
  recordingActive: boolean;
  cpuUsage: number;
  memoryUsage: number;
  gpuUsage?: number;
}

// API response types
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}

export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  pageSize: number;
  totalPages: number;
}
