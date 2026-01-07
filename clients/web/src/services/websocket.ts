/**
 * WebSocket Service
 * Conexão WebSocket para eventos em tempo real
 */

type MessageHandler = (data: any) => void;

/**
 * Gerenciador de conexão WebSocket
 */
class WebSocketService {
  private socket: WebSocket | null = null;
  private url: string;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 2000;
  private handlers: Map<string, Set<MessageHandler>> = new Map();
  private isConnecting = false;

  constructor() {
    // URL do WebSocket (via vms-events ou vms-api)
    const wsProtocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    this.url = import.meta.env.VITE_WS_URL || `${wsProtocol}//localhost:9095/ws`;
  }

  /**
   * Conecta ao servidor WebSocket
   */
  connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      if (this.socket?.readyState === WebSocket.OPEN) {
        resolve();
        return;
      }

      if (this.isConnecting) {
        reject(new Error('Conexão já em andamento'));
        return;
      }

      this.isConnecting = true;
      console.log('🔌 Conectando WebSocket:', this.url);

      try {
        this.socket = new WebSocket(this.url);

        this.socket.onopen = () => {
          console.log('✅ WebSocket conectado');
          this.reconnectAttempts = 0;
          this.isConnecting = false;
          resolve();
        };

        this.socket.onclose = (event) => {
          console.log('🔌 WebSocket desconectado:', event.reason);
          this.isConnecting = false;
          this.handleDisconnect();
        };

        this.socket.onerror = (error) => {
          console.error('❌ Erro WebSocket:', error);
          this.isConnecting = false;
          reject(error);
        };

        this.socket.onmessage = (event) => {
          this.handleMessage(event.data);
        };
      } catch (error) {
        this.isConnecting = false;
        reject(error);
      }
    });
  }

  /**
   * Desconecta do servidor
   */
  disconnect(): void {
    if (this.socket) {
      this.socket.close();
      this.socket = null;
    }
  }

  /**
   * Registra handler para um tipo de mensagem
   */
  on(type: string, handler: MessageHandler): () => void {
    if (!this.handlers.has(type)) {
      this.handlers.set(type, new Set());
    }
    this.handlers.get(type)!.add(handler);

    // Retorna função para remover o handler
    return () => {
      this.handlers.get(type)?.delete(handler);
    };
  }

  /**
   * Remove todos os handlers de um tipo
   */
  off(type: string): void {
    this.handlers.delete(type);
  }

  /**
   * Envia mensagem pelo WebSocket
   */
  send(type: string, data: any): void {
    if (this.socket?.readyState !== WebSocket.OPEN) {
      console.warn('WebSocket não conectado');
      return;
    }

    this.socket.send(JSON.stringify({ type, data }));
  }

  /**
   * Processa mensagem recebida
   */
  private handleMessage(rawData: string): void {
    try {
      const message = JSON.parse(rawData);
      const { type, data } = message;

      // Notifica handlers do tipo específico
      const handlers = this.handlers.get(type);
      if (handlers) {
        handlers.forEach((handler) => handler(data));
      }

      // Notifica handlers genéricos ('*')
      const allHandlers = this.handlers.get('*');
      if (allHandlers) {
        allHandlers.forEach((handler) => handler(message));
      }
    } catch (error) {
      console.error('Erro ao processar mensagem WS:', error);
    }
  }

  /**
   * Gerencia reconexão automática
   */
  private handleDisconnect(): void {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      const delay = this.reconnectDelay * this.reconnectAttempts;
      console.log(`🔄 Reconectando em ${delay}ms (tentativa ${this.reconnectAttempts})`);
      
      setTimeout(() => {
        this.connect().catch((err) => {
          console.error('Falha na reconexão:', err);
        });
      }, delay);
    } else {
      console.error('❌ Máximo de tentativas de reconexão atingido');
    }
  }

  /**
   * Verifica se está conectado
   */
  get isConnected(): boolean {
    return this.socket?.readyState === WebSocket.OPEN;
  }
}

// Instância singleton
export const wsService = new WebSocketService();

// Tipos de eventos comuns
export type EventType = 
  | 'camera.status'
  | 'camera.frame'
  | 'event.motion'
  | 'event.person'
  | 'event.vehicle'
  | 'event.face'
  | 'event.plate'
  | 'event.alarm'
  | 'system.status';

export default wsService;
