//! session.rs
//!
//! Глобальный контекст сессии (INIT) и внешний API доступа к нему.
//!
//! # ОПИСАНИЕ
//! Модуль предоставляет единый “чёрный ящик” для работы с параметрами текущей сессии,
//! полученными от расширения в сообщении `EXT/INIT`.
//!
//! Данные хранятся в `OnceLock<RwLock<SessionContext>>`.
//! `OnceLock` гарантирует однократную инициализацию.
//! `RwLock` позволяет безопасно читать данные из множества потоков и изменять их (например, флаги) в рантайме.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Однократная инициализация контекста сессии (`init_session_context`).
//! - Предоставление функций доступа к полям сессии (`session_id`, `window_title`, ...).
//! - Проверка соответствия `SESSION_ID` у входящих AI-директив (`validate_session`).
//!
//! # ИНВАРИАНТЫ
//! - INIT должен быть обработан до любых AI-директив и до любых хандлеров, которым нужен контекст.
//! - Повторный INIT в рамках одного процесса запрещен.

mod session_context;
#[cfg(test)]
mod test_session_test;

use std::sync::{OnceLock, RwLock, RwLockReadGuard};
use crate::agent::request::session::session_context::SessionContext;
use crate::glob::error_control::AgentError;

/// Глобальный контекст сессии (данные INIT).
///
/// Инициализируется один раз. Обернут в RwLock для потокобезопасного изменения флагов.
static SESSION_CONTEXT: OnceLock<RwLock<SessionContext>> = OnceLock::new();

/// Описание: Инициализирует глобальный контекст сессии из EXT/INIT сообщения.
///
/// Метод должен вызываться ровно один раз за время жизни процесса, сразу после получения INIT.
/// Внутри выполняется:
/// - парсинг JSON-тела INIT,
/// - формирование сервисного отчёта INIT в `REPORT`,
/// - сохранение распарсенного контекста в глобальном `SESSION_CONTEXT`.
///
/// # Ошибки
/// - `AgentError::Recoverable`: INIT не парсится или не соответствует схеме.
/// - `AgentError::Critical`: `SESSION_CONTEXT` уже инициализирован (повторный INIT запрещен).
///
/// # Побочные эффекты
/// - Перезаписывает `REPORT` отчетом INIT (opening/body/closing).
/// - При успешной инициализации сохраняет контекст в глобальный `SESSION_CONTEXT`.
pub fn init_session_context(json_body: &str) -> Result<(), AgentError> {
    let mut tmp_ctx = SessionContext::new();

    let payload_ctx = tmp_ctx.receive_session_init_request(json_body)?;

    // Оборачиваем в RwLock перед установкой в OnceLock
    SESSION_CONTEXT
        .set(RwLock::new(payload_ctx))
        .map_err(|_| AgentError::Critical(
            format!("{}, {}: повторная инициализация SESSION_CONTEXT запрещена.", file!(), line!())
        ))?;

    Ok(())
}   // init_session_context()

/// Описание: Обрабатывает сообщение CHANGE_STEP_THROUGH от расширения.
///
/// Десериализует JSON и обновляет флаг `step_through` в глобальном контексте сессии.
///
/// # Параметры
/// - `json_body`: JSON-тело EXT сообщения (без `<<<ext ... >>>ext`).
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен или ошибка блокировки записи.
/// - `AgentError::Recoverable`: Некорректный JSON.
///
/// # Побочные эффекты
/// - Изменяет флаг `step_through` в глобальном `SESSION_CONTEXT`.
pub fn change_step_through_flag(json_body: &str) -> Result<(), AgentError> {
    let lock = SESSION_CONTEXT.get().ok_or_else(|| {
        AgentError::Critical(format!("{}, {}: SESSION_CONTEXT не инициализирован (INIT_SESSION не получен).",
            file!(), line!()))
    })?;

    let mut ctx = lock.write().map_err(|e| {
        AgentError::Critical(format!("{}, {}: Ошибка захвата блокировки записи сессии: {}",
            file!(), line!(), e))
    })?;

    ctx.change_step_through(json_body)
}   // change_step_through_flag()

/// Описание: Обрабатывает сообщение CHANGE_OS_READONLY от расширения.
///
/// Десериализует JSON и обновляет флаг `os_readonly` в глобальном контексте сессии.
///
/// # Параметры
/// - `json_body`: JSON-тело EXT сообщения (без `<<<ext ... >>>ext`).
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен или ошибка блокировки записи.
/// - `AgentError::Recoverable`: Некорректный JSON.
///
/// # Побочные эффекты
/// - Изменяет флаг `os_readonly` в глобальном `SESSION_CONTEXT`.
pub fn change_os_readonly_flag(json_body: &str) -> Result<(), AgentError> {
    let lock = SESSION_CONTEXT.get().ok_or_else(|| {
        AgentError::Critical(format!("{}, {}: SESSION_CONTEXT не инициализирован (INIT_SESSION не получен).",
            file!(), line!()))
    })?;

    let mut ctx = lock.write().map_err(|e| {
        AgentError::Critical(format!("{}, {}: Ошибка захвата блокировки записи сессии: {}",
            file!(), line!(), e))
    })?;

    ctx.change_os_readonly(json_body)
}   // change_os_readonly_flag()

/// Описание: Обрабатывает сообщение AI_INPUT_GEOMETRY_UPDATE от расширения.
///
/// Десериализует JSON и обновляет поле `ai_input_rect` в глобальном контексте сессии.
///
/// # Параметры
/// - `json_body`: JSON-тело EXT сообщения (без `<<<ext ... >>>ext`).
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен или ошибка блокировки записи.
/// - `AgentError::Recoverable`: Некорректный JSON.
///
/// # Побочные эффекты
/// - Изменяет поле `ai_input_rect` в глобальном `SESSION_CONTEXT`.
pub fn change_ai_input_rect(json_body: &str) -> Result<(), AgentError> {
    let lock = SESSION_CONTEXT.get().ok_or_else(|| {
        AgentError::Critical(format!(
            "{}, {}: SESSION_CONTEXT не инициализирован (INIT_SESSION не получен).",
            file!(), line!()))
    })?;

    let mut ctx = lock.write().map_err(|e| {
        AgentError::Critical(format!("{}, {}: Ошибка захвата блокировки записи сессии: {}",
            file!(), line!(), e))
    })?;

    ctx.change_ai_input_rect(json_body)
}   // change_ai_input_rect()

/// Описание: Возвращает `session_id` текущей сессии.
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен.
///
/// # Возвращаемое значение
/// Тип: String: Клон идентификатора сессии.
pub fn session_id() -> Result<String, AgentError> {
    Ok(_get_context_read_guard()?.session_id.clone())
}   // session_id()

/// Описание: Возвращает идентификатор браузера (`browser`) из INIT.
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен.
///
/// # Возвращаемое значение
/// Тип: String: Клон идентификатора браузера.
pub fn browser() -> Result<String, AgentError> {
    Ok(_get_context_read_guard()?.browser.clone())
}   // browser()

/// Описание: Возвращает `ai_url` из INIT.
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен.
///
/// # Возвращаемое значение
/// Тип: String: Клон URL страницы AI.
pub fn ai_url() -> Result<String, AgentError> {
    Ok(_get_context_read_guard()?.ai_url.clone())
}   // ai_url()

/// Описание: Возвращает `window_name` из INIT.
///
/// Используется для поиска окна/процесса через WinAPI.
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен.
///
/// # Возвращаемое значение
/// Тип: String: Клон имени окна.
pub fn window_title() -> Result<String, AgentError> {
    Ok(_get_context_read_guard()?.window_title.clone())
}   // window_title()

/// Описание: Возвращает текущее состояние флага `os_readonly`.
///
/// Флаг запрещает или разрешает внесение изменений в хостовую систему.
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен.
///
/// # Возвращаемое значение
/// Тип: bool: Значение флага.
pub fn os_readonly() -> Result<bool, AgentError> {
    Ok(_get_context_read_guard()?.os_readonly)
}   // os_readonly()

/// Описание: Устанавливает новое значение флага `os_readonly`.
///
/// # Параметры
/// - `val`: Новое значение флага (true — только чтение, false — полный доступ).
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен или ошибка блокировки записи.
pub fn set_os_readonly(val: bool) -> Result<(), AgentError> {
    let lock = SESSION_CONTEXT.get().ok_or_else(|| {
        AgentError::Critical(format!("{}, {}: SESSION_CONTEXT не инициализирован",
            file!(), line!()))
    })?;

    let mut ctx = lock.write().map_err(|e| {
        AgentError::Critical(format!("{}, {}: Ошибка захвата блокировки записи сессии: {}",
            file!(), line!(), e))
    })?;

    ctx.os_readonly = val;
    Ok(())
}   // set_os_readonly()

/// Описание: Возвращает текущее состояние флага `step_through`.
///
/// Флаг включает/выключает пошаговый режим исполнения команд.
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен.
///
/// # Возвращаемое значение
/// Тип: bool: Значение флага.
pub fn step_through() -> Result<bool, AgentError> {
    Ok(_get_context_read_guard()?.step_through)
}   // step_through()

/// Описание: Устанавливает новое значение флага `step_through`.
///
/// # Параметры
/// - `val`: Новое значение флага (true — пошаговый режим, false — обычный режим).
///
/// # Ошибки
/// - `AgentError::Critical`: INIT ещё не был выполнен или ошибка блокировки записи.
pub fn set_step_through(val: bool) -> Result<(), AgentError> {
    let lock = SESSION_CONTEXT.get().ok_or_else(|| {
        AgentError::Critical(format!( "{}, {}: SESSION_CONTEXT не инициализирован",
            file!(), line!()))
    })?;

    let mut ctx = lock.write().map_err(|e| {
        AgentError::Critical(format!("{}, {}: Ошибка захвата блокировки записи сессии: {}",
            file!(), line!(), e))
    })?;

    ctx.step_through = val;
    Ok(())
}   // set_step_through()

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

//--------------------------------------------------------------------------------------------------
//                                 Внутренний интерфейс
//--------------------------------------------------------------------------------------------------
/// Описание: Внутренний хелпер для получения блокировки чтения.
///
/// # Ошибки
/// - `AgentError::Critical`: Если контекст не инициализирован или RwLock "отравлен" (poisoned).
fn _get_context_read_guard() -> Result<RwLockReadGuard<'static, SessionContext>, AgentError> {
    let lock = SESSION_CONTEXT.get().ok_or_else(|| {
        AgentError::Critical(format!("{}, {}: SESSION_CONTEXT не инициализирован (INIT_SESSION не получен).",
            file!(), line!()))
    })?;

    lock.read().map_err(|e| {
        AgentError::Critical(format!("{}, {}: Ошибка захвата блокировки чтения сессии: {}",
            file!(), line!(), e))
    })
}   // _get_context_read_guard()
