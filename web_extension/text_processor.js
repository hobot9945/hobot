/**
 * text_processor.js
 *
 * ОПИСАНИЕ:
 * Перехват изменений DOM и извлечение ТОЛЬКО code-block контейнеров (<pre>) для поиска директив.
 *
 * Режим "быстро оживить систему":
 * - Директивы должны появляться внутри fenced code block (бэктики), поэтому анализируем только <pre>.
 * - Любой обычный текст страницы и UI-шум игнорируем.
 *
 * Взаимодействие:
 * - На каждой пачке мутаций собираем уникальные <pre> контейнеры.
 * - Передаём их текст (innerText) в dirExtractor.acceptWebOutput().
 * - Перевзводим таймер "тишины" через dirExtractor.rearmDirectiveCompletionTimer().
 */

class TextProcessor {
    constructor(dirExtractor) {
        this.dirExtractor = dirExtractor;  // Интерфейс к DirExtractor
        this.observer = null;
        this.isObserverStarted = false;
    }

    /**
     * Фильтр элементов (упрощённый).
     *
     * Назначение:
     * - В режиме "ловим только code-block" нужно не очень много эвристик:
     *   1) зона ввода пользователя (чтобы не ловить ввод и не шуметь),
     *   2) aria-hidden (скрытые узлы),
     *   3) технические теги (на всякий случай).
     *
     * Важно:
     * - НЕ читаем innerText (дорого и нестабильно).
     * - НЕ используем эвристики по классам (btn/sidebar/nav/...) — это задача выбора code-block.
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

        // 1.3) Фолбэк для “редакторов”, которые размечают ввод через атрибуты/ARIA
        if (node.closest?.('[contenteditable="true"],[contenteditable="plaintext-only"],[role="textbox"]')) {
            return true;
        }

        // --- 2) Скрытые элементы ---
        if (node.getAttribute('aria-hidden') === 'true') return true;

        // --- 3) Технические теги ---
        const tagName = node.tagName.toUpperCase();
        const ignoredTags = [
            'SCRIPT', 'STYLE', 'NOSCRIPT', 'SVG', 'IMG',
            'BUTTON', 'INPUT', 'TEXTAREA', 'SELECT',
            'VIDEO', 'AUDIO', 'IFRAME', 'CANVAS', 'LINK', 'META'
        ];
        if (ignoredTags.includes(tagName)) return true;

        return false;

    }   // isGarbageNode()

    /**
     * Ищет ближайший контейнер code-block для узла.
     *
     * Контракт:
     * - Возвращает ближайший <pre>, если он существует.
     * - Иначе возвращает null.
     *
     * Примечание:
     * - Сейчас поддерживаем только <pre>, т.к. это самый стабильный контейнер для code-block на целевых сайтах.
     *
     * @param {Node|HTMLElement} node - Узел, от которого начинаем подъем.
     * @returns {HTMLElement|null} Элемент <pre> или null если не нашли.
     */
    getCodeBlockContainer(node) {

        const el = (node && node.nodeType === Node.ELEMENT_NODE) ? node : node?.parentElement;
        if (!el || !el.closest) return null;

        // Приоритет: PRE (обычно содержит весь код-блок целиком)
        const pre = el.closest('pre');
        if (pre) return pre;

        return null;

    }   // getCodeBlockContainer()

    /**
     * collectCodeBlocks()
     *
     * Назначение:
     * Рекурсивно обходит поддерево и собирает контейнеры <pre>.
     *
     * Алгоритм:
     * - Пытаемся найти ближайший <pre> вверх по дереву (closest).
     * - Если найден и не мусор — добавляем в Set и выходим (ниже уже не нужно).
     * - Если не найден — рекурсивно идём в дочерние узлы.
     *
     * @param {Node} node - Корень обхода.
     * @param {Set<HTMLElement>} blocksToProcess - Накопитель контейнеров <pre>.
     */
    collectCodeBlocks(node, blocksToProcess) {

        if (!node) return;

        const container = this.getCodeBlockContainer(node);
        if (container) {
            if (!this.isGarbageNode(container)) {
                blocksToProcess.add(container);
            }
            return;
        }

        if (node.nodeType === Node.ELEMENT_NODE) {
            node.childNodes.forEach(child => this.collectCodeBlocks(child, blocksToProcess));
        }

    }   // collectCodeBlocks()

    /**
     * initializeObserver()
     *
     * Назначение:
     * Создаёт MutationObserver и настраивает сбор “кандидатов текста” для DirExtractor.
     *
     * В режиме “быстро оживить систему” работаем только по директивам, которые должны быть
     * выведены внутри fenced code block (бэктики). В DOM это почти всегда <pre><code>...</code></pre>
     * или стилизованный <pre> (например, shiki). Поэтому:
     * - игнорируем весь остальной текст страницы;
     * - собираем только контейнеры <pre>/<code>;
     *
     * Алгоритм:
     * - На каждой пачке мутаций собираем Set контейнеров code-block.
     * - Источники:
     *   - childList: обходим добавленные узлы рекурсивно, ищем <pre>/<code>.
     *   - characterData: поднимаемся к ближайшему <pre>/<code> от места изменения.
     * - В конце передаём уникальные контейнеры в dirExtractor.acceptWebOutput().
     *
     * # Side effects
     * - Создаёт this.observer.
     * - При каждом пакете мутаций вызывает this.dirExtractor.acceptWebOutput() для найденных блоков.
     */
    initializeObserver() {

        this.observer = new MutationObserver((mutations) => {

            // Перехвачено любое изменение страницы. Сбрасываем флаг молчания.
            this.dirExtractor.wasThereSilence = false;

            // Набор уникальных code-block контейнеров (<pre>/<code>).
            // Set нужен, чтобы не обрабатывать один и тот же контейнер 20 раз за одну пачку мутаций.
            const blocksToProcess = new Set();

            mutations.forEach((mutation) => {

                // 1) Создание новых элементов (childList).
                // Ищем <pre>/<code> внутри addedNodes.
                if (mutation.type === 'childList') {
                    mutation.addedNodes.forEach(node => {
                        this.collectCodeBlocks(node, blocksToProcess);
                    });
                }

                // 2) Изменение текста внутри существующего элемента (characterData).
                // Поднимаемся к ближайшему <pre>/<code>.
                else if (mutation.type === 'characterData') {
                    const container = this.getCodeBlockContainer(mutation.target);
                    if (!container) return;

                    if (!this.isGarbageNode(container)) {
                        blocksToProcess.add(container);
                    }
                }

            });

            blocksToProcess.forEach((block) => {
                this.dirExtractor.acceptWebOutput(block.innerText); // pre всегда имеет .innerText
            });

        });

    }   // initializeObserver()

    /**
     * Запуск наблюдения за DOM.
     * Вызывает нативный .observe() для уже созданного экземпляра MutationObserver.
     */
    startObserver() {
        if (this.isObserverStarted) return;

        this.observer.observe(document.body, {
            childList: true,      // следить за добавлением/удалением тегов
            subtree: true,        // следить за всеми вложенными потомками
            characterData: true   // следить за изменением текста внутри тегов
        });

        this.isObserverStarted = true;
    }

    /**
     * Приостановка наблюдения за DOM.
     * Вызывает нативный .disconnect(), переставая получать новые мутации.
     */
    stopObserver() {
        if (!this.isObserverStarted) return;

        this.observer.disconnect();
        this.isObserverStarted = false;
    }

    /**
     * Отключение наблюдателя и очистка ресурсов.
     */
    cleanup() {
        if (this.observer) {
            this.observer.disconnect();
            this.observer = null;
        }
        this.isObserverStarted = false;
    }

}   // TextProcessor

// Экспортируем в глобальную область для content.js
window.TextProcessor = TextProcessor;
