/**
 * tab_switch_guard.js — Защита от переключения вкладок (Tab Switch Guard).
 *
 * ОПИСАНИЕ:
 * Модуль реализует механизм “не уходить с рабочей вкладки”, когда агент занят (isBusy=true).
 * Работает в background/service worker (Manifest V3).
 *
 * Логика:
 * - Слушаем chrome.tabs.onActivated.
 * - При попытке переключиться на другую вкладку проверяем, есть ли “защищаемая” вкладка,
 *   где агент занят.
 * - Если есть — возвращаем пользователя обратно на защищаемую вкладку, используя retry-механику.
 *
 * Важно:
 * - Модуль не хранит состояние занятости сам, он опирается на connectionManager.findBusyOtherTab().
 * - Модуль не управляет фокусом поля ввода на странице. Это задача content script (FocusManager).
 */

/**
 * Подключает Tab Switch Guard.
 *
 * Контракт:
 * connectionManager должен предоставлять метод:
 * - findBusyOtherTab(excludeTabId) -> number|null
 *
 * @param {object} connectionManager - Менеджер соединений/сессий.
 */
export function installTabSwitchGuard(connectionManager) {

    // Защита от случайной двойной установки в рамках одного жизненного цикла SW.
    if (installTabSwitchGuard._isInstalled) return;
    installTabSwitchGuard._isInstalled = true;

    chrome.tabs.onActivated.addListener((activeInfo) => {

        // activeInfo.tabId — вкладка, на которую пользователь переключился.
        // Если есть ДРУГАЯ вкладка, где агент занят — возвращаем фокус назад.
        const protectedTabId = connectionManager.findBusyOtherTab(activeInfo.tabId);

        if (protectedTabId !== null) {
            console.log(`Блокировка переключения! Вкладка ${protectedTabId} занята работой.`);

            _attemptFocusRevert(protectedTabId, 1);
        }   // if
    });
}   // installTabSwitchGuard()

// Флаг установки (как статическое поле функции).
installTabSwitchGuard._isInstalled = false;

/**
 * _attemptFocusRevert()
 *
 * Назначение:
 * Пытается вернуть фокус на вкладку с механизмом повтора (Retry).
 *
 * @param {number} targetTabId - ID вкладки, куда надо вернуться.
 * @param {number} attempt - Номер текущей попытки.
 */
function _attemptFocusRevert(targetTabId, attempt) {

    const MAX_ATTEMPTS = 5;
    const DELAY_MS = 100;

    // Сначала ждём, потом пробуем (чтобы дать браузеру “успокоиться”).
    setTimeout(() => {

        chrome.tabs.update(targetTabId, { active: true })
            .then(() => {
                // Успех — выходим молча.
            })
            .catch((err) => {

                if (attempt < MAX_ATTEMPTS) {
                    console.warn(`Попытка ${attempt} не удалась (${err.message}). Повтор...`);
                    _attemptFocusRevert(targetTabId, attempt + 1);
                } else {
                    console.error(`Не удалось вернуть фокус на ${targetTabId} после ${MAX_ATTEMPTS} попыток.`);
                }   // if/else
            });
    }, DELAY_MS);
}   // _attemptFocusRevert()