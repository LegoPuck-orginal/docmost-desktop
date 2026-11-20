# 1-Klick Skript zum Pushen zu GitHub
Set-Location $PSScriptRoot
$git = "C:\Users\tim-m\AppData\Local\Programs\Git\cmd\git.exe"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "   Docmost Desktop: Push to GitHub" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

Write-Host "Pushe Branch 'main' und Tags (v0.1.0) zu GitHub..." -ForegroundColor Yellow
& $git push -u origin main --tags

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n✓ Erfolgreich zu GitHub hochgeladen!" -ForegroundColor Green
    Write-Host "GitHub Actions startet jetzt automatisch den Build und Release-Upload." -ForegroundColor White
} else {
    Write-Warning "`nPush noch nicht möglich. Bitte erstelle das Repository 'docmost-desktop' auf https://github.com/new"
}
