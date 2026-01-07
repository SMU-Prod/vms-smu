# VMS Enterprise - Development Setup Script (PowerShell)
# Este script configura o ambiente de desenvolvimento no Windows

$ErrorActionPreference = "Stop"

Write-Host "🚀 VMS Enterprise - Development Setup" -ForegroundColor Green
Write-Host "======================================" -ForegroundColor Green
Write-Host ""

# Função para verificar se um comando existe
function Test-CommandExists {
    param($command)
    $null = Get-Command $command -ErrorAction SilentlyContinue
    return $?
}

# Verificar pré-requisitos
Write-Host "📋 Verificando pré-requisitos..." -ForegroundColor Cyan

# Rust
if (Test-CommandExists rustc) {
    $rustVersion = rustc --version
    Write-Host "✓ Rust: $rustVersion" -ForegroundColor Green
} else {
    Write-Host "✗ Rust não encontrado. Instale via: https://rustup.rs/" -ForegroundColor Red
    exit 1
}

# Docker
if (Test-CommandExists docker) {
    $dockerVersion = docker --version
    Write-Host "✓ Docker: $dockerVersion" -ForegroundColor Green
} else {
    Write-Host "⚠ Docker não encontrado. Alguns recursos podem não funcionar." -ForegroundColor Yellow
}

# Docker Compose
if (Test-CommandExists docker-compose) {
    $composeVersion = docker-compose --version
    Write-Host "✓ Docker Compose: $composeVersion" -ForegroundColor Green
} else {
    Write-Host "⚠ Docker Compose não encontrado." -ForegroundColor Yellow
}

# GStreamer
if (Test-CommandExists gst-launch-1.0) {
    $gstVersion = gst-launch-1.0 --version 2>&1 | Select-Object -First 1
    Write-Host "✓ GStreamer: $gstVersion" -ForegroundColor Green
} else {
    Write-Host "⚠ GStreamer não encontrado. Instale via: https://gstreamer.freedesktop.org/download/#windows" -ForegroundColor Yellow
    Write-Host "  Recomendado: MSVC 64-bit (desenvolvimento completo)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "📦 Instalando dependências Rust..." -ForegroundColor Cyan
cargo fetch

Write-Host ""
Write-Host "🔧 Instalando ferramentas de desenvolvimento..." -ForegroundColor Cyan

# cargo-watch para hot reload
if (-not (Test-CommandExists cargo-watch)) {
    Write-Host "Instalando cargo-watch..." -ForegroundColor Yellow
    cargo install cargo-watch
}

# cargo-audit para verificação de vulnerabilidades
if (-not (Test-CommandExists cargo-audit)) {
    Write-Host "Instalando cargo-audit..." -ForegroundColor Yellow
    cargo install cargo-audit
}

# cargo-deny para verificação de licenças
if (-not (Test-CommandExists cargo-deny)) {
    Write-Host "Instalando cargo-deny..." -ForegroundColor Yellow
    cargo install cargo-deny
}

Write-Host ""
Write-Host "🔍 Verificando código..." -ForegroundColor Cyan
try {
    cargo fmt --check
    Write-Host "✓ Formatação OK" -ForegroundColor Green
} catch {
    Write-Host "⚠ Execute 'cargo fmt' para formatar o código" -ForegroundColor Yellow
}

try {
    cargo clippy --all-targets -- -D warnings
    Write-Host "✓ Clippy OK" -ForegroundColor Green
} catch {
    Write-Host "⚠ Corrija os avisos do clippy" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "🧪 Executando testes..." -ForegroundColor Cyan
cargo test --all

Write-Host ""
Write-Host "🐋 Iniciando stack de observabilidade..." -ForegroundColor Cyan
if (Test-CommandExists docker-compose) {
    Push-Location deploy\compose
    docker-compose -f docker-compose.monitoring.yml up -d
    Pop-Location

    Write-Host ""
    Write-Host "✓ Stack de observabilidade iniciada!" -ForegroundColor Green
    Write-Host ""
    Write-Host "📊 Serviços disponíveis:" -ForegroundColor Cyan
    Write-Host "  - Grafana:      http://localhost:3000 (admin/admin)" -ForegroundColor White
    Write-Host "  - Prometheus:   http://localhost:9090" -ForegroundColor White
    Write-Host "  - Loki:         http://localhost:3100" -ForegroundColor White
    Write-Host "  - Tempo:        http://localhost:3200" -ForegroundColor White
    Write-Host "  - Alertmanager: http://localhost:9093" -ForegroundColor White
} else {
    Write-Host "⚠ Docker Compose não disponível. Pule esta etapa." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "✅ Setup completo!" -ForegroundColor Green
Write-Host ""
Write-Host "📚 Próximos passos:" -ForegroundColor Cyan
Write-Host "  1. Inicie o serviço de ingestão: cargo run -p vms-ingest" -ForegroundColor White
Write-Host "  2. Acesse Grafana em http://localhost:3000" -ForegroundColor White
Write-Host "  3. Configure uma câmera RTSP para teste" -ForegroundColor White
Write-Host ""
Write-Host "💡 Comandos úteis:" -ForegroundColor Cyan
Write-Host "  - cargo watch -x run              # Hot reload" -ForegroundColor White
Write-Host "  - cargo test                      # Executar testes" -ForegroundColor White
Write-Host "  - cargo clippy                    # Linter" -ForegroundColor White
Write-Host "  - cargo audit                     # Verificar vulnerabilidades" -ForegroundColor White
Write-Host "  - docker-compose logs -f          # Ver logs dos containers" -ForegroundColor White
Write-Host ""
Write-Host "📖 Para instalar GStreamer no Windows:" -ForegroundColor Yellow
Write-Host "  1. Baixe de https://gstreamer.freedesktop.org/download/#windows" -ForegroundColor White
Write-Host "  2. Instale MSVC 64-bit runtime E development" -ForegroundColor White
Write-Host "  3. Adicione ao PATH: C:\gstreamer\1.0\msvc_x86_64\bin" -ForegroundColor White
Write-Host "  4. Defina GSTREAMER_1_0_ROOT_MSVC_X86_64=C:\gstreamer\1.0\msvc_x86_64\" -ForegroundColor White
Write-Host ""
