//! screenshot.rs — Хэндлеры команд захвата скриншотов.
//!
//! Модуль предоставляет обработчики для снятия скриншотов и вставки их
//! в поле ввода AI через буфер обмена.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Регистрация команд скриншотов в реестре хэндлеров.
//! - Захват изображения, размещение в clipboard, вставка в окно AI.

mod test_screenshot_test;

use std::collections::HashMap;
use std::ffi::c_void;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use windows::Win32::Foundation::HWND;
use crate::{handler, library};
use crate::agent::request::session;
use crate::library::markdown_fence::{push_fenced_block, wrap_in_fence};
use crate::library::window;

/// Описание: Регистрирует обработчики команд скриншотов в карту хэндлеров.
///
/// # Параметры
/// - `handlers_map`: Карта, в которую добавляются хэндлеры.
///
/// # Побочные эффекты
/// - Модифицирует переданную карту.
pub fn handlers_map_init(handlers_map: &mut HashMap<&str, handler::HandlerFn>) {
    handlers_map.insert("get_monitor_layout", get_monitor_layout);
    handlers_map.insert("capture_window_by_title", capture_window_by_title);
    handlers_map.insert("capture_window_by_hwnd", capture_window_by_hwnd);
    handlers_map.insert("capture_region", capture_region);
    handlers_map.insert("capture_mouse_vicinity", capture_mouse_vicinity);
    handlers_map.insert("capture_monitor", capture_monitor);
    handlers_map.insert("capture_virtual_screen", capture_virtual_screen);
}   // handlers_map_init()

/// Описание: Возвращает геометрию мониторов (логическая нумерация -> физическая + размеры).
///
/// Команда не имеет параметров.
///
/// # Формат вывода
/// Возвращает Markdown-таблицу:
/// - количество мониторов (логических);
/// - строки вида: logical_index, physical_index, x, y, width, height.
///
/// Координаты (x, y) — в координатах виртуального рабочего стола (могут быть отрицательными).
///
/// # Ошибки
/// Возвращает `Err(String)`, если не удалось получить карту мониторов или их геометрию.
fn get_monitor_layout(params: &Option<Vec<String>>) -> Result<String, String> {

    // Параметров быть не должно.
    handler::check_param_count(params, 0)?;

    let count = library::screenshot::logical_monitors_count()
        .map_err(|e| format!("не удалось получить количество мониторов: {}", e))?;

    let map = library::screenshot::logical_to_physical_map()
        .map_err(|e| format!("не удалось получить карту logical->physical: {}", e))?;

    // Защита от рассинхрона (на случай изменения мониторов без перезапуска).
    if map.len() != count {
        return Err(format!(
            "рассинхрон карты мониторов: logical_monitors_count()={}, map.len()={}",
            count, map.len()
        ));
    }   // if

    let mut out = String::new();

    out.push_str("# Геометрия мониторов\n\n");
    out.push_str(&format!("Количество мониторов: **{}**\n\n", count));

    out.push_str("| logical | physical | x | y | width | height |\n");
    out.push_str("|---:|---:|---:|---:|---:|---:|\n");

    for logical_idx in 0..count {
        let physical_idx = map[logical_idx];

        let g = library::screenshot::get_monitor_geometry(logical_idx)
            .map_err(|e| format!("не удалось получить геометрию logical_index={}: {}", logical_idx, e))?;

        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            logical_idx, physical_idx, g.x, g.y, g.width, g.height
        ));
    }   // for

    Ok(out)
}   // get_monitors_geometry()

/// Описание: Снимает скриншот окна по подстроке заголовка (needle) и вставляет в поле ввода AI.
///
/// # Параметры
/// - `params`: `["<needle>"]` — подстрока заголовка окна (contains).
///
/// # Возвращаемое значение
/// Тип: String: Сообщение о выполнении + информация о курсоре.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное количество параметров (ожидается 1);
/// - окно не найдено/не удалось сфокусировать;
/// - не удалось сделать скриншот/поместить в clipboard;
/// - не удалось вставить изображение в окно AI.
///
/// # Побочные эффекты
/// - Перезаписывает системный буфер обмена.
/// - Фокусирует окно-цель (best effort) и окно AI.
/// - Генерирует Ctrl+V в окно AI.
fn capture_window_by_title(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Валидация параметров.
    handler::check_param_count(params, 1)?;
    let needle: String = handler::check_param_type(params, 0)?;

    // 2) Найти окно и сфокусировать его (используем win32tool).
    let (hwnd, win_title) = window::find_window_by_needle_and_focus(&needle)?;

    // 3) Захватить окно и положить изображение в clipboard.
    let cursor_info = library::screenshot::capture_window_to_clipboard(hwnd)?;

    // 4) Вставить картинку в чат AI (фокус + Ctrl+V).
    let ai_window_title = session::window_title()
        .map_err(|e| format!("не удалось получить window_title: {}", e))?;
    window::paste_clipboard_into_window_by_needle(ai_window_title)?;

    let out = format!(
        "Скриншот окна по needle='{}' отправлен.\nОкно='{}'.\n{}",
        needle, win_title, cursor_info.report()
    );
    Ok(wrap_in_fence(&out))
}   // capture_window_by_title()

/// Описание: Снимает скриншот окна по HWND и вставляет в поле ввода AI.
///
/// Перед захватом выполняется попытка перевести фокус на целевое окно.
///
/// # Параметры
/// - `params`: `["<hwnd>"]` — HWND в десятичном виде или hex с префиксом `0x`.
///   Примеры: `"12345678"`, `"0x0000000000123456"`.
///
/// # Возвращаемое значение
/// Тип: String: Markdown-блок с сообщением о выполнении и информацией о курсоре.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное количество параметров (ожидается 1);
/// - HWND не удалось распарсить;
/// - не удалось сфокусировать целевое окно;
/// - не удалось сделать скриншот или поместить его в clipboard;
/// - не удалось получить заголовок окна AI или выполнить вставку.
///
/// # Побочные эффекты
/// - Фокусирует целевое окно (best effort).
/// - Перезаписывает системный буфер обмена.
/// - Фокусирует окно AI.
/// - Генерирует события клавиатуры (Ctrl+V) в окне AI.
fn capture_window_by_hwnd(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Валидация параметров и парсинг HWND.
    handler::check_param_count(params, 1)?;
    let hwnd_str: String = handler::check_param_type(params, 0)?;
    let hwnd = window::parse_hwnd(&hwnd_str)?;

    // 2) Фокусировка окна-цели перед захватом.
    window::focus_window(hwnd)
        .map_err(|e| format!("не удалось сфокусировать окно HWND={}: {}", hwnd_str, e))?;

    // 3) Захватить окно и положить изображение в clipboard.
    let cursor_info = library::screenshot::capture_window_to_clipboard(hwnd)?;

    // 4) Вставить картинку в чат AI (фокус окна AI + Ctrl+V).
    let ai_window_title = session::window_title()
        .map_err(|e| format!("не удалось получить window_title: {}", e))?;
    window::paste_clipboard_into_window_by_needle(ai_window_title)?;

    // 5) Формирование отчета.
    let out = format!(
        "Скриншот окна по HWND={} отправлен.\n{}",
        hwnd_str, cursor_info.report()
    );

    Ok(wrap_in_fence(&out))
}   // capture_window_by_hwnd()

/// Описание: Снимает скриншот области виртуального рабочего стола и вставляет в поле ввода AI.
///
/// Координаты: в системе виртуального рабочего стола Windows.
/// (0,0) — левый верх primary монитора; x/y могут быть отрицательными.
///
/// # Параметры
/// - `params`: `["<x>", "<y>", "<width>", "<height>"]`
///   - `x`, `y`: i32
///   - `width`, `height`: u32
///
/// # Возвращаемое значение
/// Тип: String: Сообщение о выполнении + информация о курсоре.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное количество параметров (ожидается 4);
/// - параметры не приводятся к нужным типам;
/// - не удалось сделать скриншот/поместить в clipboard;
/// - не удалось вставить изображение в окно AI.
///
/// # Побочные эффекты
/// - Перезаписывает системный буфер обмена.
/// - Фокусирует окно AI.
/// - Генерирует Ctrl+V в окно AI.
fn capture_region(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Валидация параметров и парсинг координат.
    handler::check_param_count(params, 4)?;
    let x: i32 = handler::check_param_type(params, 0)?;
    let y: i32 = handler::check_param_type(params, 1)?;
    let width: u32 = handler::check_param_type(params, 2)?;
    let height: u32 = handler::check_param_type(params, 3)?;

    // 2) Захватить регион и положить в clipboard.
    let cursor_info = library::screenshot::capture_region_to_clipboard(x, y, width, height)?;

    // 3) Вставить картинку в чат AI.
    let ai_window_title = session::window_title()
        .map_err(|e| format!("не удалось получить window_title: {}", e))?;
    window::paste_clipboard_into_window_by_needle(ai_window_title)?;

    let out = format!("Скриншот области (x={}, y={}, {}x{}) отправлен.\n{}",
        x, y, width, height, cursor_info.report()
    );
    Ok(wrap_in_fence(&out))
}   // capture_region()

/// Описание: Снимает скриншот области вокруг курсора мыши и вставляет в поле ввода AI.
///
/// Курсор стараемся разместить в центре области (best effort, округление вниз).
///
/// # Параметры
/// - `params=[]` -> ширина/высота по умолчанию (100x70)
/// - `params=["<width>","<height>"]`
///
/// # Возвращаемое значение
/// Тип: String: Сообщение о выполнении + координаты области + cursor_info.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное число параметров (разрешено 0 или 2);
/// - width/height не парсятся или равны 0;
/// - не удалось получить позицию курсора;
/// - не удалось сделать скриншот/поместить в clipboard;
/// - не удалось вставить изображение в окно AI.
///
/// # Побочные эффекты
/// - Перезаписывает системный буфер обмена.
/// - Фокусирует окно AI.
/// - Генерирует Ctrl+V в окно AI.
fn capture_mouse_vicinity(params: &Option<Vec<String>>) -> Result<String, String> {

    let (width, height) = _parse_optional_width_height(params)?;

    // Текущая позиция курсора (виртуальный рабочий стол).
    let (cx, cy) = library::mouse::get_cursor_position()
        .map_err(|e| format!("не удалось получить позицию курсора: {}", e))?;

    // Центруем область по курсору.
    let x = cx - (width as i32) / 2;
    let y = cy - (height as i32) / 2;

    // 1) Захватить область и положить картинку в clipboard.
    let cursor_info = library::screenshot::capture_region_to_clipboard(x, y, width, height)?;

    // 2) Вставить картинку в чат AI.
    let ai_window_title = session::window_title()
        .map_err(|e| format!("не удалось получить window_title: {}", e))?;
    window::paste_clipboard_into_window_by_needle(ai_window_title)?;

    let out = format!(
        "Скриншот области вокруг курсора отправлен.\nОбласть: x={}, y={}, {}x{}\n{}",
        x, y, width, height, cursor_info.report()
    );

    Ok(wrap_in_fence(&out))
}   // capture_mouse_vicinity()

/// Описание: Снимает скриншот монитора по логическому индексу и вставляет в поле ввода AI.
///
/// Логический индекс определяется позицией монитора на виртуальном рабочем столе:
/// мониторы упорядочиваются слева направо, затем сверху вниз.
///
/// # Параметры
/// - `params`: `["<logical_index>"]` — строковое представление индекса монитора (начиная с 0).
///
/// # Возвращаемое значение
/// Тип: String: Сообщение "скриншот монитора N вставлен".
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное количество параметров (ожидается 1);
/// - параметр не является числом;
/// - логический индекс выходит за пределы количества мониторов;
/// - не удалось захватить скриншот или поместить в clipboard;
/// - не удалось получить заголовок окна AI;
/// - не удалось сфокусировать окно или отправить Ctrl+V.
///
/// # Побочные эффекты
/// - Перезаписывает системный буфер обмена.
/// - Переводит фокус на окно AI.
/// - Генерирует события клавиатуры (Ctrl+V).
fn capture_monitor(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1. Проверяем количество параметров.
    handler::check_param_count(params, 1)?;

    // 2. Парсим логический индекс монитора.
    let monitor_index: usize = handler::check_param_type(params, 0)?;

    // 3. Захватываем скриншот монитора и помещаем в clipboard.
    let cursor_info = library::screenshot::capture_monitor_to_clipboard(monitor_index)?;

    // 4. Получаем заголовок окна AI из контекста сессии.
    let window_title = session::window_title()
        .map_err(|e| format!("не удалось получить window_title: {}", e))?;

    // 5. Вставляем изображение в окно.
    window::paste_clipboard_into_window_by_needle(window_title)?;

    let out = format!("Скриншот монитора {} отправлен.\n{}", monitor_index, cursor_info.report());
    Ok(wrap_in_fence(&out))
}   // screenshot_monitor()

/// Описание: Снимает скриншот всех мониторов и вставляет в поле ввода AI.
///
/// # Алгоритм работы
/// - Захватывает RGBA-изображение всех мониторов и помещает в clipboard.
/// - Фокусирует окно AI по заголовку из контекста сессии.
/// - Отправляет Ctrl+V для вставки.
///
/// # Параметры
/// - `_params`: Не используются (команда без параметров).
///
/// # Возвращаемое значение
/// Тип: String: Сообщение "скриншот вставлен".
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - не удалось захватить скриншот или поместить в clipboard;
/// - не удалось получить заголовок окна AI (сессия не инициализирована);
/// - не удалось сфокусировать окно AI;
/// - не удалось отправить Ctrl+V.
///
/// # Побочные эффекты
/// - Перезаписывает системный буфер обмена.
/// - Переводит фокус на окно AI.
/// - Генерирует события клавиатуры (Ctrl+V).
fn capture_virtual_screen(_params: &Option<Vec<String>>) -> Result<String, String> {

    // 1. Захватываем скриншот всех мониторов и помещаем в clipboard.
    let cursor_info = library::screenshot::capture_all_monitors_to_clipboard()?;

    // 2. Получаем заголовок окна AI из контекста сессии.
    let window_title = session::window_title()
        .map_err(|e| format!("не удалось получить window_title: {}", e))?;

    // 3. Вставляем образ в окно.
    window::paste_clipboard_into_window_by_needle(window_title)?;

    let out = format!("Скриншот всех мониторов отправлен.\n{}", cursor_info.report());
    Ok(wrap_in_fence(&out))
}   // screenshot_all()

//--------------------------------------------------------------------------------------------------
//                  Внутренний интерфейс
//--------------------------------------------------------------------------------------------------

/// Парсит опциональные width/height для capture_mouse_vicinity.
///
/// # Формат
/// - `None` / `[]` -> дефолт 100x70
/// - `[width, height]` -> указанное значение
fn _parse_optional_width_height(params: &Option<Vec<String>>) -> Result<(u32, u32), String> {
    const DEFAULT_WIDTH: u32 = 150;
    const DEFAULT_HEIGHT: u32 = 70;

    let Some(v) = params.as_ref() else {
        return Ok((DEFAULT_WIDTH, DEFAULT_HEIGHT));
    };

    if v.is_empty() {
        return Ok((DEFAULT_WIDTH, DEFAULT_HEIGHT));
    }   // if

    if v.len() != 2 {
        return Err(format!(
            "Неверное число параметров: ожидалось 0 или 2, получено {}",
            v.len()
        ));
    }   // if

    let width: u32 = handler::check_param_type(params, 0)?;
    let height: u32 = handler::check_param_type(params, 1)?;

    if width == 0 || height == 0 {
        return Err("width и height должны быть > 0".to_string());
    }   // if

    Ok((width, height))
}   // _parse_optional_width_height()
