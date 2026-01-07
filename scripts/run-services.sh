#!/bin/bash
# Script para iniciar todos os serviços do VMS

set -e

cd "$(dirname "$0")/.."

echo "🚀 VMS Enterprise - Starting Services"
echo "======================================"
echo ""

# Verificar se a infraestrutura está rodando
if ! docker ps | grep -q nats; then
    echo "⚠️  Infrastructure not running!"
    echo "   Run: ./scripts/start-infrastructure.sh"
    exit 1
fi

echo "✅ Infrastructure is running"
echo ""

# Variáveis de ambiente
export NATS_URL="nats://localhost:4222"
export STORAGE_PATH="./storage"
export RUST_LOG="info"

# Configurar GStreamer no Windows
if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    export PKG_CONFIG_PATH="/c/Program Files/GStreamer/1.0/msvc_x86_64/lib/pkgconfig"
    export PKG_CONFIG="/c/Program Files/GStreamer/1.0/msvc_x86_64/bin/pkg-config.exe"
    export PATH="/c/Program Files/GStreamer/1.0/msvc_x86_64/bin:$PATH"
fi

# Criar diretório de storage
mkdir -p ./storage

echo "🎬 Starting VMS services..."
echo ""

# Função para iniciar serviço em background
start_service() {
    local name=$1
    local port=$2
    local binary=$3

    echo "▶️  Starting $name on port $port..."
    "$binary" > "logs/${name}.log" 2>&1 &
    echo $! > "logs/${name}.pid"
}

# Criar diretório de logs
mkdir -p logs

# Iniciar serviços
echo "1/7 Starting vms-storage..."
target/release/vms-storage > logs/vms-storage.log 2>&1 &
echo $! > logs/vms-storage.pid
sleep 2

echo "2/7 Starting vms-ingest..."
target/release/vms-ingest > logs/vms-ingest.log 2>&1 &
echo $! > logs/vms-ingest.pid
sleep 2

echo "3/7 Starting vms-stream..."
target/release/vms-stream > logs/vms-stream.log 2>&1 &
echo $! > logs/vms-stream.pid
sleep 2

echo "4/7 Starting vms-ai..."
target/release/vms-ai > logs/vms-ai.log 2>&1 &
echo $! > logs/vms-ai.pid
sleep 2

echo "5/7 Starting vms-api..."
target/release/vms-api > logs/vms-api.log 2>&1 &
echo $! > logs/vms-api.pid
sleep 2

echo "6/7 Starting vms-gateway..."
target/release/vms-gateway > logs/vms-gateway.log 2>&1 &
echo $! > logs/vms-gateway.pid
sleep 2

echo "7/7 Starting vms-replicator..."
target/release/vms-replicator > logs/vms-replicator.log 2>&1 &
echo $! > logs/vms-replicator.pid

sleep 3

echo ""
echo "✅ All services started!"
echo ""
echo "Services running on:"
echo "  - vms-ingest:     http://localhost:9091/metrics"
echo "  - vms-storage:    http://localhost:9092/metrics"
echo "  - vms-ai:         http://localhost:9093/metrics"
echo "  - vms-stream:     http://localhost:9094/metrics"
echo "  - vms-api:        http://localhost:8080"
echo "  - vms-gateway:    http://localhost:8081"
echo "  - vms-replicator: http://localhost:9095"
echo ""
echo "Logs available in logs/ directory"
echo ""
echo "To stop all services:"
echo "  ./scripts/stop-services.sh"
echo ""
