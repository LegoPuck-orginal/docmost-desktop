# Build-Skript für Docmost Desktop App & Windows Setup Installer
$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "   Docmost Desktop: Build & Package" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

# 1. Prüfe Voraussetzungen
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "Fehler: 'cargo' wurde im PATH nicht gefunden!"
    exit 1
}

$makensisCmd = Get-Command makensis -ErrorAction SilentlyContinue
if (-not $makensisCmd) {
    if (Test-Path "C:\Users\tim-m\w64devkit\bin\makensis.exe") {
        $makensisPath = "C:\Users\tim-m\w64devkit\bin\makensis.exe"
    } else {
        Write-Error "Fehler: 'makensis' (NSIS) wurde nicht gefunden!"
        exit 1
    }
} else {
    $makensisPath = $makensisCmd.Source
}

Write-Host "`n[1/3] Kompiliere Rust Anwendung im Release-Modus..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Rust-Build fehlgeschlagen!"
    exit $LASTEXITCODE
}

$exePath = "target\release\docmost-app.exe"
if (-not (Test-Path $exePath)) {
    Write-Error "Executable $exePath wurde nicht gefunden!"
    exit 1
}

$exeSize = (Get-Item $exePath).Length / 1MB
Write-Host ("✓ docmost-app.exe erfolgreich erstellt ({0:N2} MB)" -f $exeSize) -ForegroundColor Green

# 2. Erstelle dist Ordner
if (-not (Test-Path "dist")) {
    New-Item -ItemType Directory -Path "dist" | Out-Null
}

# 3. Erstelle Installer via NSIS
Write-Host "`n[2/3] Erstelle Windows Installer via NSIS..." -ForegroundColor Yellow
& $makensisPath "installer.nsi"
if ($LASTEXITCODE -ne 0) {
    Write-Error "NSIS Installer-Erstellung fehlgeschlagen!"
    exit $LASTEXITCODE
}

$installerPath = "dist\Docmost-Setup.exe"
if (-not (Test-Path $installerPath)) {
    Write-Error "Installer $installerPath wurde nicht erzeugt!"
    exit 1
}

$installerSize = (Get-Item $installerPath).Length / 1MB
Write-Host ("✓ Installer erfolgreich erstellt: {0} ({1:N2} MB)" -f $installerPath, $installerSize) -ForegroundColor Green

Write-Host "`n==========================================" -ForegroundColor Cyan
Write-Host " Fertig! Der Installer liegt bereit:" -ForegroundColor Green
Write-Host " dist\Docmost-Setup.exe" -ForegroundColor White
Write-Host "==========================================" -ForegroundColor Cyan
