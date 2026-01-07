#!/bin/bash
# Script para iniciar infraestrutura do VMS

set -e

cd "$(dirname "$0")/.."

echo "🚀 VMS Enterprise - Starting Infrastructure"
echo "=========================================="
echo ""

# Verificar se Docker está rodando
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker não está rodando!"
    echo "   Por favor, inicie o Docker Desktop"
    exit 1
fi

echo "✅ Docker detectado"
echo ""

# Iniciar infraestrutura
echo "📦 Starting infrastructure services..."
docker-compose -f deploy/compose/docker-compose.infrastructure.yml up -d

echo ""
echo "⏳ Waiting for services to be healthy..."
sleep 5

# Verificar status
echo ""
echo "📊 Service Status:"
docker-compose -f deploy/compose/docker-compose.infrastructure.yml ps

echo ""
echo "✅ Infrastructure ready!"
echo ""
echo "Services available at:"
echo "  - NATS:       nats://localhost:4222 (management: http://localhost:8222)"
echo "  - PostgreSQL: postgresql://vms_user:vms_password@localhost:5432/vms"
echo "  - Redis:      redis://localhost:6379"
echo "  - MinIO:      http://localhost:9001 (console) / http://localhost:9002 (API)"
echo ""
echo "Next steps:"
echo "  1. Build services: ./scripts/build-all.sh"
echo "  2. Run services:   ./scripts/run-services.sh"
echo ""
