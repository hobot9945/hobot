//! test_utils
//!
//! Утилиты тестов.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Утилиты подготовки моков Native Messaging (stdin).
//! - Утилиты упаковки сообщений в JSON-обёртку `{"text": ...}`.
//! - Утилиты вывода рабочих логов для локальной отладки тестов.

use std::fs;
use std::io::Cursor;
use crate::{writln, wrln};

/// Эмулирует файл (поток), из которого можно прочесть заданное сообщение.
///
/// Создает `Cursor` над байтовым вектором, содержащим длину сообщения (4 байта, native-endian)
/// и само сообщение. Это имитирует поведение `stdin` при получении пакета от браузера.
///
/// # Параметры
/// - `msg`: Содержимое сообщения (JSON), которое будет помещено в "файл".
///
/// # Возвращаемое значение
/// Объект типа `Cursor<Vec<u8>>`, реализующий типаж `Read`.
pub fn mock_stdin(msg: &str) -> Cursor<Vec<u8>> {
    let msg_len = msg.len() as u32;

    let mut input_data = Vec::new();
    // Native Messaging использует native-endian для длины сообщения
    input_data.extend_from_slice(&msg_len.to_ne_bytes());
    input_data.extend_from_slice(msg.as_bytes());

    Cursor::new(input_data)
}   // mock_stdin()

/// Оборачивает текст директивы в валидный JSON-пакет Native Messaging.
///
/// Формирует JSON вида: `{"text": "<raw_text>"}`.
/// Экранирование кавычек и переводов строк выполняет `serde_json`.
///
/// # Параметры
/// - `raw_text`: "Сырой" текст сообщения (например, с тегами `<<<ai ...`).
///
/// # Возвращаемое значение
/// Сериализованная JSON-строка, готовая к передаче в `mock_stdin`.
///
/// # Паника
/// Если сериализация не удалась (крайне редкий случай для простых строк).
pub fn wrap_to_native_json(raw_text: &str) -> String {
    let msg = serde_json::json!({
            "text": raw_text
        });

    serde_json::to_string(&msg).expect("Ошибка сериализации mock-пакета")
}   // wrap_to_native_json()

/// Выводит текущее содержимое `work.log` в стандартный поток вывода.
///
/// Полезно для отладки тестов, чтобы видеть, что агент реально писал в журнал.
///
/// # Поведение
/// - Если файл существует: выводит его содержимое с разделителями.
/// - Если файл не найден: выводит предупреждение с путем к файлу.
///
/// # Побочные эффекты
/// - Читает файл `work.log` с диска.
/// - Пишет содержимое в stdout.
pub fn print_work_log() {
    let path = crate::glob::config().worklog_path.clone();
    match fs::read_to_string(&path) {
        Ok(content) => writln!("\n=== WORK.LOG CONTENT ===\n{}\n========================", content),
        Err(_) => {
            wrln!("\n[!] work.log не найден по пути:", path);
        }
    }   // match
}   // print_work_log()

/// Выводит содержимое файла `error.log` в стандартный поток вывода.
///
/// Используется для отладки: позволяет проверить, какие ошибки зафиксировал агент.
///
/// # Поведение
/// - Если файл существует: выводит его содержимое с разделителями.
/// - Если файл не найден: выводит предупреждение с путем к файлу.
///
/// # Побочные эффекты
/// - Читает файл `error.log` с диска.
/// - Пишет содержимое в stdout.
pub fn print_error_log() {
    let path = crate::glob::config().errlog_path.clone();
    match fs::read_to_string(&path) {
        Ok(content) => writln!("\n=== ERROR.LOG CONTENT ===\n{}\n=========================", content),
        Err(_) => {
            wrln!("\n[!] error.log не найден по пути:", path);
        }
    }   // match
    
}   // print_error_log()

/// Удаляет файл `work.log`, если он существует.
///
/// # Побочные эффекты
/// - Удаляет файл журнала с диска (best effort).
pub fn delete_work_log() {
    let path = crate::glob::config().worklog_path.clone();

    // Если файла нет — это ок, ничего не делаем.
    if fs::metadata(&path).is_err() {
        return;
    }   // if

    // Best effort: тестам важнее продолжить работу, чем падать на cleanup.
    let _ = fs::remove_file(&path);
}   // delete_work_log()

/// Удаляет файл `error.log`, если он существует.
///
/// # Побочные эффекты
/// - Удаляет файл журнала ошибок с диска (best effort).
pub fn delete_error_log() {
    let path = crate::glob::config().errlog_path.clone();

    // Если файла нет — это ок, ничего не делаем.
    if fs::metadata(&path).is_err() {
        return;
    }   // if

    // Best effort: тестам важнее продолжить работу, чем падать на cleanup.
    let _ = fs::remove_file(&path);
}   // delete_error_log()
