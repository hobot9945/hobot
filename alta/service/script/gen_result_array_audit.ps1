<#
.SYNOPSIS
    Скрипт проверки аудита массивов (_array_audit, _element_num) в файлах primary.md и dt_fields.md.

.DESCRIPTION
    Скрипт валидирует строгое соответствие заявленной размерности массивов и фактического количества
    их элементов, опираясь исключительно на служебные маркеры.
    Поддерживается максимум двухуровневая вложенность массивов:
    1. Внешний массив: маркеры с квалификатором (например, goods._array_audit, goods._element_num).
    2. Внутренний (плоский) массив: маркеры без квалификатора (_array_audit, _element_num).
    Скрипт проверяет:
    - Непрерывность и возрастание на 1 индексов _element_num.
    - Соответствие количества элементов значению _array_audit.
    - Корректное закрытие внутренних массивов при переходе к следующему элементу внешнего.

.PARAMETER FilePath
    Путь к проверяемому .md файлу.
#>

param(
    [Parameter(Mandatory=$true)]
    [string]$FilePath
)

$ErrorActionPreference = 'Stop'

if (-not (Test-Path -LiteralPath $FilePath)) {
    Write-Output "ОШИБКА: Файл не найден: $FilePath"
    exit 1
}

$Errors = New-Object System.Collections.Generic.List[string]
function Add-Error([int]$ln, [string]$msg) { $Errors.Add("Строка ${ln}: ${msg}") | Out-Null }

# --- Состояния внешнего (квалифицированного) массива ---
$inOuter = $false
$outerName = ""
$outerExpected = 0
$outerActual = 0
$outerNextNum = 1
$outerStartLn = 0

# --- Состояния внутреннего (неквалифицированного/плоского) массива ---
$inInner = $false
$innerExpected = 0
$innerActual = 0
$innerNextNum = 1
$innerStartLn = 0

# Функция проверки и закрытия внутреннего массива
function Assert-InnerClosed() {
    if ($inInner) {
        if ($innerActual -ne $innerExpected) {
            Add-Error $innerStartLn ("Несоответствие маркеров массива. Ожидалось элементов: $innerExpected, фактически обработано _element_num: $innerActual.")
        }
        $inInner = $false
    }
}

# Функция проверки и закрытия внешнего массива
function Assert-OuterClosed() {
    if ($inOuter) {
        if ($outerActual -ne $outerExpected) {
            Add-Error $outerStartLn ("Несоответствие маркеров внешнего массива '$outerName'. Ожидалось элементов: $outerExpected, фактически обработано ${outerName}._element_num: $outerActual.")
        }
        $inOuter = $false
    }
}

$lines = Get-Content -LiteralPath $FilePath -Encoding UTF8

for ($i = 0; $i -lt $lines.Count; $i++) {
    $lnNo = $i + 1
    $line = $lines[$i]
    $trimmed = $line.Trim()

    # --- 1. Обработка маркеров ВНЕШНЕГО массива (с квалификатором, например goods._array_audit) ---

    # Маркер размерности внешнего массива: <имя>._array_audit: N
    if ($trimmed -match '^\s*-\s*([a-zA-Z0-9_]+)\._array_audit\s*:\s*(\d+)\s*$') {
        $name = $matches[1]
        $expected = [int]$matches[2]

        # Новый внешний массив. Предыдущие массивы должны быть закрыты.
        Assert-InnerClosed
        Assert-OuterClosed

        $inOuter = $true
        $outerName = $name
        $outerExpected = $expected
        $outerActual = 0
        $outerNextNum = 1
        $outerStartLn = $lnNo
    }
    # Маркер элемента внешнего массива: <имя>._element_num: i
    elseif ($trimmed -match '^\s*-\s*([a-zA-Z0-9_]+)\._element_num\s*:\s*(\d+)\s*$') {
        $name = $matches[1]
        $num = [int]$matches[2]

        if (-not $inOuter -or $name -ne $outerName) {
            Add-Error $lnNo "Маркер ${name}._element_num найден вне контекста открытого внешнего массива или не соответствует текущему массиву."
        } else {
            # Появление нового элемента внешнего массива автоматически закрывает внутренние массивы предыдущего элемента
            Assert-InnerClosed

            if ($num -ne $outerNextNum) {
                Add-Error $lnNo ("Нарушение порядка элементов внешнего массива '$name'. Ожидался: $outerNextNum, найден: $num.")
            }

            $outerActual++
            $outerNextNum = $num + 1
        }
    }

    # --- 2. Обработка маркеров ВНУТРЕННЕГО массива (без квалификатора) ---

    # Маркер размерности внутреннего массива: _array_audit: N
    elseif ($trimmed -match '^\s*-\s*_array_audit\s*:\s*(\d+)\s*$') {
        $expected = [int]$matches[1]

        # Если внутри элемента уже был открыт внутренний массив, проверяем его перед открытием нового
        Assert-InnerClosed

        $inInner = $true
        $innerExpected = $expected
        $innerActual = 0
        $innerNextNum = 1
        $innerStartLn = $lnNo
    }
    # Маркер элемента внутреннего массива: _element_num: j
    elseif ($trimmed -match '^\s*-\s*_element_num\s*:\s*(\d+)\s*$') {
        $num = [int]$matches[1]

        if (-not $inInner) {
            Add-Error $lnNo "Маркер _element_num (без квалификатора) найден вне контекста открытого массива."
        } else {
            if ($num -ne $innerNextNum) {
                Add-Error $lnNo ("Нарушение порядка элементов массива. Ожидался: $innerNextNum, найден: $num.")
            }

            $innerActual++
            $innerNextNum = $num + 1
        }
    }
}

# -------------------------------------------------------------------------
# ФИНАЛЬНЫЕ ПРОВЕРКИ
# -------------------------------------------------------------------------

# Проверяем незакрытые массивы в конце файла
Assert-InnerClosed
Assert-OuterClosed

# -------------------------------------------------------------------------
# ВЫВОД ОТЧЕТА И ЗАВЕРШЕНИЕ РАБОТЫ
# -------------------------------------------------------------------------

Write-Output "Файл: $FilePath"

if ($Errors.Count -gt 0) {
    Write-Output "НАЙДЕНЫ ОШИБКИ ($($Errors.Count)):"
    $Errors | ForEach-Object { Write-Output "  $_" }
    exit 1
} else {
    Write-Output "OK"
    exit 0
}