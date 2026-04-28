//! read_file.rs — Хэндлеры для чтения файлов с поддержкой кодировок.
//!
//! ОПИСАНИЕ:
//! Модуль предоставляет инструменты для работы с файловой системой в обход командной оболочки.
//! Это позволяет:
//! - передавать тексты любого размера;
//! - явно управлять кодировкой (UTF-8, Windows-1251 и т.д.).

use std::collections::HashMap;
use std::fs;
use encoding_rs::Encoding;
use crate::glob;
use crate::handler::{check_param_type, HandlerFn};
use crate::library::markdown_fence::wrap_in_fence;

/// Регистрирует обработчики команд работы с файлами в карту хэндлеров.
pub fn handlers_map_init(handlers_map: &mut HashMap<&str, HandlerFn>) {
    handlers_map.insert("read_file", read_file);
}   // handlers_map_init()

/// Описание: Читает файл с диска и декодирует его в текст.
///
/// # Параметры
/// - `params`: `["<path>", "<encoding>", "<enumerate>"]`
///   - `<path>`: Путь к файлу.
///   - `<encoding>` (опционально, обязан присутствовать, если есть третий параметр): Имя кодировки.
///     По умолчанию "utf-8".
///   - `<enumerate>` (опционально): Строка `"enumerate"`. Включает нумерацию строк с базой 1.
///
/// # Возвращаемое значение
/// Markdown-блок с содержимым файла.
fn read_file(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Валидация входных параметров (от 1 до 3 параметров)
    let count = params.as_ref().map_or(0, |v| v.len());
    if count < 1 || count > 3 {
        return Err(format!("Неверное число параметров: ожидалось от 1 до 3, получено {}", count));
    }   // if

    // Извлекаем обязательный путь к файлу (первый параметр)
    let path: String = check_param_type(params, 0)?;

    // Извлекаем опциональную кодировку (второй параметр)
    let encoding_label = if count >= 2 {
        // Второй параметр может быть кодировкой или флагом "enumerate" (если count == 2)
        // Но по контракту: если count == 3, то второй — кодировка. Если count == 2, то второй — кодировка.
        Some(check_param_type::<String>(params, 1)?)
    } else {
        None
    };

    // Извлекаем опциональный флаг нумерации строк (третий параметр)
    let mut enumerate = false;
    if count == 3 {
        // Третий параметр должен быть строго "enumerate"
        let enum_param: String = check_param_type(params, 2)?;
        if enum_param == "enumerate" {
            enumerate = true;
        } else {
            return Err(format!("Неверный третий параметр: '{}'. Ожидалось 'enumerate'", enum_param));
        }   // if
    }   // if

    // 2) Чтение файла как массива байт. Используем fs::read, чтобы получить все содержимое файла целиком.
    let bytes = fs::read(&path).map_err(|e| {
        // Возвращаем человеко-читаемую ошибку с путем к файлу
        format!("Ошибка чтения файла '{}': {}", path, e)
    })?;

    // 3) Выбор кодировки для декодирования
    // Если кодировка не указана — используем UTF-8 по умолчанию
    let encoding = if let Some(label) = encoding_label {
        // Ищем кодировку по строковому имени (например, "windows-1251", "ibm866")
        Encoding::for_label(label.as_bytes()).ok_or_else(|| {
            format!("Неизвестная кодировка: '{}'", label)
        })?
    } else {
        encoding_rs::UTF_8
    };

    // 4) Декодирование байтов в строку Rust (UTF-8)
    // encoding_rs возвращает (decoded_string, has_replacements, encoding_used)
    let (text, _has_replacements, _encoding_used) = encoding.decode(&bytes);
    let mut final_text = text.into_owned();

    // 5) Опциональная нумерация строк (если передан флаг "enumerate")
    // Используем глобальную функцию enumerate_lines для форматирования
    if enumerate {
        final_text = glob::enumerate_lines(&final_text, Some(1));
    }   // if

    // 6) Возвращаем результат в Markdown-блоке для удобства вывода
    Ok(wrap_in_fence(&final_text))
}   // read_file()
