//! glob.rs
//!
//! Глобальные константы, синглтоны и утилиты общего назначения.
//!
//! # Архитектура
//! Модуль предоставляет централизованный доступ к:
//! - Конфигурации (через внутренний модуль `Config`).
//! - Системе обработки ошибок (`ErrorControl`).
//!
//! # Инварианты
//! - `initialize_glob()` должен быть вызван в `main` первым, до любого использования `config()` или логирования.

use std::io::Write;
use std::sync::Mutex;
use std::sync::OnceLock;
use crate::glob::config::AppConfig;
pub(crate) use crate::glob::error_control::{AgentError, ErrorControl};

// Внутренние модули
mod config;
pub mod error_control;
pub mod stdout_sink;

/// Константы.

/// Указание макросам печати, таким как prln!() и wrln!() дополнять сообщение именем файла и номера
/// строки вызова макроса.
///
/// # Объяснение
/// Бывает что забытый макрос печатает ненужные сообщения при работе программы, но его крайне трудно
/// найти. В этом случае выставляем этот флаг и макрос показывает точку в программе где он находится.
#[allow(dead_code)]
pub const PRLN_PRINTS_FILE_LINE: bool = false;

// --- Типы сообщений между подсистемами ---

/// Расширение посылает агенту пакет инициализации.
pub const EXT_MSG_TYPE_INIT_SESSION: &str = "INIT_SESSION";

/// Расширение посылает агенту текст ошибки для проксирования к AI
pub const EXT_MSG_TYPE_PROTOCOL_ERROR: &str = "PROTOCOL_ERROR";

/// Расширение требует остановку агента.
pub const EXT_MSG_TYPE_COMPLETION: &str = "COMPLETION";

/// Хобот посылает расширению сигнал о завершении исполнения директивы.
pub const HBT_MSG_TYPE_DIRECTIVE_COMPLETED: &str = "DIRECTIVE_COMPLETED";

// --- Константы протокола обмена расширение-агент ---

/// Маркеры транспортного протокола во входящем потоке.
pub const PROTOCOL_TAG_AI_OPEN: &str = "<<<ai";
pub const PROTOCOL_TAG_AI_CLOSE: &str = ">>>ai";
pub const PROTOCOL_TAG_EXT_OPEN: &str = "<<<ext";
pub const PROTOCOL_TAG_EXT_CLOSE: &str = ">>>ext";

// --- Глобальные переменные ---

/// Предоставляет доступ к глобальной конфигурации.
/// Делегирует вызов внутреннему модулю `Config`.
///
/// # Паника
/// Если конфигурация не инициализирована (ошибка этапа разработки).
///
/// # Возвращаемое значение
/// &AppConfig: Ссылка на конфигурацию.
pub fn config() -> &'static AppConfig {
    config::get()
}   // config()

/// Инициализирует глобальные компоненты приложения.
/// Загружает конфигурацию и устанавливает её в глобальный синглтон.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - Не удалось загрузить или создать файл конфигурации.
/// - Глобальная конфигурация уже была инициализирована ранее.
pub fn initialize_glob() -> Result<(), String> {

    // 1. Инициализируем конфигурацию через внутренний модуль
    config::init()?;

    // Здесь можно добавить инициализацию других компонентов (например, ErrorControl), если потребуется

    Ok(())
}   // initialize_glob()

// --- Глобальные функции ---

pub fn send_to_stdout(msg: &str) -> Result<(), AgentError> {

    // Обертка Native Messaging
    #[derive(serde::Serialize)]
    struct NativeWrapper {
        text: String,
    }   // NativeWrapper

    let wrapper = NativeWrapper { text: msg.to_string() };

    let final_json = serde_json::to_string(&wrapper).map_err(|e| {
        AgentError::Critical(format!("{}, {}: Ошибка сериализации обертки NativeMsg: {}", file!(), line!(), e))
    })?;

    _send_raw_to_stdout(&final_json)
}

/// Отправляет произвольную строку в stdout по протоколу Native Messaging.
///
/// Функция не анализирует содержимое строки и не добавляет никаких маркеров протокола.
/// Она просто вычисляет длину, добавляет префикс и пишет в поток.
///
/// # Параметры
/// - `final_json`: Строка для отправки.
///
/// # Ошибки
/// Возвращает `AgentError::Critical`, если не удалось выполнить операцию записи в поток.
fn _send_raw_to_stdout(final_json: &str) -> Result<(), AgentError> {
    let len = final_json.len() as u32;
    let mut out = std::io::stdout();

    out.write_all(&len.to_ne_bytes())
        .map_err(|e| AgentError::Critical(
            format!("{}, {}: Ошибка записи длины в stdout: {}", file!(), line!(), e)))?;

    out.write_all(final_json.as_bytes())
        .map_err(|e| AgentError::Critical(
            format!("{}, {}: Ошибка записи тела в stdout: {}", file!(), line!(), e)))?;

    out.flush()
        .map_err(|e| AgentError::Critical(
            format!("{}, {}: Ошибка сброса буфера stdout: {}", file!(), line!(), e)))?;

    Ok(())
}   // send_raw_to_stdout()