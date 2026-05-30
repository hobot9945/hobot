# ====================================================================
# check_pendings.ps1
# Скрипт для поиска строк со статусом 'pending' в markdown-таблицах.
# Возвращает код 1, если найдены такие поля, и 0, если все чисто.
# ====================================================================

param (
    # Обязательный параметр - путь к файлу (primary.md или dt_fields.md)
    [Parameter(Mandatory=$true)]
    [string]$FilePath
)

# Проверяем физическое существование файла по указанному пути
if (-not (Test-Path -Path $FilePath)) {
    Write-Error "Файл не найден по пути: $FilePath"
    exit 1
}

Write-Host "--- Запуск проверки статусов pending в файле: $FilePath ---" -ForegroundColor Cyan

# Регулярное выражение ищет слово 'pending', зажатое между символами '|'.
# Это гарантирует, что мы ищем статус именно в колонках markdown-таблиц,
# игнорируя упоминания слова 'pending' в комментариях или описаниях.
$pattern = "\|\s*pending\s*\|"

# Читаем файл и фильтруем строки по паттерну
$results = Get-Content -Path $FilePath -Encoding UTF8 | Select-String -Pattern $pattern

if ($results) {
    Write-Host "[WARNING] ОБНАРУЖЕНЫ НЕЗАПОЛНЕННЫЕ ПОЛЯ (PENDING):" -ForegroundColor Yellow
    foreach ($match in $results) {
        # Выводим номер строки и очищенный от лишних пробелов текст строки
        Write-Host "  Строка $($match.LineNumber): $($match.Line.Trim())"
    }
    Write-Host "------------------------------------------------------------"
    Write-Host "Всего найдено незаполненных полей: $($results.Count)" -ForegroundColor Yellow
    # Возвращаем код 1 (ошибка/требуется внимание)
    exit 1
} else {
    Write-Host "[OK] Проверка пройдена. Полей со статусом 'pending' не обнаружено." -ForegroundColor Green
    # Возвращаем код 0 (успех)
    exit 0
}
