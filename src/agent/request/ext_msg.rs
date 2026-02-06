//! ext_msg
//!
//! Модуль обработки служебных сообщений расширения (Extension Messages).
//!
//! На текущем этапе обрабатывает сообщения типа `PROTOCOL_ERROR`.
//! Сообщение приходит от расширения в виде JSON-тела (без транспортных тегов).
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Десериализация служебных сообщений расширения.
//! - Формирование отчёта для ИИ в `ReportContext` (упрощённая форма `<<<hbt ... >>>hbt`).
//!
//! # ИНВАРИАНТЫ
//! - Контекст не хранит состояние между запросами: вся полезная нагрузка используется сразу.
//! - Если `handle_extension_error_request()` завершился успешно, `report_ctx` заполнен готовым
//!   сообщением для отправки в чат ИИ.
//!
//! # ПРИМЕЧАНИЯ
//! - Отчёт оформляется как “ошибка транспорта/расширения”, а не как ответ на директиву ИИ,
//!   поэтому используются теги `<<<hbt ... >>>hbt` без `DIRECTIVE_ID` / `SESSION_ID`.

#[cfg(test)]
mod test_extmsg_context_test;

use serde::Deserialize;
use crate::agent::request::report;
use crate::glob::error_control::AgentError;
use crate::library;

/// Контекст обработки служебных сообщений расширения.
///
/// Контекст сделан намеренно “пустым”: он не хранит данные между вызовами.
/// Это снижает количество состояний, которые нужно помнить и чистить.
#[derive(Debug, Default)]
pub struct ExtensionMessageContext {}

impl ExtensionMessageContext {

    /// Описание: Создает новый контекст обработки служебных сообщений расширения.
    ///
    /// # Возвращаемое значение
    /// Тип: Self: Новый `ExtensionErrorContext`.
    pub fn new() -> Self {
        Self::default()
    }   // new()

    /// Описание: Сбрасывает состояние контекста.
    ///
    /// Сейчас метод является no-op, так как контекст не хранит состояния.
    /// Оставлен для единообразия с остальными контекстами и на будущее расширение функционала.
    pub fn clear(&mut self) {
        // no-op
    }   // clear()

    /// Описание: Обрабатывает сообщение `PROTOCOL_ERROR` от расширения и формирует отчёт для ИИ.
    ///
    /// # Алгоритм работы
    /// - Десериализует `json_body` в структуру с полем `error_message`.
    /// - Формирует Markdown-отчёт и записывает его в `report_ctx`.
    ///
    /// # Параметры
    /// - `json_body`: JSON-тело EXT сообщения (без `<<<ext ... >>>ext`), содержащее:
    ///   `{ "type": "PROTOCOL_ERROR", "error_message": "..." }`.
    ///   Поле `type` на этом этапе может быть уже проверено снаружи.
    /// - `session_id`: Идентификатор сессии, полученный из `INIT` и сохранённый на уровне `RequestProcessor`.
    ///
    /// # Ошибки
    /// Возвращает `AgentError::Recoverable`, если:
    /// - `json_body` не является валидным JSON.
    /// - В JSON отсутствует `error_message` или он имеет неверный тип.
    ///
    /// # Побочные эффекты
    /// - Перезаписывает `REPORT` (opening/body/closing).
    pub fn handle_extension_message_request(&mut self, json_body: &str, session_id: &str) -> Result<(), AgentError> 
    {

        #[derive(Deserialize)]
        struct ErrorMessage {
            /// Поле протокола: `error_message`.
            error_message: String,
        }   // ErrorMessage

        let msg: ErrorMessage = serde_json::from_str(json_body).map_err(|e| {
            AgentError::Recoverable(format!("Ошибка парсинга PROTOCOL_ERROR: {}", e))
        })?;

        self._build_report(session_id, &msg.error_message);

        Ok(())
    }   // handle_extension_error_request()

}   // impl ExtensionErrorContext

// Внутренний интерфейс.
impl ExtensionMessageContext {

    /// Описание: Формирует Markdown-отчет об ошибке протокола расширения для ИИ.
    ///
    /// Отчет оформляется в упрощенной форме `<<<hbt SESSION_ID ... >>>hbt SESSION_ID`,
    /// так как событие инициировано расширением, а не директивой ИИ.
    ///
    /// # Параметры
    /// - `session_id`: Идентификатор сессии из `INIT`.
    /// - `error_message`: Текст ошибки, полученный от расширения.
    ///
    /// # Побочные эффекты
    /// - Перезаписывает `REPORT` целиком.
    fn _build_report(&self, session_id: &str, error_message: &str) {
        let opening_bracket = format!("`<<<hbt {}`\n", session_id);
        let closing_bracket = format!("`>>>hbt {}`\n", session_id);

        let err_msg = if error_message.is_empty() { "(empty)" } else { error_message };

        let mut body = String::new();
        body.push_str("# ⛔ Ошибка протокола расширения\n");
        body.push_str("**Детали**:\n");
        library::markdown_fence::push_fenced_block(&mut body, err_msg);

        let _ = report::set_work_report(&format!("{}{}{}", opening_bracket, body, closing_bracket));
    }   // _build_report()
}   // impl ExtensionErrorContext (private)
