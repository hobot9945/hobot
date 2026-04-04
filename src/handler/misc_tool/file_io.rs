//! file_io.rs — Хэндлеры для прямого чтения и записи файлов с поддержкой кодировок.
//!
//! ОПИСАНИЕ:
//! Модуль предоставляет инструменты для работы с файловой системой в обход командной оболочки.
//! Это позволяет:
//! - передавать тексты любого размера;
//! - явно управлять кодировкой (UTF-8, Windows-1251 и т.д.).

use std::collections::HashMap;
use std::fs;
use encoding_rs::Encoding;
use crate::handler::{check_param_type, HandlerFn};
use crate::library::markdown_fence::wrap_in_fence;
use crate::glob;

/// Регистрирует обработчики команд работы с файлами в карту хэндлеров.
pub fn handlers_map_init(handlers_map: &mut HashMap<&str, HandlerFn>) {
    handlers_map.insert("write_file", write_file);
    handlers_map.insert("read_file", read_file);
}   // handlers_map_init()

/// Описание: Записывает текст в файл с указанной кодировкой.
///
/// # Параметры
/// - `params`: `["<path>", "<text>", "<encoding>"]`
///   - `<path>`: Путь к файлу.
///   - `<text>`: Содержимое.
///   - `<encoding>` (опционально): Имя кодировки (например, "windows-1251"). По умолчанию "utf-8".
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - Число параметров не 2 или 3.
/// - Кодировка не распознана.
/// - Отказано в доступе (os_readonly).
/// - Ошибка записи (права, путь).
fn write_file(params: &Option<Vec<String>>) -> Result<String, String> {
    // 1) Валидация количества параметров (2 или 3).
    let count = params.as_ref().map_or(0, |v| v.len());
    if count < 2 || count > 3 {
        return Err(format!("Неверное число параметров: ожидалось 2 или 3, получено {}", count));
    }   // if

    let path: String = check_param_type(params, 0)?;
    let text: String = check_param_type(params, 1)?;
    let encoding_label = if count == 3 {
        Some(check_param_type::<String>(params, 2)?)
    } else {
        None
    };

    // 2) Безопасность.
    let action_desc = format!("запись в файл: {}", path);
    if !glob::ask_execution_permission(&action_desc) {
        return Err("Отказано в доступе: Пользователь запретил запись в файл.".to_string());
    }   // if

    // 3) Выбор кодировки.
    let encoding = if let Some(label) = encoding_label {
        Encoding::for_label(label.as_bytes()).ok_or_else(|| format!("Неизвестная кодировка: '{}'", label))?
    } else {
        encoding_rs::UTF_8
    };

    // 4) Кодирование и запись.
    let (bytes, _, _) = encoding.encode(&text);
    fs::write(&path, &bytes).map_err(|e| {
        format!("Ошибка записи в файл '{}': {}", path, e)
    })?;

    // 5) Отчет.
    let out = format!(
        "Файл успешно записан.\nПуть: {}\nКодировка: {}\nРазмер: {} байт",
        path,
        encoding.name(),
        bytes.len()
    );

    Ok(wrap_in_fence(&out))
}   // write_file()

/// Описание: Читает файл с диска и декодирует его в текст.
///
/// # Параметры
/// - `params`: `["<path>", "<encoding>"]`
///   - `<path>`: Путь к файлу.
///   - `<encoding>` (опционально): Имя кодировки. По умолчанию "utf-8".
///
/// # Возвращаемое значение
/// Markdown-блок с содержимым файла.
fn read_file(params: &Option<Vec<String>>) -> Result<String, String> {
    // 1) Валидация (1 или 2 параметра).
    let count = params.as_ref().map_or(0, |v| v.len());
    if count < 1 || count > 2 {
        return Err(format!("Неверное число параметров: ожидалось 1 или 2, получено {}", count));
    }   // if

    let path: String = check_param_type(params, 0)?;
    let encoding_label = if count == 2 {
        Some(check_param_type::<String>(params, 1)?)
    } else {
        None
    };

    // 2) Чтение байт.
    let bytes = fs::read(&path).map_err(|e| {
        format!("Ошибка чтения файла '{}': {}", path, e)
    })?;

    // 3) Выбор кодировки.
    let encoding = if let Some(label) = encoding_label {
        Encoding::for_label(label.as_bytes()).ok_or_else(|| format!("Неизвестная кодировка: '{}'", label))?
    } else {
        encoding_rs::UTF_8
    };

    // 4) Декодирование.
    let (text, _, _) = encoding.decode(&bytes);

    Ok(wrap_in_fence(&text))
}   // read_file()