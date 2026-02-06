#![cfg(test)]
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
use std::path::Path;
use crate::{writln, wrln};
use crate::glob::log_control;

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

/// Генерирует таймстамп в формате, совместимом с hobot.bat.
///
/// Формат: `YYYY-MM-DD_HH.MM.SS`
/// Пример: `2026-02-05_15.46.52`
///
/// # Примечания
/// - Используется локальное время (как в WMIC `localdatetime`).
/// - Точность до секунд (как и в батнике после обрезки).
///
/// # Возвращаемое значение
/// Тип: String: Таймстамп для имени каталога логов.
pub(crate) fn build_log_timestamp_like_bat() -> String {
    chrono::Local::now().format("%Y-%m-%d_%H.%M.%S").to_string()
}   // build_log_timestamp_like_bat()

/// Выводит текущее содержимое `work.log` в стандартный поток вывода.
///
/// Полезно для отладки тестов, чтобы видеть, что агент реально писал в “сырой” журнал:
/// - входящие директивы,
/// - отчёты по директивам,
/// - служебные сообщения.
///
/// # Поведение
/// - Строит путь: `log/<TS>/work.log`, где `<TS>` берется из `log_control`.
/// - Если файл существует: выводит его содержимое с разделителями.
/// - Если файл не найден или не читается: выводит предупреждение с путем и причиной.
///
/// # Побочные эффекты
/// - Читает `work.log` с диска.
/// - Пишет содержимое в stdout.
pub fn print_work_log() {
    let ts = log_control::log_timestamp();
    let path = Path::new("log").join(ts).join("work.log");

    match fs::read_to_string(&path) {
        Ok(content) => writln!(
            "\n=== WORK.LOG CONTENT ({}) ===\n{}\n==============================",
            path.display(),
            content
        ),
        Err(e) => {
            wrln!("\n[!] work.log не найден/не читается: {}\nПричина: {}", path.display(), e);
        }
    }   // match
}   // print_work_log()

/// Выводит содержимое файла `stderr.log` в стандартный поток вывода.
///
/// Это “журнал ошибок” процесса: весь вывод `eprintln!()` уходит туда через редирект в bat-файле.
///
/// # Поведение
/// - Строит путь: `log/<TS>/stderr.log`, где `<TS>` берется из `log_control`.
/// - Если файл существует: выводит его содержимое с разделителями.
/// - Если файл не найден или не читается: выводит предупреждение с путем и причиной.
///
/// # Побочные эффекты
/// - Читает `stderr.log` с диска.
/// - Пишет содержимое в stdout.
pub fn print_error_log() {
    let ts = log_control::log_timestamp();
    let path = Path::new("log").join(ts).join("stderr.log");

    match fs::read_to_string(&path) {
        Ok(content) => writln!(
            "\n=== STDERR.LOG CONTENT ({}) ===\n{}\n================================",
            path.display(),
            content
        ),
        Err(e) => {
            wrln!("\n[!] stderr.log не найден/не читается: {}\nПричина: {}", path.display(), e);
        }
    }   // match
}   // print_error_log()

/// Выводит содержимое файла `comment_log.md` в стандартный поток вывода.
///
/// Используется для локальной отладки тестов: позволяет быстро увидеть компактный журнал
/// (комментарии директив/команд).
///
/// # Поведение
/// - Строит путь: `log/<TS>/comment_log.md`, где `<TS>` берется из `log_control`.
/// - Если файл существует: выводит его содержимое с разделителями.
/// - Если файл не найден или не читается: выводит предупреждение с путем и причиной.
///
/// # Побочные эффекты
/// - Читает `comment_log.md` с диска.
/// - Пишет содержимое в stdout.
pub fn print_comment_log() {
    let ts = log_control::log_timestamp();
    let path = Path::new("log").join(ts).join("comment_log.md");

    match fs::read_to_string(&path) {
        Ok(content) => writln!(
            "\n=== COMMENT_LOG.MD CONTENT ({}) ===\n{}\n====================================",
            path.display(),
            content
        ),
        Err(e) => {
            wrln!("\n[!] comment_log.md не найден/не читается: {}\nПричина: {}", path.display(), e);
        }
    }   // match
}   // print_comment_log()

/// Выводит содержимое файла `logic_log.md` в стандартный поток вывода.
///
/// Используется для локальной отладки тестов: позволяет проверить, что записи “плана/логики”
/// попали в основной пользовательский журнал.
///
/// # Поведение
/// - Строит путь: `log/<TS>/logic_log.md`, где `<TS>` берется из `log_control`.
/// - Если файл существует: выводит его содержимое с разделителями.
/// - Если файл не найден или не читается: выводит предупреждение с путем и причиной.
///
/// # Побочные эффекты
/// - Читает `logic_log.md` с диска.
/// - Пишет содержимое в stdout.
pub fn print_logic_log() {
    let ts = log_control::log_timestamp();
    let path = Path::new("log").join(ts).join("logic_log.md");

    match fs::read_to_string(&path) {
        Ok(content) => writln!(
            "\n=== LOGIC_LOG.MD CONTENT ({}) ===\n{}\n==================================",
            path.display(),
            content
        ),
        Err(e) => {
            wrln!("\n[!] logic_log.md не найден/не читается: {}\nПричина: {}", path.display(), e);
        }
    }   // match
}   // print_logic_log()
