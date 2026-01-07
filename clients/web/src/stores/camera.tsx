/**
 * Camera Store (SolidJS)
 * Gerencia estado das câmeras - integrado com API real
 */

import { createContext, useContext, createSignal, ParentComponent, onMount, onCleanup, createEffect } from 'solid-js';
import { createStore } from 'solid-js/store';
import type { Camera as ApiCamera, GridCell } from '../types';
import * as api from '../services/api';
import { wsService } from '../services/websocket';

// Tipos
export interface CameraState {
  cameras: ApiCamera[];
  selectedCamera: string | null;
  layout: LayoutType;
  gridCells: GridCell[];
  loading: boolean;
  error: string | null;
}

export type LayoutType = '1x1' | '2x2' | '3x3' | '4x4';

// Mapa de layout para número de células
const layoutSizeMap: Record<LayoutType, number> = {
  '1x1': 1,
  '2x2': 4,
  '3x3': 9,
  '4x4': 16,
};

// Valor inicial - sem dados mockados!
const initialState: CameraState = {
  cameras: [],
  selectedCamera: null,
  layout: '2x2',
  gridCells: [],
  loading: false,
  error: null,
};

// Context
const CameraContext = createContext<{
  state: CameraState;
  actions: {
    loadCameras: () => Promise<void>;
    discoverCameras: (timeoutSecs?: number) => Promise<api.DiscoveredCamera[]>;
    getCameraProfiles: (params: api.ConnectCameraRequest) => Promise<api.CameraProfile[]>;
    addCamera: (data: api.AddCameraRequest) => Promise<void>;
    deleteCamera: (id: string) => Promise<void>;
    selectCamera: (id: string | null) => void;
    setLayout: (layout: LayoutType) => void;
    sendPtz: (params: Parameters<typeof api.sendPtzCommand>[0]) => Promise<void>;
    getCameraById: (id: string) => ApiCamera | undefined;
    assignCameraToCell: (cellIndex: number, cameraId: string | null) => void;
  };
}>();

// Provider
export const CameraProvider: ParentComponent = (props) => {
  const [state, setState] = createStore<CameraState>(initialState);

  // Carrega câmeras da API
  const loadCameras = async () => {
    setState('loading', true);
    setState('error', null);
    
    try {
      const cameras = await api.listCameras();
      setState('cameras', cameras.map(c => ({
        id: c.id,
        name: c.name,
        ip: '',
        port: 80,
        status: c.status,
        recording: c.status === 'recording',
        ptzEnabled: false,
        resolution: '1920x1080',
        fps: c.current_fps || 25,
        codec: 'h264' as const,
        createdAt: new Date(),
        updatedAt: new Date(),
      })));
    } catch (error: any) {
      console.error('Erro ao carregar câmeras:', error);
      setState('error', error.message || 'Erro ao carregar câmeras');
    } finally {
      setState('loading', false);
    }
  };

  // Descoberta ONVIF
  const discoverCameras = async (timeoutSecs = 5) => {
    setState('loading', true);
    try {
      return await api.discoverCameras(timeoutSecs);
    } finally {
      setState('loading', false);
    }
  };

  // Obtém profiles de uma câmera
  const getCameraProfiles = async (params: api.ConnectCameraRequest) => {
    return await api.getCameraProfiles(params);
  };

  // Adiciona câmera
  const addCamera = async (data: api.AddCameraRequest) => {
    const camera = await api.addCamera(data);
    setState('cameras', (cams) => [...cams, {
      id: camera.id,
      name: camera.name,
      ip: '',
      port: 80,
      status: camera.status,
      recording: camera.status === 'recording',
      ptzEnabled: false,
      resolution: '1920x1080',
      fps: camera.current_fps || 25,
      codec: 'h264' as const,
      createdAt: new Date(),
      updatedAt: new Date(),
    }]);
  };

  // Remove câmera
  const deleteCamera = async (id: string) => {
    await api.deleteCamera(id);
    setState('cameras', (cams) => cams.filter(c => c.id !== id));
    if (state.selectedCamera === id) {
      setState('selectedCamera', null);
    }
  };

  // Seleciona câmera
  const selectCamera = (id: string | null) => {
    setState('selectedCamera', id);
  };

  // Altera layout
  const setLayout = (layout: LayoutType) => {
    setState('layout', layout);
  };

  // Envia comando PTZ
  const sendPtz = async (params: Parameters<typeof api.sendPtzCommand>[0]) => {
    await api.sendPtzCommand(params);
  };

  // Busca câmera por ID
  const getCameraById = (id: string): ApiCamera | undefined => {
    return state.cameras.find(c => c.id === id);
  };

  // Atribui câmera a uma célula do grid
  const assignCameraToCell = (cellIndex: number, cameraId: string | null) => {
    setState('gridCells', cellIndex, 'cameraId', cameraId);
  };

  // Effect para atualizar gridCells quando layout ou cameras mudam
  createEffect(() => {
    const layoutType = state.layout;
    const cellCount = layoutSizeMap[layoutType];
    const cameras = state.cameras;

    // Cria células baseado no layout
    const cells: GridCell[] = Array.from({ length: cellCount }, (_, i) => ({
      index: i,
      cameraId: cameras[i]?.id || null, // Auto-preenche com câmeras disponíveis
    }));

    setState('gridCells', cells);
  });

  // WebSocket para atualizações de status
  onMount(() => {
    // Tenta carregar câmeras ao montar
    loadCameras().catch(console.error);

    // Escuta eventos de status
    const unsubscribe = wsService.on('camera.status', (data: any) => {
      const { cameraId, status } = data;
      setState('cameras', (camera) => camera.id === cameraId, 'status', status);
    });

    return () => {
      unsubscribe();
    };
  });

  const value = {
    state,
    actions: {
      loadCameras,
      discoverCameras,
      getCameraProfiles,
      addCamera,
      deleteCamera,
      selectCamera,
      setLayout,
      sendPtz,
      getCameraById,
      assignCameraToCell,
    },
  };

  return (
    <CameraContext.Provider value={value}>
      {props.children}
    </CameraContext.Provider>
  );
};

// Hook
export function useCamera() {
  const context = useContext(CameraContext);
  if (!context) {
    throw new Error('useCamera must be used within a CameraProvider');
  }
  return context;
}

// Alias para compatibilidade com código existente
export const useCameras = useCamera;

// Export types
export type { DiscoveredCamera, CameraProfile, ConnectCameraRequest, AddCameraRequest } from '../services/api';
