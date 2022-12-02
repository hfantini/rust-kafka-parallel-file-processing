@ECHO off

ECHO "[!] RUNNING APPLICATION"

ECHO "[!] 1x INSTANCE OF [CRAWLER]"
start .\bin\crawler.exe "CRAWLER_1" "C:/"

ECHO "[!] 5x INSTANCES OF [CLASSIFICATOR]"
start .\bin\classificator.exe "CLASSIFICATOR_1"

ECHO "[!] 5x INSTANCES OF [READER]"
start .\bin\reader.exe "READER_1"

ECHO "[!] 5x INSTANCE OF [FINDER]"
start .\bin\finder.exe "FINDER_1" "./bin/result.txt" "unix;linux;system;half;life;admin;password"

ECHO "[!] 1x INSTANCE OF [LOGGER]"
start .\bin\logger.exe "./bin/log.txt"