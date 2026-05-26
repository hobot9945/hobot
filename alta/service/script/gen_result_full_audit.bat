@echo off
:: =========================================================================
:: Полный аудит результата генерации стадии (gen_result_full_audit.bat)
::
:: Назначение: Последовательный запуск двух проверок:
::             1. gen_result_fields_audit.ps1 - проверка структуры таблиц и нумерации.
::             2. gen_result_array_audit.ps1  - проверка соответствия _array_audit.
::
:: Использование: gen_result_full_audit.bat <ПутьКФайлу>
:: Пример:        gen_result_full_audit.bat "alta\stage_2.0_result\МоскитнаяСетка\dt_fields.md"
:: =========================================================================

if "%~1"=="" (
    echo ERROR: No file path specified.
    echo Usage: %0 ^<path_to_file.md^>
    exit /b 1
)

echo === Step 1/2: Fields structure audit (gen_result_fields_audit.ps1) ===
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0gen_result_fields_audit.ps1" -FilePath "%~1"
if %ERRORLEVEL% neq 0 (
    echo.
    echo === Step 2/2 SKIPPED: file structure is broken ===
    exit /b 1
)

echo.
echo === Step 2/2: Arrays audit (gen_result_array_audit.ps1) ===
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0gen_result_array_audit.ps1" -FilePath "%~1"
exit /b %ERRORLEVEL%
