@echo off
if "%~1"=="" echo Usage: %0 ^<path_to_md_file^> && exit /b 1
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0check_stage_result.ps1" -FilePath "%~1"
