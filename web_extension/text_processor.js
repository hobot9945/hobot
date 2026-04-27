/**
 * text_processor.js
 *
 * ОПИСАНИЕ:
 * Перехват изменений DOM и извлечение code-block контейнеров (<pre> или <div>) для поиска директив.
 *
 * Стратегия поиска:
 * - Директивы появляются внутри fenced code block (бэктики), которые сайты оборачивают в <pre> или <div>.
 * - В начале работы тип сайта неизвестен (UNKNOWN). Ищем сначала в <pre>, при неудаче — в <div>.
 * - После первого успешного выделения директивы тип фиксируется и поиск ведётся только по одной ветке.
 *
 * Оптимизация:
 * - innerText — дорогая операция (вызывает reflow). Чтобы не дёргать её на каждую мутацию,
 *   используем двухуровневую детекцию:
 *   1) Лёгкий «сигнальный» поиск: проверяем textContent/data мутировавших узлов на наличие
 *      закрывающей скобки ">>>" или её части (завершающий ">").
 *   2) Тяжёлый «подтверждающий» поиск: только при обнаружении намёка берём innerText контейнера.
 *
 * Флаг isDirectiveCaught:
 * - Поднимается, когда в innerText контейнера обнаружена полноценная ">>>ai".
 * - Пока поднят, мутации внутри пойманного контейнера провоцируют повторное чтение innerText
 *   (ловим метаданные после скобки). Мутации вне контейнера сбрасывают флаг и обрабатываются
 *   в обычном режиме.
 * - Сбрасывается при false positive (>>>ai исчезла из innerText), при мутации вне контейнера
 *   или при успешном помещении директивы в шлих (из directive_extractor._moveLastDirectiveToSchlich).
 *
 * Взаимодействие:
 * - Передаёт текст (innerText) в dirExtractor.acceptWebOutput().
 */

class TextProcessor {
    constructor(dirExtractor) {
        this.dirExtractor = dirExtractor;  // Интерфейс к DirectiveExtractor
        this.observer = null;
        this.isObserverStarted = false;

        // Флаг "директива поймана". Поднимается при обнаружении ">>>ai" в innerText контейнера.
        // Пока поднят, мутации внутри пойманного контейнера провоцируют повторное чтение innerText.
        // Мутации вне контейнера сбрасывают флаг.
        // Также сбрасывается из directive_extractor при успешном помещении в шлих.
        this.isDirectiveCaught = false;

        // Ссылка на контейнер, в котором поймана директива. Нужна, чтобы при поднятом флаге
        // isDirectiveCaught отличать "свои" мутации от "чужих".
        this._caughtContainer = null;

        // --- Переменные управления таймером молчания ---
        this.wasThereSilence = true;    // флаг тишины

    }

    /**
     * isGarbageNode()
     *
     * Фильтр элементов.
     *
     * Назначение:
     * - Отсеять зону ввода пользователя (чтобы не ловить собственный ввод),
     *   aria-hidden узлы и технические теги.
     *
     * Важно:
     * - НЕ читаем innerText (дорого и нестабильно).
     *
     * @param {Node} node - DOM-узел для проверки.
     * @returns {boolean} true = мусор, пропускаем.
     */
    isGarbageNode(node) {

        if (!node || node.nodeType !== Node.ELEMENT_NODE) return false;

        // --- 1) Игнорируем зону ввода пользователя ---

        // 1.1) Если FocusManager уже захватил поле ввода — отсекаем всё внутри него
        const inputEl = window.focusManager?.inputElement;
        if (inputEl) {
            if (node === inputEl || inputEl.contains(node) || node.contains(inputEl)) {
                return true;
            }
        }

        // 1.2) Универсальный отсев для contenteditable
        if (node.isContentEditable) return true;

        // 1.3) Фолбэк для "редакторов", которые размечают ввод через атрибуты/ARIA
        if (node.closest?.('[contenteditable="true"],[contenteditable="plaintext-only"]')) {
            return true;
        }

        // --- 2) Скрытые элементы ---
        if (node.getAttribute('aria-hidden') === 'true') return true;

        // --- 3) Технические теги ---
        const tagName = node.tagName.toUpperCase();
        const ignoredTags = [
            'SCRIPT', 'STYLE', 'NOSCRIPT', 'SVG', 'IMG',
            'BUTTON', 'INPUT', 'TEXTAREA', 'SELECT',
            'VIDEO', 'AUDIO', 'IFRAME', 'CANVAS', 'LINK', 'META',
            'BLOCKQUOTE'
        ];

        // --- 4) Блоки-размышления (GLM и аналоги) ---
        // Проверяем всю цепочку предков: если узел вложен в <blockquote>, считаем мусором (сайт https://chat.z.ai/)
        if (node.closest?.('blockquote')) return true;

        if (ignoredTags.includes(tagName)) return true;

        return false;
    }   // isGarbageNode()

    /**
     * _hasCloseHint()
     *
     * Назначение:
     * Проверить текстовую строку на наличие "намёка" закрывающей скобки ">>>".
     *
     * Логика:
     * - Если текст содержит полное вхождение ">>>" — намёк найден.
     * - Иначе проверяем последний значащий символ (trimEnd). Если это ">" — тоже считаем намёком,
     *   потому что следующий текстовый блок может содержать продолжение скобки.
     * - Если найден намек, то даем грант на следующие 15 вызовов с возвратом true. Так будем отправлять текст на проверку,
     *   пока наверняка не сформируется полный закрывающий тег.
     *
     * Цена ложного срабатывания — один лишний вызов innerText.
     *
     * @param {string} text - Текст для проверки.
     * @returns {boolean} true если есть намёк на закрывающую скобку.
     */
    _hasCloseHint(text) {

        if (!text) return false;

        // Полное вхождение.
        if (text.includes(">>>")) return true;

        // Частичное: последний значащий символ — ">".
        const trimmed = text.trimEnd();
        return trimmed.length > 0 && trimmed[trimmed.length - 1] === '>';
    }   // _hasCloseHint()

    /**
     * _hasFullCloseBracket()
     *
     * Назначение:
     * Проверить текст на наличие полноценной закрывающей скобки ">>>ai".
     *
     * @param {string} text - Текст для проверки.
     * @returns {boolean}
     */
    _hasFullCloseBracket(text) {

        return !!text && text.includes(">>>ai");
    }   // _hasFullCloseBracket()

    /**
     * _findContainerUp()
     *
     * Назначение:
     * Подняться от узла вверх по дереву DOM и найти контейнер code-block.
     *
     * Логика:
     * - Если тип сайта PRE — ищем ближайший <pre> (они не вкладываются друг в друга).
     * - Если тип сайта DIV — поднимаемся по цепочке <div>, проверяя textContent на наличие ">>>ai".
     *   Берём самый верхний <div>, содержащий маркер, чтобы не остановиться на мелком вложенном.
     * - Если тип UNKNOWN — сначала пробуем <pre>, при неудаче — цепочку <div>.
     *
     * Примечание:
     * - Используем textContent (не innerText) для проверки: он не вызывает reflow и работает быстро.
     * - Дорогой innerText вызывается позже, только для финального контейнера.
     *
     * @param {Node} node - Узел, от которого начинаем подъём.
     * @returns {HTMLElement|null} Контейнер или null.
     */
    _findContainerUp(node) {

        const el = (node && node.nodeType === Node.ELEMENT_NODE) ? node : node?.parentElement;
        if (!el || !el.closest) return null;

        // Сначала ищем ближайший <pre>.
        const pre = el.closest('pre');
        if (pre) return pre;

        // <pre> не обнаружен, ищем <div>
        return this._findTopmostDivWithMarker(el);

    }   // _findContainerUp()

    /**
     * _findTopmostDivWithMarker()
     *
     * Назначение:
     * Подняться от элемента вверх по цепочке <div>, найти первый (ближайший) <div>,
     * чей textContent начинается с открывающего маркера "<<<ai" (после trim)
     * и содержит закрывающий маркер ">>>ai".
     *
     * Логика:
     * - Начинаем с ближайшего <div> (через closest).
     * - Проверяем textContent: после trimStart должен начинаться с BRA,
     *   и содержать KET (ищем с конца для оптимизации).
     * - Если оба условия выполнены — это наш контейнер, возвращаем его.
     * - Если нет — поднимаемся к родительскому <div> и повторяем.
     * - Останавливаемся на document.body.
     *
     * Примечание:
     * - textContent дешевле innerText (не вызывает reflow).
     * - Ищем первый подходящий, а не самый верхний, чтобы не захватить лишний мусор.
     *
     * @param {HTMLElement} el - Элемент, от которого начинаем подъём.
     * @returns {HTMLElement|null} Первый <div> с обоими маркерами или null.
     */
    _findTopmostDivWithMarker(el) {

        const BRA = window.Globals.DIRECTIVE_BRACKET.BRA;
        const KET = window.Globals.DIRECTIVE_BRACKET.KET;

        let current = el.closest('div');

        while (current && current !== document.body) {

            const tc = current.textContent;

            // Блок должен начинаться с открывающего маркера (после отсечения ведущих пробелов).
            if (tc.trimStart().startsWith(BRA) && tc.lastIndexOf(KET) > 0) {
                return current;
            }   // if

            // Поднимаемся к родителю.
            current = current.parentElement?.closest('div') || null;
        }   // while

        return null;
    }   // _findTopmostDivWithMarker()

    /**
     * _collectTextNodes()
     *
     * Назначение:
     * Извлечь текстовые узлы (nodeType === TEXT_NODE) из списка DOM-узлов.
     * Для узлов-элементов рекурсивно обходит потомков.
     *
     * @param {NodeList|Array} nodes - Список узлов (например, mutation.addedNodes).
     * @returns {Text[]} Массив текстовых узлов.
     */
    _collectTextNodes(nodes) {

        const result = [];

        nodes.forEach(node => {
            if (node.nodeType === Node.TEXT_NODE) {
                result.push(node);
            } else if (node.nodeType === Node.ELEMENT_NODE) {
                // TreeWalker для эффективного обхода поддерева.
                const walker = document.createTreeWalker(node, NodeFilter.SHOW_TEXT);
                let textNode;
                while ((textNode = walker.nextNode())) {
                    result.push(textNode);
                }   // while
            }   // if/else
        });

        return result;
    }   // _collectTextNodes()

    /**
     * _isMutationInsideCaughtContainer()
     *
     * Назначение:
     * Проверить, принадлежит ли мутация пойманному контейнеру.
     *
     * @param {MutationRecord} mutation - Запись мутации.
     * @returns {boolean} true если мутация произошла внутри _caughtContainer.
     */
    _isMutationInsideCaughtContainer(mutation) {

        if (!this._caughtContainer) return false;

        // Для characterData target — текстовый узел, берём его родителя.
        const node = (mutation.type === 'characterData') ? mutation.target.parentElement :
            mutation.target;
        if (!node) return false;

        return this._caughtContainer.contains(node);
    }   // _isMutationInsideCaughtContainer()

    /**
     * _processHintNode()
     *
     * Назначение:
     * Обработать узел, в котором обнаружен намёк на закрывающую скобку.
     * Поднимается к ближайшему контейнеру, проверяет innerText на полноценную ">>>ai".
     *
     * # Side effects
     * - При обнаружении ">>>ai" в innerText поднимает флаг isDirectiveCaught.
     * - Передаёт текст в dirExtractor.acceptWebOutput().
     *
     * @param {Node} node - Узел с намёком.
     */
    _processHintNode(node) {

        const container = this._findContainerUp(node);

        if (!container || this.isGarbageNode(container)) return;

        const text = container.innerText;
        if (!text) return;

        // Отдаём текст экстрактору в любом случае (раз уже заплатили за innerText).
        this.dirExtractor.acceptWebOutput(text);

        // Если в тексте есть полноценная ">>>ai" — поднимаем флаг.
        if (this._hasFullCloseBracket(text)) {
            this.isDirectiveCaught = true;
            this._caughtContainer = container;
        }   // if
    }   // _processHintNode()

    /**
     * _processCaughtState()
     *
     * Назначение:
     * Обработать мутацию внутри пойманного контейнера (isDirectiveCaught === true).
     * Берём innerText из запомненного контейнера и отдаём экстрактору.
     *
     * # Side effects
     * - Сбрасывает isDirectiveCaught при false positive (>>>ai исчезла из innerText).
     * - Передаёт текст в dirExtractor.acceptWebOutput().
     */
    _processCaughtState() {

        // Контейнер мог быть удалён из DOM (React/SPA), он будет отсоединен, при этом isConnected вернет false.
        if (!this._caughtContainer.isConnected) {
            this.isDirectiveCaught = false;
            this._caughtContainer = null;
            return;
        }   // if

        // Контейнер пойман, его не надо искать заново. Забираем его содержимое (длительная операция).
        const text = this._caughtContainer.innerText;
        if (!text) return;

        // Проверка на false positive: если ">>>ai" исчезла — сбрасываем флаг, текст не отдаём.
        if (!this._hasFullCloseBracket(text)) {
            this.isDirectiveCaught = false;
            this._caughtContainer = null;
            return;
        }   // if

        // ">>>ai" на месте — отдаём экстрактору.
        this.dirExtractor.acceptWebOutput(text);
    }   // _processCaughtState()

    /**
     * _resetCaughtState()
     *
     * Назначение:
     * Сбросить состояние "директива поймана". Вызывается при обнаружении мутации
     * вне пойманного контейнера или из directive_extractor при успешном помещении в шлих.
     */
    _resetCaughtState() {

        this.isDirectiveCaught = false;
        this._caughtContainer = null;
    }   // _resetCaughtState()

    /**
     * initializeObserver()
     *
     * Назначение:
     * Создаёт MutationObserver с двухуровневой детекцией директив.
     *
     * Алгоритм:
     * 1. Если isDirectiveCaught === true:
     *    - Проверяем, принадлежит ли мутация пойманному контейнеру.
     *    - Если да — берём innerText, отдаём экстрактору (ловим метаданные после ">>>ai").
     *    - Если нет — сбрасываем флаг и обрабатываем мутацию в обычном режиме.
     *
     * 2. Если isDirectiveCaught === false:
     *    - Для characterData: проверяем mutation.target.data на намёк (>>>/завершающий >).
     *    - Для childList: собираем текстовые узлы из addedNodes, проверяем каждый на намёк.
     *    - При обнаружении намёка — эскалируем через _processHintNode().
     *
     * # Side effects
     * - Создаёт this.observer.
     * - При каждом пакете мутаций может вызывать this.dirExtractor.acceptWebOutput().
     */
    initializeObserver() {

        this.observer = new MutationObserver((mutations) => {

            // Перехвачено изменение страницы. Сбрасываем флаг молчания.
            this.wasThereSilence = false;

            // --- Состояние "директива поймана" ---
            if (this.isDirectiveCaught) {

                // Разделяем мутации: свои (внутри контейнера) и чужие (вне его).
                let hasOwnMutation = false;
                let hasForeignMutation = false;

                for (const mutation of mutations) {
                    if (this._isMutationInsideCaughtContainer(mutation)) {
                        hasOwnMutation = true;
                    } else {
                        hasForeignMutation = true;
                    }   // if/else
                    // Если обнаружены оба типа — дальше проверять не нужно.
                    if (hasOwnMutation && hasForeignMutation) break;
                }   // for

                // Если есть мутации внутри контейнера — обрабатываем (ловим метаданные).
                if (hasOwnMutation) {
                    this._processCaughtState();
                }   // if

                // Если есть мутации вне контейнера — поток ушёл дальше, контейнер финализирован.
                if (hasForeignMutation) {
                    this._resetCaughtState();
                }   // if

                // Если были только свои мутации и флаг ещё поднят — выходим, чужие не обрабатываем.
                if (this.isDirectiveCaught) return;

                // Флаг сброшен (чужой мутацией или проверкой >>>ai в _processCaughtState()) — идем к обработке
                // чужих мутаций.
            }   // if isDirectiveCaught

            // --- Обычный режим: ищем намёк на закрывающую скобку ---
            for (const mutation of mutations) {

                // Пропускаем мутации, уже обработанные в caught-ветке выше.
                if (this._caughtContainer && this._isMutationInsideCaughtContainer(mutation)) {
                    continue;
                }   // if

                // 1) characterData: текст внутри существующего узла изменился.
                if (mutation.type === 'characterData') {

                    const data = mutation.target.data;
                    if (this._hasCloseHint(data)) {

                        // Искать родительский блок, содержащий этот текст.
                        this._processHintNode(mutation.target);

                        // Если флаг поднялся — прекращаем обработку остальных мутаций.
                        if (this.isDirectiveCaught) return;
                    }   // if
                    continue;
                }   // if characterData

                // 2) childList: новые узлы добавлены в DOM.
                if (mutation.type === 'childList' && mutation.addedNodes.length > 0) {
                    const textNodes = this._collectTextNodes(mutation.addedNodes);

                    for (const textNode of textNodes) {

                        if (this._hasCloseHint(textNode.data)) {

                            // Искать родительский блок, содержащий этот текст.
                            this._processHintNode(textNode);

                            // Если флаг поднялся — прекращаем.
                            if (this.isDirectiveCaught) return;
                        }   // if
                    }   // for
                }   // if childList
            }   // for mutations
        });
    }   // initializeObserver()

    /**
     * startObserver()
     *
     * Назначение:
     * Запуск наблюдения за DOM.
     */
    startObserver() {
        if (this.isObserverStarted) return;

        this.observer.observe(document.body, {
            childList: true,      // следить за добавлением/удалением тегов
            subtree: true,        // следить за всеми вложенными потомками
            characterData: true   // следить за изменением текста внутри тегов
        });

        this.isObserverStarted = true;
    }   // startObserver()

    /**
     * stopObserver()
     *
     * Назначение:
     * Приостановка наблюдения за DOM.
     */
    stopObserver() {
        if (!this.isObserverStarted) return;

        this.observer.disconnect();
        this.isObserverStarted = false;
    }   // stopObserver()

    /**
     * cleanup()
     *
     * Назначение:
     * Отключение наблюдателя и очистка ресурсов.
     */
    cleanup() {
        if (this.observer) {
            this.observer.disconnect();
            this.observer = null;
        }
        this.isObserverStarted = false;
        this._resetCaughtState();
    }   // cleanup()
}   // TextProcessor

// Экспортируем в глобальную область для content.js
window.TextProcessor = TextProcessor;