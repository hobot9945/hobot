/**
 * hobot_bridge.js
 *
 * Модуль отвечает за взаимодействие с "внешним миром" (Native Host через Background Script).
 *
 * ОТВЕТСТВЕННОСТЬ:
 * 1. Инициализация сессии (Connect):
 *    - Запрос init payload у background.js (он хранит per-tab данные в chrome.storage.session).
 *    - Заморозка заголовка вкладки (Title freeze) строго в payload.window_title.
 *    - Отправка приветственного пакета INIT_CONNECTION (который содержит INIT_SESSION).
 *
 * 2. Удержание фонового скрипта:
 *    - Фоновый Service Worker может засыпать при простое. Чтобы снизить риск, отправляем PING.
 *
 * 3. Транспорт (Send/Receive):
 *    - Отправка данных (директивы, логи) в Background.
 *    - Прием системных сообщений от Хобота (DIRECTIVE_COMPLETED, CRITICAL_ERROR).
 *
 * 4. Управление состоянием (State Control):
 *    - Реакция на разрыв соединения (HOBOT_DISCONNECTED).
 *    - Сброс глобального флага занятости при завершении директивы.
 */

class HobotBridge {
    constructor() {
        // Биндинг для onMessage (колбэк не меняется)
        this._handleRuntimeMessage = this._handleRuntimeMessage.bind(this);

        // Служебные поля для управления жизненным циклом
        this._pingIntervalId = null;     // Таймер удержания background.js

        // Наблюдатель за <title>, чтобы не дать сайту сбросить заголовок
        this._titleObserver = null;

        // Таймаут пересброса заголовка. Есть подозрение, что _freezeTabTitle() беспощадно бодается с кем-то, кто
        // непременно хочет изменить название вкладки. Это приводит к зацикливанию. Поэтому, имя вкладки меняем
        // не мгновенно, а с задержкой. Пока для отладки.
        this._titleFreezeTimer = null;

        // Состояние инициализации
        this.initHobotPayload = null;    // Payload INIT_SESSION
    }

    /**
     * Инициализирует мост асинхронно.
     *
     * Последовательность:
     * 1) Подписывается на сообщения от Background.
     * 2) Запускает keep-alive (PING).
     * 3) Запрашивает init payload у background.js.
     * 4) Замораживает заголовок вкладки.
     * 5) Сохраняет sessionId в Globals.
     *
     * Важно:
     * - chrome.storage.session недоступно из content script, поэтому payload берём через background.js.
     *
     * * Вызывается вручную из content.js после создания экземпляра.
     */
    async initialize() {
        // 1) Подписываемся на сообщения от Background (ответы от Хобота и системные события)
        chrome.runtime.onMessage.addListener(this._handleRuntimeMessage);

        // 2) Запускаем keep-alive таймер (не дать уснуть фоновому скрипту).
        this._startPingInterval();

        // 3) Запрашиваем init payload (из chrome.storage.session внутри background.js)
        this.initHobotPayload = await this._generateInitSessionPayload();

        if (!this.initHobotPayload) {
            throw new Error("Не удалось получить INIT_SESSION payload (background.js вернул null).");
        }

        // 4) Сохраняем sessionId в Globals (для DirExtractor)
        window.Globals.sessionId = this.initHobotPayload.session_id;

        // 5) Замораживаем заголовок вкладки строго в window_title из протокола
        this._freezeTabTitle(this.initHobotPayload.window_title);

        console.log("[HobotBridge] Initialize complete:", this.initHobotPayload.session_id);
    }

    /**
     * Устанавливает соединение с Native Host и отправляет INIT_SESSION.
     * Вызывается, когда пользователь разрешил работу (кнопка "РАБОТА").
     *
     * Предусловие: initialize() успешно завершён (initHobotPayload заполнен).
     */
    async _initializeHobot() {
        if (!this.initHobotPayload) {
            throw new Error("initialize() не вызван или завершился ошибкой.");
        }

        // 1) Формируем пакет инициализации (<<<ext { type: INIT_SESSION, payload } >>>ext)
        const jsonBody = JSON.stringify({
            type: "INIT_SESSION",
            payload: this.initHobotPayload
        }, null, 4);

        const indentedJsonBody = jsonBody.split('\n').map(line => '\t' + line).join('\n');
        const initString = `<<<ext\n${indentedJsonBody}\n>>>ext\n`;

        console.log("[HobotBridge, initializeHobot()] Connecting to Native Host...");

        // 2) Отправляем запрос на подключение (с задержкой). При обновлении страницы (F5), фокус не появляется
        // в поле ввода сразу.
        await window.Globals.delay(2000);

        return new Promise((resolve, reject) => {

            // В режиме логирования сессия не устанавливается.
            if (window.Globals.LOGGER_MODE) {
                return
            }

            // При посылке пакета инициализации поднимаем флаг, чтобы агент с гарантией получил фокус поля ввода для ответа.
            window.Globals.setIsAgentBusy(true);

            chrome.runtime.sendMessage({
                type: "INIT_CONNECTION",
                payload: initString
            }, (response) => {
                if (chrome.runtime.lastError) {
                    const err = new Error(chrome.runtime.lastError.message);
                    console.error(`[HobotBridge] Connection failed:`, err.message);
                    reject(err);
                    return;
                }

                console.log("[HobotBridge] Connection established:", JSON.stringify(response));
                window.Globals.isHobotInitialized = true;
                resolve(response);
            });
        });
    }

    /**
     * Отправляет данные (текст директивы или лог) агенту.
     * Используется модулем DirectiveExtractor (AgentIface).
     *
     * @param {string} text - Сырые данные (уже обернутые в теги <<<ai... или <<<ext...)
     * @returns {Promise<object|null>} Response или null при ошибке.
     */
    async sendToAgent(text) {
        if (!text) return null;

        return new Promise((resolve, reject) => {
            chrome.runtime.sendMessage({
                type: "SEND_TO_AGENT",
                payload: text
            }, (response) => {
                if (chrome.runtime.lastError || (response && response.status === "error")) {
                    const errMsg = chrome.runtime.lastError?.message || response?.msg;
                    console.error(`[HobotBridge, sendToAgent()] Send failed:`, errMsg);
                    window.Globals.setIsAgentBusy(false); // Ошибка посылки, сбросить флаг.
                    reject(new Error(errMsg));
                    return;
                }
                resolve(response);
            });
        });
    }

    /**
     * Очистка ресурсов при выгрузке страницы.
     * Вызывается из content.js при window.unload.
     */
    cleanup() {

        // Отключить пинг фонового скрипта.
        if (this._pingIntervalId) {
            clearInterval(this._pingIntervalId);
            this._pingIntervalId = null;
        }

        // Отключить заморозку титула вкладки.
        if (this._titleObserver) {
            try { this._titleObserver.disconnect(); } catch (_) {}
            this._titleObserver = null;
        }

        // Очистить прием сообщений от фонового скрипта.
        chrome.runtime.onMessage.removeListener(this._handleRuntimeMessage);
        console.log("[HobotBridge] Cleanup complete.");
    }

    //-------------------------------------------------------------------------------------------------------------
    //                              Внутренний интерфейс
    //-------------------------------------------------------------------------------------------------------------

    /**
     * Запускает keep-alive таймер (PING background.js каждые 20 сек).
     */
    _startPingInterval() {
        this._pingIntervalId = setInterval(() => {
            try {
                chrome.runtime.sendMessage({ type: "PING" })?.catch(() => {});
            } catch (e) {
                console.warn("[HobotBridge] Extension context lost. Stopping ping.");
                clearInterval(this._pingIntervalId);
                this._pingIntervalId = null;
            }
        }, 20000);
    }

    /**
     * Генерация INIT_SESSION payload локально.
     *
     * @returns {object} payload для инициализации
     */
    async _generateInitSessionPayload() {
        // 1) session_id: 6-значное HEX, верхний регистр
        const rnd = new Uint8Array(3); // 3 байта → 6 hex-символов
        crypto.getRandomValues(rnd);

        const sessionId = Array.from(rnd)
            .map(b => b.toString(16).padStart(2, "0"))
            .join("")
            .toUpperCase();

        // 2) browser: эвристика по userAgent
        const ua = (navigator.userAgent || "").toLowerCase();
        let browser = "unknown";
        if (ua.includes("chrome") && !ua.includes("edg")) browser = "chrome";
        else if (ua.includes("firefox")) browser = "firefox";
        else if (ua.includes("edg")) browser = "edge";

        // 3) AI URL origin
        const aiUrlOrigin = window.location.origin;

        // 4) Получить из хранилища состояние кнопок.
        let state = null;
        try {
            const resp = await chrome.runtime.sendMessage({
                type: "HOBOT_STATE_GET"
            });
            state = resp.state;
        } catch (e) {
            console.warn("[hobot_bridge.js] HOBOT_STATE_GET failed:", e?.message || String(e));
        }

        // Привести кнопки к понятному для Хобота виду.
        const os_readonly = state?.osWriteMode !== window.Globals.osWriteMode.WRITE;
        const step_through = state?.execMode !== window.Globals.execMode.AUTO;

        // Вернуть пакет инициализации с учетом текущего состояния кнопок.
        return {
            session_id: sessionId,
            browser: browser,
            ai_url: aiUrlOrigin,
            window_title: `${aiUrlOrigin} [${sessionId}]`,
            os_readonly: os_readonly,
            step_through: step_through
        };
    }

    /**
     * Замораживает document.title, чтобы SPA не перетирали его во время работы.
     *
     * При переключении чатов:
     * - Многие SPA (React Helmet и аналоги) при навигации могут ПЕРЕСОЗДАВАТЬ <title> целиком.
     * - Если MutationObserver висит на конкретном titleEl, а узел <title> заменили,
     *   observer остаётся привязан к старому (уже отсоединённому) узлу и больше не увидит изменений.
     *
     * Решение:
     * - Наблюдаем не конкретный <title>, а стабильный контейнер (document.head),
     *   чтобы ловить как изменения текста, так и замену/пересоздание самого <title>.
     * - При любом изменении убеждаемся, что <title> существует, и (с небольшой задержкой)
     *   возвращаем document.title к нужному значению.
     *
     * Важно:
     * - В протоколе это поле называется window_title.
     * - Заголовок должен быть стабильным, чтобы агент мог найти вкладку через WinAPI.
     * - Для защиты от боданий/зацикливания применяем восстановление с задержкой (debounce).
     *
     * @param {string} windowTitle
     */
    _freezeTabTitle(windowTitle) {
        if (!windowTitle || typeof windowTitle !== "string") return;

        // 1) Применяем титул немедленно
        document.title = windowTitle;

        /**
         * ensureTitleEl()
         *
         * Назначение:
         * - Убедиться, что в документе существует <title>.
         * - Если SPA удалило/пересоздало head-метаданные и <title> пропал — создаём его заново.
         *
         * @returns {HTMLTitleElement|null}
         */
        const ensureTitleEl = () => {
            let titleEl = document.querySelector("title");
            if (!titleEl) {
                titleEl = document.createElement("title");
                titleEl.textContent = windowTitle;
                document.head?.appendChild(titleEl);
            }
            return titleEl;
        };

        /**
         * scheduleTitleRestore()
         *
         * Назначение:
         * - Вернуть document.title к windowTitle не мгновенно, а с задержкой.
         * Это снижает риск "зацикливания"/боданий:
         *   браузер/SPA меняет title -> мы возвращаем -> SPA снова меняет -> ...
         *
         * Поведение:
         * - Если уже запланирован таймер восстановления — не ставим второй.
         */
        const scheduleTitleRestore = () => {
            if (document.title === windowTitle) return;

            if (!this._titleFreezeTimer) {
                this._titleFreezeTimer = setTimeout(() => {
                    document.title = windowTitle;
                    clearTimeout(this._titleFreezeTimer);
                    this._titleFreezeTimer = null;
                }, 100);
            }
        };

        // 2) Гарантируем наличие <title> перед включением наблюдения
        ensureTitleEl();

        // 3) Блокируем изменения:
        // Наблюдаем document.head (или document.documentElement, если head отсутствует),
        // чтобы переживать замену/пересоздание самого <title>.
        const root = document.head || document.documentElement;

        this._titleObserver = new MutationObserver(() => {
            // SPA могло заменить/удалить <title> — восстановим при необходимости
            ensureTitleEl();

            // И затем вернём значение заголовка (debounce)
            scheduleTitleRestore();
        });

        this._titleObserver.observe(root, {
            subtree: true,
            childList: true,
            characterData: true
        });

        console.log("[HobotBridge] Title frozen:", windowTitle);
    }

    /**
     * Основной обработчик сообщений от Background Script (колбэк не меняется).
     */
    _handleRuntimeMessage(message) {
        // 1. Сообщение от самого Агента (STDOUT)
        if (message.type === "FROM_HOBOT") {
            this._processHobotMessage(message.payload);
        }

        // 2. Системное уведомление о разрыве связи
        else if (message.type === "HOBOT_DISCONNECTED") {
            const errorMsg = message.error || "Unknown error";

            // Сначала логируем и показываем нотификацию через старый метод
            this._handleCriticalError(`Связь потеряна: ${errorMsg}`);

            // ПРОКСИРОВАНИЕ: Генерируем внутреннее событие для content.js
            window.dispatchEvent(new CustomEvent("hobot-bridge:HOBOT_DISCONNECTED", {
                detail: {error: errorMsg}
            }));
        }
    }

    /**
     * Разбор JSON-сообщений, пришедших от агента.
     * @param {object} data - JSON объект:
     *  data = {text: '<<<hbt\n{"type":"DIRECTIVE_COMPLETED"}\n>>>hbt'}
     */
    _processHobotMessage(data) {
        const agentsJson = this._parseNativeMessage(data);
        if (agentsJson == null) return;

        if (agentsJson.type === "DIRECTIVE_COMPLETED") {

            // Сбрасываем состояние занятости агента.
            window.Globals.setIsAgentBusy(false);
        } else if (agentsJson.type === "CRITICAL_ERROR") {
            const msg = agentsJson.error || "Unknown critical error";
            this._handleCriticalError(msg);
        }
    }

    /**
     * Обработка фатальных ситуаций.
     * Сбрасывает флаг занятости агента и подает системное уведомление через background.js.
     */
    _handleCriticalError(msg) {
        window.Globals.setIsAgentBusy(false);
        console.error(msg);

        chrome.runtime.sendMessage({
            type: "EXTENSION_NOTIFY",
            title: "💥 Критическая ошибка Хобота",
            message: msg
        }).catch(() => {});  // Игнорируем ошибки доставки в SW
    }

    /**
     * Парсинг сообщения, пришедшего от Хобота.
     * Хобот передает JSON в виде строки заключенный в теги <<<hbt ... >>>hbt.
     *
     * @param {object} data - JSON объект:
     *  data = {text: '<<<hbt\n{"type":"DIRECTIVE_COMPLETED"}\n>>>hbt'}
     *
     * @return
     * Возвращается JSON объект, переданный Хоботом.
     */
    _parseNativeMessage(data) {
        const rawText = data.text;
        if (!rawText) {
            console.warn("Нет payload.text");
            return;
        }

        const jsonMatch = rawText.match(/\{.*}/);
        if (!jsonMatch) {
            console.warn("Нет JSON в сообщении");
            return;
        }

        let parsed = {};
        try {
            parsed = JSON.parse(jsonMatch[0]);
        } catch (e) {
            console.log(`Parse error:`, e.message);
        }

        return parsed;
    }
}

// Экспорт в глобальную область (для content.js)
window.HobotBridge = HobotBridge;
