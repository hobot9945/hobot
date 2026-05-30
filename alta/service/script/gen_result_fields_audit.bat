@echo off
rem =========================================================================
rem Report structure validator (gen_result_fields_audit.ps1)
rem
rem Purpose: Validates primary.md or dt_fields.md for compliance
rem          with Markdown table rules and numbering.
rem
rem Usage: gen_result_fields_audit.bat <path_to_file.md>
rem Example: gen_result_fields_audit.bat "alta\stage_2.0_result\МоскитнаяСетка\dt_fields.md"
rem =========================================================================

rem 1. Check input arguments. %~1 is the path to the checked .md file.
if "%~1"=="" (
    echo ERROR: No file path specified.
    echo Usage: %~nx0 ^<path_to_file.md^>
    exit /b 1
)

rem 2. Run PowerShell script.
rem -NoProfile:        ignore user profile for clean execution.
rem -ExecutionPolicy:  Bypass execution policy restrictions in Windows.
rem %~dp0:             directory where this batch file is located.
rem "%~1":             passes the file path, stripped of quotes and re-quoted.
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0gen_result_fields_audit.ps1" -FilePath "%~1"

rem 3. Pass the exit code (0 - OK, 1 - structure errors found).
exit /b %ERRORLEVEL%