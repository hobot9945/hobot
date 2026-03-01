//! screenshot.rs — Хэндлеры команд захвата скриншотов.
//!
//! Модуль предоставляет обработчики для снятия скриншотов и добавления их
//! в глобальный `Report`.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Регистрация команд скриншотов в реестре хэндлеров.
//! - Захват изображения (RGBA) через `library::screenshot`.
//! - Добавление захваченных изображений в `report`.
//!
//! # ВАЖНО
//! Вставка изображений в окно AI НЕ выполняется в хэндлерах.
//! Отправка/вставка откладывается до общего шага в `agent.rs`, когда окно AI
//! гарантированно подготовлено (фокус, поле ввода и т.п.).

mod test_screenshot_test;

use std::collections::HashMap;
use crate::{handler, library};
use crate::agent::request::report;
use crate::library::markdown_fence::wrap_in_fence;
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

/// Описание: Снимает скриншот окна по подстроке заголовка (needle) и добавляет в `Report`.
///
/// # Параметры
/// - `params`: `["<needle>"]` — подстрока заголовка окна (contains).
///
/// # Возвращаемое значение
/// Тип: String: Сообщение о выполнении + геометрия окна + информация о курсоре.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное количество параметров (ожидается 1);
/// - окно не найдено/не удалось сфокусировать (best effort);
/// - не удалось сделать скриншот;
/// - не удалось добавить изображение в `Report`.
///
/// # Побочные эффекты
/// - Фокусирует окно-цель (best effort), чтобы скриншот соответствовал ожидаемому состоянию.
/// - Добавляет изображение в `Report.image_list`.
fn capture_window_by_title(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Валидация параметров.
    handler::check_param_count(params, 1)?;
    let needle: String = handler::check_param_type(params, 0)?;

    // 2) Найти окно и сфокусировать его (best effort).
    let wi = window::find_window_by_needle_and_focus(&needle)?;

    // 3) Захватить окно (RGBA).
    let (image, cursor_info) = library::screenshot::capture_window_rgba(wi.hwnd)?;

    // 4) Добавить изображение в REPORT.
    report::add_image(image).map_err(|e| e.to_string())?;

    let out = format!(
        "Скриншот окна по needle='{}' добавлен в отчёт.\nОкно='{}' ({}x{} @ [{},{}])\n{}",
        needle, wi.title, wi.width, wi.height, wi.x, wi.y, cursor_info.report()
    );

    Ok(wrap_in_fence(&out))
}   // capture_window_by_title()

/// Описание: Снимает скриншот окна по HWND и добавляет в `Report`.
///
/// Перед захватом выполняется попытка перевести фокус на целевое окно.
///
/// # Параметры
/// - `params`: `["<hwnd>"]` — HWND в десятичном виде или hex с префиксом `0x`.
///   Примеры: `"12345678"`, `"0x0000000000123456"`.
///
/// # Возвращаемое значение
/// Тип: String: Markdown-блок с сообщением о выполнении, геометрией и информацией о курсоре.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное количество параметров (ожидается 1);
/// - HWND не удалось распарсить;
/// - не удалось сфокусировать целевое окно;
/// - не удалось сделать скриншот;
/// - не удалось добавить изображение в `Report`.
///
/// # Побочные эффекты
/// - Фокусирует целевое окно (best effort).
/// - Добавляет изображение в `Report.image_list`.
fn capture_window_by_hwnd(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Валидация параметров и парсинг HWND.
    handler::check_param_count(params, 1)?;
    let hwnd_str: String = handler::check_param_type(params, 0)?;
    let hwnd = window::parse_hwnd(&hwnd_str)?;

    // 2) Фокусировка окна-цели перед захватом. Возвращает актуальную информацию об окне.
    let wi = window::focus_window_with_retries(hwnd)
        .map_err(|e| format!("не удалось сфокусировать окно HWND={}: {}", hwnd_str, e))?;

    // 3) Захватить окно (RGBA).
    let (image, cursor_info) = library::screenshot::capture_window_rgba(wi.hwnd)?;

    // 4) Добавить изображение в REPORT.
    report::add_image(image).map_err(|e| e.to_string())?;

    let out = format!(
        "Скриншот окна по HWND={} добавлен в отчёт.\nОкно='{}' ({}x{} @ [{},{}])\n{}",
        hwnd_str, wi.title, wi.width, wi.height, wi.x, wi.y, cursor_info.report()
    );

    Ok(wrap_in_fence(&out))
}   // capture_window_by_hwnd()

/// Описание: Снимает скриншот области виртуального рабочего стола и добавляет в `Report`.
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
/// - не удалось сделать скриншот;
/// - не удалось добавить изображение в `Report`.
///
/// # Побочные эффекты
/// - Добавляет изображение в `Report.image_list`.
fn capture_region(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Валидация параметров и парсинг координат.
    handler::check_param_count(params, 4)?;
    let x: i32 = handler::check_param_type(params, 0)?;
    let y: i32 = handler::check_param_type(params, 1)?;
    let width: u32 = handler::check_param_type(params, 2)?;
    let height: u32 = handler::check_param_type(params, 3)?;

    // 2) Захватить регион (RGBA).
    let (image, cursor_info) = library::screenshot::capture_region_rgba(x, y, width, height)?;

    // 3) Добавить изображение в REPORT.
    report::add_image(image).map_err(|e| e.to_string())?;

    let out = format!(
        "Скриншот области (x={}, y={}, {}x{}) добавлен в отчёт.\n{}",
        x, y, width, height, cursor_info.report()
    );

    Ok(wrap_in_fence(&out))
}   // capture_region()

/// Описание: Снимает скриншот области вокруг курсора мыши и добавляет в `Report`.
///
/// Курсор стараемся разместить в центре области (best effort, округление вниз).
///
/// # Параметры
/// - `params=[]` -> ширина/высота по умолчанию (150x70)
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
/// - не удалось сделать скриншот;
/// - не удалось добавить изображение в `Report`.
///
/// # Побочные эффекты
/// - Добавляет изображение в `Report.image_list`.
fn capture_mouse_vicinity(params: &Option<Vec<String>>) -> Result<String, String> {

    let (width, height) = _parse_optional_width_height(params)?;

    // Текущая позиция курсора (виртуальный рабочий стол).
    let (cx, cy) = library::mouse::get_cursor_position()
        .map_err(|e| format!("не удалось получить позицию курсора: {}", e))?;

    // Центруем область по курсору.
    let x = cx - (width as i32) / 2;
    let y = cy - (height as i32) / 2;

    // 1) Захватить область (RGBA).
    let (image, cursor_info) = library::screenshot::capture_region_rgba(x, y, width, height)?;

    // 2) Добавить изображение в REPORT.
    report::add_image(image).map_err(|e| e.to_string())?;

    let out = format!(
        "Скриншот области вокруг курсора добавлен в отчёт.\nОбласть: x={}, y={}, {}x{}\n{}",
        x, y, width, height, cursor_info.report()
    );

    Ok(wrap_in_fence(&out))
}   // capture_mouse_vicinity()

/// Описание: Снимает скриншот монитора по логическому индексу и добавляет в `Report`.
///
/// Логический индекс определяется позицией монитора на виртуальном рабочем столе:
/// мониторы упорядочиваются слева направо, затем сверху вниз.
///
/// # Параметры
/// - `params`: `["<logical_index>"]` — строковое представление индекса монитора (начиная с 0).
///
/// # Возвращаемое значение
/// Тип: String: Сообщение о выполнении + информация о курсоре.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное количество параметров (ожидается 1);
/// - параметр не является числом;
/// - логический индекс выходит за пределы количества мониторов;
/// - не удалось сделать скриншот;
/// - не удалось добавить изображение в `Report`.
///
/// # Побочные эффекты
/// - Добавляет изображение в `Report.image_list`.
fn capture_monitor(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1. Проверяем количество параметров.
    handler::check_param_count(params, 1)?;

    // 2. Парсим логический индекс монитора.
    let monitor_index: usize = handler::check_param_type(params, 0)?;

    // 3. Захватываем скриншот монитора (RGBA).
    let (image, cursor_info) = library::screenshot::capture_monitor_rgba(monitor_index)?;

    // 4. Добавляем изображение в REPORT.
    report::add_image(image).map_err(|e| e.to_string())?;

    let out = format!(
        "Скриншот монитора {} добавлен в отчёт.\n{}",
        monitor_index, cursor_info.report()
    );

    Ok(wrap_in_fence(&out))
}   // capture_monitor()

/// Описание: Снимает скриншот всех мониторов (виртуальный экран) и добавляет в `Report`.
///
/// # Параметры
/// - `_params`: Не используются (команда без параметров).
///
/// # Возвращаемое значение
/// Тип: String: Сообщение о выполнении + информация о курсоре.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - не удалось сделать скриншот;
/// - не удалось добавить изображение в `Report`.
///
/// # Побочные эффекты
/// - Добавляет изображение в `Report.image_list`.
fn capture_virtual_screen(_params: &Option<Vec<String>>) -> Result<String, String> {

    // 1. Захватываем скриншот всех мониторов (RGBA).
    let (image, cursor_info) = library::screenshot::capture_all_monitors_rgba()?;

    // 2. Добавляем изображение в REPORT.
    report::add_image(image).map_err(|e| e.to_string())?;

    let out = format!(
        "Скриншот всех мониторов добавлен в отчёт.\n{}",
        cursor_info.report()
    );

    Ok(wrap_in_fence(&out))
}   // capture_virtual_screen()

//--------------------------------------------------------------------------------------------------
//                  Внутренний интерфейс
//--------------------------------------------------------------------------------------------------

/// Парсит опциональные width/height для capture_mouse_vicinity.
///
/// # Формат
/// - `None` / `[]` -> дефолт 150x70
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