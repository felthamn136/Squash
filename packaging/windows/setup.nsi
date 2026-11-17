Unicode True

!define APP_NAME "Squash '76"
!define APP_ID "Squash76"
!define APP_EXE "squash-76.exe"
!define PUBLISHER "felthamn136"
!define VERSION "0.1.0"

Name "${APP_NAME}"
OutFile "..\..\dist\Squash-76-Setup.exe"
InstallDir "$PROGRAMFILES64\Squash '76"
InstallDirRegKey HKLM "Software\${PUBLISHER}\${APP_ID}" "InstallLocation"
RequestExecutionLevel admin
SetCompressor /SOLID lzma

Icon "..\..\packaging\windows\squash-76.ico"
UninstallIcon "..\..\packaging\windows\squash-76.ico"

Page directory
Page instfiles

Section "Application"
  SetOutPath "$INSTDIR"
  File "..\..\target\x86_64-pc-windows-gnu\release\${APP_EXE}"
  File "..\..\README.md"

  WriteUninstaller "$INSTDIR\Uninstall.exe"

  WriteRegStr HKLM "Software\${PUBLISHER}\${APP_ID}" "InstallLocation" "$INSTDIR"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" "DisplayName" "${APP_NAME}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" "DisplayVersion" "${VERSION}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" "Publisher" "${PUBLISHER}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" "InstallLocation" "$INSTDIR"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" "UninstallString" '"$INSTDIR\Uninstall.exe"'
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" "DisplayIcon" "$INSTDIR\${APP_EXE}"
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" "NoModify" 1
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" "NoRepair" 1

  CreateDirectory "$SMPROGRAMS\Squash '76"
  CreateShortCut "$SMPROGRAMS\Squash '76\Squash '76.lnk" "$INSTDIR\${APP_EXE}"
  CreateShortCut "$DESKTOP\Squash '76.lnk" "$INSTDIR\${APP_EXE}"
SectionEnd

Section "Uninstall"
  Delete "$DESKTOP\Squash '76.lnk"
  Delete "$SMPROGRAMS\Squash '76\Squash '76.lnk"
  RMDir "$SMPROGRAMS\Squash '76"

  Delete "$INSTDIR\${APP_EXE}"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\Uninstall.exe"
  RMDir "$INSTDIR"

  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}"
  DeleteRegKey HKLM "Software\${PUBLISHER}\${APP_ID}"
SectionEnd
