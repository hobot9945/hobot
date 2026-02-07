/**
 * tab_session_store.js — Хранилище per-tab данных в chrome.storage.session.
 *
 * ОПИСАНИЕ:
 * На текущем этапе модуль отвечает только за INIT_SESSION payload (протокол Hobot).
 * Данные хранятся per-tabId в chrome.storage.session, чтобы:
 * - переживать “сон” Service Worker-а;
 * - сохранять стабильный session_id на вкладку;
 * - корректно чиститься при закрытии вкладки.
 *
 * Важно:
 * - Источник истины — chrome.storage.session.
 * - Внутренний in-memory кэш НЕ используется (чтобы не обманываться жизненным циклом MV3 SW).
 */

/**
 * TabInitSessionStore
 *
 * Класс-обёртка над chrome.storage.session для per-tab данных.
 * Сейчас хранит только INIT_SESSION payload.
 */
class TabInitSessionStore {

    constructor() {

        // Ключ “карты” в chrome.storage.session:
        // { "<tabId>": <payload>, "<tabId2>": <payload2>, ... }
        this._initSessionMapKey = "initSessionPayloadByTabId";

    }   // constructor()

    /**
     * resolveAiUrlOrigin()
     *
     * Назначение:
     * Пытается получить origin AI-страницы.
     *
     * Приоритет:
     * 1) request.ai_url (если прислали из content script)
     * 2) sender.url (URL страницы, где работает content script)
     *
     * @param {object} request
     * @param {object} sender
     * @returns {string} origin (например, "https://chatgpt.com") или "unknown"
     */
    resolveAiUrlOrigin(request, sender) {

        if (request && typeof request.ai_url === "string" && request.ai_url.length > 0) {
            try {
                return new URL(request.ai_url).origin;
            } catch (_) {
                // Игнорируем, попробуем sender.url
            }   // try/catch
        }   // if

        if (sender && typeof sender.url === "string" && sender.url.length > 0) {
            try {
                return new URL(sender.url).origin;
            } catch (_) {
                // Падать не будем, вернём заглушку
            }   // try/catch
        }   // if

        return "unknown";

    }   // resolveAiUrlOrigin()

    /**
     * getOrCreateInitSessionPayload()
     *
     * Назначение:
     * Вернуть INIT_SESSION payload для конкретной вкладки.
     *
     * Поведение:
     * - Если payload для tabId уже сохранён в chrome.storage.session — возвращает его.
     * - Если нет — генерирует новый, сохраняет и возвращает.
     *
     * Важно:
     * - Деградация “молча”: ошибки storage не пробрасываются (как в текущем background.js).
     *
     * @param {number} tabId
     * @param {string} aiUrlOrigin
     * @returns {Promise<object>} payload
     */
    async getOrCreateInitSessionPayload(tabId, aiUrlOrigin) {

        const tabKey = String(tabId);

        // 1) Читаем карту.
        const result = await this._storageSessionGet([this._initSessionMapKey]);

        // 2) Достаём карту или создаём пустую.
        const map = (result && result[this._initSessionMapKey]) ? result[this._initSessionMapKey] : {};

        // 3) Fast path.
        if (map[tabKey]) {
            return map[tabKey];
        }   // if

        // 4) Генерируем новый payload.
        const payload = this._generateInitSessionPayload(aiUrlOrigin);

        // 5) Сохраняем.
        map[tabKey] = payload;
        await this._storageSessionSet({ [this._initSessionMapKey]: map });

        return payload;

    }   // getOrCreateInitSessionPayload()

    /**
     * deleteInitSessionPayload()
     *
     * Назначение:
     * Удалить сохранённый INIT_SESSION payload для вкладки tabId (когда вкладку закрыли).
     *
     * @param {number} tabId
     * @returns {Promise<void>}
     */
    async deleteInitSessionPayload(tabId) {

        const tabKey = String(tabId);

        const result = await this._storageSessionGet([this._initSessionMapKey]);
        const map = (result && result[this._initSessionMapKey]) ? result[this._initSessionMapKey] : {};

        if (!map[tabKey]) return;

        delete map[tabKey];

        await this._storageSessionSet({ [this._initSessionMapKey]: map });

    }   // deleteInitSessionPayload()

    // ==================== PRIVATE ====================

    /**
     * _generateInitSessionPayload()
     *
     * Назначение:
     * Сформировать payload INIT_SESSION согласно текущему протоколу.
     *
     * Важно:
     * - session_id: 6 HEX (A-F0-9), верхний регистр.
     * - window_title: строится как ai_url_origin + " [session_id]".
     *
     * @param {string} aiUrlOrigin
     * @returns {object}
     */
    _generateInitSessionPayload(aiUrlOrigin) {

        // 1) session_id: 6-значное HEX, верхний регистр.
        const rnd = new Uint8Array(3); // 3 байта -> 6 hex-символов
        crypto.getRandomValues(rnd);

        const sessionId = Array.from(rnd)
            .map(b => b.toString(16).padStart(2, "0"))
            .join("")
            .toUpperCase();

        // 2) browser: эвристика по userAgent (для протокола достаточно).
        const ua = (navigator.userAgent || "").toLowerCase();
        let browser = "unknown";
        if (ua.includes("chrome") && !ua.includes("edg")) browser = "chrome";
        else if (ua.includes("firefox")) browser = "firefox";
        else if (ua.includes("edg")) browser = "edge";

        return {
            session_id: sessionId,
            browser: browser,
            ai_url: aiUrlOrigin,
            window_title: `${aiUrlOrigin} [${sessionId}]`,
            os_readonly: true,  // запрещено изменение os
            step_through: true  // пошаговый режим
        };

    }   // _generateInitSessionPayload()

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

}   // TabInitSessionStore

// Singleton: удобно импортировать и использовать без ручного new.
export const tabInitSessionStore = new TabInitSessionStore();