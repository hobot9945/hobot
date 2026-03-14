/**
 * ai_input_manager.js
 *
 * ОТВЕТСТВЕННОСТЬ:
 * - Вычисление глобальной геометрии поля ввода AI в координатах виртуального рабочего стола Windows.
 * - Хранение последней успешно отправленной геометрии поля ввода.
 * - Сравнение новой геометрии с сохранённой.
 * - Формирование пакета сообщения для агента.
 * - Отправка новой геометрии агенту через HobotBridge.
 * - Подписка на DOM-события content-страницы, которые могут изменить геометрию поля ввода.
 */

class AiInputManager {

    /**
     * Создаёт менеджер геометрии поля ввода AI.
     */
    constructor(focusManager) {

        // Менеджер фокуса содержит актуальный элемент поля ввода AI.
        this._focusManager = focusManager;

        // Ссылка на транспортный мост расширения.
        // Через него геометрия будет отправляться агенту.
        this.hobotBridge = null;

        // Последняя геометрия, успешно отправленная агенту.
        // Используется для отсечения повторной отправки одинаковых координат.
        this.savedInputRect = null;

        // Текущий inputElement, на который повешены локальные слушатели.
        this._boundInputElement = null;

        // Наблюдатель за изменением размеров inputElement.
        this._inputResizeObserver = null;

        // Наблюдатель за перестройкой DOM страницы.
        this._pageMutationObserver = null;

        // Флаг: синхронизация уже запланирована, повторно не планируем.
        this._syncScheduled = false;

        // Биндинг обработчиков, чтобы их можно было безопасно передавать в слушатели.
        this._handleWindowResize = this._handleWindowResize.bind(this);
        this._handleVisualViewportResize = this._handleVisualViewportResize.bind(this);
        this._handleInputEvent = this._handleInputEvent.bind(this);
        this._handlePageMutations = this._handlePageMutations.bind(this);
    }   // constructor()

    /**
     * initialize()
     *
     * Назначение:
     * Подключить слушатели content-уровня, которые могут повлиять на геометрию поля ввода.
     *
     * Подключаются:
     * - resize окна;
     * - resize visual viewport;
     * - MutationObserver страницы;
     * - локальные слушатели текущего inputElement.
     *
     * # Побочные эффекты
     * - Подписывает менеджер на события окна и DOM.
     * - Подключает наблюдение за текущим inputElement, если он уже найден.
     */
    initialize() {
        // 1. Изменение размеров viewport страницы.
        window.addEventListener("resize", this._handleWindowResize);

        // 2. Изменение visual viewport.
        // Это полезно при zoom и некоторых браузерных режимах отображения.
        if (window.visualViewport) {
            window.visualViewport.addEventListener("resize", this._handleVisualViewportResize);
        }   // if

        // 3. Наблюдение за DOM страницы.
        // Нам важно замечать изменения layout, пересоздание поля ввода, появление боковых панелей и т.п.
        this._pageMutationObserver = new MutationObserver(this._handlePageMutations);

        if (document.body) {
            this._pageMutationObserver.observe(document.body, {
                childList: true,
                subtree: true,
                attributes: false,
                characterData: false
            });
        }   // if

        // 4. Подключаем локальные слушатели к текущему inputElement, если он уже есть.
        this._rebindInputElementListeners();

        console.log("[AiInputManager] Initialized.");
    }   // initialize()

    /**
     * cleanup()
     *
     * Назначение:
     * Снять все ранее подключённые слушатели и наблюдатели.
     *
     * # Побочные эффекты
     * - Освобождает ресурсы, связанные с подписками на DOM и window.
     */
    cleanup() {
        // 1. Снять глобальные слушатели окна.
        window.removeEventListener("resize", this._handleWindowResize);

        if (window.visualViewport) {
            window.visualViewport.removeEventListener("resize", this._handleVisualViewportResize);
        }   // if

        // 2. Остановить наблюдение за страницей.
        if (this._pageMutationObserver) {
            try {
                this._pageMutationObserver.disconnect();
            } catch (_) {}
            this._pageMutationObserver = null;
        }   // if

        // 3. Снять слушатели и observer с текущего inputElement.
        this._detachInputElementListeners();

        // 4. Сбросить флаг планирования.
        this._syncScheduled = false;

        console.log("[AiInputManager] Cleanup complete.");
    }   // cleanup()

    /**
     * onInputElementChanged()
     *
     * Назначение:
     * Сообщить менеджеру, что FocusManager сменил текущий inputElement.
     *
     * Алгоритм:
     * 1. Перевесить локальные слушатели на новый inputElement.
     * 2. Запланировать актуализацию геометрии.
     *
     * # Побочные эффекты
     * - Снимает слушатели со старого inputElement.
     * - Вешает слушатели на новый inputElement.
     * - Планирует synchronizeInputRect().
     */
    onInputElementChanged() {
        this._rebindInputElementListeners();
        this._scheduleSynchronizeInputRect("input_element_changed");
    }   // onInputElementChanged()

    /**
     * handleGeometryRecheckRequest()
     *
     * Назначение:
     * Обработать внешний запрос на перепроверку геометрии поля ввода AI.
     *
     * Метод вызывается из content.js в ответ на сообщение от background.js.
     * Сам пересчёт выполняется не напрямую, а через уже существующий внутренний
     * debounce-механизм _scheduleSynchronizeInputRect().
     *
     * # Возвращаемое значение
     * Тип: boolean
     * true  - запрос принят в обработку.
     * false - запрос проигнорирован, например для свернутого окна.
     *
     * @param {string} reason - Причина перепроверки геометрии.
     * @param {string|null} windowState - Состояние окна браузера.
     */
    handleGeometryRecheckRequest(reason = "unknown", windowState = null) {

        // Для свернутого окна экранная геометрия временно не имеет практического смысла.
        if (windowState === "minimized") {
            return false;
        }   // if

        this._scheduleSynchronizeInputRect(reason);
        return true;
    }   // handleGeometryRecheckRequest()

    /**
     * _rebindInputElementListeners()
     *
     * Назначение:
     * Перевесить локальные слушатели с прежнего inputElement на текущий inputElement из FocusManager.
     *
     * Локальные слушатели:
     * - событие input;
     * - ResizeObserver.
     */
    _rebindInputElementListeners() {
        const currentInputElement = this._focusManager?.inputElement || null;

        // Если это тот же самый элемент, перевешивать ничего не нужно.
        if (this._boundInputElement === currentInputElement) {
            return;
        }   // if

        // Снимаем слушатели со старого элемента.
        this._detachInputElementListeners();

        // Если нового элемента пока нет, просто выходим.
        if (!currentInputElement || !currentInputElement.isConnected) {
            this._boundInputElement = null;
            return;
        }   // if

        // Запоминаем новый текущий элемент.
        this._boundInputElement = currentInputElement;

        // Подписываемся на ввод текста.
        // Это полезно, если textarea растёт по высоте в процессе ввода.
        this._boundInputElement.addEventListener("input", this._handleInputEvent);

        // Подписываемся на изменение размеров самого textarea.
        this._inputResizeObserver = new ResizeObserver(() => {
            this._scheduleSynchronizeInputRect("input_resize");
        });

        this._inputResizeObserver.observe(this._boundInputElement);

        console.log("[AiInputManager] Input listeners rebound.");
    }   // _rebindInputElementListeners()

    /**
     * _detachInputElementListeners()
     *
     * Назначение:
     * Снять локальные слушатели и observer с текущего inputElement.
     */
    _detachInputElementListeners() {
        if (this._boundInputElement) {
            this._boundInputElement.removeEventListener("input", this._handleInputEvent);
        }   // if

        if (this._inputResizeObserver) {
            try {
                this._inputResizeObserver.disconnect();
            } catch (_) {}
            this._inputResizeObserver = null;
        }   // if

        this._boundInputElement = null;
    }   // _detachInputElementListeners()

    /**
     * _scheduleSynchronizeInputRect()
     *
     * Назначение:
     * Запланировать одну отложенную синхронизацию геометрии поля ввода.
     *
     * Почему не вызываем synchronizeInputRect() сразу:
     * - resize и mutation могут приходить пачками;
     * - нам не нужен шквал одинаковых сообщений агенту;
     * - достаточно одной синхронизации на ближайший кадр.
     *
     * @param {string} reason - Причина планирования (для логов).
     */
    _scheduleSynchronizeInputRect(reason) {

        if (this._syncScheduled) {
            return;
        }   // if

        this._syncScheduled = true;

        requestAnimationFrame(async () => {
            this._syncScheduled = false;

            try {
                await this.synchronizeInputRect();
            } catch (e) {
                console.warn(
                    `[AiInputManager] synchronizeInputRect failed after reason='${reason}':`,
                    e?.message || String(e)
                );
            }
        });
    }   // _scheduleSynchronizeInputRect()

    /**
     * _handleWindowResize()
     *
     * Назначение:
     * Обработать resize окна страницы.
     */
    _handleWindowResize() {
        this._scheduleSynchronizeInputRect("window_resize");
    }   // _handleWindowResize()

    /**
     * _handleVisualViewportResize()
     *
     * Назначение:
     * Обработать resize visual viewport.
     */
    _handleVisualViewportResize() {
        this._scheduleSynchronizeInputRect("visual_viewport_resize");
    }   // _handleVisualViewportResize()

    /**
     * _handleInputEvent()
     *
     * Назначение:
     * Обработать событие input на textarea.
     *
     * Важно:
     * Событие input не гарантирует изменение геометрии, но часто совпадает
     * с autoresize поля ввода, поэтому является полезным сигналом.
     */
    _handleInputEvent() {
        this._scheduleSynchronizeInputRect("input_event");
    }   // _handleInputEvent()

    /**
     * _handlePageMutations()
     *
     * Назначение:
     * Обработать перестройку DOM страницы.
     *
     * Важно:
     * MutationObserver может срабатывать очень часто.
     * Поэтому здесь мы ничего не вычисляем напрямую, а только:
     * 1. Перевешиваем слушатели, если FocusManager уже переключился на новый inputElement.
     * 2. Планируем одну отложенную синхронизацию.
     */
    _handlePageMutations() {
        this._rebindInputElementListeners();
        this._scheduleSynchronizeInputRect("page_mutation");
    }   // _handlePageMutations()

    /**
     * setHobotBridge()
     *
     * Назначение:
     * Установить ссылку на HobotBridge, через который менеджер будет отправлять
     * пакеты агенту.
     *
     * # Параметры
     * @param {Object|null} hobotBridge - Экземпляр HobotBridge или null для сброса ссылки.
     */
    setHobotBridge(hobotBridge) {
        this.hobotBridge = hobotBridge;
    }   // setHobotBridge()

    /**
     * getSavedInputRect()
     *
     * Назначение:
     * Вернуть последнюю сохранённую геометрию поля ввода.
     *
     * Возвращается копия, чтобы внешний код не мог случайно изменить внутреннее состояние
     * менеджера.
     *
     * # Возвращаемое значение
     * Тип: { x: number, y: number, width: number, height: number } | null
     */
    getSavedInputRect() {
        return this._cloneRect(this.savedInputRect);
    }   // getSavedInputRect()

    /**
     * resetSavedInputRect()
     *
     * Назначение:
     * Сбросить сохранённую геометрию.
     *
     * Полезно, если нужно принудительно считать следующее изменение "новым" и повторно
     * отправить геометрию агенту.
     */
    resetSavedInputRect() {
        this.savedInputRect = null;
    }   // resetSavedInputRect()

    /**
     * synchronizeInputRect()
     *
     * Назначение:
     * Актуализировать геометрию поля ввода для агента и здесь, в this.savedInputRect.
     *
     * Алгоритм:
     * 0. Проверить установлен ли this.hobotBridge, он устанавливается сразу после отправки пакета инициализации Хоботу
     *    из HobotBridge._initializeHobot. Без этого актуализация не имеет смысла.
     * 1. Вычислить текущую геометрию переданного элемента.
     * 2. Сравнить её с последней сохранённой геометрией.
     * 3. Если геометрия не изменилась — ничего не делать.
     * 4. Если геометрия изменилась — сформировать пакет и отправить его агенту.
     * 5. После успешной отправки сохранить новую геометрию в savedInputRect.
     *
     * # Возвращаемое значение
     * Тип: Promise<boolean>
     * true  - геометрия изменилась и была успешно отправлена агенту.
     * false - геометрия не изменилась, не была вычислена или не была отправлена.
     *
     * # Побочные эффекты
     * - Отправляет пакет агенту через HobotBridge.sendToAgent().
     * - Обновляет this.savedInputRect только после успешной отправки.
     *
     */
    async synchronizeInputRect() {

        // Без транспортного моста отправка невозможна.
        if (!this.hobotBridge) {
            // HobotBridge не установлен. Отправка геометрии невозможна.
            return false;
        }   // if

        // Вычисляем текущую геометрию поля ввода.
        const currentRect = this.getAiInputRect();
        if (!currentRect) {
            // Поле ввода не найдено или недоступно.
            return false;
        }   // if

        // Если координаты не изменились, повторно ничего не отправляем.
        if (this._isSameRect(this.savedInputRect, currentRect)) {
            return false;
        }   // if

        // Формируем текстовый пакет в формате <<<ext ... >>>ext.
        const packet = this._buildGeometryUpdatePacket(currentRect);

        try {
            // Отправляем пакет агенту.
            await this.hobotBridge.sendToAgent(packet);

            // Геометрию сохраняем только после успешной отправки.
            // Это важно: если отправка упала, при следующем событии будет повторная попытка.
            this.savedInputRect = this._cloneRect(currentRect);

            // console.log("[AiInputManager] Геометрия поля ввода отправлена агенту:", currentRect);
            return true;
        } catch (e) {
            console.error("[AiInputManager] Не удалось отправить геометрию поля ввода агенту:", e?.message || String(e));
            return false;
        }   // try/catch
    }   // synchronizeInputRect()

    /**
     * Вычисляет глобальные координаты поля ввода AI в физических пикселях виртуального рабочего стола Windows.
     * Используется локально, при актуализации геометрии, либо извне при инициализации сессии Хобота.
     *
     * Алгоритм:
     * 1. Получаем геометрию элемента в CSS-пикселях.
     * 2. Переводим DOM-координаты и размеры viewport в физические пиксели через devicePixelRatio.
     * 3. Вычисляем толщину левой/правой рамки окна браузера в физических пикселях.
     * 4. Вычисляем высоту верхней части окна браузера в физических пикселях.
     * 5. Складываем координаты окна на экране и координаты элемента внутри viewport.
     *
     * # Возвращаемое значение
     * Тип: { x: number, y: number, width: number, height: number } | null
     *
     * Побочные эффекты: this._focusManager.inputElement - поле ввода AI нужно для расчета его геометрии.
     */
    getAiInputRect() {
        if (!this._focusManager.inputElement || !this._focusManager.inputElement.isConnected) {
            console.error("[getAiInputRect] Элемент не найден или не привязан к DOM.");
            return null;
        }   // if

        // Геометрия элемента внутри viewport в CSS-пикселях.
        const rect = this._focusManager.inputElement.getBoundingClientRect();

        // Коэффициент перевода CSS-пикселей в физические пиксели.
        const dpr = window.devicePixelRatio || 1;

        // Переводим координаты и размеры элемента в физические пиксели.
        const rectLeftPx = rect.left * dpr;
        const rectTopPx = rect.top * dpr;
        const rectWidthPx = rect.width * dpr;
        const rectHeightPx = rect.height * dpr;

        // Переводим размеры viewport в физические пиксели.
        const innerWidthPx = window.innerWidth * dpr;
        const innerHeightPx = window.innerHeight * dpr;

        // Внешние размеры окна браузера уже находятся в физических пикселях.
        const outerWidthPx = window.outerWidth;
        const outerHeightPx = window.outerHeight;

        // Оценка толщины боковых рамок окна браузера.
        // Для максимизированного окна здесь обычно будет 0.
        const horizontalFramePx = Math.max(0, outerWidthPx - innerWidthPx);
        const frameLeftPx = horizontalFramePx / 2;

        // Оценка высоты верхней части окна браузера:
        // вкладки, адресная строка, панель закладок и верхняя рамка.
        const frameTopPx = Math.max(0, outerHeightPx - innerHeightPx - frameLeftPx);

        // screenX/screenY — положение внешнего окна браузера на виртуальном экране.
        const globalX = window.screenX + frameLeftPx + rectLeftPx;
        const globalY = window.screenY + frameTopPx + rectTopPx;

        return {
            x: Math.round(globalX),
            y: Math.round(globalY),
            width: Math.round(rectWidthPx),
            height: Math.round(rectHeightPx)
        };
    }   // getAiInputRect()

    /**
     * _buildGeometryUpdatePacket()
     *
     * Назначение:
     * Сформировать пакет для агента в формате расширения.
     *
     * Формат пакета:
     * <<<ext
     *     {
     *         "type": "AI_INPUT_GEOMETRY_UPDATE",
     *         "data": {
     *             "x": ...,
     *             "y": ...,
     *             "width": ...,
     *             "height": ...
     *         }
     *     }
     * >>>ext
     *
     * # Возвращаемое значение
     * Тип: string
     *
     * @param {{ x: number, y: number, width: number, height: number }} inputRect - Геометрия поля ввода.
     */
    _buildGeometryUpdatePacket(inputRect) {
        const payload = {
            type: "AI_INPUT_GEOMETRY_UPDATE",
            ai_input_rect: this._cloneRect(inputRect)
        };

        // Формируем JSON с отступами для читаемости в логах и отладке.
        const jsonBody = JSON.stringify(payload, null, 4);

        // Внутри <<<ext ... >>>ext принята дополнительная табуляция.
        const indentedJsonBody = jsonBody
            .split("\n")
            .map((line) => "\t" + line)
            .join("\n");

        return `<<<ext\n${indentedJsonBody}\n>>>ext\n`;
    }   // _buildGeometryUpdatePacket()

    /**
     * _isSameRect()
     *
     * Назначение:
     * Проверить, совпадают ли две геометрии поля ввода.
     *
     * Сравнение делается строго по четырём координатам.
     *
     * # Возвращаемое значение
     * Тип: boolean
     *
     * @param {{ x: number, y: number, width: number, height: number }|null} leftRect
     * @param {{ x: number, y: number, width: number, height: number }|null} rightRect
     */
    _isSameRect(leftRect, rightRect) {
        // Если хотя бы одной геометрии нет, считать их одинаковыми нельзя.
        if (!leftRect || !rightRect) {
            return false;
        }   // if

        return (
            leftRect.x === rightRect.x &&
            leftRect.y === rightRect.y &&
            leftRect.width === rightRect.width &&
            leftRect.height === rightRect.height
        );
    }   // _isSameRect()

    /**
     * _cloneRect()
     *
     * Назначение:
     * Создать независимую копию геометрии поля ввода.
     *
     * Это защищает внутреннее состояние менеджера от случайного изменения снаружи.
     *
     * # Возвращаемое значение
     * Тип: { x: number, y: number, width: number, height: number } | null
     *
     * @param {{ x: number, y: number, width: number, height: number }|null} rect
     */
    _cloneRect(rect) {
        if (!rect) {
            return null;
        }   // if

        return {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height
        };
    }   // _cloneRect()
}   // InputFieldGeometry

// Экспорт в глобальную область для использования в content-скриптах
window.AiInputManager = AiInputManager;
