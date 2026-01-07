#!/bin/bash
# Script para parar todos os serviços do VMS

cd "$(dirname "$0")/.."

echo "🛑 VMS Enterprise - Stopping Services"
echo "======================================"
echo ""

# Função para parar serviço
stop_service() {
    local name=$1
    local pidfile="logs/${name}.pid"

    if [ -f "$pidfile" ]; then
        local pid=$(cat "$pidfile")
        if ps -p $pid > /dev/null 2>&1; then
            echo "🛑 Stopping $name (PID: $pid)..."
            kill $pid
            rm "$pidfile"
        else
            echo "⚠️  $name not running (stale PID file)"
            rm "$pidfile"
        fi
    else
        echo "⚠️  $name PID file not found"
    fi
}

# Parar serviços
stop_service "vms-replicator"
stop_service "vms-gateway"
stop_service "vms-api"
stop_service "vms-ai"
stop_service "vms-stream"
stop_service "vms-ingest"
stop_service "vms-storage"

echo ""
echo "✅ All services stopped!"
echo ""
