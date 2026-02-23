/**
 * tab_hobot_state.js — Хранилище per-tab состояния “кнопок управления Хоботом”.
 *
 * ОПИСАНИЕ:
 * Модуль хранит состояние UI/управления Хоботом в разрезе вкладок (tabId) в chrome.storage.session, чтобы:
 * - переживать “сон” Service Worker-а (MV3);
 * - иметь единый источник истины для popup и content script (через background);
 * - корректно чиститься при закрытии вкладки.
 *
 * Важно:
 * - Источник истины — chrome.storage.session.
 * - In-memory кэш не используем.
 * - Этот модуль НЕ занимается messaging. Он только читает/пишет состояние.
 */

/**
 * TabHobotStateStore
 *
 * Класс-обёртка над chrome.storage.session для per-tab состояния.
 */
class TabHobotStateStore {

    constructor() {

        // Ключ “карты” в chrome.storage.session:
        // { "<tabId>": <state>, "<tabId2>": <state2>, ... }
        this._hobotStateMapKey = "hobotStateByTabId";

        // Разрешённые значения (совпадают с CSS-классами popup).
        this._execMode = Object.freeze({
            PAUSED: "paused",
            STEP: "step",
            AUTO: "auto",
        });

        this._osWriteMode = Object.freeze({
            READ: "read",
            WRITE: "write",
        });

    }   // constructor()

    /**
     * getOrCreateTabHobotState()
     *
     * Назначение:
     * Вернуть состояние Хобота для конкретной вкладки.
     *
     * Поведение:
     * - Если state для tabId уже сохранён в chrome.storage.session — возвращает его (после нормализации).
     * - Если нет — создаёт дефолтное, сохраняет и возвращает.
     *
     * Примечание:
     * - Деградация “молча”: ошибки storage не пробрасываются.
     *
     * @param {number} tabId
     * @returns {Promise<object>} state
     */
    async getOrCreateTabHobotState(tabId) {

        const tabKey = String(tabId);

        // 1) Читаем карту.
        const result = await this._storageSessionGet([this._hobotStateMapKey]);

        // 2) Достаём карту или создаём пустую.
        const map = (result && result[this._hobotStateMapKey]) ? result[this._hobotStateMapKey] : {};

        // 3) Fast path.
        if (map[tabKey]) {
            return map[tabKey];
        }   // if

        // 4) Создаём дефолт.
        const state = this._makeDefaultState();

        // 5) Сохраняем.
        map[tabKey] = state;
        await this._storageSessionSet({ [this._hobotStateMapKey]: map });

        return state;

    }   // getOrCreateTabHobotState()

    /**
     * getTabHobotState()
     *
     * Назначение:
     * Получить состояние вкладки, если оно существует.
     *
     * @param {number} tabId
     * @returns {Promise<object|null>} state или null если нет
     */
    async getTabHobotState(tabId) {

        const tabKey = String(tabId);

        const result = await this._storageSessionGet([this._hobotStateMapKey]);
        const map = (result && result[this._hobotStateMapKey]) ? result[this._hobotStateMapKey] : {};

        if (!map[tabKey]) return null;

        return map[tabKey];

    }   // getTabHobotState()

    /**
     * setTabHobotState()
     *
     * Назначение:
     * Установить состояние вкладки (полная замена).
     *
     * @param {number} tabId
     * @param {object} state
     * @returns {Promise<object>} сохранённое состояние
     */
    async setTabHobotState(tabId, state) {

        const tabKey = String(tabId);

        const result = await this._storageSessionGet([this._hobotStateMapKey]);
        const map = (result && result[this._hobotStateMapKey]) ? result[this._hobotStateMapKey] : {};

        map[tabKey] = state;
        await this._storageSessionSet({ [this._hobotStateMapKey]: map });

        return state;
    }   // setTabHobotState()

    /**
     * deleteTabHobotState()
     *
     * Назначение:
     * Удалить состояние вкладки (например, при закрытии вкладки).
     *
     * @param {number} tabId
     * @returns {Promise<void>}
     */
    async deleteTabHobotState(tabId) {

        const tabKey = String(tabId);

        const result = await this._storageSessionGet([this._hobotStateMapKey]);
        const map = (result && result[this._hobotStateMapKey]) ? result[this._hobotStateMapKey] : {};

        if (!map[tabKey]) return;

        delete map[tabKey];

        await this._storageSessionSet({ [this._hobotStateMapKey]: map });

    }   // deleteTabHobotState()

    // ==================== PRIVATE ====================

    /**
     * _makeDefaultState()
     *
     * @returns {{execMode: string, osWriteMode: string}}
     */
    _makeDefaultState() {

        return {
            execMode: this._execMode.PAUSED,       // дефолт: пауза
            osWriteMode: this._osWriteMode.READ,   // дефолт: чтение
        };

    }   // _makeDefaultState()

    /**
     * _storageSessionGet()
     *
     * Назначение:
     * Promise-обёртка над chrome.storage.session.get().
     *
     * Примечание:
     * - Ошибки/исключения деградируют в пустой объект.
     *
     * @param {string[]} keys
     * @returns {Promise<object>}
     */
    _storageSessionGet(keys) {

        return new Promise((resolve) => {
            try {
                chrome.storage.session.get(keys, (result) => resolve(result || {}));
            } catch (_) {
                resolve({});
            }   // try/catch
        });
    }   // _storageSessionGet()

    /**
     * _storageSessionSet()
     *
     * Назначение:
     * Promise-обёртка над chrome.storage.session.set().
     *
     * Примечание:
     * - Ошибки/исключения деградируют в “успешное завершение”.
     *
     * @param {object} obj
     * @returns {Promise<void>}
     */
    _storageSessionSet(obj) {

        return new Promise((resolve) => {
            try {
                chrome.storage.session.set(obj, () => resolve());
            } catch (_) {
                resolve();
            }   // try/catch
        });
    }   // _storageSessionSet()
}   // TabHobotStateStore

// Singleton: удобно импортировать и использовать без ручного new.
export const tabHobotStateStore = new TabHobotStateStore();