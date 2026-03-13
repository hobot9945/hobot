@echo off
setlocal EnableExtensions

:: 1) Полный вывод стека при панике Rust
set "RUST_BACKTRACE=1"

:: 2) Таймстамп через WMIC: YYYY-MM-DD_HH.MM.SS
for /f "tokens=2 delims==" %%I in ('wmic os get localdatetime /value') do set "dt=%%I"
set "TIMEMARK=%dt:~0,4%-%dt:~4,2%-%dt:~6,2%_%dt:~8,2%.%dt:~10,2%.%dt:~12,2%"

:: 3) Пути
:: %~dp0 — каталог, где лежит этот bat-файл (и hobot.exe), хвостовой слэш убирается, иначе сломается формат вызова hobot.exe
set "BASE_DIR=%~dp0"
set "BASE_DIR_WITHOUT_TAIL_SLASH=%BASE_DIR:~0,-1%"
set "LOG_ROOT=%BASE_DIR%log"
set "RUN_DIR=%LOG_ROOT%\%TIMEMARK%"
set "STDERR_FILE=%RUN_DIR%\stderr.log"

:: 4) Создание каталогов (bat отвечает за структуру и stderr.log)
if not exist "%LOG_ROOT%" mkdir "%LOG_ROOT%"
if not exist "%RUN_DIR%"  mkdir "%RUN_DIR%"

:: 5) Запуск
:: stdout (поток 1) уходит в браузер.
:: stderr (поток 2) уходит в log\run_<TS>\stderr.log
:: Первый аргумент hobot.exe = TIMEMARK (в Rust обязателен, иначе panic).
"%BASE_DIR%target\debug\hobot.exe" "%BASE_DIR_WITHOUT_TAIL_SLASH%" "%TIMEMARK%" 2> "%STDERR_FILE%"