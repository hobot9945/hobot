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

// Глобальный экземпляр менеджера соединений.
import { connectionManager } from "./connection_manager.js";

// Глобальный экземпляр генерации/хранилища инициализационного пакета для вкладки.
import { tabInitSessionStore } from "./tab_init_session_store.js";

// Фиксация вкладки в момент занятости агента.
import { installTabSwitchGuard } from "./tab_switch_guard.js";
installTabSwitchGuard(connectionManager)

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
        const aiUrlOrigin = tabInitSessionStore.resolveAiUrlOrigin(request, sender);

        tabInitSessionStore.getOrCreateInitSessionPayload(tabId, aiUrlOrigin)
            .then((payload) => {
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
        os_readonly: true,  // запрещено изменение os
        step_through: true  // пошаговый режим
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
