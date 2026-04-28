//! patch_file.rs — Хэндлер для замены/вставки/удаления диапазона строк в файле.
//!
//! ОПИСАНИЕ:
//! Модуль предоставляет команду patch_file, которая позволяет:
//! - заменить диапазон строк на новый текст;
//! - удалить диапазон строк (передав пустой new_text);
//! - вставить строки перед указанной позицией (если end_line < start_line);
//! - вставить строки в конец файла (start_line = total_lines + 1, end_line < start_line).

use std::collections::HashMap;
use std::fs;
use encoding_rs::Encoding;
use crate::handler::{check_param_type, HandlerFn};
use crate::library::markdown_fence::wrap_in_fence;
use crate::glob;

/// Регистрирует обработчик команды patch_file.
pub fn handlers_map_init(handlers_map: &mut HashMap<&str, HandlerFn>) {
    handlers_map.insert("patch_file", patch_file);
}   // handlers_map_init()

/// Описание: Заменяет, удаляет или вставляет диапазон строк в файле по номерам (1-based).
///
/// # Параметры
/// - `params`: `["<path>", "<start_line>", "<end_line>", "<new_text>", "<encoding>"]`
///   - `<path>`: Путь к файлу.
///   - `<start_line>`: Номер первой строки (1-based).
///   - `<end_line>`: Номер последней строки диапазона (1-based, включительно).
///     Если `< end_line` меньше `< start_line>` — выполняется вставка перед `< start_line>`
///     без удаления существующих строк.
///   - `<new_text>`: Текст для вставки. Пустая строка — удаление диапазона.
///   - `<encoding>` (опционально): Кодировка. По умолчанию "utf-8".
fn patch_file(params: &Option<Vec<String>>) -> Result<String, String> {
    // 1) Валидация количества параметров (4 или 5)
    let count = params.as_ref().map_or(0, |v| v.len());
    if count < 4 || count > 5 {
        return Err(format!(
            "Неверное число параметров: ожидалось 4 или 5, получено {}", count
        ));
    }   // if

    let path: String = check_param_type(params, 0)?;
    let start_line: usize = check_param_type(params, 1)?;
    let end_line: usize = check_param_type(params, 2)?;
    let new_text: String = check_param_type(params, 3)?;
    let encoding_label = if count == 5 {
        Some(check_param_type::<String>(params, 4)?)
    } else {
        None
    };

    // 2) Валидация номеров строк
    if start_line == 0 {
        return Err("start_line должен быть >= 1 (нумерация с 1).".to_string());
    }   // if

    // 3) Запрос разрешения у пользователя
    let action_desc = if end_line < start_line {
        format!("вставка строк перед строкой {} в файле: {}", start_line, path)
    } else if new_text.is_empty() {
        format!("удаление строк {}-{} в файле: {}", start_line, end_line, path)
    } else {
        format!("замена строк {}-{} в файле: {}", start_line, end_line, path)
    };

    #[cfg(not(test))]
    if !glob::ask_execution_permission(&action_desc) {
        return Err("Отказано в доступе: Пользователь запретил изменение файла.".to_string());
    }   // if

    // 4) Чтение файла и определение кодировки
    let bytes = match fs::read(&path) {
        Ok(b) => b,
        // Если файла нет — считаем его пустым (создадим при записи)
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(e) => return Err(format!("Ошибка чтения '{}': {}", path, e)),
    };

    let encoding = if let Some(label) = encoding_label {
        Encoding::for_label(label.as_bytes()).ok_or_else(|| {
            format!("Неизвестная кодировка: '{}'", label)
        })?
    } else {
        encoding_rs::UTF_8
    };

    let (decoded, _, _) = encoding.decode(&bytes);
    let file_text = decoded.into_owned();

    // 5) Определение стиля переноса строк в оригинальном файле
    let use_crlf = file_text.contains("\r\n");
    let newline_str = if use_crlf { "\r\n" } else { "\n" };

    // Запоминаем, заканчивался ли файл переводом строки
    let ends_with_newline = file_text.ends_with('\n');

    // 6) Разбиваем файл на строки
    let mut lines: Vec<&str> = file_text.lines().collect();
    let total_lines = lines.len();

    // 7) Преобразуем 1-based номера в 0-based индексы
    let start_idx = start_line.saturating_sub(1);
    let is_insert = end_line < start_line;

    // 8) Проверки выхода за пределы файла
    if is_insert {
        // Вставка: разрешаем start_idx == total_lines (вставка в конец)
        if start_idx > total_lines {
            return Err(format!(
                "start_line ({}) выходит за пределы файла (всего строк: {}). Вставка возможна до строки {}.",
                start_line, total_lines, total_lines + 1
            ));
        }   // if
    } else {
        // Замена/удаление: оба индекса должны быть внутри файла
        if start_idx >= total_lines {
            return Err(format!(
                "start_line ({}) выходит за пределы файла (всего строк: {}).",
                start_line, total_lines
            ));
        }   // if

        if end_line > total_lines {
            return Err(format!(
                "end_line ({}) выходит за пределы файла (всего строк: {}).",
                end_line, total_lines
            ));
        }   // if
    }   // if

    // Определяем диапазон удаления
    let end_idx = if is_insert {
        start_idx // Диапазон пустой, splice только вставит
    } else {
        end_line // end_line уже проверена, она <= total_lines
    };

    // 9) Нормализуем new_text и разбиваем на строки
    let new_text_norm = new_text.replace("\r\n", "\n");
    let new_lines: Vec<&str> = if new_text_norm.is_empty() {
        Vec::new() // Пустой текст — удаление
    } else {
        new_text_norm.lines().collect()
    };

    // 10) Выполняем замену/вставку/удаление через splice
    lines.splice(start_idx..end_idx, new_lines);

    // 11) Склеиваем строки обратно
    let mut final_text = lines.join(newline_str);

    // Если это вставка в конец файла — обеспечиваем перевод строки в конце,
    // аналогично поведению append.
    if is_insert && start_idx == total_lines && !new_text_norm.is_empty() {
        if !final_text.ends_with('\n') {
            final_text.push_str(newline_str);
        }   // if
    }   // if

    // Восстанавливаем хвостовой перевод строки, если он был в оригинале
    if ends_with_newline && !final_text.is_empty() && !final_text.ends_with('\n') {
        final_text.push_str(newline_str);
    }   // if

    // 12) Атомарная запись результата
    let (out_bytes, _, _) = encoding.encode(&final_text);
    let tmp_path = format!("{}.tmp", path);
    fs::write(&tmp_path, &out_bytes)
        .map_err(|e| format!("Ошибка записи во временный файл: {}", e))?;
    fs::rename(&tmp_path, &path).map_err(|e| {
        let _ = fs::remove_file(&tmp_path);
        format!("Ошибка атомарного сохранения файла: {}", e)
    })?;

    // 13) Формирование отчёта
    let action_str = if is_insert {
        format!("вставка перед строкой {}", start_line)
    } else if new_text.is_empty() {
        format!("удаление строк {}-{}", start_line, end_line)
    } else {
        format!("замена строк {}-{}", start_line, end_line)
    };

    let out = format!(
        "Патч применён: {}.\nПуть: {}\nКодировка: {}\nСтрок в файле: {} → {}\nРазмер: {} байт",
        action_str, path, encoding.name(), total_lines, lines.len(), out_bytes.len()
    );

    Ok(wrap_in_fence(&out))
}   // patch_file()

//--------------------------------------------------------------------------------------------------
//                  Дымовые тесты
//--------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_PATH: &str = "c:\\tmp\\patch_test.txt";

    fn cleanup() {
        let _ = fs::remove_file(TEST_PATH);
    }

    /// Тест: Вставка в несуществующий файл.
    /// Ожидаемое поведение: Код должен либо создать файл (если мы добавим фикс NotFound),
    /// либо вернуть понятную ошибку чтения.
    #[test]
    fn test_patch_insert_into_non_existent_file() {
        cleanup();

        let text = "Имею я златые горы\nИ есть что жрать и есть что пить";

        // Параметры: вставка перед 1-й строкой (end_line < start_line)
        let params = Some(vec![
            TEST_PATH.to_string(),
            "1".to_string(),
            "0".to_string(),
            text.to_string(),
        ]);

        let result = patch_file(&params);

        // Если ты НЕ вносил правку с ErrorKind::NotFound, тест упадет здесь (будет Err)
        assert!(result.is_ok(), "patch_file должен уметь создавать файл при вставке в начало, но вернул: {:?}", result);

        let content = fs::read_to_string(TEST_PATH).expect("Файл должен был создаться");

        // Важно: так как это вставка в "конец" (файл был пуст), сработает логика обеспечения перевода строки в конце.
    }

    /// Тест: Вставка блока текста в конец существующего файла (после 2-й строки).
    #[test]
    fn test_patch_insert_at_end_of_existing_file() {
        cleanup();

        // 1. Подготавливаем файл с двумя строками (без хвостового перевода строки)
        let initial_text = "Имею я златые горы\nИ есть что жрать и есть что пить";
        fs::write(TEST_PATH, initial_text).expect("Не удалось подготовить файл");

        let append_text = "Но, крашу разные приборы,\nЧтоб алконавтом не прослыть.\n\nНародное творчество.";

        // 2. Вставка в конец: start_line = 3 (сразу за последней), end_line = 2 (вставка)
        let params = Some(vec![
            TEST_PATH.to_string(),
            "3".to_string(),
            "2".to_string(),
            append_text.to_string(),
        ]);

        let result = patch_file(&params);

        assert!(result.is_ok(), "patch_file вернул ошибку: {:?}", result);

        // 3. Проверка содержимого
        let content = fs::read_to_string(TEST_PATH).expect("Не удалось прочитать файл");

        // Проверяем структуру:
        // Строка 1: Имею я златые горы
        // Строка 2: И есть что жрать и есть что пить
        // (авто-вставленный перевод строки между ними)
        // Строка 3: Но, крашу разные приборы,
        // ...
        // Последняя строка: Народное творчество.
        // (авто-вставленный перевод строки в самом конце)

        let expected = "Имею я златые горы\nИ есть что жрать и есть что пить\n\
                        Но, крашу разные приборы,\nЧтоб алконавтом не прослыть.\n\n\
                        Народное творчество.\n";
    }

    /// Тест: Замена диапазона строк (3-4) в середине файла.
    #[test]
    fn test_patch_replace_lines_3_4() {
        cleanup();

        // 1. Подготавливаем файл (состояние после предыдущего шага)
        let initial_text = "Имею я златые горы\n\
                            И есть что жрать и есть что пить\n\
                            Но, крашу разные приборы,\n\
                            Чтоб алконавтом не прослыть.\n\n\
                            Народное творчество.\n";
        fs::write(TEST_PATH, initial_text).expect("Не удалось подготовить файл");

        let replacement_text = "Но, крашу, крашу я заборы,\nЧтоб тунеядцем не прослыть!";

        // 2. Параметры: замена диапазона с 3 по 4 включительно
        let params = Some(vec![
            TEST_PATH.to_string(),
            "3".to_string(),
            "4".to_string(),
            replacement_text.to_string(),
        ]);

        let result = patch_file(&params);

        assert!(result.is_ok(), "patch_file (replace 3-4) вернул ошибку: {:?}", result);

        // 3. Проверка результата
        let content = fs::read_to_string(TEST_PATH).expect("Не удалось прочитать файл");

        let expected = "Имею я златые горы\n\
                        И есть что жрать и есть что пить\n\
                        Но, крашу, крашу я заборы,\n\
                        Чтоб тунеядцем не прослыть!\n\n\
                        Народное творчество.\n";

        assert_eq!(content.replace("\r\n", "\n"), expected.replace("\r\n", "\n"));
    }

    /// Тест: Вставка строк в начало файла (перед 1-й строкой).
    #[test]
    fn test_patch_insert_at_beginning() {
        cleanup();

        // 1. Подготавливаем файл (текущее состояние стиха)
        let initial_text = "Имею я златые горы\n\
                            И есть что жрать и есть что пить\n\
                            Но, крашу, крашу я заборы,\n\
                            Чтоб тунеядцем не прослыть!\n\n\
                            Народное творчество.\n";
        fs::write(TEST_PATH, initial_text).expect("Не удалось подготовить файл");

        let header_text = "Русская народная\n\n"; // С пустой строкой для отступа

        // 2. Параметры: вставка перед 1-й строкой (start=1, end=0)
        let params = Some(vec![
            TEST_PATH.to_string(),
            "1".to_string(),
            "0".to_string(),
            header_text.to_string(),
        ]);

        let result = patch_file(&params);

        assert!(result.is_ok(), "patch_file (insert at beginning) вернул ошибку: {:?}", result);

        // 3. Проверка результата
        let content = fs::read_to_string(TEST_PATH).expect("Не удалось прочитать файл");

        // Ожидаем, что заголовок появился в самом начале, а старый текст сдвинулся вниз
        let expected = "Русская народная\n\n\
                        Имею я златые горы\n\
                        И есть что жрать и есть что пить\n\
                        Но, крашу, крашу я заборы,\n\
                        Чтоб тунеядцем не прослыть!\n\n\
                        Народное творчество.\n";

        assert_eq!(content.replace("\r\n", "\n"), expected.replace("\r\n", "\n"));
    }

    /// Тест: Удаление диапазона строк (последние две строки).
    #[test]
    fn test_patch_delete_last_two_lines() {
        cleanup();

        // 1. Подготавливаем файл (состояние после вставки заголовка)
        // ВАЖНО: При вставке "Русская народная\n\n" перед 1-й строкой,
        // стих теперь начинается с 3-й строки. Подпись оказалась на 7 и 8.
        let initial_text = "Русская народная\n\n\
                            Имею я златые горы\n\
                            И есть что жрать и есть что пить\n\
                            Но, крашу, крашу я заборы,\n\
                            Чтоб тунеядцем не прослыть!\n\n\
                            Народное творчество.\n";
        fs::write(TEST_PATH, initial_text).expect("Не удалось подготовить файл");

        // 2. Параметры: удаление строк 7 и 8 (new_text = "")
        let params = Some(vec![
            TEST_PATH.to_string(),
            "7".to_string(),
            "8".to_string(),
            "".to_string(),
        ]);

        let result = patch_file(&params);

        assert!(result.is_ok(), "patch_file (delete 7-8) вернул ошибку: {:?}", result);

        // 3. Проверка результата
        let content = fs::read_to_string(TEST_PATH).expect("Не удалось прочитать файл");

        // Ожидаем, что подпись и пустая строка перед ней исчезли
        let expected = "Русская народная\n\n\
                        Имею я златые горы\n\
                        И есть что жрать и есть что пить\n\
                        Но, крашу, крашу я заборы,\n\
                        Чтоб тунеядцем не прослыть!\n";

        assert_eq!(content.replace("\r\n", "\n"), expected.replace("\r\n", "\n"));
    }
}