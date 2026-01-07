#!/bin/bash
# Script para compilar todos os serviços do VMS

set -e

cd "$(dirname "$0")/.."

echo "🔨 VMS Enterprise - Building All Services"
echo "=========================================="
echo ""

# Configurar GStreamer no Windows
if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    export PKG_CONFIG_PATH="/c/Program Files/GStreamer/1.0/msvc_x86_64/lib/pkgconfig"
    export PKG_CONFIG="/c/Program Files/GStreamer/1.0/msvc_x86_64/bin/pkg-config.exe"
    export PATH="/c/Program Files/GStreamer/1.0/msvc_x86_64/bin:$PATH"
    echo "✅ GStreamer configured for Windows"
fi

echo ""
echo "📦 Building workspace..."
cargo build --workspace --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ All services built successfully!"
    echo ""
    echo "Binaries available at:"
    echo "  - vms-ingest:     target/release/vms-ingest"
    echo "  - vms-storage:    target/release/vms-storage"
    echo "  - vms-ai:         target/release/vms-ai"
    echo "  - vms-stream:     target/release/vms-stream"
    echo "  - vms-api:        target/release/vms-api"
    echo "  - vms-gateway:    target/release/vms-gateway"
    echo "  - vms-replicator: target/release/vms-replicator"
    echo ""
else
    echo ""
    echo "❌ Build failed!"
    exit 1
fi
