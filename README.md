# Docmost Desktop Client (Clean Dark Edition)

Eine moderne, blitzschnelle und ressourcenschonende Windows Desktop-Applikation für **[Docmost](https://docmost.com)**, entwickelt in **Rust** mit nativem WebView2, Clean Dark Design und integriertem GitHub Release Auto-Updater.

---

## Features

- 🎨 **Clean Dark Design:**
  - Modernes, tiefdunkles Farbschema (`#0d0f17`, `#141722`, `#6366f1` Indigo-Akzent)
  - Windows DWM Dark Mode für native dunkle Titelleisten (Windows 10/11)
  - Feingetunte dunkle Scrollbalken und optimierte Kontraste
- ⚡ **Rust + WebView2 Performance:**
  - Startet in unter einer Sekunde
  - Minimaler Arbeitsspeicherverbrauch (<50 MB RAM im Vergleich zu 300+ MB bei Electron)
  - Volle Unterstützung für Docmost: kollaborativer Tiptap-Editor, Echtzeit-Synchronisation, Markdown, Dateiuploads und Tastatur-Shortcuts
- 🚀 **Automatischer GitHub-Updater:**
  - Prüft beim Start automatisch im Hintergrund auf neuere Releases auf GitHub
  - Zeigt bei neuen Versionen ein Update-Pop-up mit Changelog
  - 1-Klick-Download und automatischer Start des Installers
- 📦 **Windows Setup Installer (.exe):**
  - Erstellt mit NSIS (`makensis`)
  - Installiert nach `%LOCALAPPDATA%\Programs\Docmost` (keine UAC-Admin-Berechtigung zwingend erforderlich)
  - Desktop- und Startmenü-Verknüpfungen sowie sauberer Uninstaller in Windows Apps & Features
- ⚙ **Sleek Desktop Toolbar & Einstellungs-Menü:**
  - Schnelle Navigation (Zurück, Vorwärts, Neu laden, Home)
  - Server-URL flexibel anpassbar (Standard: `https://doc.home.klehausen.de`)
  - Auto-Update Ein-/Ausschalter & manueller "Jetzt suchen"-Button

---

## Projektstruktur

```
docmost-desktop/
├── Cargo.toml                    # Rust Manifest & Abhängigkeiten
├── installer.nsi                 # NSIS Windows Installer Skript
├── build.ps1                     # PowerShell-Buildskript (Cargo Release + NSIS)
├── .github/
│   └── workflows/
│       └── release.yml           # GitHub Actions CI/CD Pipeline
└── src/
    ├── main.rs                   # Einstiegspunkt, Tao Event-Loop & Wry WebView
    ├── config.rs                 # Konfigurationsmanager (JSON)
    ├── updater.rs                # GitHub Releases API Abfrage & Download
    └── ui.rs                     # Clean Dark CSS, Toolbar & Modals
```

---

## Lokale Entwicklung

### Voraussetzungen
- Rust (Cargo) 1.80+
- Windows 10/11 mit Microsoft Edge WebView2 (standardmäßig vorinstalliert)
- NSIS (`makensis.exe`) für das Erstellen des Setup-Installers

### Starten im Entwicklungsmodus
```powershell
cargo run
```

---

## Erstellen des Release-Builds & Installers

Führe einfach das mitgelieferte Build-Skript in PowerShell aus:

```powershell
.\build.ps1
```

Das Skript führt automatisch folgende Schritte aus:
1. Kompiliert das Rust-Projekt hochoptimiert (`cargo build --release`)
2. Erstellt die NSIS-Installationsdatei `dist\Docmost-Setup.exe`
3. Zeigt die Dateigrößen und Speicherorte an

---

## GitHub Auto-Update einrichten

1. Erstelle ein Repository auf GitHub (z. B. `https://github.com/LegoPuck-orginal/docmost-desktop`).
2. Passe in `src/config.rs` (oder in den App-Einstellungen) `github_repo` an.
3. Wenn du ein neues Release veröffentlichen möchtest, erstelle einfach ein Git-Tag:
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```
4. Der GitHub Actions Workflow `.github/workflows/release.yml` baut die Anwendung automatisch, erzeugt den Installer und hängt ihn als Release-Asset an.
5. Alle installierten Clients erkennen beim nächsten Start automatisch das Update und bieten die Aktualisierung an!

---

## Konfiguration

Die Konfiguration wird automatisch unter folgendem Pfad gespeichert:
`%APPDATA%\DocmostApp\config.json`

Beispiel:
```json
{
  "server_url": "https://doc.home.klehausen.de",
  "check_updates_on_startup": true,
  "github_repo": "LegoPuck-orginal/docmost-desktop",
  "clean_dark_theme": true,
  "window_width": 1280,
  "window_height": 840,
  "window_maximized": false
}
```
