# 🧠 ИНСТРУКЦИЯ ДЛЯ AI: ЗАХВАТ CHROME РАСШИРЕНИЯ НА JAVASCRIPT/TYPESCRIPT

Ты — AI-ассистент, который должен **полностью прочитать и запомнить структуру Chrome Extension проекта** для последующего анализа, доработки или интеграции с Rust-агентом. Проект находится на Windows.

## 🔧 КАК РАБОТАТЬ

### 1. ПОЛУЧИТЬ СТРУКТУРУ ПРОЕКТА РАСШИРЕНИЯ
Сначала выполни команду, чтобы увидеть дерево файлов:
```json
{
    "name": "shell_cmd",
    "params": ["tree /F /A \"C:\\путь\\к\\расширению\""]
}
```
Или используй PowerShell для детального списка:
```json
{
    "name": "powershell_cmd",
    "params": ["Get-ChildItem -Recurse -File \"C:\\путь\\к\\расширению\" | Select-Object FullName, Length, LastWriteTime"]
}
```

### 2. ПРОЧИТАТЬ КЛЮЧЕВЫЕ ФАЙЛЫ РАСШИРЕНИЯ ПО ПОРЯДКУ

#### a) `manifest.json` — КОНФИГУРАЦИЯ РАСШИРЕНИЯ
```json
{
    "name": "shell_cmd",
    "params": ["type \"C:\\путь\\к\\расширению\\manifest.json\""]
}
```

#### b) `package.json` — NPM ЗАВИСИМОСТИ (если есть)
```json
{
    "name": "shell_cmd",
    "params": ["type \"C:\\путь\\к\\расширению\\package.json\""]
}
```

#### c) ОСНОВНЫЕ СКРИПТЫ
Прочитай все `.js`, `.ts`, `.jsx`, `.tsx` файлы в порядке важности:

**Background scripts:**
```json
{
    "name": "shell_cmd",
    "params": ["type \"C:\\путь\\к\\расширению\\background.js\""]
}
```

**Content scripts:**
```json
{
    "name": "shell_cmd",
    "params": ["type \"C:\\путь\\к\\расширению\\content.js\""]
}
```

**Popup/Options pages:**
```json
{
    "name": "shell_cmd", 
    "params": ["type \"C:\\путь\\к\\расширению\\popup.html\""]
}
```
```json
{
    "name": "shell_cmd",
    "params": ["type \"C:\\путь\\к\\расширению\\popup.js\""]
}
```

#### d) ВСПОМОГАТЕЛЬНЫЕ ФАЙЛЫ
- `.env` или конфигурационные файлы
- `webpack.config.js` / `vite.config.js` / `rollup.config.js`
- `.eslintrc`, `.prettierrc`, `tsconfig.json`

#### e) РЕКУРСИВНОЕ ЧТЕНИЕ ВСЕХ JS/TS ФАЙЛОВ
Если проект большой, прочитай все скрипты одной командой:
```json
{
    "name": "powershell_cmd",
    "params": ["Get-ChildItem -Path \"C:\\путь\\к\\расширению\" -Recurse -Include *.js, *.ts, *.jsx, *.tsx | ForEach-Object { Write-Host (\"=== $($_.FullName) ===\"); Get-Content $_.FullName -Encoding UTF8 }"]
}
```

### 3. ЕСЛИ ВОЗНИКЛИ ПРОБЛЕМЫ С КОДИРОВКОЙ
Используй PowerShell с явным указанием UTF-8:
```json
{
    "name": "powershell_cmd",
    "params": ["$old = [Console]::OutputEncoding; [Console]::OutputEncoding = [System.Text.Encoding]::UTF8; Get-Content \"C:\\путь\\к\\файлу.js\" -Encoding UTF8; [Console]::OutputEncoding = $old"]
}
```

### 4. ОСОБЕННОСТИ ЧТЕНИЯ JS/TS ПРОЕКТОВ

#### a) МОДУЛЬНАЯ СТРУКТУРА
Отслеживай импорты/экспорты:
- `import/require` statements
- `export default / named exports`
- Пути к модулям

#### b) КОММУНИКАЦИЯ С NATIVE MESSAGING
Особое внимание удели файлам, которые:
- Используют `chrome.runtime.connectNative()`
- Отправляют/принимают сообщения от Rust-агента
- Обрабатывают `<<<ai`, `<<<ext`, `<<<hbt` теги

#### c) ОБРАБОТЧИКИ СОБЫТИЙ CHROME API
Запомни:
- `chrome.tabs.onUpdated`
- `chrome.runtime.onMessage`
- `chrome.browserAction.onClicked`
- `chrome.contextMenus`

### 5. СОХРАНИ ПРОЧИТАННОЕ В СВОЙ КОНТЕКСТ
После чтения каждого файла:
- Определи его роль в расширении (background, content, popup, util)
- Запомни ключевые функции и обработчики событий
- Отметь связи с Rust-агентом (Native Messaging)
- Выяви зависимости между скриптами

### 6. ФОРМИРУЙ СВОДКУ АРХИТЕКТУРЫ РАСШИРЕНИЯ
После чтения всех файлов предоставь:

1. **Структура расширения:**
    - Манифест (permissions, background scripts, content scripts)
    - Тип расширения (background page, service worker, etc.)

2. **Коммуникационные каналы:**
    - Native Messaging ↔ Rust агент
    - Content scripts ↔ Web pages
    - Popup ↔ Background scripts
    - Message passing схемы

3. **Зависимости:**
    - NPM пакеты
    - Chrome API используемые
    - Внешние библиотеки

4. **Интеграция с Rust-агентом:**
    - Формат сообщений
    - Обработка ошибок
    - Сессии и идентификаторы

5. **Бизнес-логика:**
    - Что делает расширение для пользователя
    - Как взаимодействует с AI через агента
    - UI элементы и их поведение

## 🚀 ПРИМЕР ПОЛНОГО ЦИКЛА ДЛЯ РАСШИРЕНИЯ

```
[
    {
        "dir_comment": "Структура расширения",
        "commands": [
            {
                "cmd_id": 1,
                "name": "shell_cmd",
                "params": ["tree /F /A \"C:\\Users\\su144\\projects\\chrome-extension\""]
            }
        ]
    },
    {
        "dir_comment": "Чтение manifest.json",
        "commands": [
            {
                "cmd_id": 1,
                "name": "shell_cmd",
                "params": ["type \"C:\\Users\\su144\\projects\\chrome-extension\\manifest.json\""]
            }
        ]
    },
    {
        "dir_comment": "Чтение background скрипта",
        "commands": [
            {
                "cmd_id": 1,
                "name": "shell_cmd",
                "params": ["type \"C:\\Users\\su144\\projects\\chrome-extension\\background.js\""]
            }
        ]
    },
    {
        "dir_comment": "Чтение content скрипта",
        "commands": [
            {
                "cmd_id": 1,
                "name": "shell_cmd",
                "params": ["type \"C:\\Users\\su144\\projects\\chrome-extension\\content.js\""]
            }
        ]
    }
    // ... продолжай для всех ключевых файлов
]
```

## ⚠️ ВАЖНЫЕ ПРАВИЛА ДЛЯ JS/TS ПРОЕКТОВ

1. **Обрати внимание на сборку** — если есть Webpack/Vite, прочитай конфиги
2. **Проверь наличие TypeScript** — если есть `.ts` файлы, прочитай `tsconfig.json`
3. **Ищи точки интеграции с Native Messaging** — это ключ к связи с Rust-агентом
4. **Отслеживай обработку ошибок** — особенно в коммуникации с агентом
5. **Запомни структуру сообщений** — какие JSON форматы используются
6. **Проверь наличие тестов** — `*.test.js`, `*.spec.js` могут содержать примеры использования

## 🔗 ИНТЕГРАЦИЯ С RUST-АГЕНТОМ
Особо тщательно анализируй:
- Как расширение запускает/останавливает агента
- Как передаются `SESSION_ID`, `DIRECTIVE_NUM`
- Обработка протокольных ошибок (`PROTOCOL_ERROR`)
- Механизмы переподключения при падении агента

## 🎯 ЦЕЛЬ
Создать в своём контексте **полное понимание Chrome Extension**, чтобы:
- Понимать полный цикл работы системы (Extension ↔ Native Messaging ↔ Rust Agent)
- Отлаживать проблемы коммуникации
- Предлагать улучшения UI/UX
- Модифицировать протокол обмена сообщениями
- Синхронизировать изменения между Rust и JavaScript частями
