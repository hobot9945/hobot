@echo off
setlocal

:: Имя хоста должно совпадать с полем "name" в manifest.json
set "HOST_NAME=com.example.hobot"

:: Получаем полный путь к manifest.json в текущей папке
set "MANIFEST_PATH=%~dp0manifest.json"

:: Проверка наличия файла
if not exist "%MANIFEST_PATH%" (
    echo File manifest.json is not found in the current directory!
    echo Make sure this bat-file is next to manifest.json
    pause
    exit /b 1
)

echo Registering Native Messaging Host: %HOST_NAME%
echo Path: %MANIFEST_PATH%

:: Добавляем запись в реестр (HKCU - только для текущего пользователя, права админа не требуются)
REG ADD "HKCU\Software\Google\Chrome\NativeMessagingHosts\%HOST_NAME%" /ve /t REG_SZ /d "%MANIFEST_PATH%" /f

if %ERRORLEVEL% EQU 0 (
    echo.
    echo Successfully installed
) else (
    echo.
    echo ERROR, could not write to the register.
)

pause