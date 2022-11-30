@ECHO off

ECHO "[!] RUNNING APPLICATION"

ECHO "[!] 1x INSTANCE OF [CRAWLER]"
start .\bin\crawler.exe

ECHO "[!] 5x INSTANCES OF [CLASSIFICATOR]"
start .\bin\classificator.exe
start .\bin\classificator.exe
start .\bin\classificator.exe
start .\bin\classificator.exe
start .\bin\classificator.exe

ECHO "[!] 5x INSTANCES OF [READER]"
start .\bin\reader.exe
start .\bin\reader.exe
start .\bin\reader.exe
start .\bin\reader.exe
start .\bin\reader.exe

ECHO "[!] 5x INSTANCES OF [FINDER]"
start .\bin\finder.exe
start .\bin\finder.exe
start .\bin\finder.exe
start .\bin\finder.exe
start .\bin\finder.exe

ECHO "[!] 1x INSTANCES OF [LOGGER]"
start .\bin\logger.exe