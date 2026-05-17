@echo off
setlocal EnableExtensions EnableDelayedExpansion

REM Переход в корень репозитория относительно расположения самого скрипта
cd /d "%~dp0..\..\.."

REM ==============================================================================
REM pdftojpeg.bat
REM ------------------------------------------------------------------------------
REM Назначение:
REM   Конвертировать PDF в JPEG-страницы (по одному JPEG на страницу).
REM   Используется poppler pdftoppm через обёртку:
REM     alta\service\script\pdftoppm.cmd
REM   Параметры рендера "спрятаны" внутри батника:
REM     -jpeg            : выход JPEG
REM     -gray            : градации серого (лучше для OCR, меньше размер)
REM     -jpegopt quality=90 : качество JPEG
REM
REM Использование (запускать из корня репозитория hobot):
REM   pdftojpeg.bat "path\to\file.pdf" DPI        [извлечь все страницы]
REM   pdftojpeg.bat "path\to\file.pdf" DPI N      [извлечь только страницу N]
REM
REM Выход:
REM   Файлы кладутся в: alta\work
REM   Имена: <имя_pdf_без_расширения>-N.jpg
REM
REM Важно:
REM   Перед конвертацией старые файлы с совпадающим префиксом удаляются.
REM ==============================================================================

if "%~1"=="" (
  echo Usage: %~nx0 "input.pdf" DPI [PAGE_NUM]
  exit /b 2
)
if "%~2"=="" (
  echo Usage: %~nx0 "input.pdf" DPI [PAGE_NUM]
  exit /b 2
)

REM Входной PDF и DPI
set "INPDF=%~1"
set "DPI=%~2"
set "PAGE=%~3"

REM Папка вывода (внутри проекта)
set "OUTDIR=alta\work"

REM Префикс выходных файлов = имя PDF без расширения
set "BASE=%~n1"
set "OUTPREFIX=%OUTDIR%\%BASE%"

REM Создать OUTDIR, если её нет
if not exist "%OUTDIR%" mkdir "%OUTDIR%" >nul 2>&1

REM ------------------------------------------------------------------------------
REM Перетирание и подготовка аргументов страницы:
REM ------------------------------------------------------------------------------
if defined PAGE (
    del /q "%OUTPREFIX%-%PAGE%.jpg" >nul 2>&1
    del /q "%OUTPREFIX%_%PAGE%.jpg" >nul 2>&1
    set "PAGE_ARGS=-f %PAGE% -l %PAGE%"
) else (
    del /q "%OUTPREFIX%-*.jpg" >nul 2>&1
    del /q "%OUTPREFIX%_*.jpg" >nul 2>&1
    set "PAGE_ARGS="
)

REM ------------------------------------------------------------------------------
REM Конвертация PDF -> JPEG pages
REM ------------------------------------------------------------------------------
call alta\service\script\pdftoppm.cmd -jpeg -gray -r %DPI% -jpegopt quality=90 %PAGE_ARGS% "%INPDF%" "%OUTPREFIX%"
if errorlevel 1 (
  echo ERROR: conversion failed: "%INPDF%"
  exit /b 1
)

REM ------------------------------------------------------------------------------
REM Переименование: <base>-N.jpg -> <base>_N.jpg
REM Используем powershell для всех случаев, так как он корректно обрабатывает ведущие нули и пробелы
REM ------------------------------------------------------------------------------
powershell -NoProfile -Command ^
  "$p = '%OUTPREFIX%-*.jpg';" ^
  "Get-ChildItem -LiteralPath (Split-Path $p) -Filter (Split-Path $p -Leaf) | ForEach-Object {" ^
  "  $new = $_.Name -replace '-(\d+)\.jpg$','_$1.jpg';" ^
  "  Rename-Item -LiteralPath $_.FullName -NewName $new -Force" ^
  }"

echo DPI: %DPI%
echo JPEG quality: 90
echo Mode: gray

echo --- Created files: ---
if defined PAGE (
  dir /b "%OUTPREFIX%_%PAGE%.jpg"
) else (
  dir /b "%OUTPREFIX%_*.jpg"
)

exit /b 0