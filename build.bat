@ECHO off

ECHO "[!] COMPILING WORKSPACE"
CALL cargo build --release

ECHO "[!] MOVING BINARIES"
xcopy ".\target\release\crawler.exe" ".\bin\crawler.exe*" /Y >nul
xcopy ".\target\release\classificator.exe" ".\bin\classificator.exe*" /Y >nul
xcopy ".\target\release\reader.exe" ".\bin\reader.exe*" /Y >nul
xcopy ".\target\release\finder.exe" ".\bin\finder.exe*" /Y >nul
xcopy ".\target\release\logger.exe" ".\bin\logger.exe*" /Y >nul

ECHO "APPLICATION BUILD SUCCESSFULLY!"


