# VMS Enterprise
## Video Management System de Próxima Geração
### Documento de Arquitetura Técnica Completa
**Versão 2.0** | 12/12/2025

---

## 1. Sumário Executivo

Este documento apresenta a arquitetura técnica completa de um Video Management System (VMS) de próxima geração, projetado para superar soluções tradicionais como o Digifort em termos de performance, escalabilidade e tecnologia. O sistema foi concebido com foco em quatro pilares fundamentais:

1. **Latência ultra-baixa** para streaming em tempo real
2. **Processamento de IA integrado** desde a concepção
3. **Arquitetura de microserviços** moderna e escalável
4. **Observabilidade completa** para operação 24/7

A escolha de Rust como linguagem principal para os componentes críticos de performance garante segurança de memória sem garbage collector, eliminando classes inteiras de bugs que afetam sistemas de vigilância 24/7. A arquitetura proposta suporta desde instalações pequenas com poucas câmeras até deployments enterprise com milhares de dispositivos.

---

## 2. Visão Geral da Arquitetura

### 2.1 Princípios de Design

A arquitetura segue princípios fundamentais que garantem robustez e evolução sustentável:

| Princípio | Descrição |
|-----------|-----------|
| **Performance First** | Cada decisão arquitetural prioriza latência mínima e throughput máximo |
| **Zero-Copy Where Possible** | Minimização de cópias de memória no pipeline de vídeo |
| **Horizontal Scalability** | Capacidade de escalar adicionando mais nós sem reconfiguração |
| **Fault Tolerance** | Sistema continua operando mesmo com falhas parciais |
| **AI-Native** | Processamento de IA integrado no pipeline principal |
| **Observable by Design** | Métricas, logs e traces como cidadãos de primeira classe |
| **Edge-First** | Suporte a processamento distribuído na borda |

### 2.2 Diagrama de Alto Nível

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Edge Devices   │     │  Ingestão Core   │     │  Storage Layer  │
│  (Jetson/x86)   │────▶│  Rust/GStreamer  │────▶│  Hybrid Format  │
└─────────────────┘     └──────────────────┘     └─────────────────┘
         │                       │                        │
         │                       ▼                        │
         │              ┌──────────────────┐              │
         │              │  AI Processing   │              │
         │              │  Multi-GPU Pool  │◀─────────────┘
         │              └──────────────────┘
         │                       │
         ▼                       ▼
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Local Cache    │     │  Distribution    │     │  Clients        │
│  + Alerting     │     │  WebRTC/SRT      │────▶│  Web/Desktop/   │
└─────────────────┘     └──────────────────┘     │  Mobile         │
                                                 └─────────────────┘
                                 │
                                 ▼
                        ┌──────────────────┐
                        │  Observability   │
                        │  Prometheus/Loki │
                        └──────────────────┘
```

---

## 3. Stack Tecnológica Detalhada

### 3.1 Core de Ingestão e Streaming

#### Linguagem Principal: Rust
Rust foi escolhida como linguagem principal para todos os componentes críticos de performance. A decisão é fundamentada em:
- Performance equivalente a C/C++ com garantias de segurança de memória
- Ausência de garbage collector eliminando pausas imprevisíveis
- Sistema de ownership que previne data races
- Ecossistema maduro com crates de alta qualidade

#### Framework de Mídia: GStreamer
GStreamer é o framework de pipeline de mídia mais robusto disponível:
- Suporte nativo para RTSP, ONVIF, H.264/H.265/AV1
- Arquitetura de plugins extensível
- Integração com aceleração de hardware (VAAPI/NVDEC/VideoToolbox/QSV)
- Bindings Rust via `gstreamer-rs`

#### Runtime Assíncrono: Tokio
Tokio permite gerenciar milhares de conexões simultâneas com overhead mínimo, utilizando modelo de work-stealing para distribuição eficiente de tarefas.

### 3.2 Componentes e Dependências

| Componente | Tecnologia | Justificativa |
|------------|------------|---------------|
| Linguagem Core | Rust 1.75+ | Memory-safe sem GC |
| Pipeline de Mídia | GStreamer 1.22+ | Suporte universal de codecs |
| Async Runtime | Tokio 1.x | Milhares de conexões |
| Bindings GStreamer | gstreamer-rs | API Rust idiomática |
| Codecs Auxiliares | FFmpeg (bindings) | Fallback e formatos raros |
| Serialização | Protocol Buffers | Comunicação binária eficiente |
| Observabilidade | OpenTelemetry | Métricas, logs e traces unificados |

### 3.3 Protocolos de Streaming

| Protocolo | Latência | Uso Principal | Biblioteca |
|-----------|----------|---------------|------------|
| SRT | 20-60ms | Intranet primário | srt-rs |
| WebRTC | <200ms | Web browsers | webrtc-rs |
| QUIC/HTTP3 | 50-100ms | APIs e metadados | quinn |
| LL-HLS | 1-2s | Fallback universal | Custom |

#### SRT (Secure Reliable Transport)
Protocolo primário para streaming interno:
- Latência consistentemente baixa mesmo com jitter
- Criptografia AES-128/256 integrada
- Recuperação de pacotes via ARQ
- API simples similar a sockets UDP

#### WebRTC
Distribuição para clientes web:
- Latência sub-200ms
- Funciona nativamente sem plugins
- NAT traversal automático via ICE/STUN/TURN
- Adaptive bitrate nativo

---

## 4. Camada de Armazenamento

### 4.1 Formato de Gravação Híbrido

O sistema utiliza uma abordagem híbrida que combina compatibilidade com performance:

```
Estrutura de Camadas:
├── Container Externo: MKV ou MP4 fragmentado (fMP4)
│   └── Compatível com players padrão para compliance/legal
│
├── Índice Proprietário: .vms-idx
│   └── Timestamps precisos + offsets para seek instantâneo
│   └── Metadados de IA (bounding boxes, tracks)
│
└── Sidecar de Eventos: .vms-events (Parquet)
    └── Formato columnar para analytics
    └── Exportável para PowerBI/Metabase
```

### 4.2 Estrutura de Arquivos

```
/storage/cameras/{camera_id}/{date}/
├── video_{hour}.mkv      # Dados de vídeo (1 arquivo por hora)
├── index_{hour}.vms-idx  # Índice proprietário para seek rápido
├── events_{hour}.parquet # Eventos detectados pela IA
├── thumb_{hour}.webp     # Thumbnails para preview (WebP = menor)
└── checksum_{hour}.sha256 # Integridade para DR
```

### 4.3 Bancos de Dados

| Banco | Uso | Justificativa |
|-------|-----|---------------|
| **RocksDB** | Metadados de alta escrita | Performance de escrita com LZ4 |
| **PostgreSQL + Citus** | Dados relacionais principais | Sharding horizontal para enterprise |
| **Redis + TimeSeries** | Cache e métricas temporais | Status real-time com agregação |
| **MinIO/Ceph** | Object storage distribuído | Replicação e erasure coding |

---

## 5. Processamento de IA

### 5.1 Arquitetura de Inferência Multi-GPU

O pipeline de IA suporta múltiplos backends para flexibilidade de hardware:

| GPU/Hardware | Runtime | Performance | Custo-Benefício |
|--------------|---------|-------------|-----------------|
| NVIDIA RTX/A-Series | TensorRT | ⭐⭐⭐⭐⭐ | Alto |
| NVIDIA Jetson | TensorRT | ⭐⭐⭐⭐ | Edge computing |
| AMD RDNA/CDNA | ROCm + MIGraphX | ⭐⭐⭐⭐ | Médio |
| Intel Arc/Xeon | OpenVINO | ⭐⭐⭐ | Bom custo-benefício |
| Apple Silicon | Core ML | ⭐⭐⭐⭐ | Clientes Mac/iOS |
| CPU (Fallback) | tract + ONNX Runtime | ⭐⭐ | Instalações básicas |

### 5.2 Stack de Machine Learning

```toml
# Runtimes de Inferência
tract-onnx = "0.21"       # ONNX nativo em Rust (CPU)
ort = "2.0"               # ONNX Runtime com GPU
tch-rs = "0.14"           # PyTorch bindings

# Opcional por plataforma
tensorrt-rs = "0.5"       # NVIDIA TensorRT
openvino-rs = "0.6"       # Intel OpenVINO
```

### 5.3 Modelos e Licenciamento

> ⚠️ **IMPORTANTE**: Verifique licenças antes de uso comercial!

| Funcionalidade | Modelo | Performance | Licença | Comercial |
|----------------|--------|-------------|---------|-----------|
| Detecção de Objetos | **RT-DETR-L** | 12ms/frame | Apache 2.0 | ✅ Livre |
| Detecção (Alt) | YOLO-World | 15ms/frame | GPL-3.0 | ⚠️ Consultar |
| Reconhecimento Facial | ArcFace | 8ms/face | MIT | ✅ Livre |
| Leitura de Placas (LPR) | PaddleOCR | 25ms/placa | Apache 2.0 | ✅ Livre |
| Tracking | ByteTrack | 2ms/frame | MIT | ✅ Livre |
| Pose Estimation | MediaPipe | 12ms/pessoa | Apache 2.0 | ✅ Livre |
| Anomaly Detection | Custom AE/VAE | 20ms/frame | Próprio | ✅ Próprio |

**Recomendação**: Use RT-DETR ao invés de YOLO para evitar custos de licença GPL.

---

## 6. Observabilidade

### 6.1 Stack de Monitoramento

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  VMS Services   │────▶│  OpenTelemetry  │────▶│  Backends       │
│                 │     │  Collector      │     │                 │
│  - Metrics      │     │                 │     │  - Prometheus   │
│  - Logs         │     │                 │     │  - Loki         │
│  - Traces       │     │                 │     │  - Tempo        │
└─────────────────┘     └─────────────────┘     └─────────────────┘
                                                        │
                                                        ▼
                                                ┌─────────────────┐
                                                │  Grafana        │
                                                │  Dashboards     │
                                                └─────────────────┘
                                                        │
                                                        ▼
                                                ┌─────────────────┐
                                                │  Alertmanager   │
                                                │  (SMS/Slack/    │
                                                │   Email/PD)     │
                                                └─────────────────┘
```

### 6.2 Métricas Críticas

| Categoria | Métricas | Alerta |
|-----------|----------|--------|
| **Ingestão** | `vms_camera_fps`, `vms_camera_bitrate`, `vms_camera_status` | FPS < 20 ou offline > 30s |
| **Latência** | `vms_e2e_latency_ms`, `vms_webrtc_rtt_ms` | > 500ms por 1min |
| **Storage** | `vms_disk_usage_bytes`, `vms_write_throughput` | > 85% ou < 50MB/s |
| **IA** | `vms_ai_inference_ms`, `vms_ai_queue_depth` | > 100ms ou queue > 100 |
| **Sistema** | `vms_cpu_usage`, `vms_memory_usage`, `vms_gpu_utilization` | > 90% por 5min |

### 6.3 Dashboards Recomendados

1. **Overview** - Saúde geral do sistema
2. **Cameras** - Status individual de cada câmera
3. **AI Pipeline** - Performance de inferência
4. **Storage** - Uso de disco e retenção
5. **Alerts** - Histórico de alertas

---

## 7. Arquitetura de Microserviços

### 7.1 Serviços Core

| Serviço | Responsabilidade | Escala |
|---------|------------------|--------|
| `vms-ingest` | Conexão RTSP/ONVIF, transcoding | 100-200 câmeras/instância |
| `vms-storage` | Gravação, playback, exports | Por throughput de disco |
| `vms-ai` | Pipeline de IA, detecção | Por GPU disponível |
| `vms-stream` | Distribuição WebRTC/SRT | 1000+ viewers/instância |
| `vms-api` | Gateway REST/GraphQL | Por requests/s |
| `vms-gateway` | Service discovery, config | 1 por cluster |
| `vms-replicator` | Backup, DR, sincronização | 1-3 por deployment |

### 7.2 Comunicação Entre Serviços

| Tipo | Tecnologia | Uso |
|------|------------|-----|
| Síncrono | gRPC + Protocol Buffers | APIs internas |
| Assíncrono | NATS JetStream | Frames de vídeo, eventos |
| Pub/Sub | NATS Core | Notificações real-time |

---

## 8. Clientes

### 8.1 Cliente Desktop: Tauri 2.0

```
Vantagens Tauri vs Electron:
├── Binário: ~10MB vs ~150MB
├── RAM: 50-100MB vs 300-500MB
├── Decodificação: Hardware nativa
└── Segurança: Sandbox por padrão
```

### 8.2 Cliente Web: SolidJS

- **Sem Virtual DOM** - Reatividade granular
- **Bundle menor** - ~7KB base
- **WebCodecs API** - Decodificação hardware no browser

### 8.3 Cliente Mobile: Flutter

- **Compilação AOT** - 60fps consistente
- **Cross-platform** - iOS e Android
- **Platform channels** - Acesso a decodificação nativa

---

## 9. Edge Computing

### 9.1 Arquitetura Edge

Para instalações distribuídas, o processamento na borda reduz banda e aumenta resiliência:

```
┌─────────────────────────────────────────────────────────────────┐
│                        SITE REMOTO                              │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐       │
│  │  Câmeras    │───▶│  Edge Node  │────▶│  Storage    │       │
│  │  IP         │     │  (Jetson/   │     │  Local      │       │
│  └─────────────┘     │   Mini-PC)  │     │  (7-30 dias)│       │
│                      │             │     └─────────────┘       │
│                      │  - Ingestão │            │              │
│                      │  - IA Básica│            │              │
│                      │  - Alertas  │            ▼              │
│                      └─────────────┘     ┌─────────────┐       │
│                             │            │  Replicação │       │
│                             │            │  para Cloud │       │
└─────────────────────────────│────────────└──────┬──────┘───────┘
                              │                   │
                              ▼                   ▼
                      ┌───────────────────────────────────┐
                      │         CENTRAL / CLOUD           │
                      │  - Storage centralizado           │
                      │  - IA avançada (GPUs potentes)    │
                      │  - Management e dashboards        │
                      │  - Correlação multi-site          │
                      └───────────────────────────────────┘
```

### 9.2 Benefícios Edge

| Benefício | Descrição |
|-----------|-----------|
| **Redução de Banda** | Só transmite eventos, não stream contínuo |
| **Resiliência** | Funciona offline, sincroniza depois |
| **Latência Local** | Alertas em <1s mesmo sem internet |
| **Escalabilidade** | Adiciona sites sem sobrecarregar central |

---

## 10. Segurança

### 10.1 Autenticação e Autorização

| Componente | Tecnologia | Descrição |
|------------|------------|-----------|
| Identity Provider | OAuth 2.0 / OIDC | Integração com AD, Okta, Auth0 |
| Tokens | JWT com RS256 | Assinados e com expiração curta |
| Permissões | RBAC granular | Por câmera, grupo, função |
| MFA | TOTP/WebAuthn | Obrigatório para admins |

### 10.2 Criptografia

| Camada | Tecnologia | Detalhes |
|--------|------------|----------|
| Em trânsito | TLS 1.3 | Todas as comunicações |
| Streaming | SRT AES-256 | Streams intranet |
| Em repouso | AES-256-GCM | Gravações (opcional) |
| Chaves | HashiCorp Vault | Rotação automática |

### 10.3 Segurança de Rede

```
Arquitetura de VLANs Recomendada:
├── VLAN 10: Câmeras (isolada, sem internet)
├── VLAN 20: Storage (acesso restrito)
├── VLAN 30: Management (VPN required)
├── VLAN 40: Clients (acesso controlado)
└── VLAN 50: Edge nodes (túnel criptografado)
```

### 10.4 Hardening Adicional

| Medida | Implementação |
|--------|---------------|
| Rate Limiting | 100 req/min por IP na API |
| Token de Stream | URLs assinadas com expiração de 5min |
| Audit Logging | Append-only log com hash chain |
| SBOM | Gerado em cada build para compliance |
| Vulnerability Scan | `cargo audit` + Trivy no CI |

---

## 11. Disaster Recovery e Alta Disponibilidade

### 11.1 Estratégias de Backup

| Cenário | Estratégia | RTO | RPO |
|---------|------------|-----|-----|
| Falha de disco | RAID 10 + hot spare | <5min | 0 |
| Falha de servidor | Storage replicado (MinIO) | <15min | <1min |
| Falha de site | Replicação offsite assíncrona | <1h | <15min |
| Corrupção de dados | Snapshots + rebuild de índice | <30min | <5min |

### 11.2 Alta Disponibilidade

```
┌─────────────────┐     ┌─────────────────┐
│  Primary Node   │◀───▶│  Secondary Node │
│  (Active)       │     │  (Standby)      │
└─────────────────┘     └─────────────────┘
         │                       │
         └───────────┬───────────┘
                     │
              ┌──────▼──────┐
              │  PostgreSQL │
              │  Streaming  │
              │  Replication│
              └─────────────┘
                     │
              ┌──────▼──────┐
              │  MinIO      │
              │  Erasure    │
              │  Coding     │
              └─────────────┘
```

### 11.3 Procedimentos de Recovery

1. **Rebuild de Índice**: Reconstrução automática a partir do vídeo raw
2. **Failover de Ingestão**: Câmeras com dual-stream para nós diferentes
3. **Restore de Database**: Point-in-time recovery com WAL archiving
4. **Validação de Integridade**: Checksums SHA-256 por arquivo

---

## 12. Testes e QA

### 12.1 Estratégia de Testes

| Tipo | Ferramenta | Cobertura Alvo |
|------|------------|----------------|
| Unit Tests | `cargo test` + proptest | >80% do código |
| Integration | testcontainers-rs | Todos os serviços |
| Load Testing | k6 / Locust | 1000+ viewers |
| Chaos Engineering | Chaos Mesh | Falhas de rede/disco |
| E2E Desktop | Playwright + Tauri | Fluxos críticos |
| E2E Web | Playwright | Fluxos críticos |
| Benchmark | Criterion.rs | Regressão de latência |

### 12.2 Pipeline CI/CD

```yaml
# GitHub Actions / GitLab CI
stages:
  - lint:
      - cargo fmt --check
      - cargo clippy --all-targets -- -D warnings
      
  - test:
      - cargo test --all
      - cargo test --features integration
      
  - security:
      - cargo audit
      - trivy image vms-*
      
  - benchmark:
      - cargo bench --no-run
      - ./scripts/compare-benchmarks.sh
      
  - build:
      - docker build -t vms-ingest ./services/vms-ingest
      - docker build -t vms-storage ./services/vms-storage
      # ...
      
  - deploy:
      - kubectl apply -k ./deploy/k8s/
```

### 12.3 Métricas de Qualidade

| Métrica | Threshold | Ação |
|---------|-----------|------|
| Test Coverage | >80% | Block merge |
| Clippy Warnings | 0 | Block merge |
| Benchmark Regression | <5% | Warning |
| Benchmark Regression | >10% | Block merge |
| Security Vulns (High) | 0 | Block deploy |

---

## 13. APIs e Integrações

### 13.1 APIs Expostas

| API | Tipo | Uso |
|-----|------|-----|
| `/api/v1/*` | REST (OpenAPI 3.0) | Clientes web/mobile |
| `/graphql` | GraphQL | Queries complexas |
| `vms.*` | gRPC | Integrações server-to-server |
| `/ws/*` | WebSocket | Eventos real-time |

### 13.2 Integrações Suportadas

```
vms-integrations/
├── access-control/
│   ├── hid-global/
│   ├── lenel/
│   └── c-cure/
│
├── alarms/
│   ├── bosch/
│   ├── honeywell/
│   └── intelbras/
│
├── analytics/
│   ├── powerbi/
│   ├── metabase/
│   └── grafana/
│
├── cloud/
│   ├── aws-kinesis/
│   ├── azure-video-analyzer/
│   └── gcp-video-ai/
│
├── protocols/
│   ├── mqtt/
│   ├── modbus/
│   └── bacnet/
│
└── webhooks/
    └── generic/    # POST JSON para qualquer endpoint
```

### 13.3 SDK e Documentação

| Recurso | Descrição |
|---------|-----------|
| OpenAPI Spec | Gerada automaticamente |
| SDK TypeScript | Para clientes web |
| SDK Python | Para automações |
| SDK Rust | Para integrações nativas |
| Postman Collection | Exemplos de uso |

---

## 14. Metas de Performance

### 14.1 Latência End-to-End

| Cenário | Meta | Medição |
|---------|------|---------|
| Live view intranet (SRT) | **< 100ms** | P95 |
| Live view web (WebRTC) | **< 200ms** | P95 |
| Playback seek | **< 500ms** | Qualquer ponto |
| Alerta de IA | **< 1 segundo** | Detecção → notificação |
| Export de vídeo | **< 2x tempo real** | Throughput |
| Startup do cliente | **< 3 segundos** | First frame |

### 14.2 Escalabilidade

| Componente | Capacidade por Instância |
|------------|--------------------------|
| Câmeras por ingestão | 100-200 streams 1080p ou 50-100 4K |
| Viewers simultâneos | 1000+ por servidor de streaming |
| Análise de IA | 30 câmeras simultâneas por RTX 4080 |
| Armazenamento | Petabytes com cluster distribuído |

### 14.3 Comparativo com Digifort

| Aspecto | Digifort | VMS Enterprise |
|---------|----------|----------------|
| Latência live view | 200-500ms | **50-100ms** |
| IA integrada | Módulo adicional | **Nativa no core** |
| Cliente web | ActiveX/Plugin | **WebRTC nativo** |
| API | SDK proprietário | **REST + GraphQL + gRPC** |
| Escalabilidade | Vertical | **Horizontal nativa** |
| Suporte 4K/8K | Limitado | **Full nativo** |
| Edge computing | Não | **Sim** |
| Multi-GPU | Limitado | **Pool dinâmico** |
| Observabilidade | Básica | **OpenTelemetry completo** |

---

## 15. Infraestrutura e Deploy

### 15.1 Containerização

Todos os serviços são containerizados com Docker:
- Imagens multi-stage para tamanho mínimo
- Base: `debian:bookworm-slim` ou `alpine` quando possível
- GPU: `nvidia/cuda:12.x-runtime` para serviços de IA

### 15.2 Orquestração

| Escala | Solução | Justificativa |
|--------|---------|---------------|
| Pequena (<32 câmeras) | Docker Compose | Simples, um arquivo |
| Média (32-128 câmeras) | Docker Swarm | Orquestração básica |
| Enterprise (>128 câmeras) | Kubernetes | Full HA, auto-scaling |

### 15.3 Requisitos de Hardware

#### Instalação Pequena (até 32 câmeras)
- **CPU**: Intel Core i7 / AMD Ryzen 7
- **RAM**: 32GB DDR4
- **GPU**: NVIDIA RTX 3060 ou Intel Arc A770
- **Storage**: SSD NVMe 1TB + HDD 8TB
- **Rede**: 1 Gbps

#### Instalação Média (até 128 câmeras)
- **CPU**: Intel Xeon / AMD EPYC (16+ cores)
- **RAM**: 128GB DDR5 ECC
- **GPU**: 2x NVIDIA RTX 4080 ou 1x A4000
- **Storage**: RAID NVMe + Storage 100TB+
- **Rede**: 10 Gbps

#### Instalação Enterprise (500+ câmeras)
- **Cluster Kubernetes** com múltiplos nós
- **Storage distribuído** (Ceph/MinIO)
- **GPUs dedicadas** em pool (A100/H100)
- **Rede**: 25-100 Gbps entre nós

---

## 16. Roadmap de Desenvolvimento

### 16.1 Fases do Projeto

| Fase | Duração | Entregas |
|------|---------|----------|
| **Fase 0: POC** | 2 meses | Validar GStreamer+Rust, SRT, WebRTC básico |
| **Fase 1: Core** | 4-5 meses | Ingestão, gravação híbrida, streaming, API, cliente web |
| **Fase 2: Observabilidade** | 1-2 meses | Prometheus, Loki, Grafana, alertas |
| **Fase 3: IA** | 3-4 meses | Pipeline multi-GPU, RT-DETR, eventos, busca |
| **Fase 4: Enterprise** | 4-5 meses | Desktop Tauri, mobile Flutter, HA, integrações |
| **Fase 5: Edge** | 2-3 meses | Edge nodes, replicação, modo offline |
| **Total** | **16-21 meses** | Sistema completo |

### 16.2 Detalhamento das Fases

#### Fase 0: Proof of Concept (2 meses)
- [ ] Setup projeto Rust + workspace
- [ ] Pipeline GStreamer básico (RTSP → decode → display)
- [ ] Teste de latência SRT
- [ ] WebRTC básico com webrtc-rs
- [ ] Benchmark e validação de decisões arquiteturais

#### Fase 1: Core System (4-5 meses)
- [ ] Serviço vms-ingest completo
- [ ] Formato de gravação híbrido (MKV + índice)
- [ ] Serviço vms-storage com playback
- [ ] Serviço vms-stream (WebRTC + SRT)
- [ ] API REST com OpenAPI
- [ ] Cliente web SolidJS básico
- [ ] Testes unitários e integração

#### Fase 2: Observabilidade (1-2 meses)
- [ ] Instrumentação OpenTelemetry
- [ ] Deploy Prometheus + Loki + Grafana
- [ ] Dashboards padrão
- [ ] Alertmanager + runbooks

#### Fase 3: IA e Analytics (3-4 meses)
- [ ] Pipeline de IA multi-GPU
- [ ] Integração RT-DETR + ByteTrack
- [ ] Sistema de eventos e regras
- [ ] Dashboard de analytics
- [ ] Busca por eventos e objetos

#### Fase 4: Enterprise (4-5 meses)
- [ ] Cliente desktop Tauri
- [ ] Cliente mobile Flutter
- [ ] Clustering e HA
- [ ] Integrações (alarmes, controle de acesso)
- [ ] Reconhecimento facial e LPR avançado

#### Fase 5: Edge Computing (2-3 meses)
- [ ] Imagem para Jetson/mini-PC
- [ ] Sincronização e replicação
- [ ] Modo offline com cache local
- [ ] UI de gerenciamento de sites remotos

---

## 17. Conclusão

Esta arquitetura v2.0 representa o estado da arte em sistemas de gerenciamento de vídeo, com melhorias significativas em:

1. **Observabilidade** - Monitoramento completo desde o início
2. **Flexibilidade de Hardware** - Suporte multi-GPU além de NVIDIA
3. **Resiliência** - DR e HA bem definidos
4. **Qualidade** - Estratégia de testes robusta
5. **Escalabilidade** - Edge computing para sites distribuídos
6. **Compliance** - Formato híbrido para requisitos legais
7. **Licenciamento** - Escolha de modelos open-source quando possível

Os próximos passos recomendados são:
1. Validação da arquitetura com POC (Fase 0)
2. Definição da equipe e capacitação em Rust
3. Setup de infraestrutura de desenvolvimento
4. Início do desenvolvimento da Fase 1

---

## Apêndice A: Dependências Rust (Cargo.toml)

```toml
[workspace]
members = [
    "services/vms-ingest",
    "services/vms-storage",
    "services/vms-ai",
    "services/vms-stream",
    "services/vms-api",
    "services/vms-gateway",
    "services/vms-replicator",
    "libs/vms-common",
    "libs/vms-proto",
    "libs/vms-format",
]

[workspace.dependencies]
# Runtime e Async
tokio = { version = "1", features = ["full"] }
tokio-stream = "0.1"

# Media Pipeline
gstreamer = "0.22"
gstreamer-app = "0.22"
gstreamer-video = "0.22"

# Networking
tonic = "0.11"           # gRPC
prost = "0.12"           # Protocol Buffers
axum = "0.7"             # HTTP framework
tower = "0.4"            # Middleware
hyper = "1.0"            # HTTP
quinn = "0.10"           # QUIC/HTTP3

# Streaming
webrtc = "0.9"
srt-rs = "0.3"

# Messaging
async-nats = "0.33"

# Database
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio"] }
redis = "0.24"
rocksdb = "0.21"

# AI/ML
tract-onnx = "0.21"      # ONNX CPU inference
ort = "2.0"              # ONNX Runtime (GPU)

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
parquet = "51"           # Eventos analytics

# Observability
opentelemetry = "0.21"
opentelemetry-otlp = "0.14"
tracing = "0.1"
tracing-subscriber = "0.3"
tracing-opentelemetry = "0.22"
metrics = "0.21"
metrics-exporter-prometheus = "0.12"

# Utilities
uuid = { version = "1", features = ["v4", "v7"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1"
anyhow = "1"
config = "0.14"          # Configuration management

# Security
jsonwebtoken = "9"
argon2 = "0.5"           # Password hashing
sha2 = "0.10"            # Checksums

# Testing
proptest = "1"
testcontainers = "0.15"
criterion = "0.5"
```

---

## Apêndice B: Estrutura de Projeto

```
vms-enterprise/
├── services/
│   ├── vms-ingest/       # Ingestão de câmeras
│   ├── vms-storage/      # Gravação e playback
│   ├── vms-ai/           # Pipeline de IA
│   ├── vms-stream/       # Distribuição de streams
│   ├── vms-api/          # API Gateway
│   ├── vms-gateway/      # Service discovery
│   └── vms-replicator/   # Backup e DR
│
├── libs/
│   ├── vms-common/       # Types e utils compartilhados
│   ├── vms-proto/        # Definições Protocol Buffers
│   ├── vms-format/       # Formato de gravação híbrido
│   └── vms-telemetry/    # Instrumentação OpenTelemetry
│
├── clients/
│   ├── web/              # SolidJS web app
│   ├── desktop/          # Tauri desktop app
│   └── mobile/           # Flutter mobile app
│
├── edge/
│   ├── vms-edge/         # Versão compacta para edge
│   └── configs/          # Configs por hardware (Jetson, etc)
│
├── integrations/
│   ├── access-control/
│   ├── alarms/
│   ├── analytics/
│   └── webhooks/
│
├── deploy/
│   ├── docker/           # Dockerfiles
│   ├── k8s/              # Kubernetes manifests
│   ├── compose/          # Docker Compose configs
│   └── terraform/        # IaC para cloud
│
├── monitoring/
│   ├── prometheus/       # Prometheus configs
│   ├── grafana/          # Dashboards JSON
│   ├── loki/             # Loki configs
│   └── alertmanager/     # Alert rules
│
├── tests/
│   ├── e2e/              # Playwright tests
│   ├── load/             # k6 scripts
│   └── chaos/            # Chaos engineering
│
├── scripts/
│   ├── setup-dev.sh      # Setup ambiente dev
│   ├── benchmark.sh      # Run benchmarks
│   └── generate-sbom.sh  # Gerar SBOM
│
├── docs/
│   ├── architecture/     # ADRs e diagramas
│   ├── api/              # OpenAPI specs
│   ├── runbooks/         # Procedimentos operacionais
│   └── user-guide/       # Documentação de usuário
│
├── .github/
│   └── workflows/        # CI/CD pipelines
│
├── Cargo.toml            # Workspace root
├── rust-toolchain.toml   # Versão do Rust
├── deny.toml             # cargo-deny config
└── README.md
```

---

## Apêndice C: Configuração de Observabilidade

### docker-compose.monitoring.yml

```yaml
version: '3.8'

services:
  prometheus:
    image: prom/prometheus:v2.48.0
    volumes:
      - ./monitoring/prometheus:/etc/prometheus
      - prometheus-data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.retention.time=30d'
    ports:
      - "9090:9090"

  loki:
    image: grafana/loki:2.9.0
    volumes:
      - ./monitoring/loki:/etc/loki
      - loki-data:/loki
    command: -config.file=/etc/loki/loki.yml
    ports:
      - "3100:3100"

  tempo:
    image: grafana/tempo:2.3.0
    volumes:
      - ./monitoring/tempo:/etc/tempo
      - tempo-data:/var/tempo
    command: -config.file=/etc/tempo/tempo.yml
    ports:
      - "4317:4317"   # OTLP gRPC
      - "4318:4318"   # OTLP HTTP

  grafana:
    image: grafana/grafana:10.2.0
    volumes:
      - ./monitoring/grafana/provisioning:/etc/grafana/provisioning
      - grafana-data:/var/lib/grafana
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
      - GF_USERS_ALLOW_SIGN_UP=false
    ports:
      - "3000:3000"

  alertmanager:
    image: prom/alertmanager:v0.26.0
    volumes:
      - ./monitoring/alertmanager:/etc/alertmanager
    command:
      - '--config.file=/etc/alertmanager/alertmanager.yml'
    ports:
      - "9093:9093"

  otel-collector:
    image: otel/opentelemetry-collector-contrib:0.91.0
    volumes:
      - ./monitoring/otel:/etc/otelcol
    command: --config=/etc/otelcol/otel-collector.yml
    ports:
      - "4317:4317"
      - "4318:4318"

volumes:
  prometheus-data:
  loki-data:
  tempo-data:
  grafana-data:
```

---

*Documento gerado em 12/12/2025 - VMS Enterprise v2.0*