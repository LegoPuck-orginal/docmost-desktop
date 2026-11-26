# Docmost iOS (Flutter)

Native Flutter iOS & iPadOS Client für **Docmost** im Clean Mantine Dark Design (`#141414`, `#1f1f1f`, `#228be6`).

## Enthaltene Features

- **Mantine Dark UI & 3-Striche-Menü oben rechts:** Schwebender Menü-Button mit Zurück/Vorwärts/Neu laden/Startseite, schneller Server-Auswahl, Safari-Öffnung und Einstellungs-Modal.
- **Multi-Server-Verwaltung:** Mehrere Docmost-Instanzen speichern, wechseln und entfernen (Standard: `https://doc.home.klehausen.de`).
- **Apple Pencil & Excalidraw-Modus:** Dunkles Excalidraw-Theme, Palm Rejection (`pointerType === 'pen'`), Pen-Only-Modus sowie schwebende Schnellzugriffsleiste für Stift, Radierer, Farben, Strichstärken und Undo/Redo.
- **Integrierter Medienplayer & Office-/PDF-Viewer:**
  - Spielt Videos (`MP4`, `MOV`, `M4V`, `WebM`) und Audio (`MP3`, `M4A`, `WAV`, `AAC`) direkt im Cookie-teilenden Inline-WKWebView-Player ab.
  - Zeigt PDFs & Office-Dokumente (`DOCX`, `PPTX`, `XLSX`) nativ über iOS QuickLook/PDFKit oder optional über deinen **OnlyOffice Document Server** an.

## Lokal auf einem Mac bauen

```bash
cd mobile
flutter create --org de.klehausen --project-name docmost_ios --platforms=ios .
cp ios_overrides/Info.plist ios/Runner/Info.plist
flutter pub get
dart run flutter_launcher_icons
flutter run
```

## Automatischer Build über GitHub Actions

Der Workflow [`.github/workflows/ios.yml`](../.github/workflows/ios.yml) baut automatisch auf `macos-latest`:
1. Generiert das iOS-Xcode-Projekt und das Alpha-freie App-Icon.
2. Führt `flutter analyze` und `flutter test` aus.
3. Kompiliert `flutter build ios --release --no-codesign` und lädt `Docmost-iOS-unsigned.ipa` als Workflow-Artifact (z. B. für Sideloading per AltStore / SideStore / Sideloadly oder Xcode-Signierung) hoch.
