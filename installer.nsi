Unicode True
!define PRODUCT_NAME "Docmost"
!define PRODUCT_VERSION "0.1.6"
!define PRODUCT_PUBLISHER "Docmost Desktop"
!define PRODUCT_WEB_SITE "https://github.com/LegoPuck-orginal/docmost-desktop"
!define PRODUCT_DIR_REGKEY "Software\Microsoft\Windows\CurrentVersion\App Paths\docmost-app.exe"
!define PRODUCT_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"
!define PRODUCT_UNINST_ROOT_KEY "HKCU"

SetCompressor /SOLID lzma
RequestExecutionLevel user

!include "MUI2.nsh"

!define MUI_ABORTWARNING
!define MUI_ICON "assets\icon.ico"
!define MUI_UNICON "assets\icon.ico"

; Welcome page
!insertmacro MUI_PAGE_WELCOME
; Directory page
!insertmacro MUI_PAGE_DIRECTORY
; Instfiles page
!insertmacro MUI_PAGE_INSTFILES
; Finish page
!define MUI_FINISHPAGE_RUN "$INSTDIR\docmost-app.exe"
!define MUI_FINISHPAGE_RUN_TEXT "Docmost jetzt starten"
!insertmacro MUI_PAGE_FINISH

; Uninstaller pages
!insertmacro MUI_UNPAGE_INSTFILES

; Languages
!insertmacro MUI_LANGUAGE "German"

Name "${PRODUCT_NAME} ${PRODUCT_VERSION}"
OutFile "dist\Docmost-Installer.exe"
InstallDir "$LOCALAPPDATA\Programs\Docmost"
InstallDirRegKey HKCU "${PRODUCT_DIR_REGKEY}" ""
ShowInstDetails show
ShowUnInstDetails show

Section "MainSection" SEC01
  SetOutPath "$INSTDIR"
  SetOverwrite try

  ; Programmdateien
  File "target\release\docmost-app.exe"
  CreateDirectory "$INSTDIR\assets"
  File "/oname=assets\icon.ico" "assets\icon.ico"

  ; Desktop- und Startmenü-Verknüpfungen
  CreateDirectory "$SMPROGRAMS\Docmost"
  CreateShortcut "$SMPROGRAMS\Docmost\Docmost.lnk" "$INSTDIR\docmost-app.exe" "" "$INSTDIR\assets\icon.ico" 0
  CreateShortcut "$SMPROGRAMS\Docmost\Docmost Deinstallieren.lnk" "$INSTDIR\uninstall.exe" "" "$INSTDIR\assets\icon.ico" 0
  CreateShortcut "$DESKTOP\Docmost.lnk" "$INSTDIR\docmost-app.exe" "" "$INSTDIR\assets\icon.ico" 0

  ; Uninstaller schreiben
  WriteUninstaller "$INSTDIR\uninstall.exe"

  ; Registry-Einträge für Windows-Systemsteuerung (Apps & Features)
  WriteRegStr HKCU "${PRODUCT_DIR_REGKEY}" "" "$INSTDIR\docmost-app.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayName" "$(^Name)"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "UninstallString" "$INSTDIR\uninstall.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayIcon" "$INSTDIR\assets\icon.ico"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayVersion" "${PRODUCT_VERSION}"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "URLInfoAbout" "${PRODUCT_WEB_SITE}"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "Publisher" "${PRODUCT_PUBLISHER}"
SectionEnd

Section Uninstall
  Delete "$DESKTOP\Docmost.lnk"
  Delete "$SMPROGRAMS\Docmost\Docmost.lnk"
  Delete "$SMPROGRAMS\Docmost\Docmost Deinstallieren.lnk"
  RMDir "$SMPROGRAMS\Docmost"

  Delete "$INSTDIR\assets\icon.ico"
  RMDir "$INSTDIR\assets"

  Delete "$INSTDIR\docmost-app.exe"
  Delete "$INSTDIR\uninstall.exe"
  RMDir "$INSTDIR"

  DeleteRegKey HKCU "${PRODUCT_DIR_REGKEY}"
  DeleteRegKey ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}"
  SetAutoClose true
SectionEnd
