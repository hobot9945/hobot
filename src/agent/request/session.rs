//! session.rs
//!
//! Глобальный контекст сессии (INIT) и внешний API доступа к нему.
//!
//! # ОПИСАНИЕ
//! Модуль предоставляет единый “чёрный ящик” для работы с параметрами текущей сессии,
//! полученными от расширения в сообщении `EXT/INIT`.
//!
//! После успешного INIT данные сессии сохраняются в глобальном `OnceLock` и доступны
//! хандлерам без протаскивания параметров через протокол команд.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Однократная инициализация контекста сессии (`init_session_context`).
//! - Предоставление функций доступа к полям сессии (`session_id`, `window_title`, ...).
//! - Проверка соответствия `SESSION_ID` у входящих AI-директив (`validate_session`).
//!
//! # ИНВАРИАНТЫ
//! - INIT должен быть обработан до любых AI-директив и до любых хандлеров, которым нужен контекст.
//! - Повторный INIT в рамках одного процесса запрещен.
//! - Доступ к данным сессии до INIT — критическая ошибка.

mod session_context;
mod test_session_test;

use std::sync::OnceLock;
use crate::agent::request::session::session_context::SessionContext;
use crate::glob::error_control::AgentError;

/// Глобальный контекст сессии (данные INIT).
///
/// Инициализируется один раз. После инициализации предоставляет только read-доступ. [web:117]
static SESSION_CONTEXT: OnceLock<SessionContext> = OnceLock::new();

/// Описание: Инициализирует глобальный контекст сессии из EXT/INIT сообщения.
///
/// Метод должен вызываться ровно один раз за время жизни процесса, сразу после получения INIT.
/// Внутри выполняется:
/// - парсинг JSON-тела INIT,
/// - формирование сервисного отчёта INIT в `report_ctx`,
/// - сохранение распарсенного контекста в глобальном `SESSION_CONTEXT`.
///
/// # Ошибки
/// - `AgentError::Recoverable`: INIT не парсится или не соответствует схеме.
/// - `AgentError::Critical`: `SESSION_CONTEXT` уже инициализирован (повторный INIT запрещен). [web:117]
///
/// # Побочные эффекты
/// - Перезаписывает `report_ctx` отчетом INIT (opening/body/closing).
/// - При успешной инициализации сохраняет контекст в глобальный `SESSION_CONTEXT`.
pub fn init_session_context(json_body: &str, report_ctx: &mut crate::agent::request::report::Report) 
    -> Result<(), AgentError> 
{
    let mut tmp_ctx = SessionContext::new();

    let payload_ctx = tmp_ctx.handle_session_init_request(json_body, report_ctx)?;

    SESSION_CONTEXT
        .set(payload_ctx)
        .map_err(|_| AgentError::Critical(
            format!("{}, {}: повторная инициализация SESSION_CONTEXT запрещена.", file!(), line!())
        ))?;

    Ok(())
}   // init_session_context()

/// Описание: Возвращает ссылку на глобальный контекст сессии.
///
/// Используется внутренними функциями доступа (`session_id`, `window_title`, ...).
///
/// # Ошибки
/// Возвращает `AgentError::Critical`, если INIT ещё не был выполнен (контекст не инициализирован).
///
/// # Возвращаемое значение
/// Тип: &SessionContext: Ссылка на контекст сессии.
pub fn session_context() -> Result<&'static SessionContext, AgentError> {
    SESSION_CONTEXT.get().ok_or_else(|| {
        AgentError::Critical("SESSION_CONTEXT не инициализирован (INIT_SESSION не получен).".to_string())
    })
}   // session_context()

/// Описание: Возвращает `session_id` текущей сессии.
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен.
///
/// # Возвращаемое значение
/// Тип: &str: Идентификатор сессии.
pub fn session_id() -> Result<&'static str, AgentError> {
    Ok(session_context()?.session_id.as_str())
}   // session_id()

/// Описание: Возвращает идентификатор браузера (`browser`) из INIT.
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен.
///
/// # Возвращаемое значение
/// Тип: &str: Идентификатор браузера.
pub fn browser() -> Result<&'static str, AgentError> {
    Ok(session_context()?.browser.as_str())
}   // browser()

/// Описание: Возвращает `ai_url` из INIT.
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен.
///
/// # Возвращаемое значение
/// Тип: &str: URL страницы AI.
pub fn ai_url() -> Result<&'static str, AgentError> {
    Ok(session_context()?.ai_url.as_str())
}   // ai_url()

/// Описание: Возвращает `window_name` из INIT.
///
/// Используется для поиска окна/процесса через WinAPI.
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен.
///
/// # Возвращаемое значение
/// Тип: &str: Имя окна.
pub fn window_title() -> Result<&'static str, AgentError> {
    Ok(session_context()?.window_title.as_str())
}   // window_name()

/// Описание: Проверяет соответствие `SESSION_ID` у входящей AI-директивы текущей сессии.
///
/// Используется парсером директив для защиты от “чужих” директив (другая вкладка/сессия)
/// и от рассинхронизации транспорта.
///
/// # Параметры
/// - `directive_session_id`: Значение `SESSION_ID` из тега `<<<ai DIRECTIVE_ID SESSION_ID ...`.
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен (контекст отсутствует).
/// - `AgentError::Recoverable`: `incoming_session_id` не совпадает с текущим `session_id`.
pub fn validate_session(directive_session_id: &str) -> Result<(), AgentError> {
    let expected = session_id()?;

    if expected != directive_session_id {
        return Err(AgentError::Recoverable(format!(
            "{}, {}: неверный SessionID: получен '{}', ожидался '{}'", file!(), line!(),
            directive_session_id,
            expected
        )));
    }   // if

    Ok(())
}   // validate_session()
