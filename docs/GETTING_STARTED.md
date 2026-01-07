# Getting Started - VMS Enterprise

Bem-vindo ao VMS Enterprise! Este documento irá guiá-lo através da configuração inicial e primeiros passos.

## 📋 Status do Projeto

✅ **Fase 0 - POC (Proof of Concept)** - Estrutura Inicial Completa

### O que foi implementado:

1. ✅ **Estrutura do Workspace Rust**
   - Workspace multi-crate configurado
   - 7 serviços core definidos
   - 4 bibliotecas compartilhadas

2. ✅ **Biblioteca vms-common**
   - Tipos básicos (CameraId, StreamId, Resolution, etc.)
   - Tipos para câmeras e streams
   - Sistema de configuração
   - Tratamento de erros

3. ✅ **Serviço vms-ingest (POC)**
   - Pipeline GStreamer básico
   - Suporte RTSP/H.264
   - Framework para processamento de frames

4. ✅ **Stack de Observabilidade Completa**
   - Prometheus para métricas
   - Loki para logs
   - Tempo para traces
   - Grafana para visualização
   - OpenTelemetry Collector
   - Alertmanager com regras prontas

5. ✅ **Documentação**
   - README principal
   - Guia de contribuição
   - Templates de ADR
   - Configuração de exemplo

6. ✅ **Infraestrutura**
   - Docker Compose para desenvolvimento
   - Scripts de setup (PowerShell e Bash)
   - Configurações de CI/CD prontas

## 🚀 Pré-requisitos

### Essenciais

- **Rust 1.75+** - [Instalar](https://rustup.rs/)
- **Git** - [Instalar](https://git-scm.com/)

### Opcionais (mas recomendados)

- **Docker Desktop** - [Instalar](https://www.docker.com/products/docker-desktop)
- **GStreamer 1.22+** - Necessário para compilar vms-ingest

### Para Windows

Para compilar o vms-ingest no Windows, você precisa:

1. **GStreamer MSVC Development**
   - Baixe de: https://gstreamer.freedesktop.org/download/#windows
   - Instale AMBOS:
     - GStreamer 1.0 MSVC 64-bit runtime
     - GStreamer 1.0 MSVC 64-bit development

2. **Configurar variáveis de ambiente:**
   ```powershell
   # Adicione ao PATH
   $env:Path += ";C:\gstreamer\1.0\msvc_x86_64\bin"

   # Defina GSTREAMER_1_0_ROOT_MSVC_X86_64
   $env:GSTREAMER_1_0_ROOT_MSVC_X86_64 = "C:\gstreamer\1.0\msvc_x86_64\"
   ```

3. **pkg-config para Windows**
   - Incluído no GStreamer development installer

## 🏃 Quick Start

### 1. Clone o repositório (quando disponível)

```bash
git clone https://github.com/your-org/vms-enterprise.git
cd vms-enterprise
```

### 2. Execute o setup

**Windows (PowerShell):**
```powershell
.\scripts\setup-dev.ps1
```

**Linux/macOS:**
```bash
chmod +x scripts/setup-dev.sh
./scripts/setup-dev.sh
```

### 3. Compile as bibliotecas

```bash
# Compilar todas as bibliotecas (não requer GStreamer)
cargo build -p vms-common -p vms-proto -p vms-format -p vms-telemetry
```

### 4. Execute os testes

```bash
cargo test --all
```

### 5. Inicie a stack de observabilidade

```bash
cd deploy/compose
docker-compose -f docker-compose.monitoring.yml up -d
```

Acesse:
- **Grafana**: http://localhost:3000 (admin/admin)
- **Prometheus**: http://localhost:9090
- **Alertmanager**: http://localhost:9093

## 📁 Estrutura do Projeto

```
vms-enterprise/
├── services/           # Microserviços
│   ├── vms-ingest/    # ✅ Ingestão RTSP (POC implementado)
│   ├── vms-storage/   # ⏳ Gravação (próxima fase)
│   ├── vms-ai/        # ⏳ Pipeline IA (próxima fase)
│   ├── vms-stream/    # ⏳ WebRTC/SRT (próxima fase)
│   ├── vms-api/       # ⏳ REST API (próxima fase)
│   ├── vms-gateway/   # ⏳ Service discovery (próxima fase)
│   └── vms-replicator/# ⏳ Backup/DR (próxima fase)
│
├── libs/              # Bibliotecas compartilhadas
│   ├── vms-common/    # ✅ Tipos básicos
│   ├── vms-proto/     # ⏳ Protocol Buffers
│   ├── vms-format/    # ⏳ Formato híbrido
│   └── vms-telemetry/ # ⏳ OpenTelemetry
│
├── monitoring/        # ✅ Configurações completas
├── deploy/            # ✅ Docker Compose
├── docs/              # ✅ Documentação
└── scripts/           # ✅ Scripts de setup
```

## 🎯 Próximos Passos

### Fase 1: Core System (4-5 meses)

1. **Completar vms-ingest**
   - [ ] Implementar reconexão automática
   - [ ] Adicionar suporte a múltiplas câmeras
   - [ ] Implementar health checks
   - [ ] Adicionar métricas Prometheus

2. **Implementar vms-storage**
   - [ ] Formato de gravação híbrido (MKV + índice)
   - [ ] Sistema de rotação de arquivos
   - [ ] API de playback
   - [ ] Seek rápido com índice

3. **Implementar vms-stream**
   - [ ] Servidor WebRTC
   - [ ] Servidor SRT
   - [ ] Distribuição multi-viewer
   - [ ] Adaptive bitrate

4. **Implementar vms-api**
   - [ ] REST API com Axum
   - [ ] OpenAPI/Swagger
   - [ ] Autenticação JWT
   - [ ] RBAC

## 🛠️ Desenvolvimento

### Compilar um serviço específico

```bash
cargo build -p vms-ingest
```

### Executar um serviço

```bash
cargo run -p vms-ingest
```

### Hot reload durante desenvolvimento

```bash
cargo install cargo-watch
cargo watch -x 'run -p vms-ingest'
```

### Executar testes

```bash
# Todos os testes
cargo test --all

# Testes de um pacote
cargo test -p vms-common

# Com output
cargo test -- --nocapture
```

### Verificar código

```bash
# Formatação
cargo fmt

# Linting
cargo clippy --all-targets -- -D warnings

# Verificar vulnerabilidades
cargo audit

# Verificar licenças
cargo deny check
```

## 🐛 Problemas Conhecidos

### Windows: Erro ao compilar vms-ingest

**Problema:** `pkg-config command could not be found`

**Solução:** Instale o GStreamer MSVC Development e configure as variáveis de ambiente conforme descrito acima.

### WSL: GStreamer não encontrado

**Solução:**
```bash
sudo apt-get update
sudo apt-get install -y libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev \
    gstreamer1.0-plugins-base gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly
```

### Docker: Permissões negadas

**Solução (Linux):**
```bash
sudo usermod -aG docker $USER
# Faça logout e login novamente
```

## 📚 Recursos

- **Documentação Arquitetural**: [docs/architecture/](docs/architecture/)
- **Especificação Completa**: [instruct.md](instruct.md)
- **Guia de Contribuição**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **Configuração de Exemplo**: [config.example.toml](config.example.toml)

## 💡 Dicas

1. **Use o VSCode** com as extensões:
   - rust-analyzer
   - CodeLLDB (para debugging)
   - Better TOML
   - Docker

2. **Configure o Grafana** para visualizar métricas em tempo real

3. **Use uma câmera IP real ou simulador** para testes:
   ```bash
   # Simulador RTSP (Docker)
   docker run --rm -p 8554:8554 aler9/rtsp-simple-server
   ```

## 🤝 Contribuindo

Veja [CONTRIBUTING.md](CONTRIBUTING.md) para diretrizes de contribuição.

## 📧 Suporte

- **Issues**: Para bugs e features
- **Discussions**: Para perguntas e ideias

---

**Versão**: 0.1.0 (Fase 0 - POC)
**Última atualização**: 2025-12-12
