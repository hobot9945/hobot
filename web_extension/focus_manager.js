/**
 * focus_manager.js
 *
 * **ОПИСАНИЕ:**
 *
 * Модуль отвечает за удержание фокуса в поле ввода (textarea/div) во время работы агента.
 * Это критически важно, так как агент эмулирует нажатия клавиш. Если фокус пропадет
 * (пользователь кликнул мимо), текст уйдет в пустоту. Удержание фокуса включается только когда
 * window.Globals.getIsAgentBusy() == true (т.е. во время выполнения критичных операций/директив).
 *
 * **РЕШЕНИЕ**:
 *  1. Определение целевого элемента (куда писать).
 *  2. "Ловушка фокуса" (Focus Trap) — принудительный возврат курсора, если агент занят работой.
 *  3. Адаптивность — поддержка динамических интерфейсов (React/SPA), где элементы пересоздаются.
 *  4. Эвристика (Auto-capture) — мы "учимся" на клике пользователя, определяя поле ввода.
 *     Статические селекторы удалены для универсальности.
 *
 * **ОТВЕТСТВЕННОСТЬ**:
 *  1. Поиск активного поля ввода через перехват действий пользователя (Dynamic Capture).
 *  2. Мониторинг "здоровья" элемента (React может удалить поле и создать новое).
 *  3. Перехват события `blur` и принудительный возврат фокуса, если isAgentBusy=true.
 */

class FocusManager {
    constructor() {
        this.inputElement = null;       // Ссылка на текущий DOM-элемент ввода (textarea или div contenteditable).
        this.checkInterval = null;      // ID интервала для проверки состояния элемента ввода (наличие, фокус).

        // Биндинг контекста
        // Это необходимо, так как методы передаются как callback-функции в event listeners.
        // Без bind(this) контекст 'this' внутри методов будет указывать на DOM-элемент события, а не на наш класс.
        this._heartbeat = this._heartbeat.bind(this);
        this._handleBlur = this._handleBlur.bind(this);
        this._handleWindowFocus = this._handleWindowFocus.bind(this);
        this._checkAndSetNewAiInput = this._checkAndSetNewAiInput.bind(this);
    }

    /**
     * Инициализация менеджера.
     * Запускается из content.js один раз.
     */
    initialize() {

        // 1. Запускаем цикл проверки состояния элемента ввода (наличие, фокус).
        // Проверяем состояние каждые 500 мс. Это компромисс, слишком часто (100мс) = нагрузка на CPU при проверке DOM,
        // слишком редко (1000мс) = риск, что React успеет перерисовать поле и мы его потеряем.
        this.checkInterval = setInterval(this._heartbeat, 500);

        // 2. Слушаем фокус самого окна браузера. Когда пользователь возвращается в браузер, мы должны сразу проверить,
        // на месте ли фокус.
        window.addEventListener('focus', this._handleWindowFocus);

        // 3. Подсистема "Dynamic Capture" (Динамический захват окна ввода).
        // Мы слушаем действия пользователя.
        // 'true' (useCapture) означает перехват события на стадии погружения (до того, как оно дойдет до цели).
        // Это гарантирует, что мы узнаем о клике первыми.
        document.addEventListener('focusin', this._checkAndSetNewAiInput, true);
        document.addEventListener('click', this._checkAndSetNewAiInput, true);

        // При обновлении страницы, а значит подгрузке и инициализации расширения, курсор автоматически попадает в
        // поле ввода чата. Ловим элемент ввода.
        this._checkAndSetNewAiInput();

        console.log("[FocusManager] Initialized in Auto-Capture mode.");
    }

    /**
     *  Полная очистка ресурсов при выгрузке страницы или отключении расширения.
     *  Важно удалять слушатели, чтобы не вызывать утечки памяти (Memory Leaks).
     */
    cleanup() {
        if (this.checkInterval) clearInterval(this.checkInterval);
        this._detachListeners();    // Снимаем слушатели с конкретного inputElement

        // Удаляем глобальные слушатели
        window.removeEventListener('focus', this._handleWindowFocus);
        document.removeEventListener('focusin', this._checkAndSetNewAiInput, true);
        document.removeEventListener('click', this._checkAndSetNewAiInput, true);

        this.inputElement = null;
    }

    /**
     * Колбэк, вызываемый из glob.js при изменении статуса isAgentBusy.
     * Позволяет мгновенно захватить фокус при старте работы, не дожидаясь тика таймера.
     */
    onChangeAgentBusyState() {

        if (window.Globals.getIsAgentBusy()) {
            // Здесь мы только восстанавливаем фокус, если элемент УЖЕ известен.
            // Если inputElement === null, агент будет ждать, пока юзер кликнет в поле (Dynamic Capture).
            this._enforceFocus();
        }
    }

    /**
     * Периодическая задача, выполняется таймером каждые 500мс.
     * Проверяет валидность элемента и удерживает фокус.
     */
    _heartbeat() {
        // 1. Актуализация элемента (React мог его подменить)
        // В данном режиме мы просто проверяем, жив ли текущий элемент. Новых не ищем (ждем юзера).
        // Если элемент умер, this.inputElement останется, но isConnected будет false.

        // 2. Если агент работает, фокус ДОЛЖЕН быть в поле.
        if (window.Globals.getIsAgentBusy()) {
            this._enforceFocus();
        }
    }

    /**
     * Фиксация поля ввода, чтобы было куда возвращать фокус.
     *
     * Вызывается либо из конструктора (без параметра), либо из слушателей событий focusin, click. Кто первый поймает
     * поле ввода, тот и молодец. Самый желательный вариант - конструктор, так как это самый надежный вариант.
     * Мы смотрим, куда пользователь ставит фокус руками.
     */
    _checkAndSetNewAiInput(event) {

        // Если у нас УЖЕ есть рабочий элемент и он живой, мы игнорируем клики
        // (чтобы не переключаться случайно, если юзер кликнет в другое поле поиска на странице).
        // Если нужно разрешить смену поля на лету, эту строку можно закомментировать.
        if (this.inputElement && this.inputElement.isConnected) return;

        // Выделяем претендента на поле ввода.
        let target = null;
        if (!event) {
            // Без события, вызов из конструктора. Надеемся, что активный элемент - поле ввода.
            target = document.activeElement;
        } else if (event.target) {
            // Вызов был из слушателя, фиксируем элемент, на котором случилось событие.
            target = event.target;
        } else {
            // Событие без элемента: странно, но это не к нам, выходим.
            return;
        }

        // Проверяем, похоже ли то, куда кликнул юзер, на поле ввода чата.
        if (this._isValidInputField(target)) {
            // Бинго! Запоминаем этот элемент.
            this._setNewInput(target, "User Interaction (Auto-capture)");
        }
    }

    /**
     * Валидация элемента: годится ли он для ввода текста?
     * @param {HTMLElement} el
     */
    _isValidInputField(el) {
        if (!el) return false;
        const tag = el.tagName.toUpperCase();

        // 1. TEXTAREA — стандарт для 99% чат-ботов.
        if (tag === 'TEXTAREA') return true;

        // 2. contenteditable — используется в сложных редакторах (например, Notion, иногда ChatGPT).
        // Это div или span, в котором разрешено печатать.
        if (el.isContentEditable) return true;

        // 3. INPUT (text/search) — редкость для LLM, но возможно для поисковых строк.
        return tag === 'INPUT' && (el.type === 'text' || el.type === 'search');
    }

    /**
     * Устанавливает новый активный элемент ввода и перевешивает слушатели событий.
     * @param {HTMLElement} el - Новый элемент
     * @param {string} source - Откуда пришел вызов (для логов)
     */
    _setNewInput(el, source) {
        // Защита от лишних действий, если это тот же самый элемент
        if (this.inputElement === el) return;

        // Снимаем слушатели со "старого" (потерянного/удаленного) элемента
        this._detachListeners();

        this.inputElement = el;

        // Вешаем слушатели на "новый" элемент
        this._attachListeners();

        // Перенастроить менеджер поля ввода AI на новый элемент ввода.
        window.aiInputManager?.onInputElementChanged();

        // (Опционально) Логируем событие захвата
        console.log(`[FocusManager] Input element locked via [${source}]. Tag: ${el.tagName}`);
    }

    /**
     * Подписываемся на события конкретного inputElement.
     */
    _attachListeners() {
        if (!this.inputElement) return;
        // useCapture = true. Мы хотим ловить 'blur' (потерю фокуса) на самой ранней стадии.
        this.inputElement.addEventListener('blur', this._handleBlur, true);
    }

    /**
     * Отписываемся от событий (важно для Garbage Collection).
     */
    _detachListeners() {
        if (!this.inputElement) return;
        this.inputElement.removeEventListener('blur', this._handleBlur, true);
    }

    /**
     * Ключевой метод "Ловушка".
     * Срабатывает, когда браузер сообщает, что фокус уходит с нашего элемента.
     */
    _handleBlur() {
        // 1. Если агент отдыхает (не занят) — мы не мешаем пользователю убирать фокус.
        if (!window.Globals.getIsAgentBusy()) return;

        // 2. Проверка глобального фокуса (Alt+Tab).
        // Если document.hasFocus() == false, значит пользователь переключился в другое окно ОС (Telegram, IDE).
        // В этом случае бороться за фокус БЕСПОЛЕЗНО и вредно (браузер может начать мигать в панели задач).
        // Мы просто ждем, пока он вернется (_handleWindowFocus).
        if (!document.hasFocus()) return;

        // 3. Если мы здесь, значит пользователь кликнул КУДА-ТО ЕЩЕ ВНУТРИ СТРАНИЦЫ.
        // Например, на пустое место, кнопку лайка или скроллбар.

        // Используем setTimeout(0), чтобы выкинуть возврат фокуса в конец стека вызовов (Event Loop).
        // Это нужно, потому что браузер не может вернуть фокус синхронно ПРЯМО ВО ВРЕМЯ события blur.
        setTimeout(() => {
            // Двойная проверка: пока таймер тикал, агент мог закончить работу.
            if (window.Globals.getIsAgentBusy()) {
                this.inputElement?.focus();
            }
        }, 0);
    }

    /**
     * Обработчик возвращения пользователя на вкладку (после Alt+Tab).
     */
    _handleWindowFocus() {
        // Если агент все еще работает — немедленно возвращаем фокус в поле.
        if (window.Globals.getIsAgentBusy()) {
            this._enforceFocus();
        }
    }

    /**
     * Принудительная установка фокуса (утилитарный метод).
     */
    _enforceFocus() {
        // Если inputElement еще не определен, мы ничего не можем сделать.
        if (!this.inputElement || !this.inputElement.isConnected) return;

        // Проверяем activeElement, чтобы не спамить вызовами .focus() в каждом цикле,
        // если фокус и так уже там. Это предотвращает дрожание курсора.
        if (document.activeElement !== this.inputElement) {
            this.inputElement.focus();
        }
    }
}

// Экспорт в глобальную область видимости
window.FocusManager = FocusManager;
