@echo off
:: =========================================================================
:: Валидатор аудита массивов (gen_result_array_audit.ps1)
::
:: Использование: gen_result_array_audit.bat <ПутьКФайлу>
:: =========================================================================

if "%~1"=="" (
    echo ERROR: No file path specified.
    echo Usage: %0 ^<path_to_file.md^>
    exit /b 1
)

powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0gen_result_array_audit.ps1" -FilePath "%~1"
exit /b %ERRORLEVEL%
