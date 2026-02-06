//! request.rs
//!
//! Модуль, управляющий жизненным циклом обработки одного запроса.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Чтение сырых данных из входного потока (Native Messaging).
//! - Снятие "транспортной обертки" (скобки `<<<ai ... >>>ai`).
//! - Маршрутизация данных в соответствующие контексты (`DirectiveContext`, `InitContext`, `ExtensionMsgContext`).
//! - Оркестрация выполнения команд и формирования отчета.

use std::io;
use std::io::Read;
use std::sync::OnceLock;
use regex::Regex;
use serde::Deserialize;

use crate::glob::error_control::AgentError;
use directive::DirectiveProcessor;
use ext_msg::ExtensionMessageContext;
use crate::glob;

// Внутренние модули контекстов
pub(crate) mod directive;
pub(crate) mod report;
mod ext_msg;
#[cfg(test)]
mod test_request_test;
pub(crate) mod session;
pub(crate) mod request_reader;

/// Источник входящего запроса, определенный по маркерам протокола.
#[derive(Debug, Clone, PartialEq)]
pub enum RequestSource {

    /// Директива от AI (содержит команды).
    Ai,

    /// Пакет от расширения (инициализация, остановка, сообщения об ошибках протокола).
    Extension,
}   // enum RequestType

/// Результат обработки одного входящего сообщения.
///
/// Используется агентом, чтобы понимать, был ли обработан пакет директивы ИИ,
/// и если да — какие метаданные директивы нужно использовать для протокольных сигналов.
#[derive(Debug, Clone, PartialEq)]
pub struct ProcessedRequest {
    pub req_type: RequestSource,
    pub dir_header: Option<(u32, String)>, // (dir_id, session_id) только для AiDirective
}   // ProcessedRequest

pub struct RequestProcessor {

    /// Контекст для приема и обработки директивы AI.
    directive_processor: DirectiveProcessor,

    /// Контекст обработки служебных сообщений (ошибок протокола) от расширения.
    extension_error_ctx: ExtensionMessageContext,

    /// Флаг, указывающий на необходимость штатного завершения работы агента (получен сигнал COMPLETION).
    /// Не сбрасывается. Используется один раз при завершении Хобота.
    pub(crate)  is_hobot_completion_requested: bool,
}   // struct RequestProcessor

impl RequestProcessor {
    pub fn new() -> Self {
        Self {
            directive_processor: DirectiveProcessor::new(),
            extension_error_ctx: ExtensionMessageContext::new(),
            is_hobot_completion_requested: false,
        }
    }   // new()

    /// Обрабатывает сырой текстовый запрос: снимает обертку, парсит и запускает исполнение.
    ///
    /// # Алгоритм
    /// 1. Вызывает `_unwrap_brackets` для получения типа запроса и тела JSON.
    /// 2. Для `AiDirective`:
    ///    - Инициализирует `CommandProcessor` (отчет) полученным ID.
    ///    - Парсит список команд в `DirectiveContext`.
    ///    - Если парсинг успешен — передает команды на выполнение в `CommandProcessor`.
    ///    - Если ошибка парсинга или выполнения — фиксирует её в отчете (через `set_dir_error`).
    /// 3. Для `ExtensionInfo`:
    ///    - Определяет тип служебного сообщения (INIT, COMPLETION, PROTOCOL_ERROR).
    ///    - Делегирует обработку соответствующему контексту (`init_ctx` или `extension_msg_ctx`).
    ///
    /// # Возвращаемое значение
    /// `Result<(RequestType, Option(String)), AgentError>`: Тип источник и тип обработанного запроса.
    pub fn process_request(&mut self, raw_request_text: &str)
        -> Result<(RequestSource, Option<String>), AgentError>
    {
        // 1. Распаковка запроса - убирает теги, для директивы парсит заголовок.
        let (req_type, json_body, dir_header) =
            self._unwrap_brackets(raw_request_text)?;

        // 2. Маршрутизация
        match req_type {
            RequestSource::Ai => {

                // 1. Заголовок обязателен для AI-директивы. Забираем id директивы и id сессии.
                let (dir_id, session_id) = dir_header.ok_or_else(|| {
                    AgentError::Critical("нет заголовка директивы для RequestType::AiDirective".to_string())
                })?;

                // 2. Парсим тело директивы, исполняем команды, строим отчет.
                self.directive_processor
                    .process_directive(&json_body, dir_id, session_id.clone())?;

                // 3. Возвращаем тип отработанного запроса. Поскольку это директива, передаем
                // идентификатор сессии (директива).
                Ok((RequestSource::Ai, None))
            },

            RequestSource::Extension => {
                // 1. Минимальный конверт: читаем только тип.
                
                // 1.1 Обертка для чтения типа сообщения
                #[derive(Deserialize)]
                struct ExtRequestWrapper {
                    #[serde(rename = "type")]
                    msg_type: String,
                }   // ExtTypeEnvelope

                // 1.2 Читаем JSON (извлекаем тип сообщения)
                let wrapper: ExtRequestWrapper = serde_json::from_str(&json_body).map_err(|e| {
                    AgentError::Recoverable(format!(r#"
{}, {}: oшибка в JSON сообщения расширения.
JSON:
{}

oшибка: {}"#, file!(), line!(), &json_body, e))})?;

                // 3. Маршрутизация по типу сообщения.
                let msg_type = wrapper.msg_type.as_str();
                match msg_type {
                    
                    // 3.1 Принимаем пакет инициализации.
                    glob::EXT_MSG_TYPE_INIT_SESSION => {
                        session::init_session_context(&json_body)?;
                    }

                    glob::EXT_MSG_TYPE_PROTOCOL_ERROR => {
                        self.extension_error_ctx
                            .handle_extension_message_request(&json_body, &session::session_id()?)?;
                    }

                    // 3.2 Принимаем команду завершения.
                    glob::EXT_MSG_TYPE_COMPLETION => {
                        self.is_hobot_completion_requested = true;
                        self._build_completion_report()?;
                    },

                    // 3.3 Нераспознанные типы сообщения, возвращаем ошибку.
                    msg_type => {
                        return Err(AgentError::Recoverable(format!(
                            "{}, {}: неизвестный тип EXT сообщения: {}", file!(), line!(), msg_type)));
                    }
                }   // match msg_type

                Ok((RequestSource::Extension, Some(msg_type.to_string())))
            }   // RequestType::ExtensionInfo
        }   // match req_type
    }   // process_request()

    pub fn is_report_empty(&self) -> bool {
        report::is_report_empty().unwrap()
    }

    /// Сбрасывает состояние всех внутренних контекстов.
    ///
    /// Вызывается в начале каждой итерации цикла обработки (`Agent::run`),
    /// чтобы гарантировать, что данные предыдущей директивы (ID, команды, результаты)
    /// не повлияют на обработку новой.
    pub fn clear(&mut self) {
        let _ = report::clear();
        self.directive_processor.clear();
        self.extension_error_ctx.clear();
    }   // clear()
}   // impl RequestProcessor

// Внутренний интерфейс. _unwrap_brackets() и его обслуга.
impl RequestProcessor {

    /// Раскрывает теги расширения/ИИ, извлекает и возвращает JSON-тело запроса.
    ///
    /// Определяет тип сообщения по маркерам (`<<<ext` или `<<<ai`), валидирует структуру
    /// конверта и извлекает JSON-тело.
    ///
    /// # Алгоритм работы
    /// 1. **EXT**:
    ///    - Ищет `<<<ext`.
    ///    - Ищет `>>>ext` через `rfind`, извлекает тело между тегами.
    ///    - Возвращает `(ExtensionInfo, body, None)`.
    /// 2. **AI**:
    ///    - Парсит заголовок `<<<ai DIR_ID SESSION_ID` через regex с якорем `^`.
    ///    - Ищет `>>>ai` с конца строки через `rfind` (оптимизация для больших JSON).
    ///    - Валидирует структуру хвоста `>>>ai DIR_ID SESSION_ID` локальной regex.
    ///    - Сверяет `DIR_ID` и `SESSION_ID` между открывающим и закрывающим тегами.
    ///    - Извлекает тело между заголовком и хвостом.
    ///    - Возвращает `(AiDirective, body, Some((DIR_ID, SESSION_ID)))`.
    ///
    /// # Параметры
    /// - `raw_request_text`: Полный текст сообщения из Native Messaging (с тегами протокола).
    ///
    /// # Возвращаемое значение
    /// `Result<(RequestType, String, Option<(u32, String)>), AgentError>`:
    /// - Тип запроса.
    /// - JSON-тело (без тегов).
    /// - Заголовок директивы (dir_id, session_id) только для `<<<ai ... >>>ai`.
    ///
    /// # Ошибки
    /// - `AgentError::Critical`: Ошибка компиляции регулярок, пересечение тегов.
    /// - `AgentError::Recoverable`:
    ///     - Неверный формат тегов.
    ///     - Несоответствие SessionID контексту инициализации.
    ///     - Несовпадение ID/SESSION между началом и концом.
    ///     - Пустое тело.
    fn _unwrap_brackets(&mut self, raw_request_text: &str)
                        -> Result<(RequestSource, String, Option<(u32, String)>), AgentError> {
        let text = raw_request_text.trim();

        // --- 1. Обработка EXT ---
        if text.starts_with("<<<ext") {
            let body = self._unwrap_ext_tags(text)?;
            return Ok((RequestSource::Extension, body, None));
        }   // if ext

        // --- 2. Обработка AI ---
        if text.starts_with("<<<ai") {

            // Парсим открывающий тег.
            let (start_id, start_sess, body_start_pos) = self._parse_opening_directive_tag(text)?;

            // Парсим закрывающий тег.
            let (end_id, end_sess, body_end_pos) = self._parse_closing_directive_tag(text)?;

            // 2.3 Сверки
            if start_id != end_id {
                return Err(AgentError::Recoverable(format!(
                    "несовпадение dir_id: открытие={}, закрытие={}", start_id, end_id
                )));
            }   // if

            if start_sess != end_sess {
                return Err(AgentError::Recoverable(format!(
                    "несовпадение session_id: '{}' != '{}'", start_sess, end_sess
                )));
            }   // if

            // Защита от паники слайса (если заголовок "съел" начало хвостовика)
            if body_start_pos > body_end_pos {
                return Err(AgentError::Critical(
                    "программная ошибка: пересечение заголовка и хвостовика".to_string()
                ));
            }   // if

            let body = text[body_start_pos..body_end_pos].trim().to_string();

            // Явная проверка на пустоту
            if body.is_empty() {
                return Err(AgentError::Recoverable("пустое тело директивы".to_string()));
            }   // if

            // 2.5 Возвращаем метаданные директивы явно, без побочных эффектов.
            return Ok((RequestSource::Ai, body, Some((start_id, start_sess))));
        }   // if ai

        // Если не подошло ни то, ни другое
        Err(AgentError::Recoverable(
            "неизвестный формат сообщения (не <<<ai и не <<<ext)".to_string()
        ))
    }   // _unwrap_brackets()

    /// Извлекает тело сообщения расширения из тегов `<<<ext ... >>>ext`.
    ///
    /// # Логика
    /// 1. Вычисляет позицию начала тела (после `<<<ext`).
    /// 2. Ищет закрывающий тег `>>>ext` с конца строки через `rfind`.
    /// 3. Извлекает и триммит тело между тегами.
    /// 4. Проверяет, что тело не пустое.
    ///
    /// # Параметры
    /// - `text`: Полный текст сообщения (начинающийся с `<<<ext`).
    ///
    /// # Возвращаемое значение
    /// - `Ok(String)`: Тело сообщения (JSON без тегов).
    ///
    /// # Ошибки
    /// - `AgentError::Recoverable`: Отсутствует закрывающий тег, некорректные границы, пустое тело.
    fn _unwrap_ext_tags(&self, text: &str) -> Result<String, AgentError> {
        let body_start_pos = "<<<ext".len();

        let body_end_pos = text.rfind(">>>ext").ok_or_else(|| {
            AgentError::Recoverable("отсутствует закрывающий тег '>>>ext'".to_string())
        })?;

        if body_start_pos > body_end_pos {
            return Err(AgentError::Recoverable("некорректные границы EXT сообщения".to_string()));
        }   // if

        let body = text[body_start_pos..body_end_pos].trim().to_string();
        if body.is_empty() {
            return Err(AgentError::Recoverable("пустое EXT сообщение".to_string()));
        }   // if

        Ok(body)
    }   // _unwrap_ext_tags()

    /// Парсит открывающий тег директивы AI и валидирует сессию.
    ///
    /// # Логика
    /// 1. Применяет регулярное выражение `AI_START_REGEX` к началу текста.
    /// 2. Извлекает `DIR_ID` и `SESSION_ID`.
    /// 3. Проверяет `SESSION_ID` через `init_ctx.validate_session`.
    /// 4. Возвращает распарсенные данные и позицию, где заканчивается тег (начало тела).
    ///
    /// # Параметры
    /// - `text`: Текст сообщения (начинающийся с `<<<ai`).
    ///
    /// # Возвращаемое значение
    /// - `Ok((u32, String, usize))`: Кортеж (ID директивы, ID сессии, смещение начала тела).
    ///
    /// # Ошибки
    /// - `AgentError::Critical`: Ошибка компиляции регулярного выражения.
    /// - `AgentError::Recoverable`: Неверный формат тега, нечисловой ID, неверная сессия.
    fn _parse_opening_directive_tag(&self, text: &str) -> Result<(u32, String, usize), AgentError> {

        // Инициализация регулярки START
        static AI_START_REGEX: OnceLock<Result<Regex, regex::Error>> = OnceLock::new();
        let start_re_res = AI_START_REGEX.get_or_init(|| {
            Regex::new(r#"(?s)^<<<ai\s+(?P<id>\d+)\s+(?P<sess>\S+)\s+"#)
        });
        let start_re = start_re_res.as_ref().map_err(|e| {
            AgentError::Critical(format!("ошибка компиляции AI_START_REGEX: {}", e))
        })?;

        // Парсим начало
        let start_caps = start_re.captures(text).ok_or_else(|| {
            AgentError::Recoverable("неверный формат открывающего тега <<<ai DIR_ID SESSION_ID ...".to_string())
        })?;

        let body_start_pos = start_caps.get(0).unwrap().end(); // start_caps гарантирует наличие матча
        let start_id_str = start_caps.name("id").unwrap().as_str();
        let start_sess = start_caps.name("sess").unwrap().as_str();

        // Валидация идентификатора сессии. Если сессия не совпадает с инициализированной -
        // отвергаем директиву AI.
        session::validate_session(start_sess)?;

        let start_id = start_id_str.parse::<u32>().map_err(|_|
            AgentError::Recoverable("нечисловой ID в открывающем теге".to_string())
        )?;

        Ok((start_id, start_sess.to_string(), body_start_pos))
    }   // _parse_opening_directive_tag()

    /// Парсит закрывающий тег директивы AI.
    ///
    /// # Логика
    /// 1. Ищет маркер `>>>ai` с конца строки через `rfind` (оптимизация для больших JSON).
    /// 2. Извлекает хвост строки начиная с найденной позиции.
    /// 3. Применяет регулярное выражение `AI_TAIL_REGEX` для валидации структуры хвоста.
    /// 4. Извлекает `DIR_ID` и `SESSION_ID` из закрывающего тега.
    /// 5. Возвращает распарсенные данные и позицию начала закрывающего тега (конец тела).
    ///
    /// # Параметры
    /// - `text`: Полный текст сообщения.
    ///
    /// # Возвращаемое значение
    /// - `Ok((u32, String, usize))`: Кортеж (ID директивы, ID сессии, смещение конца тела).
    ///
    /// # Ошибки
    /// - `AgentError::Critical`: Ошибка компиляции регулярного выражения.
    /// - `AgentError::Recoverable`: Отсутствует маркер `>>>ai`, неверный формат хвоста, нечисловой ID.
    fn _parse_closing_directive_tag(&self, text: &str) -> Result<(u32, String, usize), AgentError> {

        // Инициализация регулярки END (применяется только к хвосту строки)
        static AI_TAIL_REGEX: OnceLock<Result<Regex, regex::Error>> = OnceLock::new();
        let tail_re_res = AI_TAIL_REGEX.get_or_init(|| {
            // Ожидаем: >>>ai ID SESSION (и возможные пробелы до конца строки)
            Regex::new(r#"(?s)^>>>ai\s+(?P<id>\d+)\s+(?P<sess>\S+)\s*$"#)
        });
        let tail_re = tail_re_res.as_ref().map_err(|e| {
            AgentError::Critical(format!("ошибка компиляции AI_TAIL_REGEX: {}", e))
        })?;

        // Парсим конец (через rfind)
        let tag_marker = ">>>ai";
        let body_end_pos = text.rfind(tag_marker).ok_or_else(|| {
            AgentError::Recoverable(format!("отсутствует закрывающий маркер '{}'", tag_marker))
        })?;

        // Извлекаем хвост строки: ">>>ai 123 456"
        let tail_str = &text[body_end_pos..];

        // Валидируем структуру хвоста
        let tail_caps = tail_re.captures(tail_str).ok_or_else(|| {
            AgentError::Recoverable(format!("неверный формат закрывающего тега: '{}'", tail_str))
        })?;

        let end_id_str = tail_caps.name("id").unwrap().as_str();
        let end_sess = tail_caps.name("sess").unwrap().as_str();

        let end_id = end_id_str.parse::<u32>().map_err(|_|
            AgentError::Recoverable("нечисловой ID в закрывающем теге".to_string())
        )?;

        Ok((end_id, end_sess.to_string(), body_end_pos))
    }   // _parse_closing_directive_tag()

    /// Описание: Формирует Markdown-отчёт о штатном завершении работы агента (COMPLETION) для ИИ.
    ///
    /// По протоколу это “сервисное событие”, не являющееся ответом на AI-директиву, поэтому
    /// используется упрощённая форма транспортных тегов: `<<<hbt SESSION_ID ... >>>hbt SESSION_ID`.
    ///
    /// # Алгоритм работы
    /// - Получает текущий `session_id` из глобального `session` контекста.
    /// - Строит opening/closing теги с `SESSION_ID`.
    /// - Формирует короткое Markdown-сообщение:
    ///   - H1 заголовок “Хобот завершает работу”.
    ///   - Причина: COMPLETION.
    /// - Записывает сообщение в `ReportContext` (opening/body/closing).
    ///
    /// # Ошибки
    /// Возвращает `AgentError::Recoverable`, если `session_id` ещё не инициализирован (не было INIT).
    ///
    /// # Побочные эффекты
    /// - Перезаписывает `REPORT` целиком.
    fn _build_completion_report(&mut self) -> Result<(), AgentError> {
        let session_id = session::session_id()?;  // Строгий источник SESSION_ID: глобальный INIT-контекст.

        let opening_bracket = format!("`<<<hbt {}`\n", session_id);
        let closing_bracket = format!("\n`>>>hbt {}`\n", session_id);

        let mut body = String::new();
        body.push_str("# 📴 Хобот завершает работу по запросу расширения.");

        report::set_work_report(&format!("{}{}{}\n", opening_bracket, body, closing_bracket))?;

        Ok(())
    }   // _build_completion_report()
}   // impl RequestProcessor
