/**
 * extension_icon_control.js — Управление иконкой расширения.
 *
 * ОПИСАНИЕ:
 * Модуль отвечает за визуальное отображение состояния расширения через иконку в панели инструментов.
 * Иконка отражает текущий режим исполнения (execMode) и уровень доступа (osWriteMode),
 * предоставляя оператору мгновенную обратную связь без необходимости открывать popup.
 *
 * ОТВЕТСТВЕННОСТЬ:
 * 1. Покраска иконки в цвет, соответствующий execMode:
 *    - PAUSED → красный (elephant_red_16.png).
 *    - STEP   → жёлтый (elephant_yellow_16.png).
 *    - AUTO   → зелёный (elephant_green_16.png).
 *
 * 2. Индикация опасного режима:
 *    - Когда execMode !== PAUSED и osWriteMode === WRITE, разрешены операции
 *      изменения хостовой машины. Это опасное состояние.
 *    - Иконка принимает тёмно-зелёный цвет (elephant_dark_green_16.png) и мигает раз в секунду.
 *
 * 3. Архитектура:
 *    - Один глобальный таймер мигания.
 *    - Слушатели событий размещаются в background.js, модуль предоставляет публичные методы.
 *    - Состояния кнопок запрашиваются из tabHobotStateStore при необходимости.
 */

/**
 * Пути к файлам иконок.
 */
const ICON_PATHS = Object.freeze({
    PAUSED:  "/icons/elephant_dark_red_16.png",
    STEP:    "/icons/elephant_green_16.png",
    AUTO:    "/icons/elephant_yellow_16.png",
    DANGER_TIC:  "/icons/elephant_red_16.png",
    DANGER_TOC:  "/icons/elephant_black_16.png",
    DEFAULT: "/icons/elephant_black_16.png",
});

/**
 * Интервал мигания в опасном режиме (мс).
 */
const BLINK_INTERVAL_MS = 1000;

/**
 * ExtensionIconControl
 *
 * Класс управления иконкой расширения.
 * Использует один глобальный таймер мигания. Состояния кнопок не хранит,
 * запрашивает у tabHobotStateStore при необходимости.
 */
class ExtensionIconControl {

    constructor(tabHobotStateStore) {

        // Ссылка на хранилище состояний кнопок.
        this._stateStore = tabHobotStateStore;

        // ID вкладки, для которой сейчас мигает иконка.
        this._blinkingTabId = null;

        // Глобальный таймер мигания.
        this._blinkTimerId = null;

        // Текущая фаза мигания. true — DANGER, false — DEFAULT.
        this._blinkVisible = true;
    }   // constructor()

    /**
     * updateIcon()
     *
     * Назначение:
     * Обновить иконку для вкладки по переданному состоянию кнопок.
     * Единственная точка входа для установки иконки по состоянию.
     *
     * @param {number} tabId - ID вкладки.
     * @param {object} state - Состояние кнопок { execMode, osWriteMode }.
     */
    updateIcon(tabId, state) {

        if (tabId == null || !state) return;

        // Разблокировать иконку (могла быть заблокирована через resetIcon).
        chrome.action.enable(tabId);

        // Обновить вид иконы согласно state.
        this._applyStateToIcon(tabId, state);
    }   // updateIcon()

    /**
     * resetIcon()
     *
     * Назначение:
     * Сбросить иконку в дефолтное состояние (чёрный слон), остановить мигание
     * и заблокировать клик по иконке для этой вкладки.
     *
     * @param {number} tabId - ID вкладки.
     */
    resetIcon(tabId) {

        if (tabId == null) return;

        this._stopBlinking();
        this._setIcon(tabId, ICON_PATHS.DEFAULT);
        chrome.action.disable(tabId);
    }   // resetIcon()

    /**
     * onActiveTabChanged()
     *
     * Назначение:
     * Обработчик переключения вкладки. Запрашивает состояние кнопок из хранилища
     * и обновляет иконку. Вызывается из background.js.
     *
     * @param {number} tabId - ID вкладки, на которую переключились.
     */
    onActiveTabChanged(tabId) {

        this._stateStore.getTabHobotState(tabId)
            .then((state) => {
                if (state) {
                    this.updateIcon(tabId, state);
                } else {
                    this.resetIcon(tabId);
                }   // if/else
            });
    }   // onActiveTabChanged()

    /**
     * removeTab()
     *
     * Назначение:
     * Очистить ресурсы при закрытии вкладки.
     *
     * @param {number} tabId - ID закрываемой вкладки.
     */
    removeTab(tabId) {

        // Если закрылась вкладка с мигающей иконкой — остановить мигание.
        if (tabId === this._blinkingTabId) {
            this._stopBlinking();
        }   // if
    }   // removeTab()

    // ==================== PRIVATE ====================

    /**
     * _applyStateToIcon()
     *
     * Назначение:
     * Определить тип иконки (статичная или мигающая) и применить.
     *
     * @param {number} tabId - ID вкладки.
     * @param {object} state - { execMode, osWriteMode }.
     */
    _applyStateToIcon(tabId, state) {

        const isActive = state.execMode !== "paused";
        const isWriteAllowed = state.osWriteMode === "write";
        const isDanger = isActive && isWriteAllowed;

        if (isDanger) {
            this._startBlinking(tabId);
        } else {
            this._stopBlinking();
            this._setStaticIcon(tabId, state.execMode);
        }   // if/else
    }   // _applyStateToIcon()

    /**
     * _setStaticIcon()
     *
     * Назначение:
     * Установить статичную иконку по execMode.
     *
     * @param {number} tabId - ID вкладки.
     * @param {string} execMode - "paused" | "step" | "auto".
     */
    _setStaticIcon(tabId, execMode) {

        let iconPath;

        switch (execMode) {
            case "paused":
                iconPath = ICON_PATHS.PAUSED;
                break;
            case "step":
                iconPath = ICON_PATHS.STEP;
                break;
            case "auto":
                iconPath = ICON_PATHS.AUTO;
                break;
            default:
                iconPath = ICON_PATHS.DEFAULT;
        }   // switch

        this._setIcon(tabId, iconPath);
    }   // _setStaticIcon()

    /**
     * _startBlinking()
     *
     * Назначение:
     * Запустить мигание иконки. Если мигание уже запущено — перезапускает.
     *
     * @param {number} tabId - ID вкладки, для которой мигаем.
     */
    _startBlinking(tabId) {

        this._stopBlinking();

        this._blinkingTabId = tabId;
        this._blinkVisible = true;
        this._setIcon(tabId, ICON_PATHS.DANGER_TIC);

        this._blinkTimerId = setInterval(() => {

            if (this._blinkingTabId == null) return;

            if (this._blinkVisible) {
                this._setIcon(this._blinkingTabId, ICON_PATHS.DANGER_TOC);
                this._blinkVisible = false;
            } else {
                this._setIcon(this._blinkingTabId, ICON_PATHS.DANGER_TIC);
                this._blinkVisible = true;
            }   // if/else

        }, BLINK_INTERVAL_MS);
    }   // _startBlinking()

    /**
     * _stopBlinking()
     *
     * Назначение:
     * Остановить глобальный таймер мигания.
     */
    _stopBlinking() {

        if (this._blinkTimerId != null) {
            clearInterval(this._blinkTimerId);
            this._blinkTimerId = null;
        }   // if

        this._blinkingTabId = null;
        this._blinkVisible = true;
    }   // _stopBlinking()

    /**
     * _setIcon()
     *
     * Назначение:
     * Низкоуровневая установка иконки через chrome.action.setIcon().
     *
     * @param {number} tabId - ID вкладки.
     * @param {string} iconPath - Путь к файлу иконки.
     */
    _setIcon(tabId, iconPath) {

        try {
            chrome.action.setIcon({
                path: { "16": iconPath },
                tabId: tabId
            }, () => {
                if (chrome.runtime.lastError) { /* noop */ }
            });
        } catch (_) {
            // Игнорируем.
        }   // try/catch
    }   // _setIcon()
}   // ExtensionIconControl

/**
 * Фабричная функция для создания синглтона.
 *
 * @param {object} tabHobotStateStore - Хранилище состояний кнопок.
 * @returns {ExtensionIconControl}
 */
export function createExtIconControl(tabHobotStateStore) {
    return new ExtensionIconControl(tabHobotStateStore);
}