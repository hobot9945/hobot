//! win_control.rs — управление окнами.
//!
//! Модуль предоставляет обработчики для поиска окон и т.д.

mod test_win_control_test;

use std::collections::HashMap;
use windows::Win32::UI::WindowsAndMessaging::IsWindowVisible;
use crate::handler;
use crate::library::markdown_fence::push_fenced_block;
use crate::library::window;

/// Описание: Регистрирует обработчики команд скриншотов в карту хэндлеров.
///
/// # Параметры
/// - `handlers_map`: Карта, в которую добавляются хэндлеры.
///
/// # Побочные эффекты
/// - Модифицирует переданную карту.
pub fn handlers_map_init(handlers_map: &mut HashMap<&str, handler::HandlerFn>) {
    handlers_map.insert("get_window_list", get_window_list);
    handlers_map.insert("get_foreground_window_info", get_foreground_window_info);
    handlers_map.insert("find_window_info", find_window_info);
    handlers_map.insert("focus_window_by_hwnd", focus_window_by_hwnd);
    handlers_map.insert("focus_window_by_title", focus_window_by_title);
}   // handlers_map_init()

/// Описание: Возвращает список top-level окон с опциональной настройкой фильтров.
///
/// # Параметры
/// - `params=[]`
///   Эквивалентно: include_invisible=false, include_empty_title=false
/// - `params` может содержать флаги (порядок не важен):
///   - `"include_invisible"`
///   - `"include_empty_title"`
///
/// # Формат вывода
/// Markdown-таблица (по образу `get_monitor_layout`):
/// - индекс
/// - hwnd
/// - x, y, width, height
/// - title (в конце строки признаки в скобках: `foreground`, `minimized`, `inv`)
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - передан неизвестный флаг,
/// - `window::get_window_list()` вернул ошибку.
///
/// # Побочные эффекты
/// - Нет.
fn get_window_list(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Разбор параметров (порядок не важен).
    let (include_invisible, include_empty_title) = _parse_window_list_flags(params)?;

    // 2) Получить список окон из library.
    // needle=None: без фильтра по подстроке заголовка.
    let list = window::get_window_list(None, include_invisible, include_empty_title)?;

    // 3) Сформировать Markdown-вывод (таблица).
    let mut out = String::new();

    out.push_str("# Список окон\n\n");
    out.push_str(&format!("Количество окон: **{}**\n\n", list.len()));

    out.push_str("| # | hwnd | x | y | width | height | title |\n");
    out.push_str("|---:|---:|---:|---:|---:|---:|---|\n");

    for (idx, wi) in list.iter().enumerate() {

        // HWND выводим в hex для удобства.
        let hwnd_hex = format!("0x{:X}", wi.hwnd.0 as usize);

        // title может содержать символы, ломающие таблицу.
        let mut title = if wi.title.is_empty() {
            "(empty)".to_string()
        } else {
            _sanitize_md_table_cell(&wi.title)
        };   // if

        // Признаки выводим компактно в конце строки (в скобках).
        let mut flags: Vec<&str> = Vec::new();

        if wi.is_foreground {
            flags.push("foreground");
        }   // if

        if wi.is_minimized {
            flags.push("minimized");
        }   // if

        // Признак “inv” вычисляем только если этот режим вообще включён,
        // иначе inv по определению не должно быть в выборке.
        if include_invisible {
            let is_visible = unsafe { IsWindowVisible(wi.hwnd).as_bool() };
            if !is_visible {
                flags.push("inv");
            }   // if
        }   // if

        if !flags.is_empty() {
            title.push_str(&format!(" ({})", flags.join(", ")));
        }   // if

        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            idx,
            hwnd_hex,
            wi.x,
            wi.y,
            wi.width,
            wi.height,
            title
        ));
    }   // for

    Ok(out)
}   // get_window_list()

/// Описание: Возвращает HWND и полный заголовок окна, которое сейчас в foreground.
///
/// Команда не имеет параметров.
///
/// # Возвращаемое значение
/// Тип: String: Markdown-отчет с HWND и title.
///
/// # Ошибки
/// Возвращает `Err(String)`, если `GetForegroundWindow()` вернул NULL.
///
/// # Побочные эффекты
/// - Нет.
fn get_foreground_window_info(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Параметров быть не должно.
    handler::check_param_count(params, 0)?;

    // 2) Получить HWND и title активного окна.
    let win_info = window::get_foreground_window_info()?;

    // 3) Сформировать читаемый вывод.
    let hwnd_hex = format!("0x{:X}", win_info.hwnd.0 as usize);

    let mut out = String::new();
    out.push_str("# Foreground окно\n\n");
    let code_block = format!("- hwnd: `{}`\n- title: `{}`\n", hwnd_hex, win_info.title);
    push_fenced_block(&mut out,&code_block);

    Ok(out)
}   // get_foreground_window_info_handler()

/// Описание: Ищет окно по подстроке заголовка (needle) и возвращает полную информацию о нём.
///
/// Важно: хэндлер НЕ фокусирует окно и НЕ делает вставку в AI. Он только ищет и возвращает информацию.
///
/// # Параметры
/// - `params`: `["<needle>"]` — подстрока заголовка окна.
///
/// # Возвращаемое значение
/// Тип: String: Текст с найденным HWND, полным title, позицией и размером.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное количество параметров (ожидается 1);
/// - окно не найдено (или не найдено за период ретраев внутри win32tool).
///
/// # Побочные эффекты
/// - Нет.
fn find_window_info(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Валидация параметров.
    handler::check_param_count(params, 1)?;
    let needle: String = handler::check_param_type(params, 0)?;

    // 2) Ищем окно (с ретраями внутри library/window).
    let wi = window::find_window_by_needle(&needle)?;

    // 3) Формируем человекочитаемый вывод для отчёта.
    let hwnd_hex = format!("0x{:X}", wi.hwnd.0 as usize);

    let mut out = String::new();
    out.push_str("# Найдено окно\n\n");
    let code_block = format!(
        "- needle: `{}`\n- hwnd: `{}`\n- title: `{}`\n- pos: [{}, {}]\n- size: {}x{}\n",
        needle, hwnd_hex, wi.title, wi.x, wi.y, wi.width, wi.height
    );
    push_fenced_block(&mut out, &code_block);

    Ok(out)
}   // find_window_info()

/// Описание: Сфокусировать окно по HWND.
///
/// # Параметры
/// - `params`: `["<hwnd>"]`, где `<hwnd>` может быть:
///   - `"123456"` (decimal)
///   - `"0x1A2B3C"` (hex)
///
/// # Возвращаемое значение
/// Markdown-отчёт: hwnd, title, позиция и размер.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное число параметров;
/// - hwnd не парсится;
/// - `window::focus_window()` не смог сфокусировать окно (с ретраями внутри library/window).
fn focus_window_by_hwnd(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Валидация количества параметров: нужен ровно один.
    handler::check_param_count(params, 1)?;

    // 2) Достаем hwnd как строку.
    let hwnd_str: String = handler::check_param_type(params, 0)?;

    // 3) Парсим hwnd.
    let hwnd = window::parse_hwnd(&hwnd_str)?;

    // 4) Пытаемся сфокусировать окно.
    let wi = window::focus_window(hwnd)?;

    // 5) Формируем отчет.
    let hwnd_hex = format!("0x{:X}", wi.hwnd.0 as usize);

    let mut out = String::new();
    out.push_str("# Окно сфокусировано (by_hwnd)\n\n");

    let code_block = format!(
        "- hwnd_in: `{}`\n- hwnd: `{}`\n- title: `{}`\n- pos: [{}, {}]\n- size: {}x{}\n",
        hwnd_str, hwnd_hex, wi.title, wi.x, wi.y, wi.width, wi.height
    );
    push_fenced_block(&mut out, &code_block);

    Ok(out)
}   // focus_window_by_hwnd()

/// Описание: Найти окно по подстроке заголовка и сфокусировать его.
///
/// # Параметры
/// - `params`: `["<needle>"]` — подстрока заголовка окна.
///
/// # Возвращаемое значение
/// Markdown-отчёт: needle, hwnd, title, позиция и размер.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное число параметров;
/// - окно не найдено;
/// - окно не удалось сфокусировать (с ретраями внутри library/window).
fn focus_window_by_title(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Валидация: нужен ровно один параметр.
    handler::check_param_count(params, 1)?;

    // 2) Достаем needle.
    let needle: String = handler::check_param_type(params, 0)?;

    // 3) Поиск + фокус готовой функцией.
    let wi = window::find_window_by_needle_and_focus(&needle)?;

    // 4) Формируем отчет.
    let hwnd_hex = format!("0x{:X}", wi.hwnd.0 as usize);

    let mut out = String::new();
    out.push_str("# Окно сфокусировано (by_title)\n\n");

    let code_block = format!(
        "- needle: `{}`\n- hwnd: `{}`\n- title: `{}`\n- pos: [{}, {}]\n- size: {}x{}\n",
        needle, hwnd_hex, wi.title, wi.x, wi.y, wi.width, wi.height
    );
    push_fenced_block(&mut out, &code_block);

    Ok(out)
}   // focus_window_by_title()

//--------------------------------------------------------------------------------------------------
//                  Внутренние утилиты
//--------------------------------------------------------------------------------------------------

/// Разбирает список флагов хэндлера `get_window_list`.
///
/// # Правила
/// - Порядок параметров не важен.
/// - Флаг включается по наличию строки:
///   - `"include_invisible"`
///   - `"include_empty_title"`
/// - Неизвестные параметры считаются ошибкой.
///
/// # Возвращаемое значение
/// `(include_invisible, include_empty_title)`
fn _parse_window_list_flags(params: &Option<Vec<String>>) -> Result<(bool, bool), String> {

    let mut include_invisible: bool = false;
    let mut include_empty_title: bool = false;

    let Some(v) = params.as_ref() else {
        return Ok((include_invisible, include_empty_title));
    };

    for raw in v {

        let token = raw.trim().to_ascii_lowercase();

        // Пустые строки игнорируем (на случай мусорных пробелов).
        if token.is_empty() {
            continue;
        }   // if

        match token.as_str() {
            "include_invisible" => {
                include_invisible = true;
            },

            "include_empty_title" => {
                include_empty_title = true;
            },

            _ => {
                return Err(format!(
                    "Неизвестный параметр '{}'. Разрешены: include_invisible, include_empty_title",
                    raw
                ));
            }
        }   // match
    }   // for

    Ok((include_invisible, include_empty_title))
}   // _parse_window_list_flags()

/// Подготовка строки для Markdown-таблицы.
///
/// # Что делаем
/// - `|` экранируем как `\|` (иначе ломает таблицу).
/// - `\r` удаляем.
/// - `\n` заменяем на пробел (в таблице переносы строк не нужны).
fn _sanitize_md_table_cell(s: &str) -> String {
    s.replace('\r', "")
        .replace('\n', " ")
        .replace('|', "\\|")
}   // _sanitize_md_table_cell()