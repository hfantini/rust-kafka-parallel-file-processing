@ECHO off

ECHO "[!] RUNNING APPLICATION"

ECHO "[!] 1x INSTANCE OF [CRAWLER]"
start .\bin\crawler.exe "c:/"

ECHO "[!] 5x INSTANCES OF [CLASSIFICATOR]"
start .\bin\classificator.exe

ECHO "[!] 5x INSTANCES OF [READER]"
start .\bin\reader.exe

ECHO "[!] 1x INSTANCE OF [FINDER]"
start .\bin\finder.exe "./bin/result.txt" "the"

ECHO "[!] 1x INSTANCE OF [LOGGER]"
start .\bin\logger.exe "./bin/log.txt"