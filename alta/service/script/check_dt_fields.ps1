# check_dt_fields.ps1
#
# Назначение
# ----------
# Формальная (механическая) проверка dt_fields.md:
# - корректность строк таблиц (разделители '|', количество колонок)
# - корректность нумерации num (01..N без пропусков) внутри каждого блока
# - сверка фактического числа полей с маркерами _audit / _item_audit
#
# Важно: скрипт НЕ знает доменную структуру (имена разделов, документов).
# Он работает только по формальным признакам разметки.
#
# Параметры
# ---------
# -CaseName  : имя кейса (папка stage_2.0_result/<case>)
# -HobotRoot : корень проекта hobot

# PSScriptAnalyzer suppression (file-level style suppression)
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUseApprovedVerbs', '', Justification='Local helper functions')]
param(
  [Parameter(Mandatory=$true)][string]$CaseName,
  [Parameter(Mandatory=$true)][string]$HobotRoot
)

# Системная пременная - останавливать скрипт при возникновении ошибок.
$ErrorActionPreference = 'Stop'

# Проверить существование dt_fields.md
$inPath = Join-Path (Join-Path (Join-Path $HobotRoot 'alta\stage_2.0_result') $CaseName) 'dt_fields.md'
if (-not (Test-Path -LiteralPath $inPath)) {
  throw "Input not found: $inPath"
}

# ----------------------------
# Helpers / data structures
# ----------------------------

# Issue collector.
$Errors = New-Object System.Collections.Generic.List[string]
$Warnings = New-Object System.Collections.Generic.List[string]

function Add-Error([string]$msg)   { $Errors.Add($msg)   | Out-Null }
function Add-Warn([string]$msg)    { $Warnings.Add($msg) | Out-Null }

# Parse a markdown table row.
#
# Contract:
# - Row must start with '|'.
# - Splits by '|'.
# - Returns array of cells without the leading/trailing pipes.
#
# Note:
# - This parser assumes that cell values do NOT contain '|'. If they do,
#   the table becomes ambiguous and our downstream generators (like gen_dt_xml)
#   would also break.
function Split-Row([string]$row) {
  $r = $row.Trim()
  if (-not $r.StartsWith('|')) { return @() }
  $parts = $r.Split('|')
  $cells = @()
  for ($i=1; $i -lt $parts.Length-1; $i++) { $cells += $parts[$i].Trim() }
  return $cells
}

# Check if a line looks like a "data row" of dt_fields table:
# starts with | NN |
function Is-DataRow([string]$line) {
  return ($line -match '^\|\s*\d{2}\s*\|')
}

# Extract num (as int) from a data row.
function Get-RowNum([string]$line) {
  $m = [regex]::Match($line, '^\|\s*(\d{2})\s*\|')
  if (-not $m.Success) { return $null }
  return [int]$m.Groups[1].Value
}

# Parse _audit / _item_audit line; returns int or $null.
function Parse-Audit([string]$line) {
  $m = [regex]::Match($line, '^\s*_(item_)?audit\s*:\s*(\d+)\s*$')
  if (-not $m.Success) { return $null }
  return [int]$m.Groups[2].Value
}

# ----------------------------
# Main pass
# ----------------------------

# Разложить исходный файн в массив строк
$lines = Get-Content -LiteralPath $inPath -Encoding UTF8

# We validate "blocks" of consecutive data rows.
# A block ends when we see _audit: N or _item_audit: N.
#
# State variables for the current block:
$inBlock = $false
$blockStartLine = 0
$blockRowCount = 0
$expectedNextNum = 1
$lastNum = 0
$blockHadBadShape = $false

function Reset-BlockState([int]$nextLineIdx) {
  $script:inBlock = $false
  $script:blockStartLine = $nextLineIdx
  $script:blockRowCount = 0
  $script:expectedNextNum = 1
  $script:lastNum = 0
  $script:blockHadBadShape = $false
}

# Подготовить управляющие переменные.
Reset-BlockState 1

for ($i=0; $i -lt $lines.Count; $i++) {
  $lnNo = $i + 1
  $ln = $lines[$i]

  # 1) Data rows checks
  if (Is-DataRow $ln) {
    # Это строка таблицы
    if (-not $inBlock) {
      # Start of a new block: first num must be 01.
      $inBlock = $true
      $blockStartLine = $lnNo
      $expectedNextNum = 1
      $blockRowCount = 0
      $lastNum = 0
      $blockHadBadShape = $false
    }

    $cells = Split-Row $ln

    # Expected schema table columns: num, field, value, status, description, note
    # => at least 6 cells.
    if ($cells.Count -lt 6) {
      Add-Error ("Line ${lnNo}: data row has too few columns (" + $cells.Count + "). Expected >= 6. Row=`"$ln`"")
      $blockHadBadShape = $true
    }

    # num sequence
    $n = Get-RowNum $ln
    if ($null -eq $n) {
      Add-Error ("Line ${lnNo}: cannot parse num. Row=`"$ln`"")
      $blockHadBadShape = $true
    } else {
      if ($n -ne $expectedNextNum) {
        Add-Error ("Line ${lnNo}: num sequence broken. Expected {0:00}, got {1:00}." -f $expectedNextNum, $n)
      }
      $lastNum = $n
      $expectedNextNum = $n + 1
    }

    $blockRowCount++
    continue
  }

  # 2) Audit line closes current block (if any)
  $aud = Parse-Audit $ln
  if ($null -ne $aud) {
    if (-not $inBlock) {
      Add-Error ("Line ${lnNo}: found audit marker (_audit/_item_audit=$aud) but no preceding data block.")
      continue
    }

    # Compare actual rows count with audit
    if ($blockRowCount -ne $aud) {
      Add-Error ("Line ${lnNo}: audit mismatch. Expected rows=$aud, actual rows=$blockRowCount. Block started at line $blockStartLine.")
    }

    # Also check last num matches audit
    if ($lastNum -ne $aud) {
      Add-Error ("Line ${lnNo}: last num mismatch. Expected last num=$aud, got $lastNum. Block started at line $blockStartLine.")
    }

    # After audit, next block must restart numbering from 01.
    Reset-BlockState ($lnNo + 1)
    continue
  }

  # 3) Any other line: if we are inBlock, we simply allow gaps (tables are separated by headers/empty lines).
  #    We do NOT automatically close block until audit marker, because dt_fields.md may contain blank lines
  #    and intermediate headers. Audit markers are the only formal end-of-block signal.
}

# If file ended while we were inside a block => missing _audit/_item_audit.
if ($inBlock) {
  Add-Error ("EOF: reached end of file while inside a data block (started at line $blockStartLine). Missing _audit/_item_audit marker.")
}

# ----------------------------
# Report + exit code
Write-Output ('check_dt_fields: ' + $inPath)
Write-Output ('errors=' + $Errors.Count + ' warnings=' + $Warnings.Count)

if ($Warnings.Count -gt 0) {
  Write-Output '--- WARNINGS ---'
  $Warnings | ForEach-Object { Write-Output $_ }
}

if ($Errors.Count -gt 0) {
  Write-Output '--- ERRORS ---'
  $Errors | ForEach-Object { Write-Output $_ }
  exit 1
}

Write-Output 'OK'
exit 0
