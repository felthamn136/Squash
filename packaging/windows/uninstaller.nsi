Unicode True
!include "LogicLib.nsh"

!define APP_NAME "Squash '76"
!define APP_ID "Squash76"
!define APP_EXE "squash-76.exe"
!define PUBLISHER "felthamn136"

Name "${APP_NAME} Uninstaller"
OutFile "..\..\dist\Squash-76-Uninstaller.exe"
RequestExecutionLevel admin
SetCompressor /SOLID lzma
Icon "..\..\packaging\windows\squash-76.ico"

Function .onInit
  ReadRegStr $INSTDIR HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" "InstallLocation"
  ${If} $INSTDIR == ""
    ReadRegStr $INSTDIR HKLM "Software\${PUBLISHER}\${APP_ID}" "InstallLocation"
  ${EndIf}
  ${If} $INSTDIR == ""
    StrCpy $INSTDIR "$PROGRAMFILES64\Squash '76"
  ${EndIf}
FunctionEnd

Section "Main"
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
