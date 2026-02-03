//! error_control.rs
//!
//! Модуль централизованного управления ошибками.
//!
//! Функционал:
//! - Логирование в файл (если доступен).
//! - Дублирование критических ошибок в stdout (для расширения).

use std::io::Write;
use crate::agent::request::session;
use crate::glob::{show_error_message, substring};
use crate::library::window::paste_text_into_window_by_needle;

/// Если функция возвращает Result, она возвращает ошибку типа AgentError: Result<T, AgentError>.
#[derive(Debug)]
pub enum AgentError {
    Recoverable(String), // Ошибка выполнения (продолжаем работу)
    Critical(String),    // Авария (прерываем цикл или паникуем)
}

impl std::fmt::Display for AgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentError::Recoverable(s) => write!(f, "{}", s),
            AgentError::Critical(s) => write!(f, "{}", s),
        }
    }
}   // impl Display for AgentError

/// Регистрирует информационное сообщение (технический лог).
///
/// Сообщение записывается только в журнал ошибок (если доступен).
/// В stdout сообщение не отправляется.
///
/// Функция безопасна для вызова из любого потока.
///
/// # Параметры
/// - `msg`: Текст сообщения для записи.
///
/// # Побочные эффекты
/// - Пишет строку в файл журнала ошибок (best effort).
pub fn handle_log(msg: &str) {
    eprintln!("ИНФО: {}", msg);
}   // handle_log()

/// Регистрирует некритическую ошибку.
///
/// Ошибка записывается в лог (если доступен).
/// Работа приложения не прерывается.
///
/// Функция безопасна для вызова из любого потока.
pub fn handle_error(msg: &str) {

    // 1. Пишем в stderr (уйдет в лог-файл через bat-файл)
    eprintln!("{}", msg);

    // --- 2. Сообщить AI. ---
    // 2.1. Получаем заголовок окна AI.
    let window_title = match session::window_title() {
        Ok(title) => title,
        Err(e) => {
            show_error_message("Критическая ошибка Хобота",
                               &format!("Сессия не инициализирована: {}", e));
            return;
        }
    };

    // 2.2. Отправляем текст.
    let msg = msg.trim();
    if !msg.is_empty() {
        if let Err(e) = paste_text_into_window_by_needle(&window_title, msg) {
            show_error_message("Ошибка Хобота",
                               &format!("Не удалось вставить отчёт в окно AI '{}':\n{}\nТекст: '{}'",
                                        window_title, e, &substring(msg, 0, Some(100))));
            return;
        }
    }
}   // process_error()
