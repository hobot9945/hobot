//! stdout_sink.rs
//!
//! Модуль низкоуровневой отправки сообщений в стандартный вывод (stdout) для взаимодействия с расширением.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Формирование сообщений о критических ошибках в соответствие с протоколом. Обертка JSON-полезной
//!   нагрузки в текстовые маркеры Хобота (`<<<hbt ... >>>hbt`).
//! - Делегирование физической отправки функции `glob::send_to_stdout`.
//!
//! # ПРОТОКОЛ
//! Сообщения формируются в виде: `<<<hbt\n[JSON]\n>>>hbt`.

use serde::Serialize;
use crate::glob;

/// Структура сообщения о критической ошибке для протокола `protocol.md`.
///
/// Используется для уведомления расширения о фатальных сбоях агента.
#[derive(Serialize, Debug)]
struct CriticalErrorMessage {
    /// Тип сообщения. Соответствует `CRITICAL_ERROR` в спецификации.
    #[serde(rename = "type")]
    msg_type: &'static str,
    /// Текстовое описание ошибки.
    error: String,
}   // CriticalErrorMessage

/// Отправляет сообщение о критической ошибке в stdout.
///
/// Функция формирует JSON, оборачивает его в протокольные скобки и отправляет
/// через общий транспорт `glob::send_raw_to_stdout`.
///
/// # Параметры
/// - `msg`: Текст ошибки для отправки.
///
/// # Поведение при ошибках
/// Ошибки сериализации или ввода-вывода игнорируются (best effort),
/// так как данная функция вызывается при уже критической ситуации.
pub fn send_critical_to_stdout(msg: &str) {
    // Формируем полезную нагрузку
    let payload = CriticalErrorMessage {
        msg_type: "CRITICAL_ERROR",
        error: msg.to_string(),
    };

    // Сериализуем в JSON с фоллбэком на валидную заглушку
    let json_body = serde_json::to_string(&payload).unwrap_or_else(|_| {
        r#"{"msg_type":"CRITICAL_ERROR","error":"Serialization failed"}"#.to_string()
    });

    // Оборачиваем в маркеры протокола Хобота
    let protocol_message = format!("<<<hbt\n{}\n>>>hbt", json_body);

    // Отправляем через общий модуль. Ошибки игнорируем, т.к. это "последний крик".
    let _ = glob::send_to_stdout(&protocol_message);
}   // send_critical_to_stdout()