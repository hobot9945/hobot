// content.js

// Глобальные ссылки на экземпляры модулей.
// Они понадобятся в следующих частях файла (pause-state, cleanup и т.д.).
let hobotBridge = null;
let directiveExtractor = null;
let textProcessor = null;
let focusManager = null;

// Включен ли перехват текста и посылка директив Хоботу.
let isAgentPaused = true;

/**
 * Инициализация основных объектов контентной части.
 *
 * Порядок важен:
 * 1) Создаем HobotBridge (транспорт) на верхнем уровне.
 * 2) Передаем мост в DirExtractor через конструктор.
 * 3) Создаем TextProcessor и FocusManager.
 * 4) Публикуем focusManager глобально (glob.js дергает его при setAgentBusy()).
 *
 * Важно:
 * - Здесь мы только создаем объекты и поднимаем инфраструктуру.
 * - Запуск перехвата DOM (startObserver) и включение логики “Пауза/Работа” будут отдельно.
 */
async function _prepareContentObjects() {

    // 0) FocusManager: удержание фокуса.
    // glob.js вызывает window.focusManager.onChangeAgentBusyState(), поэтому публикуем сразу. Важно вызвать до
    // инициализации моста, там флаг выставляется перед посылкой пакета инициализации.
    focusManager = new window.FocusManager();
    focusManager.initialize();
    window.focusManager = focusManager;

    // 1) Транспорт: связь с background.js и нативным агентом. hobotBridge.initialize() асинхронный, вызывается вручную позже.
    hobotBridge = new window.HobotBridge();

    // 2) Асинхронная инициализация транспорта:
    //    - получение INIT_SESSION payload через messaging,
    //    - установка Globals.sessionId,
    //    - заморозка title.
    await hobotBridge.initialize();
    await hobotBridge._initializeHobot();

    // 3) Синхронизация состояния кнопок: за время задержки в _initializeHobot() пользователь мог
    //    переключить режимы, поэтому дублируем актуальное состояние отдельными директивами. Делаем небольшую задержку,
    //    чтобы дать Хоботу время принять пакет инициализации.
    await window.Globals.delay(500);
    await _syncStatesToHobot();

    // 4) Экстрактор директив. Получает транспорт как зависимость.
    directiveExtractor = new window.DirectiveExtractor(hobotBridge);

    // 5) TextProcessor инициализируется сразу (создаём MutationObserver), но старт наблюдения включается только
    // в режиме "РАБОТА".
    textProcessor = new window.TextProcessor(directiveExtractor);
    directiveExtractor.setTextProcessor(textProcessor);
    textProcessor.initializeObserver();

}   // _prepareContentObjects()

/**
 * Начало работы для этой вкладки.
 *
 * # Side effects
 * - Запускает/останавливает перехват текста через applyPauseState().
 */
(async () => {

    // Запросить/создать дефолтное состояние “кнопок управления Хоботом” для этой вкладки.
    let resp = null;
    try {
        resp = await chrome.runtime.sendMessage({ type: "HOBOT_STATE_ENSURE" });

        if (resp?.status === "ok") {
            window.Globals.buttonState = resp.state; // инфраструктура: хранение
        }   // if
    } catch (e) {
        console.error("[content.js] CRITICAL. HOBOT_STATE_ENSURE failed:", e?.message || String(e));

        // Расширение не запускаем.
        return;
    }   // try/catch

    // Актуализировать иконку расширения.
    chrome.runtime.sendMessage({ type: "UPDATE_EXTENSION_ICON" }).catch(() => {});

    const isButtonStatePaused = resp.state.execMode === window.Globals.execMode.PAUSED;
    if (isButtonStatePaused !== isAgentPaused) {
        await applyPauseState(isButtonStatePaused);
    }
})(); // IIFE init pause state

/**
 * applyPauseState()
 *
 * Назначение:
 * Управляет состоянием контентной части расширения “Пауза / Работа”.
 *
 * В режиме ПАУЗА:
 * - Останавливает MutationObserver, чтобы не читать текст страницы.
 *
 * В режиме РАБОТА:
 * - Гарантирует, что инфраструктура (bridge/extractor/processor/focus) поднята.
 * - Запускает MutationObserver для слежения за code-block (<pre>) и извлечения директив.
 * Алгоритм:
 * - Если isHobotPaused=true:
 *   - Остановить перехват текста (если был запущен) и выйти.
 * - Если isHobotPaused=false:
 *   - Если Хобот ещё не инициализирован:
 *     - Попытаться выполнить _prepareContentObjects().
 *     - При ошибке: залогировать и отправить уведомление, затем выйти (observer не запускаем).
 *   - Запустить наблюдение.
 *
 * Побочные эффекты:
 * - Может инициировать создание/инициализацию объектов контентной части через _prepareContentObjects().
 * - Пишет флаг window.Globals.isHobotInitialized=true при успешной инициализации.
 * - Отправляет сообщение в background через chrome.runtime.sendMessage при ошибке инициализации.
 *
 * @param {boolean} isHobotPaused - true, если агент на паузе.
 */
async function applyPauseState(isHobotPaused) {

    // 1) ПАУЗА: перехват текста запрещён, observer выключаем и выходим.
    if (isHobotPaused) {
        if (isAgentPaused) {
            // Агент уже на паузе.
            return;
        } else {
            // Поставить агента на паузу.
            textProcessor?.stopObserver();
            isAgentPaused = true;
            return;
        }
    }

    // 2) РАБОТА: Если это первый запуск, поднять инфраструктуру (bridge/extractor/processor/focus).
    if (!window.Globals.isHobotInitialized) {
        try {
            // Фиксируем, что инициализация выполнена (чтобы не делать её повторно). Выставляем этот флаг заранее,
            // потому что инициализация занимает время (секунды), между тем может прийти второй запрос по кнопке.
            window.Globals.isHobotInitialized = true;

            // Поднимаем контентные объекты и выполняем инициализацию транспорта.
            await _prepareContentObjects();
        } catch (e) {
            // Ошибка инициализации: логируем, уведомляем пользователя и НЕ запускаем observer.
            console.error("[content.js, applyPauseState]", e);

            // Уведомление пользователю.
            chrome.runtime.sendMessage({
                type: "EXTENSION_NOTIFY",
                title: "💥 [content.js]: Критическая ошибка в ходе инициализации",
                message: e.message || String(e)
            }).catch(() => {});

            return; // Не стартуем observer при ошибке
        }
    }

    // 3) РАБОТА: запускаем наблюдение за DOM (перехват текста).
    textProcessor.startObserver();
    isAgentPaused = false;
} // applyPauseState()

/**
 * _sendExtensionStateToHobot()
 *
 * Назначение:
 * Отправить директиву расширения Хоботу через hobot_bridge.
 *
 * Формат:
 * <<<ext
 *     {
 *         "type": "<directiveType>",
 *         "value": <value>
 *     }
 * >>>ext
 *
 * @param {string} directiveType - Тип директивы (CHANGE_STEP_THROUGH, CHANGE_OS_READONLY).
 * @param {boolean} value - Значение.
 */
async function _sendExtensionStateToHobot(directiveType, value) {

    const payload = {
        type: directiveType,
        value: value
    };

    const jsonBody = JSON.stringify(payload, null, 4);
    const indentedJsonBody = jsonBody.split('\n').map(line => '\t' + line).join('\n');
    const packet = `<<<ext\n${indentedJsonBody}\n>>>ext\n`;

    try {
        await hobotBridge.sendToAgent(packet);
        console.log(`[content.js] Sent ${directiveType}=${value} to Hobot.`);
    } catch (e) {
        console.warn(`[content.js] Failed to send ${directiveType}:`, e?.message || String(e));
    }
}   // _sendExtensionStateToHobot()

/**
 * _syncStatesToHobot()
 *
 * Назначение:
 * Отправить Хоботу актуальное состояние кнопок управления (execMode, osWriteMode).
 *
 * Используется после инициализации Хобота: за время задержки в _initializeHobot()
 * пользователь мог переключить кнопки, и состояние в init-пакете уже неактуально.
 *
 * # Side effects
 * - Отправляет два пакета <<<ext...>>>ext через hobotBridge.sendToAgent().
 */
async function _syncStatesToHobot() {

    const state = window.Globals.buttonState;
    if (!state) {
        console.warn("[content.js] _syncStatesToHobot: hobotState not available.");
        return;
    }

    const stepThrough = state.execMode !== window.Globals.execMode.AUTO;
    const osReadonly = state.osWriteMode !== window.Globals.osWriteMode.WRITE;

    await _sendExtensionStateToHobot("CHANGE_STEP_THROUGH", stepThrough);
console.log("CHANGE_STEP_THROUGH");
    await _sendExtensionStateToHobot("CHANGE_OS_READONLY", osReadonly);
console.log("CHANGE_OS_READONLY");
}   // _syncStatesToHobot()

//----------------------------------------------------------------------------------------------------------------
//                                      Слушатели
//----------------------------------------------------------------------------------------------------------------
/**
 * Слушаем изменения состояния "кнопок управления Хоботом" от background.
 *
 * При изменении состояния:
 * 1) Обновляем режим паузы (start/stop observer).
 * 2) Отправляем Хоботу директивы об изменении режимов (step_through, os_readonly).
 */
chrome.runtime.onMessage.addListener((msg) => {

    // Принимаем только сообщения о кнопках.
    if (msg?.type !== "HOBOT_STATE_CHANGED") return;

    const newState = msg.state;
    const oldState = window.Globals.buttonState;

    // 1) Сохраняем новое состояние в window.Globals.
    window.Globals.buttonState = newState;

    // 2) Применяем изменения execMode (паузы/работы).
    const isButtonStatePaused = newState.execMode === window.Globals.execMode.PAUSED;
    if (isButtonStatePaused !== isAgentPaused) {
        applyPauseState(isButtonStatePaused).then(() => {});
    }

    // 3) Отправляем директивы Хоботу для синхронизации его состояния. (Отправляем только если инфраструктура поднята
    // и Хобот работает).
    if (window.Globals.isHobotInitialized && hobotBridge) {

        // 3.1) При изменении кнопки исполнения, отсылаем новое состояние Хоботу.
        if (oldState?.execMode !== newState.execMode) {
            // Состояние кнопки изменилось. Отсылаем:
            // paused/step → step_through=true, auto → step_through=false
            const stepThrough = newState.execMode !== window.Globals.execMode.AUTO;
            _sendExtensionStateToHobot("CHANGE_STEP_THROUGH", stepThrough).catch(() => {});
        }   // if

        // 3.2) Изменение osWriteMode → CHANGE_OS_READONLY
        if (oldState?.osWriteMode !== newState.osWriteMode) {
            // read → os_readonly=true, write → os_readonly=false
            const osReadonly = newState.osWriteMode !== window.Globals.osWriteMode.WRITE;
            _sendExtensionStateToHobot("CHANGE_OS_READONLY", osReadonly).catch(() => {});
        }   // if
    }   // if
}); // chrome.runtime.onMessage.addListener

//--------------------------------------------------------------------------------------------------------------------
//                                  ОЧИСТКА РЕСУРСОВ ПРИ UNLOAD
//--------------------------------------------------------------------------------------------------------------------

// Удаляем все слушатели событий при выгрузке страницы
window.addEventListener('unload', () => {
    hobotBridge?.cleanup?.();
    textProcessor?.cleanup?.();
    focusManager?.cleanup?.();
});
