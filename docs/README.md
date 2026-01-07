# VMS Enterprise
## Video Management System de Próxima Geração

[![Rust Version](https://img.shields.io/badge/rust-1.75+-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Sistema de gerenciamento de vídeo moderno construído em Rust com foco em:
- 🚀 Latência ultra-baixa (<100ms)
- 🤖 IA integrada desde a concepção
- 📊 Observabilidade completa
- 🔄 Arquitetura de microserviços escalável

## 🏗️ Arquitetura

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Edge Devices   │────▶│  Ingestão Core   │────▶│  Storage Layer  │
│  (Jetson/x86)   │     │  Rust/GStreamer  │     │  Hybrid Format  │
└─────────────────┘     └──────────────────┘     └─────────────────┘
```

## 🚀 Quick Start

### Pré-requisitos

- Rust 1.75+
- GStreamer 1.22+
- Docker & Docker Compose (para observabilidade)

### Instalação

```bash
# Clone o repositório
git clone https://github.com/your-org/vms-enterprise.git
cd vms-enterprise

# Instale as dependências
cargo build

# Execute os testes
cargo test

# Inicie o ambiente de desenvolvimento
./scripts/setup-dev.sh
```

## 📦 Componentes

### Serviços Core

- **vms-ingest**: Ingestão de câmeras RTSP/ONVIF
- **vms-storage**: Gravação e playback
- **vms-ai**: Pipeline de IA multi-GPU
- **vms-stream**: Distribuição WebRTC/SRT
- **vms-api**: Gateway REST/GraphQL
- **vms-gateway**: Service discovery
- **vms-replicator**: Backup e DR

### Bibliotecas Compartilhadas

- **vms-common**: Tipos e utilitários compartilhados
- **vms-proto**: Definições Protocol Buffers
- **vms-format**: Formato de gravação híbrido
- **vms-telemetry**: Instrumentação OpenTelemetry

## 🎯 Stack Tecnológica

| Componente | Tecnologia |
|------------|------------|
| Linguagem Core | Rust 1.75+ |
| Pipeline de Mídia | GStreamer 1.22+ |
| Async Runtime | Tokio 1.x |
| Streaming | SRT, WebRTC, QUIC |
| IA/ML | ONNX Runtime, TensorRT |
| Observabilidade | OpenTelemetry, Prometheus, Grafana |

## 📊 Performance

- **Latência intranet**: <100ms (P95)
- **Latência web**: <200ms (P95)
- **Capacidade**: 100-200 câmeras por instância
- **Viewers**: 1000+ simultâneos

## 🔧 Desenvolvimento

```bash
# Executar formatação
cargo fmt

# Executar linter
cargo clippy --all-targets -- -D warnings

# Executar testes
cargo test --all

# Executar benchmarks
cargo bench

# Verificar vulnerabilidades
cargo audit
```

## 📖 Documentação

- [Arquitetura Completa](docs/architecture/README.md)
- [API Documentation](docs/api/README.md)
- [Runbooks](docs/runbooks/README.md)
- [Guia do Usuário](docs/user-guide/README.md)

## 🛣️ Roadmap

- [x] **Fase 0**: POC (2 meses)
- [ ] **Fase 1**: Core System (4-5 meses)
- [ ] **Fase 2**: Observabilidade (1-2 meses)
- [ ] **Fase 3**: IA e Analytics (3-4 meses)
- [ ] **Fase 4**: Enterprise (4-5 meses)
- [ ] **Fase 5**: Edge Computing (2-3 meses)

## 📄 Licença

Licenciado sob MIT ou Apache-2.0, à sua escolha.

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor, leia nosso [guia de contribuição](CONTRIBUTING.md).

## 📧 Contato

Para questões e suporte, abra uma issue no GitHub.

---

**Versão**: 0.1.0 (Fase 0 - POC)
