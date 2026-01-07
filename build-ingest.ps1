# Build script para vms-ingest com GStreamer
# Configura PATH e compila

Write-Host "🔧 Configurando ambiente GStreamer..." -ForegroundColor Cyan

# Recarregar PATH
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","User") + ";" + [System.Environment]::GetEnvironmentVariable("Path","Machine")

# Verificar pkg-config
Write-Host "📦 Verificando pkg-config..." -ForegroundColor Cyan
$pkgConfigVersion = pkg-config --version 2>$null
if ($pkgConfigVersion) {
    Write-Host "✅ pkg-config encontrado: v$pkgConfigVersion" -ForegroundColor Green
} else {
    Write-Host "❌ pkg-config não encontrado!" -ForegroundColor Red
    exit 1
}

# Verificar GStreamer
Write-Host "🎬 Verificando GStreamer..." -ForegroundColor Cyan
$gstVersion = pkg-config --modversion gstreamer-1.0 2>$null
if ($gstVersion) {
    Write-Host "✅ GStreamer encontrado: v$gstVersion" -ForegroundColor Green
} else {
    Write-Host "⚠️  GStreamer não encontrado via pkg-config" -ForegroundColor Yellow
}

# Compilar
Write-Host "`n🔨 Compilando vms-ingest..." -ForegroundColor Cyan
cargo build --package vms-ingest

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n✅ Compilação concluída com sucesso!" -ForegroundColor Green
} else {
    Write-Host "`n❌ Falha na compilação!" -ForegroundColor Red
    exit 1
}
