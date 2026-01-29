/**
 * directive_extractor.js
 *
 * НАЗНАЧЕНИЕ:
 * Извлечение и валидация директив из текстового потока, полученного от TextProcessor.
 *
 * ТЕКУЩАЯ МОДЕЛЬ (после переработки):
 * 1) TextProcessor отправляет сюда ТЕКСТ code-block (<pre>) как строку.
 * 2) Здесь накапливается "сырой хвост" в webOutputBuffer (может быть незавершённым).
 * 3) Таймер "тишины" (rearmDirectiveCompletionTimer) по истечению паузы переносит данные в schlich.
 * 4) _processWebOutput() парсит schlich конечным автоматом и выделяет директивы <<<ai ... >>>ai.
 *
 * ОТВЕТСТВЕННОСТЬ:
 * 1) Накопление текста code-block в webOutputBuffer.
 * 2) Отложенная "сборка" данных по таймеру в schlich.
 * 3) Выделение директив, проверка session_id и строгой последовательности DIRECTIVE_NUM.
 * 4) Отправка валидных директив агенту через bridge.sendToAgent().
 */
class DirectiveExtractor {
    constructor(bridge) {

        // Интерфейс к Хоботу через фоновый скрипт.
        this.bridge = bridge;

        // Текст от старого захода.
        this.savedText = "";

        // Накопительный буфер перехваченного текста (строка). acceptWebOutput() дописывает в конец дельты.
        // processWebOutput() проверяет не сформировалась ли целая директива, если да, то забирает ее, проверяет
        // ее на соответствие протоколу и отправляет агенту.
        this.webOutputBuffer = "";

        // --- Переменные управления таймером молчания ---
        this.wasThereSilence = true;    // флаг тишины
        this._directiveCompletionTimerId = null;   // id таймера "тишины"

        // --- Переменные управления processWebOutput() ---

        // Шлих - сырая порода, из которой processWebOutput() будет извлекать директиву. К этой строке добавляется
        // содержимое webOutputBuffer, который после этого очищается.
        this.schlich = "";

        // Номер последней исполненной директивы
        this._lastDirectiveNum = 0;

        // Указатель, с какой позиции начинать следующий поиск в webOutputBuffer.
        // Это оптимизация: буфер растёт справа, поэтому нет смысла каждый раз искать с нуля.
        this._searchIndex = 0;

        // Индекс не просмотренной processWebOutput() части буфера.
        this._scannedIndex = 0;

        // После выделения директивы здесь лежат данные для проверки.
        this._extractedDirective = null;        // от <<<ai ... >>>ai DIRECTIVE_NUM SESSION_ID
        this._dirNumFromOpenTag = null;         // число как строка
        this._sessIdFromOpenTag = null;
        this._dirNumFromCloseTag = null;        // число как строка
        this._sessIdFromCloseTag = null;

        // Флаг остановки фонового цикла (если понадобится при выгрузке страницы).
        this._stopProcessing = false;

        // Запускаем фоновый процесс слежения за буфером.
        if (!window.Globals.LOGGER_MODE) {
            this._processWebOutput().catch((e) => {
                this._reportProtocolErrorToAiAndUser(`_processWebOutput crashed: ${e?.message || String(e)}`);
            });
        } else {
            this._processWebOutputLogging().catch((e) => {
                this._reportProtocolErrorToAiAndUser(`_processWebOutputLogging crashed: ${e?.message || String(e)}`);
            });
        }

        // Запускаем активатор процессора.
        this.setupProcessWebOutputActivator();
    }   // constructor()

    /**
     * Принимает строку текста из code-block и дописывает дельту в webOutputBuffer.
     *
     * Контракт:
     * - Вход: string (обычно innerText контейнера <pre>).
     * - Выход: void.
     *
     * Логика:
     * - savedText хранит последнее полностью увиденное значение.
     * - Если новый текст начинается с savedText — дописываем только хвост (дельту).
     * - Иначе считаем, что блок перерисовали (SPA) и принимаем текст целиком.
     *
     * @param {string} newText - Текущий текст code-block (обычно <pre>.innerText).
     */
    acceptWebOutput(newText) {

        // Чистим текст от хвостового перевода строки. Зачем? Строка всегда заканчивается переводом, даже если это середина
        // слова и будет продолжение. Новая порция этот перевод строки убирает и лепит новый текст. В результате
        // старая строка не является частью новой.
        const currentText = newText.trimEnd();
        if (!currentText) return; // Если после trim пусто — выходим

        // Если текст стал длиннее (допечатался), выделяем дельту
        let delta;
        if (currentText.startsWith(this.savedText) && currentText.length >= this.savedText.length) {
            delta = currentText.substring(this.savedText.length);
        } else {
            delta = currentText;
        }
// console.log(`currentText='${currentText}'\nthis.savedText='${this.savedText}'\ndelta='${delta}'`);
        this.savedText = currentText;

        // Если есть дельта — добавляем в строковый буфер
        if (delta) {
            this.webOutputBuffer += delta;
            this._clearWebOutputBuffer();
        }   // if
    }   // acceptWebOutput()

    /**
     * rearmDirectiveCompletionTimer()
     *
     * Назначение:
     * Периодически проверяет флаг “тишины”.
     *
     * Если случился период тишины, т.е. флаг не опущен:
     * - переносим данные из webOutputBuffer в schlich (через _onDirectiveCompletionTimeout()).
     *
     * # Side effects
     * - Ставит новый таймер на window.Globals.DIRECTIVE_COMPLETION_TIMEOUT.
     */
    setupProcessWebOutputActivator() {

        setInterval(() => {
            if (this.wasThereSilence && this.webOutputBuffer.length !== 0) {
                this._moveLastDirectiveToSchlich();
            }

            // Снова заряжаем флаг тишины.
            this.wasThereSilence = true;
        }, window.Globals.DIRECTIVE_COMPLETION_TIMEOUT);
    }   // rearmDirectiveCompletionTimer()

    /**
     * _clearWebOutputBuffer()
     *
     * Назначение:
     * - Защититься от раздувания webOutputBuffer (и потенциальных зависаний браузера).
     *
     * Правила очистки (как в задаче):
     * 1) Если в буфере есть маркер OPEN (<<<ai) — считаем всё ДО него мусором и отрезаем.
     *    Используем lastIndexOf, чтобы оставить "самую свежую" потенциальную директиву.
     * 2) Если маркера нет — оставляем только короткий хвост длиной (OPEN.length - 1),
     *    на случай если OPEN начал печататься, но ещё не допечатан целиком.
     */
    _clearWebOutputBuffer() {

        const OPEN = window.Globals?.DIRECTIVE_BRACKET?.BRA;
        const MAX_LEN = 2048;

        const buf = this.webOutputBuffer;
        if (buf.length <= MAX_LEN) return;


        // Ищем первый открывающий маркер. Чистим все до него.
        const firstIdx = buf.indexOf(OPEN);
        if (firstIdx !== -1) {
            this.webOutputBuffer = buf.substring(firstIdx);
            return;
        }

        // OPEN не найден: оставляем хвост (OPEN.length - 1)
        const tailLen = OPEN.length - 1;
        this.webOutputBuffer = tailLen > 0 ? buf.slice(-tailLen) : "";
    }

    /**
     * _moveLastDirectiveToSchlich()
     *
     * Назначение:
     * Извлечь из webOutputBuffer последнюю директиву текущей сессии и перенести её в schlich.
     *
     * Идея:
     * - Сканируем буфер с конца и ищем ближайшие валидные теги OPEN/CLOSE с метаданными и session_id.
     * - Если в хвосте только OPEN (директива не завершена) — оставляем хвост в webOutputBuffer, ничего не переносим.
     * - Если найден CLOSE — ищем к нему ближайший валидный OPEN той же сессии и переносим диапазон [OPEN..CLOSE+metadata].
     *
     * # Side effects
     * - Может обрезать/очищать this.webOutputBuffer.
     * - При успехе дописывает this.schlich.
     *
     * @returns {boolean} true если директива перенесена в schlich, иначе false.
     */
    _moveLastDirectiveToSchlich() {

        // Берем ид сессии. Если его еще нет, то любая текст считается мусором.
        const currentSessionId = window.Globals?.sessionId;
        if (!currentSessionId) {
            this.webOutputBuffer = "";
            return false;
        }

        const OPEN = window.Globals.DIRECTIVE_BRACKET.BRA;
        const CLOSE = window.Globals.DIRECTIVE_BRACKET.KET;

        // При пустом буфере делать нечего, возвращаемся.
        const buf = this.webOutputBuffer || "";
        if (buf.length === 0) {
            this.webOutputBuffer = "";
            return false;
        }

        /**
         * Ищет в направлении от хвоста к голове ближайший валидный тег (OPEN или CLOSE) с метаданными и текущим
         * session_id в диапазоне [0..fromExclusive).
         *
         * @param {number} fromExclusive
         *
         * @returns {{ kind: "open"|"close", index: number, endIndex: number, dirNum: number } | null}
         * где index - индекс начала маркера, endIndex - индекс вслед за метаданными.
         */
        const findPrevValidTag = (fromExclusive) => {

            // let cursor = Math.min(fromExclusive, buf.length);
            let cursor = fromExclusive;

            while (cursor > 0) {

                const searchPos = cursor - 1;

                const openIdx = buf.lastIndexOf(OPEN, searchPos);
                const closeIdx = buf.lastIndexOf(CLOSE, searchPos);

                if (openIdx === -1 && closeIdx === -1) {
                    return null;
                }

                const isOpen = openIdx > closeIdx;
                const idx = isOpen ? openIdx : closeIdx;
                const markerLen = isOpen ? OPEN.length : CLOSE.length;

                // Хвост от начала метаданных после маркера до конца буфера.
                const tail = buf.substring(idx + markerLen);

                // Формат метаданных одинаков для обоих маркеров.
                const metaRegex = /^\s+(\d+)\s+(\S{6})/;

                // Проверяем парсятся ли метаданные за маркером.
                const m = tail.match(metaRegex);
                if (!m) {
                    cursor = idx; // этот маркер без метаданных игнорируем
                    continue;
                }

                // Проверяем валидность метаданных
                const dirNum = parseInt(m[1], 10);
                const sessId = m[2];
                if (!Number.isFinite(dirNum) || sessId !== currentSessionId) {
                    cursor = idx; // чужая сессия или мусорные метаданные, игнорируем
                    continue;
                }

                return {
                    kind: isOpen ? "open" : "close",
                    index: idx,
                    endIndex: idx + markerLen + m[0].length, // позиция сразу после метаданных тега
                    dirNum: dirNum
                };
            }

            return null;
        };

        // 1) Ищем первый валидный тег с конца. Если ничего нет, чистим буфер, уходим.
        const lastTag = findPrevValidTag(buf.length);
        if (!lastTag) {
            this.webOutputBuffer = ""; // всё мусор
            return false;
        }

        // 2) Если первым попался OPEN — оставляем хвост (вдруг допечатается позже), но в schlich не переносим.
        if (lastTag.kind === "open") {
            this.webOutputBuffer = buf.substring(lastTag.index); // чистим мусор до OPEN
            return false;
        }

        // 3) Первым попался CLOSE — ищем пару OPEN, двигаясь к началу.
        let mainClose = lastTag;

        while (true) {

            const prevTag = findPrevValidTag(mainClose.index);
            if (!prevTag) {
                // CLOSE есть, OPEN не нашли — считаем всё мусором
                this.webOutputBuffer = "";
                return false;
            }

            if (prevTag.kind === "close") {
                // По ТЗ: если встретили CLOSE — он становится "основным"
                mainClose = prevTag;
                continue;
            }

            // 4) Нашли последнюю директиву, это OPEN: переносим ТОЛЬКО её, остальное выбрасываем.
            const directive = buf.substring(prevTag.index, mainClose.endIndex) + "\n";
            this.schlich += directive;
            this.webOutputBuffer = "";

            if (window.Globals.IS_DEBUG) {
                console.log(`schlich="${this.schlich}"`);
            }

            return true;
        }
    }   // _moveLastDirectiveToSchlichOrDrop()

    /**
     * _processWebOutput()
     *
     * Назначение:
     * Фоновый асинхронный цикл обработки накопленного текстового буфера (`webOutputBuffer`).
     * Реализует конечный автомат (State Machine) для выделения директив в формате протокола.
     *
     * Архитектура:
     * - Использует два пилота: `pilot` (текущее действие) и `secondPilot` (следующее действие после проверки данных).
     * - Состояния (`Action`) представляют этапы поиска и анализа директив.
     * - Индексы `_searchIndex` и `_scannedIndex` управляют прогрессом сканирования буфера.
     *
     * Алгоритм:
     * 1. Начинает с состояния `CHECK_NEW_DATA_AVAILABILITY`.
     * 2. При наличии данных переходит в состояние, указанное `secondPilot`.
     * 3. Поиск открывающего маркера (`<<<ai`) с очисткой мусора.
     * 4. Парсинг метаданных открывающего тега (номер директивы, идентификатор сессии).
     * 5. Поиск закрывающего маркера (`>>>ai`) внутри директивы.
     * 6. Парсинг метаданных закрывающего тега.
     * 7. Вырезка готовой директивы из буфера.
     * 8. Проверка целостности директивы и отправка агенту.
     * 9. Возврат к поиску следующей директивы.
     *
     * # Паника
     * - При программной ошибке (переход в `UNDEFINED`) сбрасывает состояние и уведомляет оператора.
     * - Критические ошибки парсинга приводят к очистке буфера и возврату в начальное состояние.
     *
     * # Побочные эффекты
     * - Мутирует `schlich` (удаляет обработанные части).
     * - Обновляет внутренние поля (`_searchIndex`, `_scannedIndex`, `_lastDirectiveNum`).
     * - Отправляет директивы агенту через `this.bridge.sendToAgent`.
     * - Генерирует уведомления об ошибках протокола.
     *
     * # Возвращаемое значение
     * - Promise, который выполняется при остановке цикла (например, при `_stopProcessing = true`).
     */
    async _processWebOutput(){

        const OPEN = window.Globals?.DIRECTIVE_BRACKET?.BRA || "<<<ai";
        const CLOSE = window.Globals?.DIRECTIVE_BRACKET?.KET || ">>>ai";

        // Пауза между проверками, когда в буфере "тишина".
        const SLEEP_MS = 100;

        // Если тега "<<<ai" нет, считаем накопленный текст мусором.
        // Чтобы мусор не раздувал память, после порога режем буфер.
        const MAX_GARBAGE_LENGTH = 2000;

        // Хвост, который оставляем для поиска открывающего тега (без параметров):
        // - при обрезке мусора;
        // - при неуспешном поиске (чтобы "добрать" частично допечатанный тег на следующем тике).
        const SHORT_SAFETY_TAIL = Math.max(OPEN.length, CLOSE.length) - 1;

        // Хвост, который оставляем для поиска закрывающего тега с параметрами. Хватило бы CLOSE.length+15, включая
        // закрывающий пробел, но сделаем запас. Вдруг придет фантазия увеличить иды.
        const FULL_SAFETY_TAIL = CLOSE.length + 20;

        // Действия
        const Action = Object.freeze({

            // Неопределённое состояние. Второй пилот принимает его, когда достаточно первого.
            UNDEFINED: -1,

            // Проверяем есть ли в буфере данные, готовые для анализа.
            CHECK_NEW_DATA_AVAILABILITY: 0,

            // Ищем заголовок открывающего тега <<<ai.
            FIND_OPEN_TAG_MARKER: 1,

            // Принимаем метаданные - номер директивы(число) и идентификатор сессии открывающего тега.
            PARSE_OPEN_TAG_METADATA: 2,

            // Ищем заголовки закрывающего/открывающего тега, кто первый.
            FIND_ANY_TAG_MARKER: 3,

            // Принимаем метаданные - номер директивы(число) и идентификатор сессии закрывающего тега.
            PARSE_CLOSE_TAG_METADATA: 4,

            // Вырезаем выделенную директиву из буфера.
            EXTRACT_DIRECTIVE: 5,

            // Гасим флаг isInsideDirective и переходим к проверке и отсылке директивы.
            CHECK_AND_SEND_DIRECTIVE: 6
        });

        // Главная управляющая переменная.
        let pilot = Action.CHECK_NEW_DATA_AVAILABILITY;

        // Управляющая переменная второй фазы. Иногда требуется не просто уйти к какому-то действию, но и дать задание
        // куда идти после его исполнения.
        let secondPilot = Action.FIND_OPEN_TAG_MARKER;
        while (!this._stopProcessing) {
            switch (pilot) {

                // Проверяем есть ли в буфере данные, готовые для анализа. По результату, либо идем к поиску открывающего тега,
                // либо выжидаем таймаут и идем на себя.
                case Action.CHECK_NEW_DATA_AVAILABILITY: {

                    // Если новых данных нет — спим.
                    if (this._scannedIndex >= this.schlich.length) {
                        await this._delay(SLEEP_MS);
                        pilot = Action.CHECK_NEW_DATA_AVAILABILITY;

                        break;
                    }   // if

                    // Есть данные для обработки. Переходим туда, куда указывает второй пилот.
                    pilot = secondPilot;
                    secondPilot = Action.UNDEFINED;
                    break;
                }   // case CHECK_NEW_DATA_AVAILABILITY

                // Ищем заголовок открывающего тега <<<ai. Просто заголовок, без метаданных. Попутно подчищаем головной мусор.
                // По результатам либо идем к приему метаданных, либо к проверке доступности данных для анализа (пункт 0).
                case Action.FIND_OPEN_TAG_MARKER: {
                    const openIndex = this.schlich.indexOf(OPEN, this._scannedIndex);

                    if (openIndex === -1) {

                        // Маркера нет — весь буфер просмотрен. Если буфер разросся, считаем его мусором и режем,
                        // оставляя хвост который может содержать частичный открывающий тег.
                        if (this.schlich.length > MAX_GARBAGE_LENGTH) {

                            // Чистим мусор - почти весь буфер.
                            this.schlich = this.schlich.slice(-SHORT_SAFETY_TAIL);

                            // После физической обрезки буфера индексы теряют смысл — начинаем заново.
                            this._searchIndex = 0;
                            this._scannedIndex = 0;
                        } else {

                            // Буфер пока маленький, не чистим, корректируем указатель просмотренного текста,
                            // с учетом того что мог остаться нераспознанный зачаток тега в конце.
                            this._scannedIndex = this.schlich.length;
                        }   // if/else

                        // Идем к п.0
                        pilot = Action.CHECK_NEW_DATA_AVAILABILITY;
                        secondPilot = Action.FIND_OPEN_TAG_MARKER; // Сохраняем цель поиска
                        break;
                    }   // if

                    // Маркер найден. Всё слева от него — мусор, отбрасываем.
                    if (openIndex > 0) {
                        this.schlich = this.schlich.substring(openIndex);
                    }   // if

                    // ВАЖНО: как только нашли сигнатуру OPEN, двигаем _searchIndex в конец "<<<ai".
                    // Далее в PARSE_OPEN_METADATA будем смотреть ТОЛЬКО метаданные после сигнатуры.
                    this._searchIndex = OPEN.length;
                    this._scannedIndex = this._searchIndex;

                    // Мы нашли маркер открывающего тега, переходим к разбору его метаданных.
                    pilot = Action.PARSE_OPEN_TAG_METADATA;
                    secondPilot = Action.UNDEFINED;
                    break;
                }   // case FIND_OPEN_TAG_MARKER

                // Принимаем метаданные - номер директивы(число) и идентификатор сессии открывающего тега. По результату,
                // либо поднимаем флаг isInsideDirective и переходим к поиску любого тега, либо сваливаемся в пункт 0.
                case Action.PARSE_OPEN_TAG_METADATA: {
                    // Тело после "<<<ai". _searchIndex, _scannedIndex указывают на пробел после маркера.
                    const tail = this.schlich.substring(this._searchIndex);

                    // Парсим ТОЛЬКО метаданные (без "<<<ai" в regex)
                    const metaRegex = new RegExp(`^\\s+(\\d+)\\s+(\\S{6})`);
                    const metaMatch = tail.match(metaRegex);

                    if (!metaMatch) {
                        // Метаданных пока нет или это псевдо-тег из текста.
                        // Если данных после сигнатуры мало — идем за новыми данными, потом возвращаемся в наш кейс.
                        if (tail.length < FULL_SAFETY_TAIL) {
                            this._scannedIndex = this.schlich.length;
                            pilot = Action.CHECK_NEW_DATA_AVAILABILITY;
                            secondPilot = Action.PARSE_OPEN_TAG_METADATA;
                            break;
                        }   // if

                        if (window.Globals.IS_DEBUG) {
                            console.log(`Невалидный заголовок директивы:
schlich="${this.schlich}"`);
                        }

                        // Если хвост уже большой, а метаданные так и не появились — считаем все начало буфера,
                        // включая открывающий маркер мусором и идем к поиску следующего OPEN.
                        this.schlich = this.schlich.substring(OPEN.length);
                        this._searchIndex = 0;
                        this._scannedIndex = 0;

                        pilot = Action.CHECK_NEW_DATA_AVAILABILITY;
                        secondPilot = Action.FIND_OPEN_TAG_MARKER;
                        break;
                    }   // if

                    // Метаданные получены.
                    const dirNumFromOpenTag = parseInt(metaMatch[1], 10);
                    const sessIdFromOpenTag = metaMatch[2];

                    // Чужая сессия или прошлая директива — молча игнорируем, ищем следующий OPEN.
                    if (sessIdFromOpenTag !== window.Globals.sessionId ||
                        dirNumFromOpenTag <= this._lastDirectiveNum) {

                        if (window.Globals.IS_DEBUG) {
                            console.log(`Чужая сессия или прошлая директива:
schlich="${this.schlich}"`);
                        }

                        // Отрезаем голову вместе с открывающим маркером.
                        this.schlich = this.schlich.substring(OPEN.length);
                        this._searchIndex = 0;
                        this._scannedIndex = 0;

                        pilot = Action.FIND_OPEN_TAG_MARKER;
                        secondPilot = Action.UNDEFINED;
                        break;
                    }   // if

                    // Сохраняем метаданные для будущего контроля
                    this._dirNumFromOpenTag = dirNumFromOpenTag;
                    this._sessIdFromOpenTag = sessIdFromOpenTag;

                    // Продвигаем _searchIndex в конец за метаданные.
                    // metaMatch[0] включает ведущие пробелы, номер и session_id.
                    this._searchIndex = this._searchIndex + metaMatch[0].length;
                    this._scannedIndex = this._searchIndex; // Синхронизируем с searchIndex

                    pilot = Action.FIND_ANY_TAG_MARKER;
                    secondPilot = Action.UNDEFINED;
                    break;
                }   // case PARSE_OPEN_TAG_METADATA

                // Ищем заголовки закрывающего/открывающего тега, кто первый. По результату либо переходим к приему
                // метаданных, либо сваливаемся в п. 2, прием метаданных открывающего тега.
                case Action.FIND_ANY_TAG_MARKER: {
                    // Ищем следующий OPEN/CLOSE, начиная с _searchIndex (после открывающего тега).
                    const openIndex = this.schlich.indexOf(OPEN, this._searchIndex);
                    const closeIndex = this.schlich.indexOf(CLOSE, this._searchIndex);

                    // Если раньше попался новый OPEN — значит предыдущий старт был ложным/искаженным.
                    // Без протокольных ошибок, просто отбрасываем гнилую голову и идем к приему метаданных нового OPEN.
                    if (openIndex !== -1 && (closeIndex === -1 || openIndex < closeIndex)) {
                        this.schlich = this.schlich.substring(openIndex);

                        if (window.Globals.IS_DEBUG) {
                            console.log(`Повторный открывающий маркер:
schlich="${this.schlich}"`);
                        }

                        this._searchIndex = OPEN.length; // конец сигнатуры нового OPEN
                        this._scannedIndex = this._searchIndex; // Синхронизируем, т.к. маркер уже обработан

                        pilot = Action.PARSE_OPEN_TAG_METADATA;
                        secondPilot = Action.UNDEFINED;
                        break;
                    }   // if

                    // CLOSE не найден — ждём допечатывания, сохраняя хвост.
                    if (closeIndex === -1) {
                        this._searchIndex = Math.max(0, this.schlich.length - SHORT_SAFETY_TAIL);
                        this._scannedIndex = this.schlich.length;

                        pilot = Action.CHECK_NEW_DATA_AVAILABILITY;
                        secondPilot = Action.FIND_ANY_TAG_MARKER; // Продолжим поиск закрывающего тега
                        break;
                    }   // if

                    // CLOSE найден — переходим к разбору его метаданных.
                    // Фиксируем позицию начала закрывающего тега для последующего парсинга.
                    this._searchIndex = closeIndex + CLOSE.length;  // Позиция после ">>>ai"
                    this._scannedIndex = this._searchIndex;         // Синхронизируем
                    pilot = Action.PARSE_CLOSE_TAG_METADATA;
                    secondPilot = Action.UNDEFINED;
                    break;
                }   // case FIND_ANY_TAG_MARKER

                // Принимаем метаданные - номер директивы(число) и идентификатор сессии закрывающего тега.
                // Валидирует формат и соответствие текущей сессии. При успехе сохраняет позицию конца директивы
                // для последующей вырезки.
                case Action.PARSE_CLOSE_TAG_METADATA: {

                    // this._searchIndex указывает на позицию ПОСЛЕ ">>>ai", т.е. на начало метаданных.
                    const tail = this.schlich.substring(this._searchIndex);

                    // Ожидаем метаданные: "DIRECTIVE_NUM SESSION_ID"
                    const metaRegex = /^\s+(\d+)\s+(\S{6})/;
                    const match = tail.match(metaRegex);

                    if (!match) {

                        // Если хвост достаточно длинный и метаданные всё равно не валидны — сбрасываемся в ноль.
                        if (tail.length >= FULL_SAFETY_TAIL) {

                            if (window.Globals.IS_DEBUG) {
                                console.log(`Закрывающий маркер не найден:
schlich="${this.schlich}"`);
                            }

                            // Обрезаем голову буфера вместе с маркером >>>ai, сбрасываем индексы, идем к п.0
                            this.schlich = this.schlich.substring(this._searchIndex);
                            this._searchIndex = 0;
                            this._scannedIndex = 0;
                            pilot = Action.CHECK_NEW_DATA_AVAILABILITY;
                            secondPilot = Action.FIND_OPEN_TAG_MARKER;
                            break;
                        }

                        // Ждем допечатывания параметров.
                        this._scannedIndex = this.schlich.length;
                        pilot = Action.CHECK_NEW_DATA_AVAILABILITY;
                        secondPilot = Action.PARSE_CLOSE_TAG_METADATA;
                        break;
                    }

                    // Сохраняем метаданные.
                    this._dirNumFromCloseTag = parseInt(match[1], 10);
                    this._sessIdFromCloseTag = match[2];

                    // Переходим к вырезке директивы. _searchIndex, _scannedIndex устанавливаем после конца директивы
                    this._searchIndex = this._searchIndex + match[0].length;
                    this._scannedIndex = this._searchIndex;
                    pilot = Action.EXTRACT_DIRECTIVE;
                    secondPilot = Action.UNDEFINED;
                    break;
                }   // case PARSE_CLOSE_TAG_METADATA

                // Вырезаем выделенную директиву из буфера. this._searchIndex указывает на позицию ПОСЛЕ конца
                // директивы (после ">>>ai DIRECTIVE_NUM SESSION_ID").
                case Action.EXTRACT_DIRECTIVE: {

                    // Вырезаем директиву от начала буфера до this._searchIndex (конец директивы).
                    // Сохраняем вырезанную директиву для последующей проверки и отправки.
                    this._extractedDirective = this.schlich.substring(0, this._searchIndex) + "\n";

                    // Удаляем вырезанную часть из буфера.
                    this.schlich = this.schlich.substring(this._searchIndex);

                    // Сбрасываем индексы: буфер начинается с нового содержимого.
                    this._searchIndex = 0;
                    this._scannedIndex = 0;

                    // Переходим к проверке и отправке директивы.
                    pilot = Action.CHECK_AND_SEND_DIRECTIVE;
                    secondPilot = Action.UNDEFINED;
                    break;

                }   // case EXTRACT_DIRECTIVE

                // Проверяем и отсылаем директиву.
                case Action.CHECK_AND_SEND_DIRECTIVE: {

                    // При любом исходе уйдем к п.0
                    pilot = Action.CHECK_NEW_DATA_AVAILABILITY;
                    secondPilot =  Action.FIND_OPEN_TAG_MARKER;
// console.log(`lastDirectiveNum=${this._lastDirectiveNum}, dirNumFromOpenTag=${this._dirNumFromOpenTag}`);
                    // Проверяем директиву по метаданным.
                    if (!this._checkDirectiveTags()) {

                        if (window.Globals.IS_DEBUG) {
                            console.log(`Директива не прошла проверку:
_extractedDirective="${this._extractedDirective}"`);
                        }

                        // Директива не прошла проверку — начинаем сначала.
                        break;
                    }   // if

                    // Директива валидна. Отправляем её агенту. Перед посылкой директивы поднимаем флаг, чтобы агент с
                    // гарантией получил фокус поля ввода для ответа.
                    try {
                        // При посылке директивы поднимаем флаг, чтобы агент с гарантией получил фокус поля ввода для ответа.
                        window.Globals.setIsAgentBusy(true);

                        await this.bridge.sendToAgent(this._extractedDirective);
                    } catch (e) {
                        this._reportProtocolErrorToAiAndUser?.(`[directive_extractor, CHECK_AND_SEND_DIRECTIVE]:
Ошибка отсылки директивы Хоботу. Ошибка ${e.message}`);

                        // При ошибке отправки тоже начинаем сначала.
                        break;
                    }   // try/catch

                    // Директива успешно отправлена. Учитываем её номер.
                    // Номер берём из закрывающего тега (он уже проверен на соответствие открывающему).
                    this._lastDirectiveNum = this._dirNumFromCloseTag;

                    // Начинаем поиск следующей директивы.
                    break;
                }   // case CHECK_AND_SEND_DIRECTIVE

                // Неопределённое состояние. Попадание сюда — программная ошибка.
                // Сообщаем оператору и сбрасываемся к поиску открывающего тега.
                case Action.UNDEFINED: {

                    this._reportProtocolErrorToUser?.(
                        `[directive_extractor, _processWebOutput] Программная ошибка: автомат перешёл в состояние UNDEFINED. Сброс.`
                    );

                    // Сбрасываем все состояния и начинаем с чистого листа.
                    this._searchIndex = 0;
                    this._scannedIndex = 0;

                    // Переходим к началу — поиску открывающего тега.
                    pilot = Action.CHECK_NEW_DATA_AVAILABILITY;
                    secondPilot =  Action.FIND_OPEN_TAG_MARKER;
                    break;
                }   // case UNDEFINED
            }   // switch
        }   // while
    }   // processWebOutput()

    /**
     * _checkDirectiveTags()
     *
     * Назначение:
     * Валидирует директиву по метаданным тегов (без анализа содержимого внутри скобок).
     * Использует сохранённые метаданные из _dirNumFromOpenTag, _sessIdFromOpenTag, _dirNumFromCloseTag, _sessIdFromCloseTag.
     *
     * Правила:
     * 1) Если session_id любой из скобок не совпадает с текущей сессией -> false (молча).
     * 2) Если номер директивы не число (уже проверено) -> false (молча).
     * 3) Если номер директивы <= последней обработанной -> false (молча).
     * 4) Если номер директивы > последней обработанной + 1 -> сообщаем оператору и AI, затем false.
     * 5) Если номер директивы открывающего тега не совпадает с закрывающей -> сообщаем оператору и AI, затем false.
     *
     * @returns {boolean} true, если директива валидна.
     */
    _checkDirectiveTags() {

        const currentSessionId = window.Globals.sessionId;
        if (!currentSessionId) {
            this._reportProtocolErrorToUser(`window.Globals.sessionId не инициализирован. Ошибка разработки.`);
            return false;
        }

        // 1) Проверка сессий.
        if (this._sessIdFromOpenTag !== currentSessionId || this._sessIdFromCloseTag !== currentSessionId) {
            // Молча игнорируем.
            return false;
        }

        // 2) Номера директив уже являются числами (парсились в соответствующих кейсах).
        // Проверяем соответствие друг другу.
        if (this._dirNumFromOpenTag !== this._dirNumFromCloseTag) {
            this._reportProtocolErrorToAiAndUser?.(
                `Номер директивы в открывающей и закрывающей скобках не совпадают. <<<ai =${this._dirNumFromOpenTag} >>>ai=${this._dirNumFromCloseTag}`
            );
            return false;
        }

        // 3) Проверка на повторную директиву.
        if (this._dirNumFromOpenTag <= this._lastDirectiveNum) {
            // Молча игнорируем.
            return false;
        }

        // 4) Проверка на разрыв последовательности.
        const expected = this._lastDirectiveNum + 1;
        if (this._dirNumFromOpenTag > expected) {
            this._reportProtocolErrorToAiAndUser?.(
                `Номер директивы прыгнул. ожидалось: ${expected}, получено: ${this._dirNumFromOpenTag}`
            );
            return false;
        }

        // Всё OK.
        return true;
    }

    /**
     * _processWebOutputLogging()
     *
     * Назначение:
     * Пересылать буфер как есть Хоботу для логирования. Нужно для отладки, чтобы увидеть сырой перехваченный текст.
     *
     * @return {Promise<void>}
     * @private
     */
    async _processWebOutputLogging() {

        // Пауза между проверками, когда в буфере "тишина".
        const SLEEP_MS = 100;

        while (!this._stopProcessing) {

            // Если буфер не пустой, отправляем, если пустой, ждем.
            if (this.schlich) {
// await this._delay(3000); // компенсация магии агента (RequestProcessor::read_raw_request() вставляет перевод строки в журнал)
                await this.bridge.sendToAgent(this.schlich);
                this.schlich = "";
            } else {

                // Если новых данных нет — спим.
                await this._delay(SLEEP_MS);
            }   // if

        }
    }

    /**
     * _reportProtocolErrorToUser()
     *
     * Назначение:
     * Сообщить о проблеме протокола пользователю.
     *
     * @param {string} msg
     */
    _reportProtocolErrorToUser(msg) {

        // Уведомить оператора.
        chrome.runtime.sendMessage({
            type: "EXTENSION_NOTIFY",
            title: "Ошибка протокола расширения",
            message: msg
        }).catch(() => {});

    }   // _reportProtocolError()

    /**
     * _reportProtocolErrorToAiAndUser()
     *
     * Назначение:
     * Сообщить о проблеме протокола AI и пользователю.
     *
     * @param {string} msg
     */
    _reportProtocolErrorToAiAndUser(msg) {

        // Послать сообщение об ошибке в сторону AI через Хобота.
        try {
            const errorPacket = {
                type: "PROTOCOL_ERROR",
                error_message: msg
            };
            const payload = `<<<ext\n${JSON.stringify(errorPacket)}\n>>>ext\n`;

            this.bridge?.sendToAgent?.(payload)?.catch?.(() => {});


        } catch (_) {
            // Игнорируем любые ошибки внутри обработчика ошибок.
        }   // try/catch

        // Уведомить оператора.
        chrome.runtime.sendMessage({
            type: "EXTENSION_NOTIFY",
            title: "Ошибка протокола расширения",
            message: msg
        }).catch(() => {});

    }   // _reportProtocolError()

    /**
     * _delay()
     *
     * Назначение:
     * Асинхронная пауза на N миллисекунд.
     *
     * @param {number} ms
     * @returns {Promise<void>}
     */
    _delay(ms) {

        return new Promise(resolve => setTimeout(resolve, ms));

    }   // _delay()

}   // DirectiveExtractor

// Экспортируем в глобальную область, чтобы content.js увидел класс
window.DirectiveExtractor = DirectiveExtractor;
