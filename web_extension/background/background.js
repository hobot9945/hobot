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

// Синглтон для управления данными кнопок popup.js в хранилище сессии.
import { tabHobotStateStore } from "./tab_hobot_state.js";

import { createExtIconControl } from "./extension_icon_control.js";
const extIconControl = createExtIconControl(tabHobotStateStore);

// Фиксация вкладки в момент занятости агента.
import { installTabSwitchGuard } from "./tab_switch_guard.js";
installTabSwitchGuard(connectionManager)

// =================================================================================
// 1. ОБРАБОТЧИК СООБЩЕНИЙ ОТ CONTENT SCRIPT (Вкладки)
// =================================================================================
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
    // Игнорируем сообщения без привязки к вкладке. Пытаемся получить ID вкладки двумя способами:
    //     //    - Из sender (если пишет content script)
    //     //    - Из request (если пишет popup или кто-то еще)
    const tabId = sender.tab?.id || request.tabId;
    if (!tabId) return;

    // --- СЦЕНАРИЙ A: Инициализация соединения ---
    // Вкладка загрузилась (или перезагрузилась) и просит подключить Агента. Передаем пакет инициализации Хоботу.
    if (request.type === "INIT_CONNECTION") {
        console.log(`Запрос на инициализацию от вкладки ${tabId}`);
        connectionManager.connect(tabId, request.payload);
        sendResponse({ status: "connected" });
        return true;
    }

    // --- СЦЕНАРИЙ: Keep-Alive ---
    // Просто возвращаем true, показывая, что мы живы. Это сбрасывает таймер бездействия Service Worker-а.
    if (request.type === "PING") {
        sendResponse({ status: "pong" });
        return false; // Синхронный ответ
    }

    // --- СЦЕНАРИЙ: Отправка данных Агенту ---
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

    // --- СЦЕНАРИЙ: Обновление статуса занятости ---
    // Content Script сообщает о смене статуса (начало/конец работы директивы).
    if (request.type === "UPDATE_BUSY_STATE") {

        connectionManager.setBusy(tabId, request.busy);
        sendResponse({ status: "ack" });
        return false; // Синхронный ответ
    }

    // --- СЦЕНАРИЙ: Создать дефолтное состояние вкладки при отсутствии, вернуть state ---
    if (request.type === "HOBOT_STATE_ENSURE") {

        const tabId = _resolveTabId(request, sender);
        if (tabId == null) {
            sendResponse({ status: "error", message: "No tabId for HOBOT_STATE_ENSURE" });
            return false;
        }   // if

        tabHobotStateStore.getOrCreateTabHobotState(tabId)
            .then((state) => {
                sendResponse({ status: "ok", tabId: tabId, state: state });
            });

        return true; // ответ будет асинхронно
    }   // if HOBOT_STATE_ENSURE

    // --- СЦЕНАРИЙ: Получить состояние вкладки (без создания) ---
    if (request.type === "HOBOT_STATE_GET") {

        const tabId = _resolveTabId(request, sender);
        if (tabId == null) {
            sendResponse({ status: "error", message: "No tabId for HOBOT_STATE_GET" });
            return false;
        }   // if

        tabHobotStateStore.getTabHobotState(tabId)
            .then((state) => {
                // state может быть null -> это ок
                sendResponse({ status: "ok", tabId: tabId, state: state });
            });

        return true;
    }   // if HOBOT_STATE_GET

    // --- СЦЕНАРИЙ: Установить состояние вкладки (полная замена) + уведомить content script ---
    if (request.type === "HOBOT_STATE_SET") {

        const tabId = _resolveTabId(request, sender);
        if (tabId == null) {
            sendResponse({ status: "error", message: "No tabId for HOBOT_STATE_SET" });
            return false;
        }   // if

        tabHobotStateStore.setTabHobotState(tabId, request.state)
            .then((state) => {

                // 1) Ответ отправителю (popup или кто угодно).
                sendResponse({ status: "ok", tabId: tabId, state: state });

                // 2) Пушим обновление в content script этой вкладки.
                // Ошибка доставки (например, вкладка без контентного скрипта) не должна ломать логику.
                chrome.tabs.sendMessage(tabId, {
                    type: "HOBOT_STATE_CHANGED",
                    tabId: tabId,
                    state: state
                }).catch(() => {});

            });

        return true;
    }   // if HOBOT_STATE_SET

    // --- СЦЕНАРИЙ: Актуализация иконки расширения ---
    if (request.type === "UPDATE_EXTENSION_ICON") {

        const tabId = _resolveTabId(request, sender);
        if (tabId == null) {
            sendResponse({ status: "error", message: "No tabId for UPDATE_EXTENSION_ICON" });
            return false;
        }   // if

        tabHobotStateStore.getTabHobotState(tabId)
            .then((state) => {
                if (state) {
                    // Данные для вкладки существуют, то есть вкладка наша: обновить вид иконы
                    extIconControl.updateIcon(tabId, state);
                } else {
                    // Данных для вкладки нет, вкладка чужая: блокировать икону.
                    extIconControl.resetIcon(tabId);
                }   // if/else
                sendResponse({ status: "ok" });
            });

        return true;
    }   // if UPDATE_EXTENSION_ICON

    // --- СЦЕНАРИЙ: Полная деактивация UI при смерти Агента ---
    if (request.type === "HOBOT_DEATH_UI_RESET") {
        const tabId = _resolveTabId(request, sender);

        if (tabId != null) {

            // 1) Оповещаем popup (если он открыт), чтобы он закрылся.
            chrome.runtime.sendMessage({ type: "HOBOT_DEATH_UI_RESET", tabId: tabId }).catch(() => {});

            // 2) Сбрасываем иконку и блокируем клики (disable).
            extIconControl.resetIcon(tabId);

            // 3) Удаляем состояние из хранилища.
            tabHobotStateStore.deleteTabHobotState(tabId).catch(() => {});
        }
        return false;
    }

    // --- Подача уведомлений. Контентные скрипты не могут делать это сами, просят background.js. ---
    // Агент не имеет собственных средств передачи сообщений пользователю, за исключением подачи их
    // в поле ввода веб-интерфейса, что засоряло бы диалог. Поэтому, нотификации. Агент шлет аварийные
    // сообщения сюда.
    if (request.type === "EXTENSION_NOTIFY") {
console.log(`получено сообщение EXTENSION_NOTIFY: ${request.message}`);
        chrome.notifications.create({
            type: "basic",
            iconUrl: "/icons/elephant_32.png",  // Иконка из твоего manifest
            title: request.title || "Хобот",
            message: request.message
        });
        return false;   // Ответ не требуется.
    }
});

// =================================================================================
// 3. АКТУАЛИЗАЦИЯ ИКОНКИ ПРИ ПЕРЕКЛЮЧЕНИИ ВКЛАДОК
// =================================================================================
chrome.tabs.onActivated.addListener((activeInfo) => {
    extIconControl.onActiveTabChanged(activeInfo.tabId);
});

// =================================================================================
// 4. ОЧИСТКА ПРИ ЗАКРЫТИИ ВКЛАДКИ
// =================================================================================
chrome.tabs.onRemoved.addListener((tabId) => {

    // Очистка иконки (остановка мигания, если было).
    extIconControl.removeTab(tabId);

    // Удаляем состояние “кнопок управления Хоботом” для закрытой вкладки.
    tabHobotStateStore.deleteTabHobotState(tabId).catch(() => {});
});

/**
 * _resolveTabId()
 *
 * Назначение:
 * Определить tabId для запроса состояния Хобота.
 *
 * Правило:
 * 1) Если запрос пришел из content script -> используем sender.tab.id (payload.tabId игнорируем).
 * 2) Если запрос пришел из popup -> sender.tab отсутствует, берем request.tabId.
 *
 * @param {object} request
 * @param {object} sender
 * @returns {number|null}
 */
function _resolveTabId(request, sender) {

    const fromSender = sender?.tab?.id;
    if (typeof fromSender === "number") {
        return fromSender;
    }   // if

    const fromRequest = request?.tabId;
    if (typeof fromRequest === "number") {
        return fromRequest;
    }   // if

    return null;
}   // _resolveTabId()