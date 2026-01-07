# 🤖 VMS AI Setup Guide

## Visão Geral

O serviço `vms-ai` processa frames de vídeo em tempo real usando modelos ONNX para:
- **Detecção de Objetos** (RT-DETR, YOLO, etc)
- **Tracking** (ByteTrack)
- **Geração de Eventos** (publicados no NATS)

## Arquitetura

```
vms.frames.{camera_id}  →  vms-ai  →  vms.events.ai.{camera_id}
     (NATS input)          (Process)       (NATS output)
```

### Fluxo:
1. ✅ vms-ai se inscreve em `vms.frames.>` (todos os frames)
2. ✅ Processa 1 frame a cada 30 (1 FPS em câmera 30 FPS)
3. ✅ Detecta objetos com RT-DETR
4. ✅ Faz tracking com ByteTrack
5. ✅ Publica evento em `vms.events.ai.{camera_id}`

---

## Configuração

### Variáveis de Ambiente

```bash
# NATS connection
export NATS_URL="nats://localhost:4222"

# Model path (ONNX format)
export AI_MODEL_PATH="./models/rtdetr.onnx"

# Logging
export RUST_LOG="info"
```

---

## Modelos Suportados

### 1. RT-DETR (Recomendado) ⭐

**Por quê usar**: Licença Apache 2.0 (livre para uso comercial)

**Download**:
```bash
# Criar diretório
mkdir -p models

# Download RT-DETR-L (via PyTorch/export)
# Opção 1: Export yourself (requer Python)
pip install torch ultralytics
python -c "
from ultralytics import RTDETR
model = RTDETR('rtdetr-l.pt')
model.export(format='onnx')
"
mv rtdetr-l.onnx models/rtdetr.onnx

# Opção 2: Download pre-converted (se disponível)
# wget https://path-to-onnx/rtdetr-l.onnx -O models/rtdetr.onnx
```

**Classes**: 80 classes do COCO
**Performance**: ~12ms/frame (GPU), ~50ms/frame (CPU)

### 2. YOLOv8 (Alternativa)

⚠️ **ATENÇÃO**: Licença GPL-3.0 - requer cuidado para uso comercial

**Download**:
```bash
pip install ultralytics
python -c "
from ultralytics import YOLO
model = YOLO('yolov8n.pt')
model.export(format='onnx')
"
mv yolov8n.onnx models/yolo.onnx
```

### 3. Modelo Customizado

**Requisitos**:
- Formato ONNX
- Input shape: `[batch, 3, height, width]` (RGB)
- Output: Detecções no formato YOLO/DETR

---

## Classes COCO (80 classes)

```
person, bicycle, car, motorcycle, airplane, bus, train, truck,
boat, traffic light, fire hydrant, stop sign, parking meter, bench,
bird, cat, dog, horse, sheep, cow, elephant, bear, zebra, giraffe,
backpack, umbrella, handbag, tie, suitcase, frisbee, skis, snowboard,
sports ball, kite, baseball bat, baseball glove, skateboard, surfboard,
tennis racket, bottle, wine glass, cup, fork, knife, spoon, bowl,
banana, apple, sandwich, orange, broccoli, carrot, hot dog, pizza,
donut, cake, chair, couch, potted plant, bed, dining table, toilet,
tv, laptop, mouse, remote, keyboard, cell phone, microwave, oven,
toaster, sink, refrigerator, book, clock, vase, scissors, teddy bear,
hair drier, toothbrush
```

---

## Modo de Operação

### Modo 1: Com Modelo (Detecção Ativa)

```bash
export AI_MODEL_PATH="./models/rtdetr.onnx"
./target/release/vms-ai
```

**Logs**:
```
🚀 VMS AI Service starting...
📡 NATS connected: nats://localhost:4222
🤖 AI Model path: ./models/rtdetr.onnx
✅ Model loaded successfully
🤖 AI processor started
🎯 Detected 3 objects
📤 Published AI event: 3 detections, 2 tracks
```

### Modo 2: Sem Modelo (Pass-through)

Se o arquivo de modelo não existir, o serviço roda sem detecção:

```bash
# Sem configurar AI_MODEL_PATH
./target/release/vms-ai
```

**Logs**:
```
🚀 VMS AI Service starting...
📡 NATS connected: nats://localhost:4222
⚠️  Model file not found - Running in pass-through mode
   To enable AI: place model at ./models/rtdetr.onnx
🤖 AI processor started
```

---

## Eventos Gerados

### Formato do Evento

Subject: `vms.events.ai.{camera_id}`

Payload (JSON):
```json
{
  "camera_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2025-12-12T23:30:00Z",
  "event_type": "object_detection",
  "frame_number": 1234,
  "detections": [
    {
      "class_id": 0,
      "class_name": "person",
      "confidence": 0.95,
      "bbox": {
        "x": 100.0,
        "y": 200.0,
        "width": 150.0,
        "height": 300.0
      }
    },
    {
      "class_id": 2,
      "class_name": "car",
      "confidence": 0.87,
      "bbox": {
        "x": 400.0,
        "y": 300.0,
        "width": 200.0,
        "height": 150.0
      }
    }
  ]
}
```

---

## Consumir Eventos de IA

### Exemplo em Rust:

```rust
use async_nats;

#[tokio::main]
async fn main() -> Result<()> {
    let client = async_nats::connect("nats://localhost:4222").await?;
    let mut subscriber = client.subscribe("vms.events.ai.>").await?;

    while let Some(msg) = subscriber.next().await {
        let event: AIEvent = serde_json::from_slice(&msg.payload)?;
        println!("Camera {}: {} detections",
            event.camera_id,
            event.detections.len()
        );
    }

    Ok(())
}
```

### Exemplo em Python:

```python
import nats
import json

async def main():
    nc = await nats.connect("nats://localhost:4222")

    async def message_handler(msg):
        event = json.loads(msg.data)
        print(f"Camera {event['camera_id']}: {len(event['detections'])} detections")

        for det in event['detections']:
            print(f"  - {det['class_name']}: {det['confidence']:.2f}")

    await nc.subscribe("vms.events.ai.*", cb=message_handler)

    # Keep running
    await asyncio.Event().wait()

if __name__ == '__main__':
    import asyncio
    asyncio.run(main())
```

---

## Performance Tuning

### 1. Ajustar Intervalo de Processamento

Editar `src/nats_consumer.rs`:
```rust
let process_interval = 30; // Process 1 frame per second at 30fps
// Mudar para:
let process_interval = 15; // Process 2 frames per second
```

### 2. Threshold de Confiança

```rust
let detector = ObjectDetector::new(path, 0.5)?; // 50% confidence
// Mudar para:
let detector = ObjectDetector::new(path, 0.7)?; // 70% (menos false positives)
```

### 3. Multi-GPU (Futuro)

Preparado para:
- NVIDIA TensorRT
- AMD ROCm
- Intel OpenVINO

---

## Métricas Prometheus

Endpoint: `http://localhost:9093/metrics`

```
vms_ai_frames_processed_total    # Total de frames processados
vms_ai_detections_total           # Total de objetos detectados
vms_ai_inference_time_ms          # Tempo de inferência (futuro)
```

---

## Troubleshooting

### Erro: "Model not loaded"
```
⚠️  Model not loaded: ... - Running without detection
```
**Solução**: Verificar se o arquivo ONNX existe no caminho configurado

### Erro: "Failed to connect to NATS"
```
Failed to connect to NATS: Connection refused
```
**Solução**: Iniciar infraestrutura: `./scripts/start-infrastructure.sh`

### Performance Lenta
**Sintomas**: FPS baixo, alta latência
**Soluções**:
1. Reduzir intervalo de processamento
2. Usar modelo menor (yolov8n ao invés de yolov8x)
3. Adicionar GPU (TensorRT)
4. Aumentar threshold de confiança

---

## Roadmap

- [ ] Suporte TensorRT (NVIDIA GPU)
- [ ] Suporte OpenVINO (Intel GPU/CPU)
- [ ] Suporte ROCm (AMD GPU)
- [ ] Suporte Core ML (Apple Silicon)
- [ ] Reconhecimento Facial
- [ ] Leitura de Placas (LPR)
- [ ] Anomaly Detection
- [ ] Pose Estimation

---

**Versão**: 0.1.0
**Última Atualização**: 12/12/2025
