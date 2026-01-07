# 🚀 VMS Enterprise - Quick Start Guide
**Tempo estimado**: 15-30 minutos

---

## ✅ Pré-requisitos

Antes de começar, você precisa ter instalado:

- [x] **Docker Desktop** (para infraestrutura)
- [x] **Rust 1.75+** ([rustup.rs](https://rustup.rs))
- [x] **GStreamer 1.22+** (Windows: [aqui](https://gstreamer.freedesktop.org/download/))
- [x] **Git**

**Verificação**:
```bash
docker --version          # Docker version 20.10+
rustc --version          # rustc 1.75+
gst-inspect-1.0 --version  # GStreamer 1.22+
```

---

## 🎯 Passo 1: Clone e Configure

```bash
# Clone o repositório
git clone <seu-repo>
cd vms-enterprise/vms

# Configure GStreamer (Windows)
export PKG_CONFIG_PATH="/c/Program Files/GStreamer/1.0/msvc_x86_64/lib/pkgconfig"
export PKG_CONFIG="/c/Program Files/GStreamer/1.0/msvc_x86_64/bin/pkg-config.exe"
export PATH="/c/Program Files/GStreamer/1.0/msvc_x86_64/bin:$PATH"
```

---

## 🐳 Passo 2: Inicie a Infraestrutura

```bash
cd ..  # Voltar para raiz do projeto
./scripts/start-infrastructure.sh
```

**O que isso faz**:
- ✅ Inicia NATS (message broker)
- ✅ Inicia PostgreSQL (banco de dados)
- ✅ Inicia Redis (cache)
- ✅ Inicia MinIO (object storage)

**Verificação**:
```bash
docker ps  # Deve mostrar 4 containers rodando
curl http://localhost:8222/healthz  # NATS health check
```

---

## 🔨 Passo 3: Compile os Serviços

```bash
./scripts/build-all.sh
```

**Tempo estimado**: 5-15 minutos (primeira vez)

**O que isso faz**:
- Compila workspace completo em release mode
- Gera 7 binários otimizados
- Configura GStreamer automaticamente (Windows)

**Binários gerados**:
```
target/release/
├── vms-ingest      # Ingestão RTSP
├── vms-storage     # Gravação MKV
├── vms-ai          # Detecção IA
├── vms-stream      # Distribuição WebRTC/SRT
├── vms-api         # REST API
├── vms-gateway     # Service discovery
└── vms-replicator  # Backup/DR
```

---

## 🎬 Passo 4: Inicie os Serviços

```bash
./scripts/run-services.sh
```

**O que isso faz**:
- Cria diretório `storage/` para gravações
- Inicia os 7 serviços em background
- Gera logs em `logs/*.log`
- Gera PIDs em `logs/*.pid`

**Verificação**:
```bash
# Health checks
curl http://localhost:9091/metrics  # vms-ingest
curl http://localhost:9092/metrics  # vms-storage
curl http://localhost:9093/metrics  # vms-ai
curl http://localhost:9094/metrics  # vms-stream
curl http://localhost:8080/health   # vms-api

# Logs
tail -f logs/vms-ingest.log
tail -f logs/vms-storage.log
tail -f logs/vms-ai.log
```

---

## 📹 Passo 5: Teste com Câmera Simulada

### Opção A: FFmpeg (Recomendado para Teste)

```bash
# Instalar MediaMTX (servidor RTSP leve)
# Windows: Download de https://github.com/bluenviron/mediamtx/releases
# Ou use Docker:
docker run --rm -d -p 8554:8554 --name mediamtx bluenviron/mediamtx

# Gerar stream de teste com FFmpeg
ffmpeg -re \
    -f lavfi -i testsrc=size=1920x1080:rate=30 \
    -f lavfi -i sine=frequency=1000 \
    -pix_fmt yuv420p \
    -c:v libx264 \
    -b:v 2M \
    -g 60 \
    -preset ultrafast \
    -tune zerolatency \
    -f rtsp \
    rtsp://localhost:8554/test
```

### Opção B: Câmera IP Real

Edite `services/vms-ingest/src/main.rs` e configure sua câmera:

```rust
let cameras = vec![
    CameraConfig::new(
        "Câmera Entrada".to_string(),
        "rtsp://192.168.1.100:554/stream1".to_string(),  // Seu RTSP URL
    )
    .with_credentials("admin".to_string(), "password".to_string()),
];
```

Recompile:
```bash
cargo build --release -p vms-ingest
./scripts/stop-services.sh
./scripts/run-services.sh
```

---

## 🔍 Passo 6: Verifique o Pipeline

### 1. Frames sendo ingeridos
```bash
curl http://localhost:9091/metrics | grep vms_total_frames_ingested
# vms_total_frames_ingested 1234
```

### 2. Frames sendo gravados
```bash
ls -lh storage/cameras/
# Deve mostrar diretórios por câmera

ls -lh storage/cameras/{camera-id}/{date}/
# video_00.mkv    - Vídeo gravado
# index_00.json   - Índice de seek
```

### 3. Frames sendo distribuídos
```bash
curl http://localhost:9094/metrics | grep vms_distributor_frames_total
# vms_distributor_frames_total 1234
```

### 4. IA processando (se modelo carregado)
```bash
curl http://localhost:9093/metrics | grep vms_ai_detections_total
# vms_ai_detections_total 45
```

---

## 🤖 Passo 7: Habilitar IA (Opcional)

### Download de Modelo ONNX de Teste

```bash
# Criar diretório
mkdir -p models

# Opção 1: YOLO Nano (menor, mais rápido)
pip install ultralytics
python -c "from ultralytics import YOLO; YOLO('yolov8n.pt').export(format='onnx')"
mv yolov8n.onnx models/model.onnx

# Opção 2: RT-DETR (melhor qualidade)
# Requer exportar manualmente ou baixar pre-converted
```

### Configurar e Reiniciar

```bash
export AI_MODEL_PATH="./models/model.onnx"
./scripts/stop-services.sh
./scripts/run-services.sh
```

### Verificar Detecções

```bash
# Logs do vms-ai
tail -f logs/vms-ai.log
# Deve mostrar:
# 🎯 Detected 3 objects
# 📤 Published AI event: 3 detections, 2 tracks
```

---

## 📊 Passo 8: Monitoramento

### Prometheus + Grafana (Opcional)

```bash
docker-compose -f deploy/compose/docker-compose.monitoring.yml up -d
```

**Acessos**:
- Grafana: http://localhost:3000 (admin/admin)
- Prometheus: http://localhost:9090
- Loki: http://localhost:3100

---

## 🛑 Parar Tudo

```bash
# Parar serviços VMS
./scripts/stop-services.sh

# Parar infraestrutura
docker-compose -f deploy/compose/docker-compose.infrastructure.yml down

# Parar monitoramento (se iniciado)
docker-compose -f deploy/compose/docker-compose.monitoring.yml down
```

---

## 🐛 Troubleshooting

### Problema: Compilação falha com erro GStreamer

**Erro**: `pkg-config: command not found` ou `gstreamer-1.0 not found`

**Solução**:
```bash
# Windows: Configure variáveis
export PKG_CONFIG_PATH="/c/Program Files/GStreamer/1.0/msvc_x86_64/lib/pkgconfig"
export PKG_CONFIG="/c/Program Files/GStreamer/1.0/msvc_x86_64/bin/pkg-config.exe"

# Compile novamente
./scripts/build-all.sh
```

### Problema: NATS connection refused

**Erro**: `Failed to connect to NATS: Connection refused`

**Solução**:
```bash
# Verificar se NATS está rodando
docker ps | grep nats

# Se não estiver, iniciar infraestrutura
./scripts/start-infrastructure.sh
```

### Problema: Porta já em uso

**Erro**: `Address already in use (os error 48)`

**Solução**:
```bash
# Verificar portas em uso
netstat -an | grep LISTEN | grep "8080\|9091\|9092\|9093\|9094"

# Parar serviços conflitantes
./scripts/stop-services.sh
```

### Problema: Sem frames sendo ingeridos

**Verificações**:
```bash
# 1. Verificar logs do vms-ingest
tail -f logs/vms-ingest.log

# 2. Testar RTSP manualmente
ffplay rtsp://localhost:8554/test

# 3. Verificar câmera configurada
curl http://localhost:9091/metrics
```

---

## 📚 Próximos Passos

Agora que o sistema está rodando:

1. ✅ **Ver documentação de IA**: `docs/AI_SETUP.md`
2. ✅ **Ver progresso do projeto**: `PROGRESS.md`
3. ✅ **Ver status geral**: `STATUS.md`
4. ✅ **Criar cliente web** (próximo passo)
5. ✅ **Configurar alertas**
6. ✅ **Adicionar mais câmeras**

---

## 🎯 Arquitetura em Produção

```
┌─────────────┐
│  Câmeras IP │──RTSP─────┐
└─────────────┘           │
                          ▼
                  ┌───────────────┐
                  │  vms-ingest   │
                  │  (GStreamer)  │
                  └───────┬───────┘
                          │ frames
                          ▼
                  ┌───────────────┐
                  │     NATS      │◀────── Message Broker
                  └──┬─────────┬──┘
                     │         │
        ┌────────────┘         └─────────────┐
        │ frames                  frames     │
        ▼                                    ▼
┌───────────────┐                  ┌─────────────────┐
│  vms-storage  │                  │   vms-stream    │
│  (Gravação)   │                  │  (WebRTC/SRT)   │
└───────────────┘                  └─────────────────┘
        │                                    │
        ▼                                    ▼
┌───────────────┐                  ┌─────────────────┐
│   vms-ai      │                  │    Viewers      │
│  (Detecção)   │                  │  (Web/Mobile)   │
└───────────────┘                  └─────────────────┘
        │
        ▼ events
┌───────────────┐
│ vms.events.ai │──────────────────── Eventos/Alertas
└───────────────┘
```

---

**Tempo total**: ~15-30 minutos ✅
**Próximo**: Cliente Web e Testes E2E

---

**Versão**: 0.1.0
**Última Atualização**: 12/12/2025
