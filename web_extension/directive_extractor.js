/**
 * directive_extractor.js
 *
 * НАЗНАЧЕНИЕ:
 * Извлечение и валидация директив из текстового потока, полученного от TextProcessor.
 *
 * ТЕКУЩАЯ МОДЕЛЬ (после переработки):
 * 1) TextProcessor отправляет сюда ТЕКСТ code-block (<pre>) как строку.
 * 2) Здесь накапливается "сырой хвост" в webOutputBuffer (может быть незавершённым).
 * 3) Периодическая проверка периода тишины. Если в течение этого времени новая директива не была поймана и в буфере
 *    что-то есть, takeStabilizedBuffer() перебрасывает его в шлих.
 * 4) _processWebOutput() парсит schlich конечным автоматом и выделяет директивы <<<ai ... >>>ai.
 *
 * ОТВЕТСТВЕННОСТЬ:
 * 1) Накопление текста code-block в webOutputBuffer.
 * 2) Отложенная "сборка" данных по таймеру в schlich.
 * 3) Выделение директив, проверка session_id и строгой последовательности DIRECTIVE_NUM.
 * 4) Отправка валидных директив агенту через bridge.sendToAgent().
 */

// Таймаут молчания для захвата директивы. Если MutationObserver молчит дольше этого времени, то считаем что AI
// высказался и ждет. То есть директива готова, можно ее отсылать агенту. Время в мс.
const DIRECTIVE_COMPLETION_TIMEOUT = 1000;

// Ограничение на длину сообщения об ошибке.
const ERROR_MSG_MAX_LEN = 100;

class DirectiveExtractor {

    constructor(bridge) {

        // Интерфейс к Хоботу через фоновый скрипт.
        this.bridge = bridge;

        // Привязка к текстовому процессору. Понадобилась для доступа к флагу textProcessor.isDirectiveCaught из
        // _moveLastDirectiveToSchlich(). Устанавливается сеттером из content.js.
        this.textProcessor = null;

        // Перед передачей текста в acceptWebOutput() text_processor устанавливает это поле в реальный тип парсинга.
        this.webParsingType = window.Globals.webParsingType;

        // Принятое от текстового процессора содержимое текстового блока. Это будет либо директива, либо мусор, если
        // пойман неправильный блок. После периода молчания буфер проверяется на наличие директивы. Если она там есть,
        // она перекладывается в шлих.
        this._webOutputBuffer = "";

        // --- Переменные управления processWebOutput() ---

        // Шлих - сырая порода, из которой processWebOutput() будет извлекать директиву. К этой строке добавляется
        // содержимое webOutputBuffer, который после этого очищается.
        this._schlich = "";

        // Копия последней принятой директивы. Используется для выявления дублей.
        this._lastDirective = "";

        // Номер последней отправленной директивы
        this._lastDirectiveNum = 0;

        // Указатель, с какой позиции начинать следующий поиск в шлихе.
        this._searchIndex = 0;

        // Индекс не просмотренной части шлиха.
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
        this.takeStabilizedBuffer();
    }   // constructor()

    /**
     * Принимает строку текста из code-block и сохраняет в webOutputBuffer.
     *
     * Контракт:
     * - Вход: string (обычно innerText контейнера <pre>).
     * - Выход: void.
     *
     * @param {string} newText - Текущий текст code-block (обычно <pre>.innerText).
     */
    acceptWebOutput(newText) {

        this._webOutputBuffer = newText;
    }   // acceptWebOutput()

    /**
     * setTextProcessor()
     *
     * Назначение:
     * Установить ссылку на TextProcessor после создания.
     * Нужно из-за циклической зависимости: DirectiveExtractor создаётся раньше TextProcessor,
     * но должен иметь доступ к его флагу isDirectiveCaught для сброса.
     *
     * @param {TextProcessor} textProcessor
     */
    setTextProcessor(textProcessor) {
        this.textProcessor = textProcessor;
    }   // setTextProcessor()

    /**
     * takeStabilizedBuffer()
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
    takeStabilizedBuffer() {

        setInterval(() => {
            // Если в течение всего
            if (this.textProcessor.wasThereSilence && this._webOutputBuffer.length !== 0) {
                this._moveLastDirectiveToSchlich();
            }

            // Снова заряжаем флаг тишины.
            this.textProcessor.wasThereSilence = true;
        }, DIRECTIVE_COMPLETION_TIMEOUT);
    }   // takeStabilizedBuffer()

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

        // Берем ид сессии. Если его еще нет, то любой текст в буфере считается мусором.
        const currentSessionId = window.Globals?.sessionId;
        if (!currentSessionId) {
            this._webOutputBuffer = "";
            return false;
        }

        // Сокращения для скобок директивы.
        const BRA = window.Globals.DIRECTIVE_BRACKET.BRA;
        const KET = window.Globals.DIRECTIVE_BRACKET.KET;

        // При пустом буфере делать нечего, возвращаемся. Попутно перекодируем NBSP в обычные пробелы, последующий
        // код, что в расширении, что в агенте их не ждет.
        const buf = this._webOutputBuffer.replace(/[\u00A0\u202F]/g, " ") || "";
        if (buf.length === 0) {
            this._webOutputBuffer = "";
            return false;
        }

        /**
         * Ищет в направлении от хвоста к голове ближайший валидный тег (BRA или KET) с метаданными и текущим
         * session_id в диапазоне [0..fromExclusive).
         *
         * @param {number} fromExclusive
         *
         * @returns {{ kind: "bra"|"ket", index: number, endIndex: number, dirNum: number } | null}
         * где index - индекс начала маркера, endIndex - индекс вслед за метаданными.
         */
        const findPrevValidTag = (fromExclusive) => {

            // let cursor = Math.min(fromExclusive, buf.length);
            let cursor = fromExclusive;

            while (cursor > 0) {

                const searchPos = cursor - 1;

                const openIdx = buf.lastIndexOf(BRA, searchPos);
                const closeIdx = buf.lastIndexOf(KET, searchPos);

                if (openIdx === -1 && closeIdx === -1) {
                    return null;
                }

                const isOpen = openIdx > closeIdx;
                const idx = isOpen ? openIdx : closeIdx;
                const markerLen = isOpen ? BRA.length : KET.length;

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
                    kind: isOpen ? "bra" : "ket",
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
            this._webOutputBuffer = ""; // всё мусор
            return false;
        }

        // 2) Если первым попался открывающий тег — оставляем хвост (вдруг допечатается позже), но в schlich не переносим.
        if (lastTag.kind === "bra") {
            this._webOutputBuffer = buf.substring(lastTag.index); // чистим мусор до OPEN
            return false;
        }

        // 3) Первым попался CLOSE — ищем пару OPEN, двигаясь к началу.
        let mainClose = lastTag;

        while (true) {

            const prevTag = findPrevValidTag(mainClose.index);
            if (!prevTag) {
                // CLOSE есть, OPEN не нашли — считаем всё мусором
                this._webOutputBuffer = "";
                return false;
            }

            if (prevTag.kind === "ket") {
                // По ТЗ: если встретили CLOSE — он становится "основным"
                mainClose = prevTag;
                continue;
            }

            // --- 4) Нашли последнюю директиву, забираем ее в шлих, остальное выбрасываем. Корректируем флаги парсинга. ---

            // 4.1 Проверяем, не является ли директива дупликатом предыдущей. Для скорости, сначала сравниваем длины
            // старой и новой директив.
            const directive = buf.substring(prevTag.index, mainClose.endIndex) + "\n";
            if (directive.length === this._lastDirective.length && directive === this._lastDirective) {

                // Новая директива идентична старой. Игнорируем ее, очищая буфер - она мусор.
                this._webOutputBuffer = "";
                return false;
            }

            // Директива отличается от прошлой, пускаем ее в дело.
            this._schlich += directive;
            this._lastDirective = directive;
            this._webOutputBuffer = "";

            // 4.2 Сбрасываем флаг "пойман конец директивы" в текстовом процессоре, чтобы обеспечить переход
            //     к ловле следующей директивы.
            textProcessor.isDirectiveCaught = false;

            // 4.3 Подтверждаем что директива выделена при таком-то типе парсинга веб-сайта, чтобы дальше не парсить
            //     только по этому типу.
            window.Globals.webParsingType = this.webParsingType;

            // 4.4 Возможно, пишем шлих в журнал.
            if (window.Globals.IS_DEBUG) {
                console.log(`schlich="${this._schlich}"`);
            }

            return true;
        }
    }   // _moveLastDirectiveToSchlichOrDrop()

    /**
     * _processWebOutput()
     *
     * Назначение:
     * Фоновый асинхронный цикл обработки принятого от текстового процессора материала.
     * Реализует конечный автомат (State Machine) для выделения директив, оформленных согласно протоколу.
     *
     * Примечание. Метод был построен в расчете на то, что данные поступают постепенно, хотя в нынешней реализации
     *   это не так. Через шлих передается для проверки и обработки целая тримованная директива с переводом строки в конце.
     *   Поэтому, код можно было бы упростить, но пока так, тем более, что код рабочий.
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
     * 7. Вырезка готовой директивы из шлиха.
     * 8. Проверка целостности директивы и отправка агенту.
     * 9. Возврат к поиску следующей директивы.
     *
     * # Паника
     * - При программной ошибке (переход в `UNDEFINED`) сбрасывает состояние и уведомляет оператора.
     * - Критические ошибки парсинга приводят к очистке шлиха и возврату в начальное состояние.
     *
     * # Побочные эффекты
     * - Мутирует `schlich` (удаляет обработанные части).
     * - Обновляет внутренние поля (`_searchIndex`, `_scannedIndex`, `_lastDirectiveNum`).
     * - Отправляет директивы агенту через `this.bridge.sendToAgent`.
     * - Генерирует уведомления об ошибках протокола.
     *
     * # Возвращаемое значение
     * - Promise, который запускает бесконечный цикл обработки шлиха.
     */
    async _processWebOutput(){

        const BRA = window.Globals.DIRECTIVE_BRACKET.BRA;
        const KET = window.Globals.DIRECTIVE_BRACKET.KET;

        // Пауза между проверками, когда ждем новых данных в шлихе.
        const SLEEP_MS = 100;

        // Если тега "<<<ai" нет, считаем накопленный текст мусором.
        // Чтобы мусор не раздувал память, после порога режем буфер.
        const MAX_GARBAGE_LENGTH = 2000;

        // Хвост, который оставляем для поиска открывающего тега (без параметров):
        // - при обрезке мусора;
        // - при неуспешном поиске (чтобы "добрать" частично допечатанный тег на следующем тике).
        const SHORT_SAFETY_TAIL = Math.max(BRA.length, KET.length) - 1;

        // Хвост, который оставляем для поиска закрывающего тега с параметрами. Хватило бы KET.length+15, включая
        // закрывающий пробел, но сделаем запас. Вдруг придет фантазия увеличить иды.
        const FULL_SAFETY_TAIL = KET.length + 20;

        // Действия
        const Action = Object.freeze({

            // Неопределённое состояние. Второй пилот принимает его, когда достаточно первого.
            UNDEFINED: -1,

            // Проверяем есть ли в буфере данные, готовые для анализа.
            CHECK_NEW_DATA_AVAILABILITY: 0,

            // Ищем заголовок открывающего тега <<<ai.
            FIND_OPEN_TAG_MARKER: 1,

            // Принимаем метаданные - номер директивы(число) и идентификатор сессии открывающего тега.
            PARSE_BRA_METADATA: 2,

            // Ищем заголовки закрывающего/открывающего тега, кто первый.
            FIND_ANY_TAG_MARKER: 3,

            // Принимаем метаданные - номер директивы(число) и идентификатор сессии закрывающего тега.
            PARSE_KET_METADATA: 4,

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

        // Бесконечный цикл. Остаемся в нем все время работы расширения. Снаружи, асинхронно для нас, шлих дописывается
        // новыми данными. В текущей реализации это целые директивы. Сейчас цикл рассчитан на дописывание частями.
        // В текущей реализации это излишнее усложнение, но пока архитектура парсинга не проверена, оставляем так.
        while (!this._stopProcessing) {
            switch (pilot) {

                // Проверяем есть ли в буфере данные, готовые для анализа. По результату, либо идем к поиску открывающего
                // тега, либо выжидаем таймаут и идем на себя.
                case Action.CHECK_NEW_DATA_AVAILABILITY: {

                    // Если новых данных нет — спим.
                    if (this._scannedIndex >= this._schlich.length) {
                        await this._delay(SLEEP_MS);

                        // Новых данных нет, ждем. Второй пилот знает куда идти, когда придут данные, его не трогаем.
                        pilot = Action.CHECK_NEW_DATA_AVAILABILITY;

                        break;
                    }   // if

                    // Есть данные для обработки. Переходим туда, куда указывает второй пилот.
                    pilot = secondPilot;
                    secondPilot = Action.UNDEFINED;
                    break;
                }   // case CHECK_NEW_DATA_AVAILABILITY

                // Ищем заголовок открывающего тега <<<ai. Просто заголовок, без метаданных. Попутно подчищаем головной мусор.
                // Если найден, идем к приему метаданных, если нет, в начало, к ожиданию данных (пункт 0).
                case Action.FIND_OPEN_TAG_MARKER: {
                    const openIndex = this._schlich.indexOf(BRA, this._scannedIndex);

                    if (openIndex === -1) {

                        // Маркера нет — весь буфер просмотрен. Если буфер разросся, считаем его мусором и режем,
                        // оставляя хвост который может содержать частичный открывающий тег.
                        if (this._schlich.length > MAX_GARBAGE_LENGTH) {

                            // Чистим мусор - почти весь буфер.
                            this._schlich = this._schlich.slice(-SHORT_SAFETY_TAIL);

                            // После физической обрезки буфера индексы теряют смысл — начинаем заново.
                            this._searchIndex = 0;
                            this._scannedIndex = 0;
                        } else {

                            // Буфер пока маленький, не чистим, корректируем указатель просмотренного текста,
                            // с учетом того что мог остаться нераспознанный зачаток тега в конце.
                            this._scannedIndex = this._schlich.length;
                        }   // if/else

                        // Идем к п.0
                        pilot = Action.CHECK_NEW_DATA_AVAILABILITY;
                        secondPilot = Action.FIND_OPEN_TAG_MARKER; // Сохраняем цель поиска
                        break;
                    }   // if

                    // Маркер найден. Всё слева от него — мусор, отбрасываем.
                    if (openIndex > 0) {
                        this._schlich = this._schlich.substring(openIndex);
                    }   // if

                    // Как только нашли сигнатуру BRA, двигаем _searchIndex в конец "<<<ai".
                    // Далее в PARSE_OPEN_TAG_METADATA будем смотреть ТОЛЬКО метаданные после сигнатуры.
                    this._searchIndex = BRA.length;
                    this._scannedIndex = this._searchIndex;

                    // Мы нашли маркер открывающего тега, переходим к разбору его метаданных.
                    pilot = Action.PARSE_BRA_METADATA;
                    secondPilot = Action.UNDEFINED;
                    break;
                }   // case FIND_OPEN_TAG_MARKER

                // Принимаем метаданные - номер директивы(число) и идентификатор сессии открывающего тега. По результату,
                // либо переходим к поиску любого тега, либо сваливаемся в пункт 0.
                case Action.PARSE_BRA_METADATA: {
                    // Тело после "<<<ai". _searchIndex, _scannedIndex указывают на пробел после маркера.
                    const tail = this._schlich.substring(this._searchIndex);

                    // Парсим ТОЛЬКО метаданные (без "<<<ai" в regex)
                    const metaRegex = new RegExp(`^\\s+(\\d+)\\s+(\\S{6})`);
                    const metaMatch = tail.match(metaRegex);

                    if (!metaMatch) {
                        // Метаданных пока нет или это псевдо-тег из текста.
                        // Если данных после сигнатуры мало — идем за новыми данными, потом возвращаемся в наш кейс.
                        if (tail.length < FULL_SAFETY_TAIL) {
                            this._scannedIndex = this._schlich.length;
                            pilot = Action.CHECK_NEW_DATA_AVAILABILITY;
                            secondPilot = Action.PARSE_BRA_METADATA;
                            break;
                        }   // if

                        this._reportProtocolErrorToAiAndUser(
                            `Невалидный заголовок директивы: "${this._schlich.substring(this._searchIndex, ERROR_MSG_MAX_LEN)}"`);

                        if (window.Globals.IS_DEBUG) {
                            console.log(`Невалидный заголовок директивы:
schlich="${this._schlich}"`);
                        }

                        // Если хвост уже большой, а метаданные так и не появились — считаем все начало буфера,
                        // включая открывающий маркер мусором и идем к поиску следующего BRA.
                        this._schlich = this._schlich.substring(BRA.length);
                        this._searchIndex = 0;
                        this._scannedIndex = 0;

                        pilot = Action.CHECK_NEW_DATA_AVAILABILITY;
                        secondPilot = Action.FIND_OPEN_TAG_MARKER;
                        break;
                    }   // if

                    // Метаданные получены.
                    const dirNumFromOpenTag = parseInt(metaMatch[1], 10);
                    const sessIdFromOpenTag = metaMatch[2];

                    // Чужая сессия или прошлая директива — молча игнорируем, ищем следующий BRA.
                    if (sessIdFromOpenTag !== window.Globals.sessionId ||
                        dirNumFromOpenTag <= this._lastDirectiveNum) {

                        if (sessIdFromOpenTag !== window.Globals.sessionId) {
                            this._reportProtocolErrorToAiAndUser(
                                `Текущая сессия ${window.Globals.sessionId}, но использована ${sessIdFromOpenTag}:
"${this._schlich.substring(this._searchIndex, ERROR_MSG_MAX_LEN)}"`);
                        }
                        if (dirNumFromOpenTag <= this._lastDirectiveNum) {
                            this._reportProtocolErrorToAiAndUser(
                                `Ожидается директива ${this._lastDirectiveNum + 1}, но использован номер ${dirNumFromOpenTag}:
"${this._schlich.substring(this._searchIndex, ERROR_MSG_MAX_LEN)}"`);
                        }

                        if (window.Globals.IS_DEBUG) {
                            console.error(`Чужая сессия или прошлая директива:
schlich="${this._schlich}"`);
                        }

                        // Отрезаем голову вместе с открывающим маркером.
                        this._schlich = this._schlich.substring(BRA.length);
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
                    // Ищем следующий BRA/KET, начиная с _searchIndex (после открывающего тега).
                    const openIndex = this._schlich.indexOf(BRA, this._searchIndex);
                    const closeIndex = this._schlich.indexOf(KET, this._searchIndex);

                    // Если раньше попался новый BRA — значит предыдущий старт был ложным/искаженным.
                    // Без протокольных ошибок, просто отбрасываем гнилую голову и идем к приему метаданных нового BRA.
                    if (openIndex !== -1 && (closeIndex === -1 || openIndex < closeIndex)) {
                        this._schlich = this._schlich.substring(openIndex);

                        if (window.Globals.IS_DEBUG) {
                            console.error(`Повторный открывающий маркер:
schlich="${this._schlich}"`);
                        }

                        this._searchIndex = BRA.length; // конец сигнатуры нового BRA
                        this._scannedIndex = this._searchIndex; // Синхронизируем, т.к. маркер уже обработан

                        pilot = Action.PARSE_BRA_METADATA;
                        secondPilot = Action.UNDEFINED;
                        break;
                    }   // if

                    // KET не найден — ждём допечатывания, сохраняя хвост.
                    if (closeIndex === -1) {
                        this._searchIndex = Math.max(0, this._schlich.length - SHORT_SAFETY_TAIL);
                        this._scannedIndex = this._schlich.length;

                        pilot = Action.CHECK_NEW_DATA_AVAILABILITY;
                        secondPilot = Action.FIND_ANY_TAG_MARKER; // Продолжим поиск закрывающего тега
                        break;
                    }   // if

                    // KET найден — переходим к разбору его метаданных.
                    // Фиксируем позицию начала закрывающего тега для последующего парсинга.
                    this._searchIndex = closeIndex + KET.length;  // Позиция после ">>>ai"
                    this._scannedIndex = this._searchIndex;         // Синхронизируем
                    pilot = Action.PARSE_KET_METADATA;
                    secondPilot = Action.UNDEFINED;
                    break;
                }   // case FIND_ANY_TAG_MARKER

                // Принимаем метаданные - номер директивы(число) и идентификатор сессии закрывающего тега.
                // Валидирует формат и соответствие текущей сессии. При успехе сохраняет позицию конца директивы
                // для последующей вырезки.
                case Action.PARSE_KET_METADATA: {

                    // this._searchIndex указывает на позицию ПОСЛЕ ">>>ai", т.е. на начало метаданных.
                    const tail = this._schlich.substring(this._searchIndex);

                    // Ожидаем метаданные: "DIRECTIVE_NUM SESSION_ID"
                    const metaRegex = /^\s+(\d+)\s+(\S{6})/;
                    const match = tail.match(metaRegex);

                    if (!match) {

                        // Если хвост достаточно длинный и метаданные всё равно не валидны — сбрасываемся в ноль.
                        if (tail.length >= FULL_SAFETY_TAIL) {

                            if (window.Globals.IS_DEBUG) {
                                console.error(`Закрывающий маркер не найден:
schlich="${this._schlich}"`);
                            }

                            // Обрезаем голову буфера вместе с маркером >>>ai, сбрасываем индексы, идем к п.0
                            this._schlich = this._schlich.substring(this._searchIndex);
                            this._searchIndex = 0;
                            this._scannedIndex = 0;
                            pilot = Action.CHECK_NEW_DATA_AVAILABILITY;
                            secondPilot = Action.FIND_OPEN_TAG_MARKER;
                            break;
                        }

                        // Ждем допечатывания параметров.
                        this._scannedIndex = this._schlich.length;
                        pilot = Action.CHECK_NEW_DATA_AVAILABILITY;
                        secondPilot = Action.PARSE_KET_METADATA;
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
                    this._extractedDirective = this._schlich.substring(0, this._searchIndex) + "\n";

                    // Удаляем вырезанную часть из буфера.
                    this._schlich = this._schlich.substring(this._searchIndex);

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
                    secondPilot = Action.FIND_OPEN_TAG_MARKER;

                    // Проверяем директиву по метаданным.
                    if (!this._checkDirectiveTags()) {

                        if (window.Globals.IS_DEBUG) {
                            console.error(`Директива не прошла проверку:
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
                        this._reportProtocolErrorToAiAndUser(`[directive_extractor, CHECK_AND_SEND_DIRECTIVE]:
Ошибка отсылки директивы Хоботу. Ошибка ${e.message}`);

                        // При ошибке отправки тоже начинаем сначала.
                        break;
                    }   // try/catch

                    // Номер следующей директивы берём из закрывающего тега (он уже проверен на соответствие открывающему).
                    this._lastDirectiveNum = this._dirNumFromCloseTag;

                    // Начинаем поиск следующей директивы.
                    break;
                }   // case CHECK_AND_SEND_DIRECTIVE

                // Неопределённое состояние. Попадание сюда — программная ошибка.
                // Сообщаем оператору и сбрасываемся к поиску открывающего тега.
                case Action.UNDEFINED: {

                    console.log(
                        `[directive_extractor, _processWebOutput] Программная ошибка: автомат перешёл в состояние UNDEFINED. Сброс.`
                    );

                    // Сбрасываем все состояния и начинаем с чистого листа.
                    this._searchIndex = 0;
                    this._scannedIndex = 0;

                    // Переходим к началу — поиску открывающего тега.
                    pilot = Action.CHECK_NEW_DATA_AVAILABILITY;
                    secondPilot = Action.FIND_OPEN_TAG_MARKER;
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
            this._reportProtocolErrorToAiAndUser(
                `Номер директивы в открывающей и закрывающей скобках не совпадают. <<<ai =${this._dirNumFromOpenTag} >>>ai=${this._dirNumFromCloseTag}`
            );
            return false;
        }

        // 3) Проверка на повторную директиву.
        if (this._dirNumFromOpenTag <= this._lastDirectiveNum) {
            this._reportProtocolErrorToAiAndUser(
                `Номер директивы повторился. ожидалось: ${this._lastDirectiveNum + 1}, получено: ${this._dirNumFromOpenTag}`
            );
            return false;
        }

        // 4) Проверка на разрыв последовательности.
        const expected = this._lastDirectiveNum + 1;
        if (this._dirNumFromOpenTag > expected) {
            this._reportProtocolErrorToAiAndUser(
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
            if (this._schlich) {
// await this._delay(3000); // компенсация магии агента (RequestProcessor::read_raw_request() вставляет перевод строки в журнал)
                await this.bridge.sendToAgent(this._schlich);
                this._schlich = "";
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

        console.error(msg);

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
     * Примечание:
     *      Произошло на chat.deepseek.com - периодически стал теряться таймер, что приводило к вечному ожиданию.
     *  Решение: добавил страховочный таймер. Будет срабатывать хотя бы один из двух.
     *
     * @param {number} ms
     * @returns {Promise<void>}
     */
    _delay(ms) {

        return new Promise(resolve => {
            let shortTimer = setTimeout(() => {
                clearTimeout(longTimer);
                resolve();
            }, ms);

            let longTimer = setTimeout(() => {
                console.log(`[directive_extractor::_delay]: короткий таймер не сработал, сработала страховка длинным таймером`);
                clearTimeout(shortTimer);
                resolve();
            }, 10*ms)
        });
    }   // _delay()

}   // DirectiveExtractor

// Экспортируем в глобальную область, чтобы content.js увидел класс
window.DirectiveExtractor = DirectiveExtractor;
