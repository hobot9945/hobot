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
    handlers_map.insert("read_file_by_template", read_file_by_template);
}   // handlers_map_init()

/// Описание: Читает файл целиком или заданный диапазон строк.
///
/// # Поддерживаемые форматы параметров:
/// 1. `["<path>"]` — Весь файл, UTF-8, без номеров.
/// 2. `["<path>", "<encoding>"]` — Весь файл, кодировка, без номеров.
/// 3. `["<path>", "<encoding>", "enumerate"]` — Весь файл, кодировка, с номерами.
/// 4. `["<path>", "<encoding>", "<from>"]` — От строки <from> до конца, с номерами.
/// 5. `["<path>", "<encoding>", "<from>", "<to>"]` — Диапазон [от, до], с номерами.
///
/// # Особенности:
/// - Если задан диапазон (даже если только начало) или флаг "enumerate",
///   текст выводится с нумерацией строк: ` 42: Текст`.
/// - Параметры <from> и <to> — номера строк начиная с 1.
/// - Перед блоком кода всегда выводится заголовок: `## Выведено <n> строк из <m>`.
fn read_file(params: &Option<Vec<String>>) -> Result<String, String> {
    // --- 1. Валидация количества параметров (1-4) ---
    let count = params.as_ref().map_or(0, |v| v.len());
    if count < 1 || count > 4 {
        return Err(format!("Неверное число параметров: ожидалось от 1 до 4, получено {}", count));
    }

    // Путь к файлу — всегда первый параметр
    let path: String = check_param_type(params, 0)?;

    // Кодировка — второй параметр (по умолчанию utf-8)
    let encoding_label = if count >= 2 {
        Some(check_param_type::<String>(params, 1)?)
    } else {
        None
    };

    // --- 2. Чтение и декодирование ---
    let bytes = fs::read(&path).map_err(|e| format!("Ошибка чтения файла '{}': {}", path, e))?;

    let encoding = if let Some(label) = encoding_label {
        Encoding::for_label(label.as_bytes()).ok_or_else(|| format!("Неизвестная кодировка: '{}'", label))?
    } else {
        encoding_rs::UTF_8
    };

    let (decoded, _, _) = encoding.decode(&bytes);
    let content = decoded.into_owned();

    // Разбиваем текст на строки для работы с диапазонами
    let all_lines: Vec<&str> = content.lines().collect();
    let m = all_lines.len(); // Общее количество строк в файле

    // --- 3. Определение диапазона и режима нумерации ---
    let mut start_line = 1;
    let mut end_line = m;
    let mut is_numbered = false;

    if count == 3 {
        let p2 = &params.as_ref().unwrap()[2];
        if p2 == "enumerate" {
            // Режим: весь файл с нумерацией строк
            is_numbered = true;
        } else if let Ok(val) = p2.parse::<usize>() {
            // Режим: от строки N до конца файла с нумерацией
            start_line = val;
            is_numbered = true;
        } else {
            return Err(format!("Неверный третий параметр: '{}'. Ожидалось 'enumerate' или номер строки.", p2));
        }
    } else if count == 4 {
        // Режим: строгий диапазон [от, до] с нумерацией
        start_line = check_param_type(params, 2)?;
        end_line = check_param_type(params, 3)?;
        is_numbered = true;
    }

    // Валидация границ диапазона (1-based)
    if start_line == 0 {
        return Err("Ошибка: номер начальной строки должен быть >= 1".to_string());
    }

    // Если файл не пуст и старт за его пределами — это ошибка
    if start_line > m && m > 0 {
        return Err(format!("Ошибка: начальная строка ({}) за пределами файла (всего {} строк).", start_line, m));
    }

    // Если конечная точка за пределами файла — ограничиваем её концом файла (насыщение)
    if end_line > m {
        end_line = m;
    }

    // Проверка логичности диапазона
    if end_line < start_line && m > 0 {
        return Err(format!("Ошибка: конечная строка ({}) меньше начальной ({}).", end_line, start_line));
    }

    // --- 4. Сборка результата ---
    let mut out = String::new();

    // Вычисляем 0-based индексы для слайсинга вектора строк
    let start_idx = start_line.saturating_sub(1);
    let end_idx = end_line; // Верхняя граница в слайсах не включается

    // Получаем нужный срез строк. Если файл пуст, срез будет пустым.
    let displayed_lines = if m == 0 { &[] } else { &all_lines[start_idx..end_idx] };
    let n = displayed_lines.len(); // Количество фактически выводимых строк

    // Формируем заголовок отчета: ## Выведено 50 строк из 200
    out.push_str(&format!("## Выведено {} строк из {}\n\n", n, m));

    let mut fragment = String::new();
    // Определяем ширину колонки номера для красивого выравнивания (по максимальному номеру в блоке)
    let width = end_line.to_string().len();

    for (i, line) in displayed_lines.iter().enumerate() {
        if is_numbered {
            let current_no = start_line + i;
            // Формат нумерации: "  42: Текст строки"
            fragment.push_str(&format!("{:>width$}: {}\n", current_no, line, width = width));
        } else {
            // Режим простого вывода (без номеров строк)
            fragment.push_str(line);
            fragment.push_str("\n");
        }
    }

    // Оборачиваем результат в стандартный Markdown-забор
    out.push_str(&wrap_in_fence(&fragment));

    Ok(out)
}

/// Описание: Ищет совпадения по шаблону в файле и выводит блоки текста вокруг них.
/// Близко расположенные совпадения объединяются в единые блоки для экономии места.
///
/// # Алгоритм работы:
/// 1. Парсинг параметров: путь, шаблон, кодировка (default: utf-8), отступы (default: 10).
/// 2. Чтение файла в байты и декодирование согласно указанной кодировке.
/// 3. Поиск всех строк, содержащих точное вхождение шаблона (регистрозависимо).
/// 4. Отбор первых 5 находок и их группировка: если контексты соседних находок
///    пересекаются или касаются, они сливаются в один блок вывода.
/// 5. Формирование Markdown-отчета с заголовками и нумерованными строками.
///
/// # Параметры
/// - `params`: `["<path>", "<template>", "<encoding>", "<before>", "<after>"]`
fn read_file_by_template(params: &Option<Vec<String>>) -> Result<String, String> {
    // --- 1. Валидация и разбор параметров ---
    let count = params.as_ref().map_or(0, |v| v.len());
    if count < 2 || count > 5 {
        return Err(format!("Неверное число параметров: ожидалось от 2 до 5, получено {}", count));
    }

    let path: String = check_param_type(params, 0)?;
    let template: String = check_param_type(params, 1)?;
    if template.is_empty() {
        return Err("Ошибка: поисковый шаблон не может быть пустым.".to_string());
    }

    // Кодировка опциональна, но обязательна при наличии параметров отступов
    let encoding_label = if count >= 3 { Some(check_param_type::<String>(params, 2)?) } else { None };
    // Число строк контекста до и после совпадения
    let margin_before = if count >= 4 { check_param_type::<usize>(params, 3)? } else { 10 };
    let margin_after = if count >= 5 { check_param_type::<usize>(params, 4)? } else { 10 };

    // --- 2. Чтение и декодирование файла ---
    let bytes = fs::read(&path).map_err(|e| format!("Ошибка чтения файла '{}': {}", path, e))?;

    let encoding = if let Some(label) = encoding_label {
        Encoding::for_label(label.as_bytes()).ok_or_else(|| format!("Неизвестная кодировка: '{}'", label))?
    } else {
        encoding_rs::UTF_8
    };

    let (decoded, _, _) = encoding.decode(&bytes);
    let content = decoded.into_owned();

    // --- 3. Поиск всех совпадений ---
    let lines: Vec<&str> = content.lines().collect();
    let mut all_match_indices = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        if line.contains(&template) {
            all_match_indices.push(idx);
        }
    }

    let k = all_match_indices.len();
    let mut out = String::new();

    // Формирование общего заголовка по количеству находок
    if k == 0 {
        out.push_str("## Не найдено совпадений.\n");
        return Ok(out);
    }

    if k >= 5 {
        out.push_str(&format!("## Найдено {} совпадений. Выводятся первые 5.\n", k));
    } else {
        out.push_str(&format!("## Найдено {} совпадений.\n", k));
    }

    // --- 4. Группировка первых 5 совпадений ---
    // Ограничиваемся первыми пятью вхождениями согласно ТЗ
    let first_matches: Vec<(usize, usize)> = all_match_indices.iter().take(5)
        .enumerate()
        .map(|(i, &idx)| (i + 1, idx)) // (порядковый_номер, индекс_строки)
        .collect();

    // Список групп для вывода. Группа содержит список порядковых номеров и индексов строк.
    let mut groups: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();

    if !first_matches.is_empty() {
        let mut current_group_ord = vec![first_matches[0].0];
        let mut current_group_idx = vec![first_matches[0].1];

        for &(ord, idx) in first_matches.iter().skip(1) {
            let last_idx = *current_group_idx.last().unwrap();

            // Проверяем расстояние между текущей и предыдущей находкой.
            // Если контексты "до" и "после" пересекаются или соприкасаются, объединяем в одну группу.
            if idx - last_idx <= margin_before + margin_after {
                current_group_ord.push(ord);
                current_group_idx.push(idx);
            } else {
                // Завершаем текущую группу и начинаем новую
                groups.push((current_group_ord, current_group_idx));
                current_group_ord = vec![ord];
                current_group_idx = vec![idx];
            }
        }
        groups.push((current_group_ord, current_group_idx));
    }

    // --- 5. Формирование блоков вывода по группам ---
    for (ords, idxs) in groups {
        let first_match_idx = idxs[0];
        let last_match_idx = *idxs.last().unwrap();

        // Заголовок конкретного блока (может содержать несколько совпадений)
        if ords.len() == 1 {
            out.push_str(&format!("\n## Совпадение {}: строка {}\n", ords[0], first_match_idx + 1));
        } else {
            let ords_str: Vec<String> = ords.iter().map(|n| n.to_string()).collect();
            let idxs_str: Vec<String> = idxs.iter().map(|n| (n + 1).to_string()).collect();
            out.push_str(&format!("\n## Совпадения {}: строки {}\n", ords_str.join(", "), idxs_str.join(", ")));
        }

        // Вычисляем границы вывода с учетом отступов, не выходя за пределы файла
        let start = first_match_idx.saturating_sub(margin_before);
        let end = std::cmp::min(lines.len(), last_match_idx + margin_after + 1);

        let mut fragment = String::new();
        // Динамически определяем ширину колонки номера строки для выравнивания
        let width = end.to_string().len();

        for current_idx in start..end {
            let is_match = idxs.contains(&current_idx);
            // Используем '>' для совпадений и ':' для контекста
            let sep = if is_match { "> " } else { ": " };
            let line_no = current_idx + 1;

            // Формат строки: "   42: Текст" или "   43> Находка"
            fragment.push_str(&format!("{:>width$}{}{}\n", line_no, sep, lines[current_idx], width = width));
        }

        // Оборачиваем каждый блок в Markdown fenced block
        out.push_str(&wrap_in_fence(&fragment));
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_PATH: &str = "c:\\tmp\\read_by_template_test.txt";
    const OUT_PATH: &str = "c:\\tmp\\read_by_template_test.md";

    fn cleanup() {
        let _ = fs::remove_file(TEST_PATH);
        let _ = fs::remove_file(OUT_PATH);
    }
    #[test]
    fn test_read_file_by_template_simple() {
        cleanup();

        // 1. Подготавливаем файл
        let initial_text = "Русская народная\n\n\
                            Имею я златые горы\n\
                            И есть что жрать и есть что пить\n\
                            Но, крашу, крашу я заборы,\n\
                            Чтоб тунеядцем не прослыть!\n\n\
                            Народное творчество.\n";
        fs::write(TEST_PATH, initial_text).expect("Не удалось подготовить файл");

        // 2. Параметры: ищем "крашу", кодировка utf-8, 2 строки до, 2 строки после
        let params = Some(vec![
            TEST_PATH.to_string(),
            "крашу".to_string(),
            "utf-8".to_string(),
            "2".to_string(),
            "2".to_string(),
        ]);

        let result = read_file_by_template(&params);
        assert!(result.is_ok(), "Хандлер вернул ошибку: {:?}", result);

        let out = result.unwrap();

        fs::write(OUT_PATH, &out).expect("Не удалось подготовить файл");

        // Проверка заголовков
        assert!(out.contains("## Найдено 1 совпадений."));
        assert!(out.contains("## Совпадение 1: строка 5"));

        // Проверка формата строк (номер, разделитель, текст)
        // Строка 5 должна быть с '>'
        assert!(out.contains("5> Но, крашу, крашу я заборы,"));
        // Строка 3 (контекст до) должна быть с ':'
        assert!(out.contains("3: Имею я златые горы"));
        // Строка 7 (контекст после) должна быть с ':'
        assert!(out.contains("7: "));
    }
}