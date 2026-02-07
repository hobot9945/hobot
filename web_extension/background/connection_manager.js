/**
 * connection_manager.js
 *
 * ОПИСАНИЕ:
 * Менеджер соединений (tab <-> Native Messaging Port) для фонового Service Worker (MV3).
 *
 * Модуль отвечает за:
 * - запуск/перезапуск нативного хоста для конкретной вкладки;
 * - хранение активных портов по tabId;
 * - прозрачное проксирование сообщений: Хобот -> content script (через background);
 * - корректное завершение нативного процесса при закрытии вкладки / переподключении (F5);
 * - хранение флага занятости per-tab (для Focus Guard в background.js).
 *
 * Важно:
 * - Этот модуль НЕ валидирует содержимое сообщений и НЕ понимает протокол директив.
 * - Он только доставляет текст туда/сюда и хранит состояние соединений.
 */

// Имя хоста, зарегистрированного в реестре Windows (Native Messaging Host)
const HOST_NAME = "com.example.hobot";

/**
 * Класс для управления сессиями (вкладка <-> нативный порт).
 *
 * Хранилище:
 * - key: tabId (number)
 * - value: { port: chrome.runtime.Port, isBusy: boolean }
 * Этот объект нужен, поскольку фоновый скрипт может работать на несколько вкладок, каждая со своим экземпляром Хобота.
 */
class ConnectionManager {

    constructor() {

        // Карта активных сессий.
        this.sessionMap = new Map();
    }   // constructor()

    /**
     * connect()
     *
     * Назначение:
     * Создать (или пересоздать) подключение к Native Host для вкладки tabId и настроить слушатели.
     *
     * Алгоритм:
     * 1) Если для tabId уже есть сессия — закрыть её (remove). Старый Port будет закрыт с задержкой.
     * 2) Создать новый Port через chrome.runtime.connectNative().
     * 3) Зарегистрировать новый Port в sessionMap.
     * 4) Подписаться на события Port:
     *    - onMessage: пробросить сообщение в content script.
     *    - onDisconnect: если это "текущий" Port — уведомить вкладку и удалить сессию.
     * 5) Отправить init-пакет в Native Host.
     *
     * @param {number} tabId - ID вкладки
     * @param {string} initPacketString - Инициализационный пакет (строка, уже обёрнутая в <<<ext ... >>>ext)
     *
     * # Side effects
     * - Запускает нативный процесс (hobot.exe) через Native Messaging.
     * - Навешивает слушатели на Port.
     * - Пишет в консоль.
     */
    connect(tabId, initPacketString) {
        try {
            // 1) Если уже есть сессия на tabId — закрываем её (например, при F5), точнее, запускаем процесс закрытия.
            // Запись для сессии удаляется из карты.
            if (this.sessionMap.has(tabId)) {
                console.log(`Обнаружена старая сессия для tab ${tabId}. Переподключение.`);
                this.remove(tabId);
            }

            // 2) Открытие нового порта (запуск процесса hobot.exe)
            console.log(`Запуск Native Host для tab ${tabId}...`);
            const port = chrome.runtime.connectNative(HOST_NAME);

            // 3) Регистрируем сессию. Возможно, в карте еще есть запись закрывающейся сессии с этим же tabId, но с
            // другим портом.
            this._add(tabId, port);

            // 4) onMessage: Хобот -> content script (вкладка). Сообщения ловятся в модуле hobot_bridge.js.
            port.onMessage.addListener((msg) => {

                // Сообщения от устаревшего порта игнорируем.
                const session = this.sessionMap.get(tabId);
                if (!session || session.port !== port) return;

                chrome.tabs.sendMessage(tabId, {
                    type: "FROM_HOBOT",
                    payload: msg
                }).catch(err => {
                    console.warn(`Вкладка ${tabId} недоступна для ответа: ${err.message}`);

                    // Если вкладка мертва — нативный процесс нам больше не нужен.
                    // (remove() безопасен: он удалит именно текущую запись).
                    this.remove(tabId);
                });
            });

            // 5) onDisconnect: процесс хоста завершился или соединение разорвано.
            port.onDisconnect.addListener(() => {

                // Если это не текущий порт — просто игнорируем событие (оно от старого порта).
                const session = this.sessionMap.get(tabId);
                if (!session || session.port !== port) return;

                // Это дисконнект текущего порта.
                const lastError = chrome.runtime.lastError;
                const errorMsg = lastError ? lastError.message : "Агент упал";
                console.log(`Агент отключился (tab ${tabId}). Причина: ${errorMsg}`);

                // Посылаем сообщение в hobot_bridge.js.
                chrome.tabs.sendMessage(tabId, {
                    type: "HOBOT_DISCONNECTED",
                    error: errorMsg
                }).catch(() => {});

                // Удаляем вкладку из карты.
                this.sessionMap.delete(tabId);
            });

            // 6) Отправляем init-пакет.
            this.sendToNative(tabId, initPacketString);
            console.log(`Native Host подключен и инициализирован (tab ${tabId})`);

        } catch (err) {
            console.error(`Фатальная ошибка при подключении Native Host: ${err.message}`);
        }   // try/catch
    }   // connect()

    /**
     * _add()
     *
     * Назначение:
     * Зарегистрировать новую сессию в карте.
     *
     * @param {number} tabId
     * @param {chrome.runtime.Port} port
     */

    _add(tabId, port) {
        this.sessionMap.set(tabId, {
            port: port,
            isBusy: false,
        });
    }   // _add()

    /**
     * remove()
     *
     * Назначение:
     * Закрыть сессию и нативный порт для конкретной вкладки.
     *
     * Поведение:
     * - Пытается отправить агенту "COMPLETION" (fire-and-forget),
     *   затем через 3 секунды рвёт порт.
     *
     * @param {number} tabId
     *
     * # Side effects
     * - Посылает служебный пакет в нативный процесс.
     * - Закрывает native port (с задержкой).
     * - Удаляет запись из sessionMap сразу, чтобы новый connect(tabId) мог создать новую сессию.
     */
    remove(tabId) {

        const session = this.sessionMap.get(tabId);
        if (!session) return;

        // Важно: сохраняем ссылку на port локально, потому что запись из map мы удалим.
        const port = session.port;

        // Удаляем запись сразу: это ключ к корректному переподключению без гонок.
        this.sessionMap.delete(tabId);
        console.log(`Сессия вкладки ${tabId} закрывается.`);

        try {
            // 1) Попытка вежливого завершения (fire-and-forget).
            const completionJson = JSON.stringify({ type: "COMPLETION" }, null, 4);

            // Добавляем отступ табом к каждой строке (как в исходном background.js).
            const indentedCompletion = completionJson
                .split("\n")
                .map(line => "\t" + line)
                .join("\n");

            const byePacket = `<<<ext\n${indentedCompletion}\n>>>ext\n`;
            port.postMessage({ text: byePacket });

            // 2) Насильственное закрытие с задержкой (дать агенту шанс закрыться корректно).
            setTimeout(() => {
                try {
                    port.disconnect();
                } catch (_) {
                    // Игнорируем.
                }
            }, 3000);
        } catch (_) {

            // Игнорируем ошибки (например, если процесс агента уже упал).

            try {
                // Даже если postMessage не удалось — пробуем разорвать порт.
                port.disconnect();
            } catch (_) {
                // Игнорируем.
            }   // try/catch
        }   // try/catch
    }   // remove()

    /**
     * setBusy()
     *
     * Назначение:
     * Установить статус занятости для вкладки (для Focus Guard в background.js).
     *
     * @param {number} tabId
     * @param {boolean} isBusy
     */
    setBusy(tabId, isBusy) {
        const session = this.sessionMap.get(tabId);
        if (session) {
            session.isBusy = isBusy;
        }   // if
    }   // setBusy()

    /**
     * sendToNative()
     *
     * Назначение:
     * Отправить payload в нативный порт.
     *
     * @param {number} tabId
     * @param {string} payload - Строка, которую агент ожидает прочитать (обычно <<<ai...>>>ai или <<<ext...>>>ext)
     * @returns {boolean} true если отправили, false если сессии нет.
     *
     * # Errors
     * - Может бросить исключение, если port.postMessage упал (например, порт уже разорван).
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

            console.error(`Ошибка отправки (tab ${tabId}): ${e?.message || String(e)}`);
            throw e;

        }   // try/catch

    }   // sendToNative()

    /**
     * findBusyOtherTab()
     *
     * Назначение:
     * Найти вкладку (кроме excludeTabId), которая сейчас отмечена как busy.
     *
     * @param {number} excludeTabId
     * @returns {number|null}
     */
    findBusyOtherTab(excludeTabId) {

        for (const [id, session] of this.sessionMap) {
            if (session.isBusy && id !== excludeTabId) {
                return id;
            }
        }   // for

        return null;

    }   // findBusyOtherTab()

    /**
     * isThereSession()
     *
     * Назначение:
     * Проверить, есть ли активная сессия для вкладки.
     *
     * @param {number} tabId
     * @returns {boolean}
     */
    isThereSession(tabId) {

        return this.sessionMap.has(tabId);

    }   // isThereSession()
}   // ConnectionManager

// Singleton: удобно импортировать и использовать без ручного new.
export const connectionManager = new ConnectionManager();