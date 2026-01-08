# VMS Development Helper v1.2
# Run services easily

param(
    [Parameter(Position=0)]
    [string]$Command = "help"
)

$VMS_ROOT = "c:\monitoring\backend\vms"

function Stop-Services {
    Write-Host "Stopping all VMS services..." -ForegroundColor Yellow
    Get-Process -Name "vms-api","vms-stream","vms-ingest","node" -ErrorAction SilentlyContinue | Stop-Process -Force
    @(9095, 9094, 1420, 1421) | ForEach-Object {
        Get-NetTCPConnection -LocalPort $_ -ErrorAction SilentlyContinue | ForEach-Object {
            Stop-Process -Id $_.OwningProcess -Force -ErrorAction SilentlyContinue
        }
    }
    Start-Sleep 1
    Write-Host "Services stopped." -ForegroundColor Green
}

function Start-Api {
    Write-Host "Starting VMS API on :9095..." -ForegroundColor Cyan
    Start-Process powershell -ArgumentList "-NoExit","-Command","cd $VMS_ROOT; cargo run -p vms-api" -WindowStyle Normal
}

function Start-Stream {
    Write-Host "Starting VMS Stream (WebRTC) on :9094..." -ForegroundColor Cyan
    $cmd = @"
`$env:GSTREAMER_1_0_ROOT_MSVC_X86_64='C:\Program Files\gstreamer\1.0\msvc_x86_64'
`$env:PATH=`$env:GSTREAMER_1_0_ROOT_MSVC_X86_64 + '\bin;' + `$env:PATH
cd '$VMS_ROOT'
cargo run -p vms-stream
"@
    Start-Process powershell -ArgumentList "-NoExit","-Command",$cmd -WindowStyle Normal
}

function Start-Admin {
    Write-Host "Starting VMS Admin..." -ForegroundColor Cyan
    Start-Process powershell -ArgumentList "-NoExit","-Command","cd $VMS_ROOT\apps\vms-admin; npm run tauri dev" -WindowStyle Normal
}

function Start-Viewer {
    Write-Host "Starting VMS Viewer..." -ForegroundColor Cyan
    Start-Process powershell -ArgumentList "-NoExit","-Command","cd $VMS_ROOT\apps\vms-viewer; npm run tauri dev" -WindowStyle Normal
}

function Start-Web {
    Write-Host "Starting VMS Web Client on :1422..." -ForegroundColor Cyan
    Start-Process powershell -ArgumentList "-NoExit","-Command","cd $VMS_ROOT\web-client; npm run dev -- --port 1422" -WindowStyle Normal
    Start-Sleep 3
    Start-Process "http://localhost:1422"
}

function Restart-All {
    Stop-Services
    Start-Sleep 2
    Start-Api
    Start-Sleep 5
    Start-Stream
}

switch ($Command) {
    "stop"    { Stop-Services }
    "api"     { Start-Api }
    "stream"  { Start-Stream }
    "admin"   { Start-Admin }
    "viewer"  { Start-Viewer }
    "web"     { Start-Web }
    "all"     { Restart-All }
    "restart" { Restart-All }
    default {
        Write-Host "VMS Development Helper v1.2" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "Usage: .\vms.ps1 <command>" -ForegroundColor White
        Write-Host ""
        Write-Host "Commands:" -ForegroundColor Yellow
        Write-Host "  stop     - Stop all services"
        Write-Host "  api      - Start VMS API (:9095) - Auth, Cameras, MJPEG, WebRTC signaling"
        Write-Host "  stream   - Start VMS Stream (:9094) - GStreamer WebRTC"
        Write-Host "  admin    - Start VMS Admin (Tauri)"
        Write-Host "  viewer   - Start VMS Viewer (Tauri)"
        Write-Host "  web      - Start Web Client (:1422) - Browser WebRTC"
        Write-Host "  all      - Start API + Stream"
        Write-Host "  restart  - Stop all, then start API + Stream"
    }
}
