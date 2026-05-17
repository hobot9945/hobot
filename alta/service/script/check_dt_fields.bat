@echo off
setlocal EnableExtensions

rem check_dt_fields.bat
rem Тонкая обертка над check_dt_fields.ps1.

set "CASE_NAME=%~1"
if "%CASE_NAME%"=="" (
  echo Usage: %~nx0 "<CASE_NAME>"
  exit /b 2
)

set "HOBOT_ROOT=C:\Users\su144\RustroverProjects\rustdev\hobot"
set "PS1=%HOBOT_ROOT%\alta\service\script\check_dt_fields.ps1"

powershell -NoProfile -ExecutionPolicy Bypass -File "%PS1%" -CaseName "%CASE_NAME%" -HobotRoot "%HOBOT_ROOT%"
exit /b %ERRORLEVEL%
