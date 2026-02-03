/**
 * background.js — Ядро расширения "AI Native Bridge" (Hobot).
 *
 * ОПИСАНИЕ:
 * Этот скрипт выполняет роль центрального маршрутизатора (Router) и менеджера сессий.
 * Он обеспечивает постоянный двусторонний канал связи между веб-страницей (Content Script)
 * и локальным процессом агента «Хобот» через Native Messaging API.
 *
 * Архитектурно скрипт является «тонким» посредником: он не анализирует содержимое
 * пакетов (JSON, директивы), а прозрачно туннелирует поток данных.
 *
 * Важная особенность — управление глобальным состоянием занятости. Поскольку Service Worker
 * видит все вкладки, именно он реализует механизм блокировки (Focus Guard), запрещая
 * пользователю переключаться на другие вкладки браузера во время выполнения агентом
 * критических операций.
 *
 * ОТВЕТСТВЕННОСТЬ:
 * 1. Управление жизненным циклом нативного процесса (агент Хобот):
 *    - Инициализация соединения через chrome.runtime.connectNative.
 *    - Поддержание постоянного канала связи (Long-lived connection).
 *    - Корректное закрытие порта при закрытии вкладки (предотвращение "зомби"-процессов).
 *    - Попутно, предоставляет доступ к chrome.storage.local popup-скрипту.
 *
 * 2. Маршрутизация сообщений (Transparent Proxy):
 *    - Content Script -> Background -> Хобот (Stdin).
 *    - Хобот (Stdout) -> Background -> Content Script.
 *    - Полная прозрачность: фоновый скрипт не валидирует и не изменяет передаваемые данные.
 *
 * 3. "Focus Guard" (Защита фокуса):
 *    - Агрегация статусов занятости (isAgentBusy) от всех активных вкладок.
 *    - Принудительный возврат фокуса на рабочую вкладку при попытке переключения.
 */

// Имя хоста, зарегистрированное в реестре Windows (Native Messaging Host)
const HOST_NAME = "com.example.hobot";

/**
 * Класс для управления сессиями (вкладка <-> нативный порт). Инкапсулирует работу с портами и флагами занятости.
 * Этот объект нужен, поскольку фоновый скрипт может работать на несколько вкладок, каждая со своим экземпляром Хобота.
 */
class ConnectionManager {

    constructor() {

        // Хранилище сессий.
        // Ключ: tabId (Number) - ид. вкладки
        // Значение: { port: Port, isBusy: Boolean }
        this.sessionMap = new Map();
    }

    /**
     * Создает подключение к нативному приложению и настраивает слушатели.
     * @param {number} tabId - ID вкладки
     * @param {object} initPacketString - Данные для инициализации контекста (URL, Title)
     */
    connect(tabId, initPacketString) {
        let isOldPortClosing = false;   // Перед созданием коннекта старому агенту послан сигнал закрытия.
        try {
            // 1. Очистка старого соединения, если оно зависло (например, при F5)
            // И заодно проверка на зомби-процессы
            if (this.sessionMap.has(tabId)) {
                console.log(`Обнаружена старая сессия для tab ${tabId}. Переподключение.`);
                this.remove(tabId);
                isOldPortClosing = true;
            }

            // 2. Открытие нового порта (запуск процесса hobot.exe)
            console.log(`Запуск Native Host для tab ${tabId}...`);
            const port = chrome.runtime.connectNative(HOST_NAME);

            // 3. Настройка слушателей порта

            // Входящие сообщения от Хобота (STDOUT Агента -> Расширение -> Вкладка) пробрасываются контентному скрипту,
            // точнее, dir_extractor.js.
            port.onMessage.addListener((msg) => {

                chrome.tabs.sendMessage(tabId, {
                    type: "FROM_HOBOT",
                    // payload содержит JSON вида:
                    // payload = {text: '<<<hbt\n{<поля JSON структуры>}\n>>>hbt'}
                    payload: msg
                }).catch(err => {
                    console.warn(`Вкладка ${tabId} недоступна для ответа: ${err.message}`);
                    // Если вкладка мертва, процесс агента нам больше не нужен
                    this.remove(tabId);
                });
            });

            // Разрыв соединения с Хоботом (Process Crash или закрытие). Пробрасываются hobot_bridge, но только в том
            // случае, если Хобот НЕ ожидает завершения (по данным в карте вкладок). Иначе, просто игнорируем.
            port.onDisconnect.addListener(() => {
                const lastError = chrome.runtime.lastError;
                const errorMsg = lastError ? lastError.message : "Агент упал";

                if (this.sessionMap.has(tabId)) {
                    if (!this.sessionMap.get(tabId).isOldPortClosing) {
                        // Агент завершился незапланированно. Стучим, убираем из карты.
                        console.log(`Агент упал (tab ${tabId}). Причина: ${errorMsg}`);

                        // Уведомляем вкладку, если она жива.
                        chrome.tabs.sendMessage(tabId, {
                            type: "HOBOT_DISCONNECTED",
                            error: errorMsg
                        }).catch(() => {
                        });

                        // Убираем порт из карты.
                        this.sessionMap.delete(tabId);
                    } else {
                        // Ожидалось закрытие агента, просто сбрасываем флаг.
                        this.sessionMap.get(tabId).isOldPortClosing = false;
                    }
                }
            });

            // 4. Регистрация сессии
            this._add(tabId, port, isOldPortClosing);

            // 5. Отправка пакета инициализации (проксирование от AgentIface к Хоботу).
            this.sendToNative(tabId, initPacketString);
            console.log(`Native Host подключен и инициализирован (tab ${tabId})`);

        } catch (err) {
            console.error(`Фатальная ошибка при подключении Native Host: ${err.message}`);
        }
    }

    /**
     * Регистрирует новое соединение (внутренний метод).
     * Гарантия от зомби: Мы явно закрываем предыдущий порт для этого tabId (в connect).
     */
    _add(tabId, port, isOldPortClosing) {
        this.sessionMap.set(tabId, {
            port: port,
            isBusy: false,
            isOldPortClosing: isOldPortClosing
        });
    }

    /**
     * Удаляет сессию и закрывает нативный порт.
     * Реализует протокол вежливого завершения (п. 2.3 ТЗ).
     */
    remove(tabId) {
        if (this.sessionMap.has(tabId)) {
            const session = this.sessionMap.get(tabId);
            try {
                // 1. Попытка вежливого прощания (Fire-and-forget)
                // Шлем сигнал COMPLETION, чтобы агент мог корректно завершить потоки/файлы.
                // Мы не ждем ответа DIRECTIVE_COMPLETED, т.к. вкладка закрывается мгновенно.
                const completionJson = JSON.stringify({type: "COMPLETION"}, null, 4);

                // Добавляем отступ 4 пробела к каждой строке
                const indentedCompletion = completionJson.split('\n').map(line => '\t' + line).join('\n');

                const byePacket = `<<<ext\n${indentedCompletion}\n>>>ext\n`;
                session.port.postMessage({text: byePacket});

                // 2. Насильственное закрытие соединения. Это вызовет закрытие STDIN у агента. Немного откладываем,
                // чтобы агент имел возможность закрыться самостоятельно.
                setTimeout(() => {
                    session.port.disconnect();
                }, 3000);
            } catch (e) {
                // Игнорируем ошибки (например, если процесс агента уже упал сам)
            }

            // 3. Очистка памяти
            this.sessionMap.delete(tabId);
            console.log(`Сессия вкладки ${tabId} закрывается.`);
        }
    }

    /**
     * Устанавливает статус занятости для вкладки.
     */
    setBusy(tabId, isBusy) {
        const session = this.sessionMap.get(tabId);
        if (session) {
            session.isBusy = isBusy;
        }
    }

    /**
     * Отправляет payload в нативный порт.
     * @returns {boolean} Успешность отправки
     */
    sendToNative(tabId, payload) {
        const session = this.sessionMap.get(tabId);
        if (!session) return false;

        try {
            // Chrome Native Messaging требует отправлять JSON.
            // Оборачиваем данные в { text: ... } для агента.
            session.port.postMessage({ text: payload });
            return true;
        } catch (e) {
            console.error(`Ошибка отправки (tab ${tabId}): ${e.message}`);
            throw e; // Пробрасываем наверх для обработки в sendResponse
        }
    }

    /**
     * Ищет любую вкладку, которая сейчас занята (isBusy = true),
     * исключая переданную в аргументе (excludeTabId).
     * Нужен для Focus Guard.
     * @returns {number|null} ID занятой вкладки или null
     */
    findBusyOtherTab(excludeTabId) {
        for (const [id, session] of this.sessionMap) {
            if (session.isBusy && id !== excludeTabId) {
                return id;
            }
        }
        return null;
    }

    /**
     * Проверяет, есть ли активная сессия для вкладки
     */
    isThereSession(tabId) {
        return this.sessionMap.has(tabId);
    }
}

// Глобальный экземпляр менеджера соединений
const connectionManager = new ConnectionManager();


// =================================================================================
// 1. ОБРАБОТЧИК СООБЩЕНИЙ ОТ CONTENT SCRIPT (Вкладки)
// =================================================================================
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
    // Игнорируем сообщения без привязки к вкладке
    const tabId = sender.tab?.id;
    if (!tabId) return;

    // --- СЦЕНАРИЙ A: Инициализация соединения ---
    // Вкладка загрузилась (или перезагрузилась) и просит подключить Агента. Передаем пакет инициализации Хоботу.
    if (request.type === "INIT_CONNECTION") {
        console.log(`Запрос на инициализацию от вкладки ${tabId}`);
        connectionManager.connect(tabId, request.payload);
        sendResponse({ status: "connected" });
        return true;
    }

    // --- СЦЕНАРИЙ B: Keep-Alive ---
    // Просто возвращаем true, показывая, что мы живы. Это сбрасывает таймер бездействия Service Worker-а.
    if (request.type === "PING") {
        sendResponse({ status: "pong" });
        return false; // Синхронный ответ
    }

    // --- СЦЕНАРИЙ C: Отправка данных Агенту ---
    // Content Script хочет передать данные (директива или лог) в STDIN.
    if (request.type === "SEND_TO_AGENT") {
        if (connectionManager.isThereSession(tabId)) {
            try {
                connectionManager.sendToNative(tabId, request.payload);
                sendResponse({ status: "sent" });
            } catch (e) {
                sendResponse({ status: "error", msg: e.message });
            }
        } else {
            console.warn(`Попытка отправки без соединения (tab ${tabId})`);
            sendResponse({ status: "error", msg: "No active connection" });
        }
        return true;
    }

    // --- СЦЕНАРИЙ C: Обновление статуса занятости ---
    // Content Script сообщает о смене статуса (начало/конец работы директивы).
    if (request.type === "UPDATE_BUSY_STATE") {
        connectionManager.setBusy(tabId, request.busy);
        sendResponse({ status: "ack" });
        return false; // Синхронный ответ
    }

    // --- СЦЕНАРИЙ X: Получить/создать INIT_SESSION payload для вкладки ---
    // Content Script (HobotBridge) просит init-пакет. Храним per-tab в chrome.storage.session.
    if (request.type === "GET_INIT_SESSION_PAYLOAD") {
        const aiUrlOrigin = _resolveAiUrlOrigin(request, sender);

        _getOrCreateInitSessionPayload(tabId, aiUrlOrigin, (payload) => {
            sendResponse({ status: "ok", payload: payload });
        });

        return true; // ВАЖНО: sendResponse будет вызван асинхронно из callback storage.get/set
    }

    // --- Подача уведомлений. Контентные скрипты не могут делать это сами, просят background.js. ---
    // Агент не имеет собственных средств передачи сообщений пользователю, за исключением подачи их
    // в поле ввода веб-интерфейса, что засоряло бы диалог. Поэтому, нотификации. Агент шлет аварийные
    // сообщения сюда.
    if (request.type === "EXTENSION_NOTIFY") {
        chrome.notifications.create({
            type: "basic",
            iconUrl: "icons/elephant_32.png",  // Иконка из твоего manifest
            title: request.title || "Хобот",
            message: request.message
        });
        return false;   // Ответ не требуется.
    }
});


// =================================================================================
// 2. FOCUS GUARD (Защита от переключения вкладок)
// =================================================================================
// Согласно п.7 ТЗ: Если агент работает, пользователь не должен покидать вкладку.

chrome.tabs.onActivated.addListener((activeInfo) => {
    // Спрашиваем менеджер: есть ли ДРУГАЯ занятая вкладка?
    const protectedTabId = connectionManager.findBusyOtherTab(activeInfo.tabId);

    if (protectedTabId !== null) {
        console.log(`Блокировка переключения! Вкладка ${protectedTabId} занята работой.`);

        // Запускаем рекурсивную функцию попыток
        _attemptFocusRevert(protectedTabId, 1);
    }
});

/**
 * Пытается вернуть фокус на вкладку с механизмом повтора (Retry).
 * @param {number} targetTabId - ID вкладки, куда надо вернуться.
 * @param {number} attempt - Номер текущей попытки.
 */
function _attemptFocusRevert(targetTabId, attempt) {
    const MAX_ATTEMPTS = 5;
    const DELAY_MS = 100;

    // Сначала ждем, потом пробуем (чтобы дать браузеру отдохнуть сразу)
    setTimeout(() => {
        chrome.tabs.update(targetTabId, { active: true })
            .then(() => {
                // Успех - выходим молча
            })
            .catch((err) => {
                // Если ошибка и есть попытки в запасе
                if (attempt < MAX_ATTEMPTS) {
                    console.warn(`Попытка ${attempt} не удалась (${err.message}). Повтор...`);
                    _attemptFocusRevert(targetTabId, attempt + 1);
                } else {
                    console.error(`Не удалось вернуть фокус на ${targetTabId} после ${MAX_ATTEMPTS} попыток.`);
                }
            });
    }, DELAY_MS);
}

// =================================================================================
// 3. ОЧИСТКА ПРИ ЗАКРЫТИИ ВКЛАДКИ
// =================================================================================
chrome.tabs.onRemoved.addListener((tabId) => {

    // Удалить элемент карты для закрываемой вкладки.
    _deleteInitSessionPayload(tabId);

    // Делегируем очистку менеджеру
    if (connectionManager.isThereSession(tabId)) {
        console.log(`Вкладка ${tabId} закрыта. Очистка сессии.`);
        connectionManager.remove(tabId);
    }
});

// =================================================================================
// Пакеты инициализации для контентных скриптов. Свой для каждого tabId.
// =================================================================================

const INIT_SESSION_MAP_KEY = "initSessionPayloadByTabId";

/**
 * Пытается получить origin AI-страницы.
 * Приоритет:
 * 1) request.ai_url (если прислали из content script)
 * 2) sender.url (URL страницы, где работает content script)
 */
function _resolveAiUrlOrigin(request, sender) {
    if (request && typeof request.ai_url === "string" && request.ai_url.length > 0) {
        try {
            return new URL(request.ai_url).origin;
        } catch (_) {
            // Игнорируем, попробуем sender.url
        }
    }

    if (sender && typeof sender.url === "string" && sender.url.length > 0) {
        try {
            return new URL(sender.url).origin;
        } catch (_) {
            // Падать не будем, вернем заглушку
        }
    }

    return "unknown";
}


/**
 * Формирует payload для INIT_SESSION согласно протоколу.
 *
 * Важно:
 * - session_id: 6 HEX (A-F0-9), верхний регистр.
 * - window_title: строится как ai_url + " [HBT-session_id]".
 *
 * @param {string} aiUrlOrigin - origin страницы AI (например, https://chatgpt.com)
 * @returns {object} JSON-объект payload для пакета INIT_SESSION.
 */
function _generateInitSessionPayload(aiUrlOrigin) {

    // 1) session_id: 6-значное HEX, верхний регистр
    const rnd = new Uint8Array(3); // 3 байта -> 6 hex-символов
    crypto.getRandomValues(rnd);

    const sessionId = Array.from(rnd)
        .map(b => b.toString(16).padStart(2, "0"))
        .join("")
        .toUpperCase();

    // 2) browser: эвристика по userAgent (для твоего протокола достаточно)
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
        os_readonly: true
    };
}

/**
 * Возвращает payload INIT_SESSION для конкретной вкладки (tabId).
 *
 * Поведение:
 * - Если для tabId уже есть сохранённый payload в chrome.storage.session — возвращаем его.
 * - Если нет — генерируем новый payload, сохраняем его в chrome.storage.session и возвращаем.
 *
 * Данные хранятся в chrome.storage.session в виде "карты" (обычного объекта):
 * {
 *   "<tabId>": { session_id, browser, ai_url, window_title },
 *   "<tabId2>": { ... }
 * }
 *
 * Важно:
 * - Функция асинхронная: результат отдаётся через callback.
 * - Здесь нет обработки ошибок storage (chrome.runtime.lastError) — по текущей логике деградируем молча.
 *
 * @param {number} tabId - ID вкладки, для которой нужен init payload.
 * @param {string} aiUrlOrigin - Origin AI страницы (например, "https://chatgpt.com").
 * @param {(payload: object) => void} callback - Вызывается ровно один раз, когда payload готов.
 */
function _getOrCreateInitSessionPayload(tabId, aiUrlOrigin, callback) {

    // 1) Читаем из session storage весь объект-карту по ключу INIT_SESSION_MAP_KEY.
    // chrome.storage.* API асинхронный -> результат приходит в callback get().
    chrome.storage.session.get([INIT_SESSION_MAP_KEY], (result) => {

        // 2) Достаём карту из result (или создаём пустую, если ключа ещё нет).
        // result имеет вид: { [INIT_SESSION_MAP_KEY]: <значение> }
        const map = (result && result[INIT_SESSION_MAP_KEY]) ? result[INIT_SESSION_MAP_KEY] : {};

        // 3) tabId приводим к строке (в объектах ключи фактически строки).
        const key = String(tabId);

        // 4) Если payload для этой вкладки уже есть — сразу отдаём его наружу.
        // Это “fast path”: никаких записей, только callback.
        if (map[key]) {
            callback(map[key]);
            return;
        }

        // 5) Payload нет — создаём новый.
        // Генератор должен создать session_id и собрать протокольные поля.
        const payload = _generateInitSessionPayload(aiUrlOrigin);

        // 6) Кладём в карту под ключом вкладки.
        map[key] = payload;

        // 7) Записываем обновлённую карту обратно в storage.session.
        // Пока set() не завершится, мы НЕ вызываем callback — чтобы гарантировать,
        // что дальнейшие запросы из этой же вкладки увидят сохранённое значение.
        chrome.storage.session.set({ [INIT_SESSION_MAP_KEY]: map }, () => {
            callback(payload);
        });
    });
}   // _getOrCreateInitSessionPayload()


/**
 * Удаляет сохранённый payload INIT_SESSION для вкладки tabId из chrome.storage.session.
 *
 * Поведение:
 * - Читает карту INIT_SESSION_MAP_KEY.
 * - Если записи для tabId нет — ничего не делает.
 * - Если запись есть — удаляет ключ и сохраняет карту обратно.
 *
 * @param {number} tabId - ID вкладки, которую закрыли/очищаем.
 */
function _deleteInitSessionPayload(tabId) {

    // 1) Читаем текущую карту.
    chrome.storage.session.get([INIT_SESSION_MAP_KEY], (result) => {
        const map = (result && result[INIT_SESSION_MAP_KEY]) ? result[INIT_SESSION_MAP_KEY] : {};
        const key = String(tabId);

        // 2) Если нечего удалять — выходим без записи.
        if (!map[key]) return;

        // 3) Удаляем запись конкретной вкладки.
        delete map[key];

        // 4) Пишем обновлённую карту обратно.
        chrome.storage.session.set({ [INIT_SESSION_MAP_KEY]: map });
    });
}   // _deleteInitSessionPayload()
