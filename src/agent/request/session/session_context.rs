//! session/session_context.rs
//!
//! Внутренний подмодуль обработки INIT-сообщения.
//!
//! # ОПИСАНИЕ
//! Модуль предоставляет тип `SessionContext` и методы для обработки EXT/INIT.
//! Он используется только родительским модулем `session`.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Десериализация сообщения INIT.
//! - Формирование отчёта об инициализации в `ReportContext`.
//! - Предоставление контейнера данных `SessionContext` для сохранения в глобальном стейте `session`.
//!
//! # ИНВАРИАНТЫ
//! - Все элементы этого модуля доступны только родителю (`session`), наружу не экспортируются.

use serde::Deserialize;
use crate::agent::request::{report, session};
use crate::glob;
use crate::glob::error_control::AgentError;
use crate::library::markdown_fence;

/// Данные сессии из INIT.
///
/// Видимость `pub(super)`: тип виден только родительскому модулю `session`.
#[derive(Debug, Deserialize, Clone, Default)]
pub(super) struct SessionContext {
    pub(super) session_id: String,      // Идентификатор сессии.
    pub(super) browser: String,         // Идентификатор браузера (chrome/firefox/edge...).
    pub(super) ai_url: String,          // URL страницы AI.
    pub(super) window_title: String,    // Имя окна windows (как видит ОС).
    pub(super) os_readonly: bool,       // Запрет/разрешение на внесение изменений в хостовую систему.
    pub(super) step_through: bool       // Пошаговое исполнение.
}   // SessionContext

impl SessionContext {

    /// Описание: Создает новый пустой контекст.
    ///
    /// # Возвращаемое значение
    /// Тип: Self: Пустой `SessionContext`.
    pub(super) fn new() -> Self {
        Self::default()
    }   // new()

    /// Описание: Обрабатывает INIT-сообщение расширения: парсит JSON и формирует отчёт.
    ///
    /// # Алгоритм работы
    /// - Десериализует `json_body` в структуру `InitEnvelope { payload: SessionContext }`.
    /// - Формирует сервисный отчет об успешной инициализации в `report_ctx`.
    /// - Возвращает распарсенный `SessionContext` для сохранения в глобальном `OnceLock`.
    ///
    /// # Параметры
    /// - `json_body`: JSON-тело INIT (без `<<<ext ... >>>ext`).
    ///
    /// # Ошибки
    /// Возвращает `AgentError::Recoverable`, если:
    /// - `json_body` не является валидным JSON.
    /// - JSON не соответствует схеме INIT (нет `payload` или поля неверного типа).
    ///
    /// # Возвращаемое значение
    /// Тип: SessionContext: Данные сессии для сохранения в `session::SESSION_CONTEXT`.
    ///
    /// # Побочные эффекты
    /// - Перезаписывает `REPORT` (work_report/comment_report) сервисным отчетом INIT.
    pub(super) fn receive_session_init_request(&mut self, json_body: &str)
                                               -> Result<SessionContext, AgentError>
    {
        #[derive(Deserialize)]
        struct InitSessWrapper {
            payload: SessionContext,
        }   // InitEnvelope

        let wrapper: InitSessWrapper = serde_json::from_str(json_body).map_err(|e| {
            AgentError::Recoverable(format!(r#"{}, {}: ошибка в JSON INIT_SESSION сообщения.
JSON:
    {}

ошибка: {}"#, file!(), line!(), json_body, e))})?;

        self._build_report(&wrapper.payload)?;

        Ok(wrapper.payload)
    }   // handle_extension_init_request()

    /// Описание: Обрабатывает сообщение CHANGE_STEP_THROUGH от расширения.
    ///
    /// Десериализует JSON-тело и обновляет флаг `step_through` в контексте сессии.
    ///
    /// # Параметры
    /// - `json_body`: JSON-тело EXT сообщения (без `<<<ext ... >>>ext`), содержащее:
    ///   `{ "type": "CHANGE_STEP_THROUGH", "value": true/false }`.
    ///
    /// # Ошибки
    /// Возвращает `AgentError::Recoverable`, если JSON некорректен или не содержит поле `value`.
    pub(super) fn change_step_through(&mut self, json_body: &str) -> Result<(), AgentError> {

        #[derive(Deserialize)]
        struct ChangeFlag {
            value: bool,
        }   // ChangeFlag

        let parsed: ChangeFlag = serde_json::from_str(json_body).map_err(|e| {
            AgentError::Recoverable(format!(
                "{}, {}: ошибка в JSON CHANGE_STEP_THROUGH:\nJSON:\n\t{}\nошибка: {}",
                file!(), line!(), json_body, e
            ))
        })?;

        self.step_through = parsed.value;
        Ok(())
    }   // change_step_through()

    /// Описание: Обрабатывает сообщение CHANGE_OS_READONLY от расширения.
    ///
    /// Десериализует JSON-тело и обновляет флаг `os_readonly` в контексте сессии.
    ///
    /// # Параметры
    /// - `json_body`: JSON-тело EXT сообщения (без `<<<ext ... >>>ext`), содержащее:
    ///   `{ "type": "CHANGE_OS_READONLY", "value": true/false }`.
    ///
    /// # Ошибки
    /// Возвращает `AgentError::Recoverable`, если JSON некорректен или не содержит поле `value`.
    pub(super) fn change_os_readonly(&mut self, json_body: &str) -> Result<(), AgentError> {

        #[derive(Deserialize)]
        struct ChangeFlag {
            value: bool,
        }   // ChangeFlag

        let parsed: ChangeFlag = serde_json::from_str(json_body).map_err(|e| {
            AgentError::Recoverable(format!(
                "{}, {}: ошибка в JSON CHANGE_OS_READONLY:\nJSON:\n\t{}\nошибка: {}",
                file!(), line!(), json_body, e
            ))
        })?;

        self.os_readonly = parsed.value;
        Ok(())
    }   // change_os_readonly()
}   // impl SessionContext

// Внутренний интерфейс.
impl SessionContext {

    /// Описание: Формирует Markdown-отчет об успешной инициализации (INIT) для ИИ.
    ///
    /// Отчет оформляется в упрощенной форме `<<<hbt ... >>>hbt` (без DIRECTIVE_ID / SESSION_ID),
    /// так как событие инициировано расширением, а не директивой ИИ.
    ///
    /// # Параметры
    /// - `payload`: Полезная нагрузка INIT.
    ///
    /// # Побочные эффекты
    /// - Перезаписывает `REPORT.work_report` целиком.
    fn _build_report(&self, payload: &SessionContext) -> Result<(), AgentError> {
        let opening_bracket = format!("`<<<hbt {}`\n", payload.session_id);
        let closing_bracket = format!("`>>>hbt {}`\n", payload.session_id);

        let mut body = String::new();

        body.push_str("# 🚀 Хобот готов к работе.\n\n");
        body.push_str("**Контекст сессии:**\n");

        // Сформировать блок контекста сессии и добавить в тело, окружив забором.
        let mut sess_ctx = String::new();
        sess_ctx.push_str(&format!("session_id: \"{}\"\n", payload.session_id));
        sess_ctx.push_str(&format!("next_directive_num: {}\n", 1));
        sess_ctx.push_str(&format!("browser: \"{}\"\n", payload.browser));
        sess_ctx.push_str(&format!("ai_url: \"{}\"\n", payload.ai_url));
        sess_ctx.push_str(&format!("window_title: \"{}\"\n", payload.window_title));
        sess_ctx.push_str(&format!("exec_directory: \"{}\"\n", glob::exec_dir()));
        markdown_fence::push_fenced_block(&mut body, &sess_ctx);

        // Сформировать рекомендацию AI по загрузке промпта.
        body.push_str("**Загрузка промпта (инструкция для AI)**\n");
        body.push_str("Загружается один раз за всю историю чата. Проверь, загружен ли файл \
prompt_hobot_prompt.md. Если нет, загрузи, иначе - пропусти этот шаг. Открытие новой сессии работы с Хоботом \
НЕ является переоткрытием чата и НЕ требует новой загрузки инструкции.  Для загрузки нужно вывести \
следующий текст (подставив актуальный session_id), включая забор из бэктиков:\n");
        let prompt_file = &glob::config::get().prompt_on_hobot;
        let prompt_file_with_doubled_backslashes = prompt_file.replace("\\", "\\\\");
        let mut prompt = String::new();
        prompt.push_str(&format!(r#"```hobot
<<<ai 1 session_id
{{
    "commands": [
        {{
            "cmd_id": 1,
            "name": "shell_cmd",
            "params": ["type \"{ex_dir_double}{pr_fl}\""]
        }}
    ]
}}
>>>ai 1 session_id
```"#, ex_dir_double = glob::exec_dir_with_doubled_backslashes(),
                                 pr_fl = prompt_file_with_doubled_backslashes));
        markdown_fence::push_fenced_block(&mut body, &prompt);

        report::set_work_report(&format!("{}{}{}\n", opening_bracket, body, closing_bracket))
    }   // _build_report()

}   // impl SessionContext (private)
