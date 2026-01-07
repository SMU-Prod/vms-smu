#!/bin/bash

# VMS Enterprise - Development Setup Script
# Este script configura o ambiente de desenvolvimento

set -e

echo "🚀 VMS Enterprise - Development Setup"
echo "======================================"
echo ""

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Função para verificar se um comando existe
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Verificar pré-requisitos
echo "📋 Verificando pré-requisitos..."

# Rust
if command_exists rustc; then
    RUST_VERSION=$(rustc --version)
    echo -e "${GREEN}✓${NC} Rust: $RUST_VERSION"
else
    echo -e "${RED}✗${NC} Rust não encontrado. Instale via: https://rustup.rs/"
    exit 1
fi

# Docker
if command_exists docker; then
    DOCKER_VERSION=$(docker --version)
    echo -e "${GREEN}✓${NC} Docker: $DOCKER_VERSION"
else
    echo -e "${YELLOW}⚠${NC} Docker não encontrado. Alguns recursos podem não funcionar."
fi

# Docker Compose
if command_exists docker-compose; then
    COMPOSE_VERSION=$(docker-compose --version)
    echo -e "${GREEN}✓${NC} Docker Compose: $COMPOSE_VERSION"
else
    echo -e "${YELLOW}⚠${NC} Docker Compose não encontrado."
fi

# GStreamer (opcional no Windows, mas recomendado)
if command_exists gst-launch-1.0; then
    GST_VERSION=$(gst-launch-1.0 --version | head -1)
    echo -e "${GREEN}✓${NC} GStreamer: $GST_VERSION"
else
    echo -e "${YELLOW}⚠${NC} GStreamer não encontrado. Instale via: https://gstreamer.freedesktop.org/"
fi

echo ""
echo "📦 Instalando dependências Rust..."
cargo fetch

echo ""
echo "🔧 Instalando ferramentas de desenvolvimento..."

# cargo-watch para hot reload
if ! command_exists cargo-watch; then
    echo "Instalando cargo-watch..."
    cargo install cargo-watch
fi

# cargo-audit para verificação de vulnerabilidades
if ! command_exists cargo-audit; then
    echo "Instalando cargo-audit..."
    cargo install cargo-audit
fi

# cargo-deny para verificação de licenças
if ! command_exists cargo-deny; then
    echo "Instalando cargo-deny..."
    cargo install cargo-deny
fi

echo ""
echo "🔍 Verificando código..."
cargo fmt --check || echo -e "${YELLOW}⚠${NC} Execute 'cargo fmt' para formatar o código"
cargo clippy --all-targets -- -D warnings || echo -e "${YELLOW}⚠${NC} Corrija os avisos do clippy"

echo ""
echo "🧪 Executando testes..."
cargo test --all

echo ""
echo "🐋 Iniciando stack de observabilidade..."
if command_exists docker-compose; then
    cd deploy/compose
    docker-compose -f docker-compose.monitoring.yml up -d
    cd ../..

    echo ""
    echo -e "${GREEN}✓${NC} Stack de observabilidade iniciada!"
    echo ""
    echo "📊 Serviços disponíveis:"
    echo "  - Grafana:     http://localhost:3000 (admin/admin)"
    echo "  - Prometheus:  http://localhost:9090"
    echo "  - Loki:        http://localhost:3100"
    echo "  - Tempo:       http://localhost:3200"
    echo "  - Alertmanager: http://localhost:9093"
else
    echo -e "${YELLOW}⚠${NC} Docker Compose não disponível. Pule esta etapa."
fi

echo ""
echo "✅ Setup completo!"
echo ""
echo "📚 Próximos passos:"
echo "  1. Inicie o serviço de ingestão: cargo run -p vms-ingest"
echo "  2. Acesse Grafana em http://localhost:3000"
echo "  3. Configure uma câmera RTSP para teste"
echo ""
echo "💡 Comandos úteis:"
echo "  - cargo watch -x run           # Hot reload"
echo "  - cargo test                   # Executar testes"
echo "  - cargo clippy                 # Linter"
echo "  - cargo audit                  # Verificar vulnerabilidades"
echo "  - docker-compose logs -f       # Ver logs dos containers"
echo ""
