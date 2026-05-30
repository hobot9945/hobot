@echo off
rem ====================================================================
rem check_xml.bat
rem One-file XML validation utility.
rem Validates all .xml files in the specified directory (well-formed check).
rem Returns exit code 1 if any errors are found, 0 otherwise.
rem Usage: check_xml.bat <path_to_directory>
rem ====================================================================

if "%~1"=="" (
    echo Usage: %~nx0 ^<path_to_directory^>
    exit /b 1
)
powershell -NoProfile -Command "$files = Get-ChildItem '%~1' -Filter *.xml; if ($files.Count -eq 0) { Write-Host '[WARNING] No XML files found in the directory.' -ForegroundColor Yellow; exit 0 }; $err = $false; foreach ($f in $files) { $x = New-Object System.Xml.XmlDocument; try { $x.Load($f.FullName); Write-Host ('  [OK] ' + $f.Name) -ForegroundColor Green } catch { Write-Host ('  [ERROR] ' + $f.Name) -ForegroundColor Red; Write-Host ('    Details: ' + $_.Exception.Message) -ForegroundColor DarkRed; $err = $true } }; if ($err) { exit 1 } else { exit 0 }"
exit /b %errorlevel%
