@ECHO OFF

ECHO "[!] Compiling Crawler"
CALL cargo build --release --manifest-path ./src/crawler/Cargo.toml

ECHO "[!] Compiling Classificator"
CALL cargo build --release --manifest-path ./src/classificator/Cargo.toml

ECHO "[!] Compiling Reader"
CALL cargo build --release --manifest-path ./src/reader/Cargo.toml

ECHO "[!] Compiling Finder"
CALL cargo build --release --manifest-path ./src/finder/Cargo.toml

ECHO "[!] Preparing BIN folder"
mkdir "bin" 2>nul

ECHO "[!] Moving binaries"
xcopy ".\src\crawler\target\release\crawler.exe" ".\bin\crawler.exe*" /Y >nul
xcopy ".\src\classificator\target\release\classificator.exe" ".\bin\classificator.exe*" /Y >nul
xcopy ".\src\reader\target\release\reader.exe" ".\bin\reader.exe*" /Y >nul
xcopy ".\src\finder\target\release\finder.exe" ".\bin\finder.exe*" /Y >nul

ECHO "Application compiled successfully!"


