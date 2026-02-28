// content.js

// Глобальные ссылки на экземпляры модулей.
// Они понадобятся в следующих частях файла (pause-state, cleanup и т.д.).
let hobotBridge = null;
let directiveExtractor = null;
let textProcessor = null;

// FocusManager: удержание фокуса. Вызывается первым, до инициализации всего остального. Нужно потому что при загрузке
// страницы поле ввода сфокусировано автоматически, а в конструкторе это поле зафиксируется.
let focusManager = new window.FocusManager();
focusManager.initialize();
window.focusManager = focusManager;

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

/**
 * _handleAgentDeath()
 *
 * Назначение:
 * Финализация работы при критической ошибке или дисконнекте Хобота.
 */
function _handleAgentDeath(msg) {
    console.error("[content.js] Получен сигнал дисконнекта агента от моста. Остановка работы с агентом.", msg);

    // 1) Останавливаем перехват текста.
    textProcessor?.stopObserver();

    // 2) Сбрасываем флаги.
    window.Globals.isHobotInitialized = false;
    isAgentPaused = true;

    // 3) Просим background «погасить» иконку.
    chrome.runtime.sendMessage({ type: "HOBOT_DEATH_UI_RESET" }).catch(() => {});
}

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

/**
 * Слушаем проксированное сообщение от HobotBridge о потере связи с агентом.
 */
window.addEventListener("hobot-bridge:HOBOT_DISCONNECTED", (event) => {
    _handleAgentDeath(event.detail.error);
});

//--------------------------------------------------------------------------------------------------------------------
//                                  ОЧИСТКА РЕСУРСОВ ПРИ UNLOAD
//--------------------------------------------------------------------------------------------------------------------

// Удаляем все слушатели событий при выгрузке страницы
window.addEventListener('unload', () => {
    hobotBridge?.cleanup?.();
    textProcessor?.cleanup?.();
    focusManager?.cleanup?.();
});

// //--------------------------------------------------------------------------------------------------------------------
// //                                  Тест эмуляции drag and drop
// //--------------------------------------------------------------------------------------------------------------------
//
// /**
//  * testFileDrop()
//  *
//  * Назначение:
//  * PoC — проверка программного drop/paste файла в поле ввода AI-чата.
//  *
//  * Для теста используется минимальный PDF, созданный в памяти.
//  * В продакшене данные файла будут приходить от Хобота через Native Messaging (base64).
//  *
//  * Алгоритм:
//  * 1. Получить целевой элемент (поле ввода, зафиксированное FocusManager).
//  * 2. Создать File-объект с тестовым содержимым.
//  * 3. Обернуть в DataTransfer.
//  * 4. Попробовать три способа доставки:
//  *    a) drop — цепочка dragenter → dragover → drop (на input и до 5 предков).
//  *    b) paste — ClipboardEvent с файлом.
//  *    c) hidden file input — программная подстановка в <input type="file">.
//  *
//  * Ограничения:
//  * - Синтетические события имеют event.isTrusted === false.
//  *   Некоторые сайты проверяют этот флаг и игнорируют "ненастоящие" события.
//  *   Если ни один способ не сработает, потребуется CDP.
//  * - DataTransfer в синтетических событиях может быть ограничен браузером.
//  *
//  * # Вызов
//  * Из DevTools: testFileDrop()
//  * Из content.js: await testFileDrop()
//  */
// async function testFileDrop() {
//
//     // ---------------------------------------------------------------
//     // 1. Целевой элемент
//     // ---------------------------------------------------------------
//     // FocusManager к этому моменту должен был захватить поле ввода.
//     // Если его нет — тест невозможен, пользователь должен кликнуть в поле чата.
//     const inputEl = window.focusManager?.inputElement;
//     if (!inputEl || !inputEl.isConnected) {
//         console.error("[testFileDrop] ❌ Поле ввода не найдено.",
//             "Кликни в поле ввода чата, чтобы FocusManager его захватил.");
//         return;
//     }
//     console.log(`[testFileDrop] 🎯 Цель: <${inputEl.tagName}>, class="${inputEl.className}"`);
//
//     // ---------------------------------------------------------------
//     // 2. Создание тестового PDF-файла
//     // ---------------------------------------------------------------
//     // Минимальный валидный PDF (1 пустая страница, ~300 байт).
//     // Для PoC содержимое не критично — чат отправит файл на сервер.
//     // Главное: MIME-тип и расширение файла.
//     const file = _createTestPdf();
//     console.log(`[testFileDrop] 📄 ${file.name}, ${file.size} байт`);
//
//     // ---------------------------------------------------------------
//     // 3. DataTransfer — контейнер для передачи файла через события
//     // ---------------------------------------------------------------
//     // DataTransfer — стандартный браузерный объект, через который
//     // drag-and-drop и clipboard передают данные обработчикам.
//     const dt = new DataTransfer();
//     dt.items.add(file);
//     let dropAccepted = false;
//
//     // ---------------------------------------------------------------
//     // 4c. Способ C: Скрытый <input type="file">
//     // ---------------------------------------------------------------
//     // Многие сайты используют невидимый <input type="file"> для загрузки.
//     // Мы можем программно подставить в него файл и вызвать событие change.
//     // Это самый надёжный способ, если такой input существует на странице.
//     if (!dropAccepted) {
//         console.log("[testFileDrop] --- Способ C: Hidden <input type='file'> ---");
//         const fileInputs = document.querySelectorAll('input[type="file"]');
//
//         if (fileInputs.length > 0) {
//             for (const fi of fileInputs) {
//                 // Создаём ОТДЕЛЬНЫЙ DataTransfer для каждого input
//                 // (браузер может сбросить files после присвоения).
//                 const fdt = new DataTransfer();
//                 fdt.items.add(file);
//
//                 // Программная подстановка файла в input.
//                 // fi.files — сеттер, принимает FileList. DataTransfer.files — это FileList.
//                 fi.files = fdt.files;
//
//                 // Диспатчим change — именно на это событие обычно подписан фреймворк.
//                 fi.dispatchEvent(new Event('change', { bubbles: true }));
//                 console.log(`[testFileDrop] ✅ Файл подставлен в <input type="file">`);
//                 dropAccepted = true;
//             }
//         } else {
//             console.log("[testFileDrop] <input type='file'> не найден на странице.");
//         }
//     }
//
//     // ---------------------------------------------------------------
//     // 4a. Способ A: Drop
//     // ---------------------------------------------------------------
//     // Полная цепочка: dragenter → dragover → drop.
//     // Без dragenter и dragover фреймворки (React, Vue) часто игнорируют drop.
//     //
//     // Пробуем на самом inputEl и поднимаемся к предкам: SPA-фреймворки
//     // нередко вешают обработчик drop на контейнер-обёртку, а не на само поле.
//     //
//     // dispatchEvent() возвращает true, если preventDefault() НЕ вызван.
//     // Если вызван — значит, обработчик принял событие и обработал файл.
//     console.log("[testFileDrop] --- Способ A: Drop ---");
//     let el = inputEl;
//
//     for (let depth = 0; depth < 6 && el && el !== document.body; depth++) {
//         const tag = `<${el.tagName}> .${el.className || '(no class)'}`;
//
//         // Общие свойства для всех событий цепочки:
//         // bubbles: true    — событие всплывает вверх по DOM (критично для React).
//         // cancelable: true — обработчик сможет вызвать preventDefault().
//         // dataTransfer     — наш контейнер с файлом.
//         const props = { bubbles: true, cancelable: true, dataTransfer: dt };
//
//         el.dispatchEvent(new DragEvent('dragenter', props));  // "файл вошёл в зону"
//         el.dispatchEvent(new DragEvent('dragover', props));   // "файл висит над зоной"
//         const prevented = !el.dispatchEvent(new DragEvent('drop', props)); // "файл брошен"
//
//         if (prevented) {
//             console.log(`[testFileDrop] ✅ Drop принят: ${tag}`);
//             // dropAccepted = true;
//
//             // Отправляем глобальные события "мышка ушла", чтобы попытаться
//             // сбросить внутренний стейт библиотеки корректно
//             document.documentElement.dispatchEvent(new DragEvent('dragleave', { bubbles: true }));
//             window.dispatchEvent(new DragEvent('dragleave', { bubbles: true }));
//
//             // --- ТОЧЕЧНАЯ ОЧИСТКА ОВЕРЛЕЯ С ЗАДЕРЖКОЙ ---
//             // Даем React 150 миллисекунд на перерисовку, а затем убиваем узел
//             setTimeout(() => {
//                 const pTags = document.querySelectorAll('p');
//                 for (const p of pTags) {
//                     if (p.textContent.trim() === 'Drop files here') {
//                         const overlay = p.closest('.absolute');
//                         if (overlay) {
//                             console.log('[testFileDrop] Точечно удаляем оверлей после задержки:', overlay);
//                             overlay.remove();
//                         }
//                         break;
//                     }
//                 }
//             }, 150);
//
//             break;  // Дальше не пробуем, чтобы не дублировать загрузку.
//         } else {
//             console.log(`[testFileDrop]    ${tag} — не принял`);
//         }
//
//         el = el.parentElement;
//     }
//
//     // ---------------------------------------------------------------
//     // 5. Итог
//     // ---------------------------------------------------------------
//     console.log("[testFileDrop] 🏁 Тест завершён. Проверь UI чата визуально:");
//     console.log("  - Появилось ли превью/иконка файла?");
//     console.log("  - Появился ли индикатор загрузки?");
// }
//
// /**
//  * _createTestPdf()
//  *
//  * Назначение:
//  * Создать минимальный PDF-файл в памяти (1 пустая страница).
//  * Заглушка для PoC. В продакшене вместо этого будут реальные данные от Хобота.
//  *
//  * @returns {File}
//  */
// function _createTestPdf() {
//
//     const content = [
//         "%PDF-1.0",
//         "1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj",
//         "2 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj",
//         "3 0 obj<</Type/Page/MediaBox[0 0 612 792]/Parent 2 0 R>>endobj",
//         "xref", "0 4",
//         "0000000000 65535 f ",
//         "0000000009 00000 n ",
//         "0000000058 00000 n ",
//         "0000000115 00000 n ",
//         "trailer<</Size 4/Root 1 0 R>>",
//         "startxref", "190", "%%EOF"
//     ].join("\n");
//
//     return new File([content], "user_guide.pdf", {
//         type: "application/pdf",
//         lastModified: Date.now()
//     });
// }
//
// // Экспорт в глобальную область для вызова из DevTools.
// window.testFileDrop = testFileDrop;