@echo off
setlocal

:: 1. Включаем полный вывод стека при панике Rust
set RUST_BACKTRACE=1

:: 2. Формирование временной метки через WMIC (не зависит от локали Windows)
for /f "tokens=2 delims==" %%I in ('wmic os get localdatetime /value') do set "dt=%%I"
set "YYYY=%dt:~0,4%"
set "MM=%dt:~4,2%"
set "DD=%dt:~6,2%"
set "HH=%dt:~8,2%"
set "Min=%dt:~10,2%"
set "Sec=%dt:~12,2%"

set "TIMEMARK=%YYYY%%MM%%DD%_%HH%%Min%%Sec%"

:: 3. Настройка путей
:: %~dp0 — путь к каталогу, где лежит этот bat-файл (с завершающим слэшем)
set "BASE_DIR=%~dp0"
set "LOG_DIR=%BASE_DIR%err_log"
set "LOG_FILE=%LOG_DIR%\error_%TIMEMARK%.log"

:: 4. Создание каталога для логов, если он не существует
if not exist "%LOG_DIR%" (
    mkdir "%LOG_DIR%"
)

:: 5. Запуск агента с перенаправлением stderr
:: stdout (поток 1) уходит в браузер.
:: stderr (поток 2) уходит в наш файл.
"%BASE_DIR%hobot.exe" 2> "%LOG_FILE%"