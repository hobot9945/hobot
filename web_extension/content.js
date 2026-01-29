// content.js

// Глобальные ссылки на экземпляры модулей.
// Они понадобятся в следующих частях файла (pause-state, cleanup и т.д.).
let hobotBridge = null;
let directiveExtractor = null;
let textProcessor = null;
let focusManager = null;

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

    // 3) Экстрактор директив. Получает транспорт как зависимость.
    directiveExtractor = new window.DirectiveExtractor(hobotBridge);

    // 4) TextProcessor инициализируется сразу (создаём MutationObserver), но старт наблюдения включается только
    // в режиме "РАБОТА". TextProcessor больше не пытается собирать "весь текст страницы".
    // Он извлекает только code-block контейнеры (<pre>) и отправляет их текст в DirectiveExtractor.
    textProcessor = new window.TextProcessor(directiveExtractor);
    textProcessor.initializeObserver();

}   // _prepareContentObjects()

// --- УПРАВЛЕНИЕ СОСТОЯНИЕМ (ПАУЗА / РАБОТА) ---

/**
 * applyPauseState()
 *
 * Назначение:
 * Управляет состоянием контентной части расширения “Пауза / Работа”.
 *
 * В режиме ПАУЗА:
 * - Останавливает MutationObserver, чтобы не читать/парсить текст страницы.
 *
 * В режиме РАБОТА:
 * - Гарантирует, что инфраструктура (bridge/extractor/processor/focus) поднята.
 * - Запускает MutationObserver для слежения за code-block (<pre>) и извлечения директив.
 * Алгоритм:
 * - Если isPaused=true:
 *   - Остановить перехват текста (если был запущен) и выйти.
 * - Если isPaused=false:
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
 * @param {boolean} isPaused - true, если агент на паузе.
 */
async function applyPauseState(isPaused) {

    // 1) ПАУЗА: перехват текста запрещён, observer выключаем и выходим.
    if (isPaused) {
        textProcessor?.stopObserver();
        return;
    }

    // 2) РАБОТА: Если это первый запуск, инфраструктура должна быть поднята (bridge/extractor/processor/focus).
    if (!window.Globals.isHobotInitialized) {
        try {
            // Поднимаем контентные объекты и выполняем инициализацию транспорта.
            await _prepareContentObjects();

            // Фиксируем, что инициализация выполнена (чтобы не делать её повторно).
            window.Globals.isHobotInitialized = true;
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
} // applyPauseState()

/**
 * Инициализация состояния "Пауза / Работа" при загрузке страницы.
 *
 * Описание:
 * - Читает сохранённый флаг hobotPaused из chrome.storage.local.
 * - Если ключ отсутствует (первый запуск) — считает, что режим "Пауза" включён.
 * - Применяет состояние через applyPauseState().
 *
 * Алгоритм работы:
 * - Запросить hobotPaused из storage.
 * - Нормализовать значение (по умолчанию true).
 * - Вызвать applyPauseState(isPaused).
 *
 * # Side effects
 * - Асинхронно читает chrome.storage.local.
 * - Запускает/останавливает перехват текста через applyPauseState().
 */
(async () => {
    const result = await new Promise(resolve => {
        chrome.storage.local.get(['hobotPaused'], resolve);
    });

    const isPaused = result.hobotPaused !== undefined ? result.hobotPaused : true;
    await applyPauseState(isPaused);
})(); // IIFE init pause state

/**
 * Слушатель изменений состояния "Пауза / Работа" в реальном времени (от popup).
 *
 * Описание:
 * - Реагирует на изменения ключа hobotPaused в chrome.storage.
 * - Вызывает applyPauseState() с новым значением.
 *
 * # Side effects
 * - Подписывается на chrome.storage.onChanged.
 * - Асинхронно запускает applyPauseState() при изменениях.
 */
chrome.storage.onChanged.addListener((changes) => {
    if (changes.hobotPaused !== undefined) {
        (async () => {
            await applyPauseState(changes.hobotPaused.newValue);
        })(); // IIFE apply pause state change
    }
}); // chrome.storage.onChanged.addListener

// --- ОЧИСТКА РЕСУРСОВ ПРИ UNLOAD ---

// Удаляем все слушатели событий при выгрузке страницы
window.addEventListener('unload', () => {
    hobotBridge?.cleanup?.();
    textProcessor?.cleanup?.();
    focusManager?.cleanup?.();
});
