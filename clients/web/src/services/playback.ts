/**
 * Playback Service
 * API client para playback de gravações
 */

import { apiClient } from './api';

// Tipos
export interface TimelineSegment {
  start: string;
  end: string;
  file_path: string;
  size_bytes: number;
  has_motion: boolean;
  has_events: string[];
}

export interface MotionZone {
  timestamp: string;
  duration_ms: number;
  confidence: number;
}

export interface TimelineEvent {
  id: string;
  timestamp: string;
  event_type: string;
  priority: string;
}

export interface Bookmark {
  id: string;
  camera_id: string;
  timestamp: string;
  user_id: string;
  note: string;
  tags: string[];
  created_at: string;
  updated_at: string;
}

export interface Timeline {
  camera_id: string;
  start: string;
  end: string;
  resolution: TimelineResolution;
  segments: TimelineSegment[];
  motion_zones: MotionZone[];
  events: TimelineEvent[];
  bookmarks: Bookmark[];
}

export type TimelineResolution = '1s' | '10s' | '1m' | '10m' | '1h';

export interface GetTimelineParams {
  cameraId: string;
  start: Date;
  end: Date;
  resolution?: TimelineResolution;
}

export interface StreamRecordingParams {
  cameraId: string;
  start: Date;
  end?: Date;
  speed?: number;
}

export interface ExportVideoParams {
  camera_id: string;
  start: Date;
  end: Date;
  format: 'mp4' | 'avi';
  include_watermark: boolean;
}

export interface CreateBookmarkParams {
  camera_id: string;
  timestamp: Date;
  user_id: string;
  note: string;
  tags?: string[];
}

export interface UpdateBookmarkParams {
  note?: string;
  tags?: string[];
}

export interface ListBookmarksParams {
  camera_id?: string;
  start?: Date;
  end?: Date;
  tags?: string[];
}

// Service
const STORAGE_API_URL = import.meta.env.VITE_STORAGE_URL || 'http://localhost:9092';

export const playbackService = {
  /**
   * Obter timeline de gravações
   */
  async getTimeline(params: GetTimelineParams): Promise<Timeline> {
    const { cameraId, start, end, resolution = '1m' } = params;

    const queryParams = new URLSearchParams({
      start: start.toISOString(),
      end: end.toISOString(),
      resolution,
    });

    const response = await fetch(
      `${STORAGE_API_URL}/api/v1/recordings/${cameraId}/timeline?${queryParams}`
    );

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`Failed to get timeline: ${error}`);
    }

    return response.json();
  },

  /**
   * Obter URL para stream de gravação
   */
  getStreamUrl(params: StreamRecordingParams): string {
    const { cameraId, start, end, speed = 1.0 } = params;

    const queryParams = new URLSearchParams({
      start: start.toISOString(),
      speed: speed.toString(),
    });

    if (end) {
      queryParams.append('end', end.toISOString());
    }

    return `${STORAGE_API_URL}/api/v1/recordings/${cameraId}/stream?${queryParams}`;
  },

  /**
   * Exportar vídeo
   */
  async exportVideo(params: ExportVideoParams): Promise<{ export_id: string }> {
    const response = await fetch(`${STORAGE_API_URL}/api/v1/recordings/export`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        ...params,
        start: params.start.toISOString(),
        end: params.end.toISOString(),
      }),
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`Failed to export video: ${error}`);
    }

    return response.json();
  },

  // === Bookmarks ===

  /**
   * Criar bookmark
   */
  async createBookmark(params: CreateBookmarkParams): Promise<Bookmark> {
    const response = await fetch(`${STORAGE_API_URL}/api/v1/bookmarks`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        ...params,
        timestamp: params.timestamp.toISOString(),
      }),
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`Failed to create bookmark: ${error}`);
    }

    return response.json();
  },

  /**
   * Listar bookmarks
   */
  async listBookmarks(params: ListBookmarksParams): Promise<Bookmark[]> {
    const queryParams = new URLSearchParams();

    if (params.camera_id && params.start && params.end) {
      queryParams.append('camera_id', params.camera_id);
      queryParams.append('start', params.start.toISOString());
      queryParams.append('end', params.end.toISOString());
    } else if (params.tags && params.tags.length > 0) {
      queryParams.append('tags', params.tags.join(','));
    } else {
      throw new Error('Must specify camera_id with start/end, or tags');
    }

    const response = await fetch(
      `${STORAGE_API_URL}/api/v1/bookmarks?${queryParams}`
    );

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`Failed to list bookmarks: ${error}`);
    }

    return response.json();
  },

  /**
   * Obter bookmark por ID
   */
  async getBookmark(id: string): Promise<Bookmark> {
    const response = await fetch(`${STORAGE_API_URL}/api/v1/bookmarks/${id}`);

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`Failed to get bookmark: ${error}`);
    }

    return response.json();
  },

  /**
   * Atualizar bookmark
   */
  async updateBookmark(id: string, params: UpdateBookmarkParams): Promise<Bookmark> {
    const response = await fetch(`${STORAGE_API_URL}/api/v1/bookmarks/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(params),
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`Failed to update bookmark: ${error}`);
    }

    return response.json();
  },

  /**
   * Deletar bookmark
   */
  async deleteBookmark(id: string): Promise<void> {
    const response = await fetch(`${STORAGE_API_URL}/api/v1/bookmarks/${id}`, {
      method: 'DELETE',
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`Failed to delete bookmark: ${error}`);
    }
  },
};
