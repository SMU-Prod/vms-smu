# VMS Development Helper
# Run services easily

param(
    [Parameter(Position=0)]
    [string]$Command = "help"
)

$VMS_ROOT = "c:\monitoring\backend\vms"

function Stop-Services {
    Write-Host "Stopping all VMS services..." -ForegroundColor Yellow
    Get-Process -Name "vms-server","vms-node","vms-admin","vms-viewer","node" -ErrorAction SilentlyContinue | Stop-Process -Force
    @(9095, 8090, 1420, 1421) | ForEach-Object {
        Get-NetTCPConnection -LocalPort $_ -ErrorAction SilentlyContinue | ForEach-Object {
            Stop-Process -Id $_.OwningProcess -Force -ErrorAction SilentlyContinue
        }
    }
    Start-Sleep 1
    Write-Host "Services stopped." -ForegroundColor Green
}

function Start-Server {
    Write-Host "Starting VMS Server on :9095..." -ForegroundColor Cyan
    Start-Process powershell -ArgumentList "-NoExit","-Command","cd $VMS_ROOT; cargo run -p vms_server" -WindowStyle Normal
}

function Start-Node {
    Write-Host "Starting VMS Node on :8090..." -ForegroundColor Cyan
    $gst = "C:\Program Files\gstreamer\1.0\msvc_x86_64"
    Start-Process powershell -ArgumentList "-NoExit","-Command","`$env:GSTREAMER_1_0_ROOT_MSVC_X86_64='$gst'; `$env:PATH=`"`$env:GSTREAMER_1_0_ROOT_MSVC_X86_64\bin;`$env:PATH`"; cd $VMS_ROOT; cargo run -p vms_node" -WindowStyle Normal
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
    Start-Server
    Start-Sleep 5
    Start-Node
}

switch ($Command) {
    "stop"    { Stop-Services }
    "server"  { Start-Server }
    "node"    { Start-Node }
    "admin"   { Start-Admin }
    "viewer"  { Start-Viewer }
    "web"     { Start-Web }
    "all"     { Restart-All }
    "restart" { Restart-All }
    default {
        Write-Host "VMS Development Helper" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "Usage: .\vms.ps1 <command>" -ForegroundColor White
        Write-Host ""
        Write-Host "Commands:" -ForegroundColor Yellow
        Write-Host "  stop     - Stop all services"
        Write-Host "  server   - Start VMS Server (:9095)"
        Write-Host "  node     - Start VMS Node (:8090)"
        Write-Host "  admin    - Start VMS Admin (Tauri)"
        Write-Host "  viewer   - Start VMS Viewer (Tauri)"
        Write-Host "  web      - Start Web Client (:1422) - Browser WebRTC"
        Write-Host "  all      - Start Server + Node"
        Write-Host "  restart  - Stop all, then start Server + Node"
    }
}
