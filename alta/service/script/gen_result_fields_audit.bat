@echo off
:: =========================================================================
:: Валидатор структуры отчетов (gen_result_fields_audit.ps1)
::
:: Назначение: Проверка primary.md или dt_fields.md на соответствие
::             правилам Markdown-таблиц и нумерации.
::
:: Использование: gen_result_fields_audit.bat <ПутьКФайлу>
:: Пример:        gen_result_fields_audit.bat "alta\stage_2.0_result\МоскитнаяСетка\dt_fields.md"
:: =========================================================================

:: 1. Проверка входящих аргументов. %~1 - путь к проверяемому файлу .md
if "%~1"=="" (
    echo echo ERROR: No file path specified.
    echo echo Usage: %0 ^<path_to_file.md^>
    exit /b 1
)

:: 2. Запуск PowerShell скрипта.
:: -NoProfile:        игнорировать профиль пользователя для чистоты запуска.
:: -ExecutionPolicy:  Bypass обходит ограничения на запуск скриптов в Windows.
:: %~dp0:             папка, в которой лежит этот bat-файл.
:: "%~1":             передает путь к файлу, очищенный от кавычек и заново обернутый (на случай пробелов).
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0gen_result_fields_audit.ps1" -FilePath "%~1"

:: 3. Передача кода завершения (0 - OK, 1 - найдены ошибки структуры).
exit /b %ERRORLEVEL%