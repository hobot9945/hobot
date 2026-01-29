@echo off
setlocal

set "HOST_NAME=com.example.hobot"

echo Unregistering Native Messaging Host: %HOST_NAME%

:: Удаляем ключ из реестра
REG DELETE "HKCU\Software\Google\Chrome\NativeMessagingHosts\%HOST_NAME%" /f

if %ERRORLEVEL% EQU 0 (
    echo.
    echo SUCCESS. The agent is successfully removed from register.
) else (
    echo.
    echo ERROR. The key was not found in register.
)

pause