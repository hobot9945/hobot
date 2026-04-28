//! write_append_file.rs — Хэндлеры для записи и дозаписи файлов с поддержкой кодировок.
//!
//! Команда write_file поддерживает четвёртый параметр "append" для дозаписи в конец файла.
//! При дозаписи автоматически добавляется перевод строки, если файл не заканчивается им.

use crate::glob;
use crate::handler::{check_param_type, HandlerFn};
use crate::library::markdown_fence::wrap_in_fence;
use encoding_rs::Encoding;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;

/// Регистрирует обработчики команд работы с файлами.
pub fn handlers_map_init(handlers_map: &mut HashMap<&str, HandlerFn>) {
    handlers_map.insert("write_file", write_file);
}   // handlers_map_init()

/// Описание: Записывает или дозаписывает текст в файл с указанной кодировкой.
///
/// # Параметры
/// - `params`: `["<path>", "<text>", "<encoding>", "<mode>"]`
///   - `<path>`: Путь к файлу.
///   - `<text>`: Текст для записи.
///   - `<encoding>` (опционально, обязателен если есть четвёртый параметр):
///     Имя кодировки (`"utf-8"`, `"windows-1251"`, `"ibm866"`). По умолчанию `"utf-8"`.
///   - `<mode>` (опционально): `"append"` — дозапись в конец файла.
///     Без параметра — полная перезапись.
///
/// # Поведение при append:
/// - Если файл существует и не заканчивается переводом строки (`\n` или `\r\n`),
///   автоматически добавляется перевод строки перед новым текстом.
/// - Стиль переноса строки определяется по содержимому файла (CRLF/LF).
/// - Если файл не существует — текст записывается без добавления перевода строки.
///
/// # Примеры:
/// - `["file.txt", "Новый текст"]` → перезапись, utf-8
/// - `["file.txt", "Текст", "windows-1251"]` → перезапись, windows-1251
/// - `["file.txt", "Строка", "utf-8", "append"]` → дозапись, utf-8
fn write_file(params: &Option<Vec<String>>) -> Result<String, String> {
    // 1) Валидация количества параметров (от 2 до 4)
    let count = params.as_ref().map_or(0, |v| v.len());
    if count < 2 || count > 4 {
        return Err(format!(
            "Неверное число параметров: ожидалось от 2 до 4, получено {}", count
        ));
    }   // if

    let path: String = check_param_type(params, 0)?;
    let text: String = check_param_type(params, 1)?;

    // Извлекаем опциональную кодировку (третий параметр)
    let encoding_label = if count >= 3 {
        Some(check_param_type::<String>(params, 2)?)
    } else {
        None
    };

    // Извлекаем опциональный флаг дозаписи (четвёртый параметр)
    let mut is_append = false;
    if count == 4 {
        let mode: String = check_param_type(params, 3)?;
        if mode == "append" {
            is_append = true;
        } else {
            return Err(format!(
                "Неверный четвёртый параметр: '{}'. Ожидалось 'append'.", mode
            ));
        }   // if
    }   // if

    // 2) Запрос разрешения у пользователя
    #[cfg(not(test))]
    {
        let action = if is_append {
            format!("дозапись (append) в файл: {}", path)
        } else {
            format!("полная перезапись файла: {}", path)
        };
        if !glob::ask_execution_permission(&action) {
            return Err("Отказано в доступе: Пользователь запретил изменение файла.".to_string());
        }   // if
    }

    // 3) Выбор кодировки
    let encoding = if let Some(label) = encoding_label {
        Encoding::for_label(label.as_bytes()).ok_or_else(|| {
            format!("Неизвестная кодировка: '{}'", label)
        })?
    } else {
        encoding_rs::UTF_8
    };

    // 4) Кодирование текста
    let (bytes, _, _) = encoding.encode(&text);

    // 5) Выполнение записи
    if is_append {
        // Дозапись в конец файла (с созданием, если не существует)
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| format!("Не удалось открыть файл '{}' для дозаписи: {}", path, e))?;

        // Проверяем, нужно ли добавить перевод строки перед новым текстом.
        // Читаем существующий файл и проверяем его последний байт.
        let needs_newline = _file_needs_trailing_newline(&path)?;

        // Если файл существует и не заканчивается переводом строки — добавляем
        if needs_newline {
            // Определяем стиль переноса строки по содержимому файла
            let newline_bytes = _detect_newline_bytes(&path);
            file.write_all(&newline_bytes)
                .map_err(|e| format!("Ошибка добавления перевода строки: {}", e))?;
        }   // if

        // Записываем основной текст
        file.write_all(&bytes)
            .map_err(|e| format!("Ошибка дозаписи в файл '{}': {}", path, e))?;
    } else {
        // Полная перезапись файла
        fs::write(&path, &bytes)
            .map_err(|e| format!("Ошибка записи в файл '{}': {}", path, e))?;
    }   // if

    // 6) Формирование отчёта
    let mode_str = if is_append { "дозаписан (append)" } else { "перезаписан" };

    let out = format!(
        "Файл успешно {}.\nПуть: {}\nКодировка: {}\nРазмер: {} байт",
        mode_str, path, encoding.name(), bytes.len()
    );

    Ok(wrap_in_fence(&out))
}   // write_file()

//--------------------------------------------------------------------------------------------------
//                  Внутренний интерфейс
//--------------------------------------------------------------------------------------------------

/// Проверяет, нужно ли добавить перевод строки перед дозаписью.
///
/// Работает напрямую с байтами: `0x0A` — это `\n` (LF).
/// Если файл заканчивается на `0x0A`, значит перевод строки есть (покрывает и LF, и CRLF).
fn _file_needs_trailing_newline(path: &str) -> Result<bool, String> {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return Ok(false), // Файл не существует — не нужно добавлять
    };

    // Пустой файл — не нужно добавлять
    if bytes.is_empty() {
        return Ok(false);
    }   // if

    // Проверяем последний байт: если это 0x0A (\n), значит перевод строки уже есть
    Ok(bytes.last() != Some(&0x0A))
}   // file_needs_trailing_newline()

/// Определяет стиль переноса строки по содержимому файла и возвращает байты этого стиля.
///
/// Ищет последовательность `0x0D 0x0A` (\r\n). Если найдена — возвращает CRLF,
/// иначе — LF.
fn _detect_newline_bytes(path: &str) -> Vec<u8> {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return vec![0x0A], // По умолчанию LF
    };

    // Ищем CRLF в байтах файла
    let has_crlf = bytes.windows(2).any(|w| w == &[0x0D, 0x0A]);

    if has_crlf {
        vec![0x0D, 0x0A] // CRLF
    } else {
        vec![0x0A]       // LF
    }   // if
}   // detect_newline_bytes()

//--------------------------------------------------------------------------------------------------
//                  Дымовые тесты
//--------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_PATH: &str = "c:\\tmp\\test.txt";

    /// Очищает тестовый файл перед каждым тестом.
    fn cleanup() {
        let _ = fs::remove_file(TEST_PATH);
    }

    /// Тест: запись двух строк в новый файл.
    #[test]
    fn test_write_file_two_lines() {
        cleanup();

        let text = "Имею я златые горы\nИ есть что жрать и есть что пить";
        let params = Some(vec![TEST_PATH.to_string(), text.to_string()]);

        let result = write_file(&params);

        assert!(result.is_ok(), "write_file вернул ошибку: {:?}", result);

        // Проверяем содержимое
        let content = fs::read_to_string(TEST_PATH).expect("Не удалось прочитать файл");
        assert_eq!(content, text);
    }

    /// Тест: дозапись в пустой файл (файл не существует).
    /// Перевод строки не должен добавляться перед текстом.
    #[test]
    fn test_append_to_new_file() {

        let text = "Имею я златые горы\nИ есть что жрать и есть что пить";
        let params = Some(vec![
            TEST_PATH.to_string(),
            text.to_string(),
            "utf-8".to_string(),
            "append".to_string(),
        ]);

        let result = write_file(&params);

        assert!(result.is_ok(), "write_file append вернул ошибку: {:?}", result);

        // Проверяем: не должно быть лидирующего перевода строки
        let content = fs::read_to_string(TEST_PATH).expect("Не удалось прочитать файл");
        assert_eq!(content, text, "В начале не должно быть перевода строки");
    }

    /// Тест: дозапись в файл, не заканчивающийся переводом строки.
    /// Должен автоматически добавиться \n перед новым текстом.
    #[test]
    fn test_append_adds_newline() {

        test_write_file_two_lines();

        let append_text = "Но, крашу, крашу я заборы\nЧтоб тунеядцем не прослыть.";
        let params = Some(vec![
            TEST_PATH.to_string(),
            append_text.to_string(),
            "utf-8".to_string(),
            "append".to_string(),
        ]);

        let result = write_file(&params);

        assert!(result.is_ok(), "write_file append вернул ошибку: {:?}", result);
    }

    /// Тест: дозапись в файл, уже заканчивающийся переводом строки.
    /// Лишний перевод строки не должен добавляться.
    #[test]
    fn test_append_no_extra_newline() {
        cleanup();

        // Записываем файл С переводом строки в конце
        fs::write(TEST_PATH, "первая строка\n").expect("Не удалось создать файл");

        let append_text = "вторая строка";
        let params = Some(vec![
            TEST_PATH.to_string(),
            append_text.to_string(),
            "utf-8".to_string(),
            "append".to_string(),
        ]);

        let result = write_file(&params);

        assert!(result.is_ok(), "write_file append вернул ошибку: {:?}", result);
    }
}