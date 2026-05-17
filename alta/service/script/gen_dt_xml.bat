@echo off
setlocal EnableExtensions

rem gen_dt_xml.bat
rem Тонкая обертка над gen_dt_xml.ps1.

set "CASE_NAME=%~1"
if "%CASE_NAME%"=="" (
  echo Usage: %~nx0 "<CaseName>"
  exit /b 2
)

set "HOBOT_ROOT=C:\Users\su144\RustroverProjects\rustdev\hobot"
set "PS1=%HOBOT_ROOT%\alta\service\script\gen_dt_xml.ps1"

powershell -NoProfile -ExecutionPolicy Bypass -File "%PS1%" -CaseName "%CASE_NAME%" -HobotRoot "%HOBOT_ROOT%"
exit /b %ERRORLEVEL%
