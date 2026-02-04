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
use crate::agent::request::report;
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
    /// - Перезаписывает `REPORT_CONTEXT` (opening/body/closing) сервисным отчетом INIT.
    pub(super) fn handle_session_init_request(&mut self, json_body: &str)
                                              -> Result<SessionContext, AgentError>
    {
        #[derive(Deserialize)]
        struct InitEnvelope {
            payload: SessionContext,
        }   // InitEnvelope

        let env: InitEnvelope = serde_json::from_str(json_body).map_err(|e| {
            AgentError::Recoverable(format!(r#"{}, {}: ошибка в JSON INIT_SESSION сообщения.
JSON:
    {}

ошибка: {}"#, file!(), line!(), json_body, e))})?;

        self._build_report(&env.payload);

        Ok(env.payload)
    }   // handle_extension_init_request()
}   // impl SessionContext

// Внутренний интерфейс.
impl SessionContext {

    /// Описание: Формирует Markdown-отчет об успешной инициализации (INIT) для ИИ.
    ///
    /// Отчет оформляется в упрощенной форме `<<<hbt ... >>>hbt` (без DIRECTIVE_ID / SESSION_ID),
    /// так как событие инициировано расширением, а не директивой ИИ.
    ///
    /// # Параметры
    /// - `report_ctx`: Контекст отчёта, который будет перезаписан готовым сообщением.
    /// - `payload`: Полезная нагрузка INIT.
    ///
    /// # Побочные эффекты
    /// - Перезаписывает `report_ctx` целиком.
    fn _build_report(&self, payload: &SessionContext) {
        let opening_bracket = format!("`<<<hbt {}`\n", payload.session_id);
        let closing_bracket = format!("`>>>hbt {}`\n", payload.session_id);

        let mut body = String::new();

        body.push_str("# 🚀 Хобот готов к работе.\n\n");
        body.push_str("**Контекст сессии:**\n");

        let mut payload_dump = String::new();
        payload_dump.push_str(&format!("session_id: \"{}\"\n", payload.session_id));
        payload_dump.push_str(&format!("next_directive_num: {}\n", 1));
        payload_dump.push_str(&format!("browser: \"{}\"\n", payload.browser));
        payload_dump.push_str(&format!("ai_url: \"{}\"\n", payload.ai_url));
        payload_dump.push_str(&format!("window_title: \"{}\"\n", payload.window_title));

        markdown_fence::push_fenced_block(&mut body, &payload_dump);

        let _ = report::set_text(&format!("{}{}{}", opening_bracket, body, closing_bracket));
    }   // _build_report()

}   // impl SessionContext (private)
