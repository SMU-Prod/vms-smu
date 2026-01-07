# Arquitetura do VMS Enterprise

Esta pasta contém a documentação arquitetural do sistema.

## 📚 Documentos

### Architecture Decision Records (ADRs)

Os ADRs documentam decisões arquiteturais importantes e suas justificativas.

- [ADR-001: Escolha do Rust como Linguagem Principal](adr-001-rust-language.md)
- [ADR-002: GStreamer para Pipeline de Mídia](adr-002-gstreamer.md)
- [ADR-003: Arquitetura de Microserviços](adr-003-microservices.md)
- [ADR-004: Formato de Gravação Híbrido](adr-004-hybrid-storage.md)
- [ADR-005: Stack de Observabilidade](adr-005-observability.md)

### Diagramas

- [Diagrama de Alto Nível](diagrams/high-level.md)
- [Fluxo de Dados](diagrams/data-flow.md)
- [Arquitetura de Rede](diagrams/network.md)
- [Pipeline de IA](diagrams/ai-pipeline.md)

## 🏗️ Visão Geral

O VMS Enterprise é construído com uma arquitetura de microserviços que prioriza:

1. **Performance** - Latência ultra-baixa (<100ms)
2. **Escalabilidade** - Horizontal scaling nativo
3. **Confiabilidade** - Fault tolerance e HA
4. **Observabilidade** - Métricas, logs e traces completos

## 🔧 Componentes Principais

### Services

- **vms-ingest**: Ingestão de streams RTSP/ONVIF
- **vms-storage**: Gravação e playback de vídeo
- **vms-ai**: Pipeline de IA para detecção e análise
- **vms-stream**: Distribuição WebRTC/SRT
- **vms-api**: API Gateway REST/GraphQL
- **vms-gateway**: Service discovery e configuração
- **vms-replicator**: Backup e disaster recovery

### Libraries

- **vms-common**: Tipos e utilitários compartilhados
- **vms-proto**: Definições Protocol Buffers
- **vms-format**: Formato de gravação híbrido
- **vms-telemetry**: Instrumentação OpenTelemetry

## 🌊 Fluxo de Dados

```
Câmera IP (RTSP)
    │
    ▼
vms-ingest (GStreamer)
    │
    ├──▶ vms-storage (Gravação)
    │
    ├──▶ vms-ai (Processamento)
    │
    └──▶ vms-stream (Distribuição)
         │
         └──▶ Clientes (WebRTC)
```

## 📊 Tecnologias Principais

| Camada | Tecnologia |
|--------|------------|
| Linguagem | Rust 1.75+ |
| Pipeline Mídia | GStreamer 1.22+ |
| Async Runtime | Tokio |
| Streaming | WebRTC, SRT, QUIC |
| IA/ML | ONNX Runtime, TensorRT |
| Observabilidade | OpenTelemetry, Prometheus |
| Storage | RocksDB, PostgreSQL, MinIO |
| Messaging | NATS JetStream |

## 🔐 Segurança

- TLS 1.3 para todas as comunicações
- SRT AES-256 para streams
- JWT com RS256 para autenticação
- RBAC granular para autorização
- Criptografia em repouso opcional

## 📈 Metas de Performance

| Métrica | Alvo |
|---------|------|
| Latência intranet | <100ms (P95) |
| Latência web | <200ms (P95) |
| Playback seek | <500ms |
| Alerta de IA | <1s |

## 🚀 Roadmap

Veja [../../instruct.md](../../instruct.md) para o roadmap completo de desenvolvimento.

## 📝 Contribuindo

Ao fazer mudanças arquiteturais significativas, crie um ADR seguindo o template em `adr-template.md`.
