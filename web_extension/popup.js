/**
 * popup.js — UI-переключатели (без сохранения состояния)
 *
 * НАЗНАЧЕНИЕ:
 * - Управляет двумя кнопками в popup:
 *   1) "Исполнение команд ИИ" — 3 режима: ПАУЗА -> ШАГ -> АВТО.
 *   2) "Разрешение записи"   — 2 режима: ЧТЕНИЕ -> ЗАПИСЬ.
 *
 * ВАЖНО:
 * - На этом этапе состояние НЕ сохраняем (без chrome.storage).
 * - Каждый раз при открытии popup состояние стартует из HTML (по class кнопок).
 */

// ===== DOM-элементы =====

const execBtn = document.getElementById("exec-mode-btn");   // Кнопка "Исполнение команд ИИ"
const osWriteBtn = document.getElementById("os-write-btn"); // Кнопка "Разрешение записи"

// ===== Enum-подобные константы состояний =====

/**
 * Состояния исполнения команд ИИ.
 *
 * Примечание:
 * - Значения совпадают с CSS-классами кнопки execBtn (см. popup.html).
 */
const ExecMode = Object.freeze({
    PAUSED: "paused", // красный
    STEP: "step",     // желтый
    AUTO: "auto",     // зеленый
});

/**
 * Состояния разрешения записи.
 *
 * Примечание:
 * - Значения совпадают с CSS-классами кнопки osWriteBtn (см. popup.html).
 */
const OsWriteMode = Object.freeze({
    READ: "read",   // желтый
    WRITE: "write", // зеленый
});

/**
 * Таблица режимов исполнения: порядок определяет, как переключаемся по кругу.
 * Здесь же храним отображаемый текст кнопки.
 */
const execModes = [
    { mode: ExecMode.PAUSED, text: "⏸ ПАУЗА" },
    { mode: ExecMode.STEP,   text: "⏭ ШАГ" },
    { mode: ExecMode.AUTO,   text: "▶ АВТО" },
];

// ===== Текущее состояние (живёт только пока открыт popup) =====
let currentExecMode = ExecMode.PAUSED;     // Дефолт на случай, если запрос не прошёл
let currentOsWriteMode = OsWriteMode.READ;

// Выполняется по клику на иконе расширения.
let currentTabId = null;
(async () => {

    // 1) Определяем “вкладку popup-а” = активная вкладка.
    currentTabId = await _getActiveTabId();
    if (currentTabId == null) {
        console.warn("[popup.js] Active tabId not found.");
        return;
    }   // if

    // 2) Просим background вернуть state или создать и вернуть дефолтное состояние при отсутствии.
    let resp = null;
    try {
        resp = await chrome.runtime.sendMessage({
            type: "HOBOT_STATE_ENSURE",
            tabId: currentTabId
        });
    } catch (e) {
        console.warn("[popup.js] HOBOT_STATE_ENSURE failed:", e?.message || String(e));
        return;
    }   // try/catch
    if (!resp || resp.status !== "ok" || !resp.state) {
        console.warn("[popup.js] Bad HOBOT_STATE_ENSURE response:", resp);
        return;
    }   // if

    // 3) Применяем состояние к UI.
    currentExecMode = resp.state.execMode;
    currentOsWriteMode = resp.state.osWriteMode;
    _applyExecMode(currentExecMode);
    _applyOsWriteMode(currentOsWriteMode);
})(); // IIFE init popup state

/**
 * Применяет состояние режима исполнения к кнопке execBtn.
 *
 * @param {"paused"|"step"|"auto"} mode
 */
function _applyExecMode(mode) {
    // Находим запись в таблице. Если кто-то передал мусор — деградируем в PAUSED.
    const m = execModes.find(x => x.mode === mode) || execModes[0];

    // Важно: className заменяет все классы целиком — у нас ровно один класс-состояние.
    execBtn.className = m.mode;
    execBtn.textContent = m.text;
}   // _applyExecMode()

/**
 * Применяет состояние разрешения записи к кнопке osWriteBtn.
 *
 * @param {"read"|"write"} mode
 */
function _applyOsWriteMode(mode) {
    if (mode === OsWriteMode.WRITE) {
        osWriteBtn.className = OsWriteMode.WRITE;
        osWriteBtn.textContent = "✍ ЗАПИСЬ";
    } else {
        osWriteBtn.className = OsWriteMode.READ;
        osWriteBtn.textContent = "📖 ЧТЕНИЕ";
    }   // if/else
}   // _applyOsWriteMode()

/**
 * Получить tabId активной вкладки текущего окна.
 *
 * @returns {Promise<number|null>}
 */
function _getActiveTabId() {

    return new Promise((resolve) => {
        try {
            chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
                const tabId = tabs?.[0]?.id;
                resolve(typeof tabId === "number" ? tabId : null);
            });
        } catch (_) {
            resolve(null);
        }   // try/catch
    });

}   // _getActiveTabId()

/**
 * _saveStateToBackground()
 *
 * Назначение:
 * Сохранить текущее состояние кнопок в background-хранилище для currentTabId
 * и актуализировать иконку расширения.
 */
async function _saveStateToBackground() {

    if (currentTabId == null) return;

    try {
        // 1) Сохраняем состояние кнопок.
        await chrome.runtime.sendMessage({
            type: "HOBOT_STATE_SET",
            tabId: currentTabId,
            state: {
                execMode: currentExecMode,
                osWriteMode: currentOsWriteMode
            }
        });

        // 2) Актуализируем иконку расширения.
        await chrome.runtime.sendMessage({
            type: "UPDATE_EXTENSION_ICON",
            tabId: currentTabId
        });
    } catch (_) {
        // Деградируем молча: popup — UI, не должен падать.
    }   // try/catch

}   // _saveStateToBackground()

// ===== Обработчики кликов =====

/**
 * Клик по кнопке "Исполнение команд ИИ":
 * переключаем режим по кругу:
 * PAUSED -> STEP -> AUTO -> PAUSED -> ...
 */
execBtn.onclick = () => {

    // Находим индекс текущего режима.
    const idx = execModes.findIndex(x => x.mode === currentExecMode);

    // Идём к следующему по кругу.
    const nextIdx = (idx + 1) % execModes.length;
    currentExecMode = execModes[nextIdx].mode;

    // Применяем в UI.
    _applyExecMode(currentExecMode);

    // Сохраняем состояние кнопок в хранилище.
    _saveStateToBackground().then(() => {});
};  // execBtn.onclick

/**
 * Клик по кнопке "Разрешение записи":
 * тумблер:
 * READ <-> WRITE
 */
osWriteBtn.onclick = () => {
    currentOsWriteMode = (currentOsWriteMode === OsWriteMode.READ)
        ? OsWriteMode.WRITE
        : OsWriteMode.READ;

    _applyOsWriteMode(currentOsWriteMode);

    _saveStateToBackground().then(() => {});
};  // osWriteBtn.onclick

// Слушаем команду на принудительное закрытие нашего окна, если агент умер. Сообщение посылается из background.js.
chrome.runtime.onMessage.addListener((msg) => {
    // Проверяем, что сообщение для нас (нашей вкладки)
    if (msg.type === "HOBOT_DEATH_UI_RESET" && msg.tabId === currentTabId) {
        window.close();
    }
});