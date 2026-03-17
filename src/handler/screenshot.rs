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
use crate::library::screenshot::image_grid::GridColor;
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
/// Опционально может наложить координатную сетку поверх изображения.
///
/// # Параметры
/// - `params=["<needle>"]` — обычный скриншот без сетки
/// - `params=["<needle>", "<grid_step>"]` — скриншот с сеткой и цветом по умолчанию
/// - `params=["<needle>", "<grid_step>", "<grid_color>"]` — скриншот с сеткой и заданным цветом
///
/// Формат `grid_color`:
/// - строка вида `"(r, g, b, a)"`
/// - `r`, `g`, `b` — `0..=255`
/// - `a` — `0.0..=1.0`
///
/// Цвет по умолчанию:
/// - `"(255, 0, 0, 0.4)"`
///
/// # Возвращаемое значение
/// Тип: String: Сообщение о выполнении + геометрия окна + информация о курсоре.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное количество параметров;
/// - `needle` пустой;
/// - `grid_step < 5`;
/// - строка цвета имеет неверный формат;
/// - окно не найдено/не удалось сфокусировать;
/// - не удалось сделать скриншот;
/// - не удалось наложить сетку;
/// - не удалось добавить изображение в `Report`.
///
/// # Побочные эффекты
/// - Фокусирует окно-цель (best effort).
/// - Добавляет изображение в `Report.image_list`.
fn capture_window_by_title(params: &Option<Vec<String>>) -> Result<String, String> {
    const DEFAULT_GRID_COLOR: &str = "(255, 0, 0, 0.4)";

    // Разрешённые форматы:
    // 1 параметр: needle
    // 2 параметра: needle, grid_step
    // 3 параметра: needle, grid_step, grid_color
    let param_count = params.as_ref().map_or(0, |v| v.len());
    if param_count != 1 && param_count != 2 && param_count != 3 {
        return Err(format!(
            "Неверное число параметров: ожидалось 1, 2 или 3, получено {}",
            param_count
        ));
    }   // if

    // 1) needle
    let needle: String = handler::check_param_type(params, 0)?;
    if needle.trim().is_empty() {
        return Err("needle пустой".to_string());
    }   // if

    // 2) Опциональные параметры сетки.
    let grid_params = _parse_optional_grid_params(params, 1)?;

    // 3) Найти окно и сфокусировать его (best effort).
    let wi = window::find_window_by_needle_and_focus(&needle)?;

    // 4) Захватить окно (RGBA).
    let (mut image, cursor_info) = library::screenshot::capture_window_rgba(wi.hwnd)?;

    // 5) Опционально наложить сетку.
    if let Some((grid_step, grid_color)) = grid_params {
        image = library::screenshot::image_grid::add_grid(&image, grid_step, grid_color)?;
    }   // if

    // 6) Добавить изображение в REPORT.
    report::add_image(image).map_err(|e| e.to_string())?;

    let out = format!(
        "Скриншот окна по needle='{}' добавлен в отчёт.\nОкно='{}' ({}x{} @ [{},{}])\n{}",
        needle, wi.title, wi.width, wi.height, wi.x, wi.y, cursor_info.report()
    );

    Ok(wrap_in_fence(&out))
}   // capture_window_by_title()

/// Описание: Снимает скриншот окна по HWND и добавляет в `Report`.
///
/// Опционально может наложить координатную сетку поверх изображения.
///
/// Перед захватом выполняется попытка перевести фокус на целевое окно.
///
/// # Параметры
/// Разрешённые форматы:
/// - `["<hwnd>"]`
/// - `["<hwnd>", "<grid_step>"]`
/// - `["<hwnd>", "<grid_step>", "<grid_color>"]`
///
/// Где:
/// - `hwnd`: HWND в десятичном виде или hex с префиксом `0x`;
/// - `grid_step`: `u32`, `>= 5`;
/// - `grid_color`: строка вида `"(r, g, b, a)"`.
///
/// # Возвращаемое значение
/// Тип: String: Markdown-блок с сообщением о выполнении, геометрией и информацией о курсоре.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное количество параметров;
/// - HWND не удалось распарсить;
/// - шаг сетки меньше 5;
/// - цвет сетки имеет неверный формат;
/// - не удалось сфокусировать окно;
/// - не удалось сделать скриншот;
/// - не удалось наложить сетку;
/// - не удалось добавить изображение в `Report`.
///
/// # Побочные эффекты
/// - Фокусирует целевое окно (best effort).
/// - Добавляет изображение в `Report.image_list`.
fn capture_window_by_hwnd(params: &Option<Vec<String>>) -> Result<String, String> {

    let param_count = params.as_ref().map_or(0, |v| v.len());
    if param_count != 1 && param_count != 2 && param_count != 3 {
        return Err(format!(
            "Неверное число параметров: ожидалось 1, 2 или 3, получено {}",
            param_count
        ));
    }   // if

    // 1) HWND.
    let hwnd_str: String = handler::check_param_type(params, 0)?;
    let hwnd = window::parse_hwnd(&hwnd_str)?;

    // 2) Опциональные параметры сетки.
    let grid_params = _parse_optional_grid_params(params, 1)?;

    // 3) Фокусировка окна-цели перед захватом.
    let wi = window::focus_window_with_retries(hwnd)
        .map_err(|e| format!("не удалось сфокусировать окно HWND={}: {}", hwnd_str, e))?;

    // 4) Захватить окно (RGBA).
    let (mut image, cursor_info) = library::screenshot::capture_window_rgba(wi.hwnd)?;

    // 5) Опционально наложить сетку.
    if let Some((grid_step, grid_color)) = grid_params {
        image = library::screenshot::image_grid::add_grid(&image, grid_step, grid_color)?;
    }   // if

    // 6) Добавить изображение в REPORT.
    report::add_image(image).map_err(|e| e.to_string())?;

    let out = format!(
        "Скриншот окна по HWND={} добавлен в отчёт.\nОкно='{}' ({}x{} @ [{},{}])\n{}",
        hwnd_str, wi.title, wi.width, wi.height, wi.x, wi.y, cursor_info.report()
    );

    Ok(wrap_in_fence(&out))
}   // capture_window_by_hwnd()

/// Описание: Снимает скриншот области виртуального рабочего стола и добавляет в `Report`.
///
/// Опционально может наложить координатную сетку поверх изображения.
///
/// Координаты: в системе виртуального рабочего стола Windows.
/// (0,0) — левый верх primary монитора; x/y могут быть отрицательными.
///
/// # Параметры
/// Разрешённые форматы:
/// - `["<x>", "<y>", "<width>", "<height>"]`
/// - `["<x>", "<y>", "<width>", "<height>", "<grid_step>"]`
/// - `["<x>", "<y>", "<width>", "<height>", "<grid_step>", "<grid_color>"]`
///
/// Где:
/// - `x`, `y`: `i32`
/// - `width`, `height`: `u32`
/// - `grid_step`: `u32`, `>= 5`
/// - `grid_color`: строка вида `"(r, g, b, a)"`
///
/// # Возвращаемое значение
/// Тип: String: Сообщение о выполнении + информация о курсоре.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное количество параметров;
/// - параметры не приводятся к нужным типам;
/// - `grid_step < 5`;
/// - цвет сетки имеет неверный формат;
/// - не удалось сделать скриншот;
/// - не удалось наложить сетку;
/// - не удалось добавить изображение в `Report`.
///
/// # Побочные эффекты
/// - Добавляет изображение в `Report.image_list`.
fn capture_region(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Базовые параметры региона.
    let param_count = params.as_ref().map_or(0, |v| v.len());
    if param_count != 4 && param_count != 5 && param_count != 6 {
        return Err(format!(
            "Неверное число параметров: ожидалось 4, 5 или 6, получено {}",
            param_count
        ));
    }   // if

    let x: i32 = handler::check_param_type(params, 0)?;
    let y: i32 = handler::check_param_type(params, 1)?;
    let width: u32 = handler::check_param_type(params, 2)?;
    let height: u32 = handler::check_param_type(params, 3)?;

    // 2) Опциональные параметры сетки.
    let grid_params = _parse_optional_grid_params(params, 4)?;

    // 3) Захватить регион (RGBA).
    let (mut image, cursor_info) = library::screenshot::capture_region_rgba(x, y, width, height)?;

    // 4) Опционально наложить сетку.
    if let Some((grid_step, grid_color)) = grid_params {
        image = library::screenshot::image_grid::add_grid(&image, grid_step, grid_color)?;
    }   // if

    // 5) Добавить изображение в REPORT.
    report::add_image(image).map_err(|e| e.to_string())?;

    let out = format!(
        "Скриншот области (x={}, y={}, {}x{}) добавлен в отчёт.\n{}",
        x, y, width, height, cursor_info.report()
    );

    Ok(wrap_in_fence(&out))
}   // capture_region()

/// Описание: Снимает скриншот области вокруг курсора мыши и добавляет в `Report`.
///
/// Опционально может наложить координатную сетку поверх изображения.
///
/// Курсор стараемся разместить в центре области (best effort, округление вниз).
///
/// # Параметры
/// Поддерживаемые форматы:
/// - `[]`
/// - `["<width>", "<height>"]`
/// - `["<width>", "<height>", "<grid_step>"]`
/// - `["<width>", "<height>", "<grid_step>", "<grid_color>"]`
///
/// Где:
/// - `width`, `height`: `u32`, `> 0`
/// - `grid_step`: `u32`, `>= 5`
/// - `grid_color`: строка вида `"(r, g, b, a)"`
///
/// # Возвращаемое значение
/// Тип: String: Сообщение о выполнении + координаты области + cursor_info.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - число параметров некорректно;
/// - width/height не парсятся или равны 0;
/// - шаг сетки меньше 5;
/// - цвет сетки имеет неверный формат;
/// - не удалось получить позицию курсора;
/// - не удалось сделать скриншот;
/// - не удалось наложить сетку;
/// - не удалось добавить изображение в `Report`.
///
/// # Побочные эффекты
/// - Добавляет изображение в `Report.image_list`.
fn capture_mouse_vicinity(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Разбор параметров команды.
    let (width, height, grid_params) = _parse_mouse_vicinity_params(params)?;

    // 2) Текущая позиция курсора (виртуальный рабочий стол).
    let (cx, cy) = library::mouse::get_cursor_position()
        .map_err(|e| format!("не удалось получить позицию курсора: {}", e))?;

    // 3) Центруем область по курсору.
    let x = cx - (width as i32) / 2;
    let y = cy - (height as i32) / 2;

    // 4) Захватить область (RGBA).
    let (mut image, cursor_info) = library::screenshot::capture_region_rgba(x, y, width, height)?;

    // 5) Опционально наложить сетку.
    if let Some((grid_step, grid_color)) = grid_params {
        image = library::screenshot::image_grid::add_grid(&image, grid_step, grid_color)?;
    }   // if

    // 6) Добавить изображение в REPORT.
    report::add_image(image).map_err(|e| e.to_string())?;

    let out = format!(
        "Скриншот области вокруг курсора добавлен в отчёт.\nОбласть: x={}, y={}, {}x{}\n{}",
        x, y, width, height, cursor_info.report()
    );

    Ok(wrap_in_fence(&out))
}   // capture_mouse_vicinity()

/// Описание: Снимает скриншот монитора по логическому индексу и добавляет в `Report`.
///
/// Опционально может наложить координатную сетку поверх изображения.
///
/// Логический индекс определяется позицией монитора на виртуальном рабочем столе:
/// мониторы упорядочиваются слева направо, затем сверху вниз.
///
/// # Параметры
/// Разрешённые форматы:
/// - `["<logical_index>"]`
/// - `["<logical_index>", "<grid_step>"]`
/// - `["<logical_index>", "<grid_step>", "<grid_color>"]`
///
/// Где:
/// - `logical_index`: индекс монитора, начиная с 0;
/// - `grid_step`: `u32`, `>= 5`;
/// - `grid_color`: строка вида `"(r, g, b, a)"`.
///
/// # Возвращаемое значение
/// Тип: String: Сообщение о выполнении + информация о курсоре.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное количество параметров;
/// - логический индекс не является числом;
/// - шаг сетки меньше 5;
/// - цвет сетки имеет неверный формат;
/// - логический индекс выходит за пределы количества мониторов;
/// - не удалось сделать скриншот;
/// - не удалось наложить сетку;
/// - не удалось добавить изображение в `Report`.
///
/// # Побочные эффекты
/// - Добавляет изображение в `Report.image_list`.
fn capture_monitor(params: &Option<Vec<String>>) -> Result<String, String> {

    let param_count = params.as_ref().map_or(0, |v| v.len());
    if param_count != 1 && param_count != 2 && param_count != 3 {
        return Err(format!(
            "Неверное число параметров: ожидалось 1, 2 или 3, получено {}",
            param_count
        ));
    }   // if

    // 1) Логический индекс монитора.
    let monitor_index: usize = handler::check_param_type(params, 0)?;

    // 2) Опциональные параметры сетки.
    let grid_params = _parse_optional_grid_params(params, 1)?;

    // 3) Захватить скриншот монитора (RGBA).
    let (mut image, cursor_info) = library::screenshot::capture_monitor_rgba(monitor_index)?;

    // 4) Опционально наложить сетку.
    if let Some((grid_step, grid_color)) = grid_params {
        image = library::screenshot::image_grid::add_grid(&image, grid_step, grid_color)?;
    }   // if

    // 5) Добавить изображение в REPORT.
    report::add_image(image).map_err(|e| e.to_string())?;

    let out = format!(
        "Скриншот монитора {} добавлен в отчёт.\n{}",
        monitor_index, cursor_info.report()
    );

    Ok(wrap_in_fence(&out))
}   // capture_monitor()

/// Описание: Снимает скриншот всех мониторов (виртуальный экран) и добавляет в `Report`.
///
/// Опционально может наложить координатную сетку поверх изображения.
///
/// # Параметры
/// Разрешённые форматы:
/// - `[]`
/// - `["<grid_step>"]`
/// - `["<grid_step>", "<grid_color>"]`
///
/// Где:
/// - `grid_step`: `u32`, `>= 5`;
/// - `grid_color`: строка вида `"(r, g, b, a)"`.
///
/// # Возвращаемое значение
/// Тип: String: Сообщение о выполнении + информация о курсоре.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное количество параметров;
/// - шаг сетки меньше 5;
/// - цвет сетки имеет неверный формат;
/// - не удалось сделать скриншот;
/// - не удалось наложить сетку;
/// - не удалось добавить изображение в `Report`.
///
/// # Побочные эффекты
/// - Добавляет изображение в `Report.image_list`.
fn capture_virtual_screen(params: &Option<Vec<String>>) -> Result<String, String> {

    let param_count = params.as_ref().map_or(0, |v| v.len());
    if param_count != 0 && param_count != 1 && param_count != 2 {
        return Err(format!(
            "Неверное число параметров: ожидалось 0, 1 или 2, получено {}",
            param_count
        ));
    }   // if

    // 1) Опциональные параметры сетки.
    let grid_params = _parse_optional_grid_params(params, 0)?;

    // 2) Захватить скриншот всех мониторов (RGBA).
    let (mut image, cursor_info) = library::screenshot::capture_all_monitors_rgba()?;

    // 3) Опционально наложить сетку.
    if let Some((grid_step, grid_color)) = grid_params {
        image = library::screenshot::image_grid::add_grid(&image, grid_step, grid_color)?;
    }   // if

    // 4) Добавить изображение в REPORT.
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

/// Описание: Парсит опциональные параметры координатной сетки в хвосте списка параметров.
///
/// Формат:
/// - только обязательные параметры команды:
///   `params.len() == base_param_count`
/// - обязательные параметры + шаг сетки:
///   `params.len() == base_param_count + 1`
/// - обязательные параметры + шаг сетки + цвет:
///   `params.len() == base_param_count + 2`
///
/// Если шаг сетки не передан, сетка считается отключённой.
///
/// Если шаг передан, но цвет не передан, используется цвет по умолчанию:
/// `"(255, 0, 0, 0.4)"`.
///
/// # Параметры
/// - `params`: Полный список параметров команды.
/// - `base_param_count`: Число обязательных параметров конкретной команды.
///
/// # Возвращаемое значение
/// Тип: `Result<Option<(u32, GridColor)>, String>`
/// - `Ok(None)` — сетка не запрошена;
/// - `Ok(Some((step, color)))` — сетка запрошена, параметры успешно распознаны;
/// - `Err(String)` — формат параметров неверен.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - общее число параметров не равно `base_param_count`, `base_param_count + 1` или `base_param_count + 2`;
/// - шаг сетки не парсится как `u32`;
/// - шаг сетки меньше 5;
/// - цвет не соответствует формату `"(r, g, b, a)"`.
fn _parse_optional_grid_params(params: &Option<Vec<String>>, base_param_count: usize)
    -> Result<Option<(u32, GridColor)>, String>
{
    const DEFAULT_GRID_COLOR: &str = "(255, 0, 0, 0.4)";

    let param_count = params.as_ref().map_or(0, |v| v.len());

    if param_count != base_param_count &&
        param_count != base_param_count + 1 &&
        param_count != base_param_count + 2
    {
        return Err(format!(
            "Неверное число параметров: ожидалось {}, {} или {}, получено {}",
            base_param_count,
            base_param_count + 1,
            base_param_count + 2,
            param_count
        ));
    }   // if

    // Если дополнительных параметров нет — сетка отключена.
    if param_count == base_param_count {
        return Ok(None);
    }   // if

    // Шаг сетки находится сразу после обязательных параметров.
    let grid_step: u32 = handler::check_param_type(params, base_param_count)?;
    if grid_step < 5 {
        return Err(format!(
            "Шаг сетки должен быть >= 5, получено {}",
            grid_step
        ));
    }   // if

    // Цвет либо передан явно, либо берётся по умолчанию.
    let grid_color = if param_count == base_param_count + 2 {
        let color_str: String = handler::check_param_type(params, base_param_count + 1)?;
        _parse_grid_color(&color_str)?
    } else {
        _parse_grid_color(DEFAULT_GRID_COLOR)?
    };

    Ok(Some((grid_step, grid_color)))
}   // _parse_optional_grid_params()

/// Описание: Парсит параметры команды `capture_mouse_vicinity`.
///
/// Поддерживаемые форматы:
/// - `[]`
/// - `["<width>", "<height>"]`
/// - `["<width>", "<height>", "<grid_step>"]`
/// - `["<width>", "<height>", "<grid_step>", "<grid_color>"]`
///
/// Где:
/// - `width`, `height`: `u32`, `> 0`
/// - `grid_step`: `u32`, `>= 5`
/// - `grid_color`: строка вида `"(r, g, b, a)"`
///
/// Если размер не передан, используется значение по умолчанию `150x70`.
///
/// Если шаг сетки не передан, сетка считается отключённой.
///
/// Если шаг передан, но цвет не передан, используется цвет по умолчанию:
/// `"(255, 0, 0, 0.4)"`.
///
/// # Возвращаемое значение
/// Тип: `Result<(u32, u32, Option<(u32, GridColor)>), String>`
/// - `Ok((width, height, None))` — сетка не запрошена;
/// - `Ok((width, height, Some((step, color))))` — сетка запрошена;
/// - `Err(String)` — параметры команды некорректны.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - число параметров не равно 0, 2, 3 или 4;
/// - `width`/`height` не парсятся или равны 0;
/// - `grid_step` не парсится или меньше 5;
/// - `grid_color` имеет неверный формат.
fn _parse_mouse_vicinity_params(
    params: &Option<Vec<String>>,
) -> Result<(u32, u32, Option<(u32, GridColor)>), String> {
    const DEFAULT_WIDTH: u32 = 150;
    const DEFAULT_HEIGHT: u32 = 70;
    const DEFAULT_GRID_COLOR: &str = "(255, 0, 0, 0.4)";

    let param_count = params.as_ref().map_or(0, |v| v.len());

    if param_count != 0 && param_count != 2 && param_count != 3 && param_count != 4 {
        return Err(format!(
            "Неверное число параметров: ожидалось 0, 2, 3 или 4, получено {}",
            param_count
        ));
    }   // if

    // --- 1. Размер области ---
    let (width, height) = if param_count == 0 {
        (DEFAULT_WIDTH, DEFAULT_HEIGHT)
    } else {
        let width: u32 = handler::check_param_type(params, 0)?;
        let height: u32 = handler::check_param_type(params, 1)?;

        if width == 0 || height == 0 {
            return Err("width и height должны быть > 0".to_string());
        }   // if

        (width, height)
    };

    // --- 2. Параметры сетки ---
    let grid_params = if param_count == 0 || param_count == 2 {
        None
    } else {
        let grid_step: u32 = handler::check_param_type(params, 2)?;
        if grid_step < 5 {
            return Err(format!(
                "Шаг сетки должен быть >= 5, получено {}",
                grid_step
            ));
        }   // if

        let grid_color = if param_count == 4 {
            let color_str: String = handler::check_param_type(params, 3)?;
            _parse_grid_color(&color_str)?
        } else {
            _parse_grid_color(DEFAULT_GRID_COLOR)?
        };

        Some((grid_step, grid_color))
    };

    Ok((width, height, grid_params))
}   // _parse_mouse_vicinity_params()

/// Описание: Парсит строку цвета сетки формата `"(r, g, b, a)"`.
///
/// Формат:
/// - `r`, `g`, `b`: целые числа `0..=255`
/// - `a`: вещественное число `0.0..=1.0`
///
/// Примеры:
/// - `"(255, 0, 0, 0.4)"`
/// - `"(0,128,255,1.0)"`
/// - `"( 255 , 255 , 0 , 0.25 )"`
///
/// # Возвращаемое значение
/// Тип: `image_grid::GridColor`: Цвет линии сетки в формате RGBA, где alpha уже
/// преобразована в диапазон `0..=255`.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - строка не начинается с `(` или не заканчивается `)`;
/// - количество компонентов не равно 4;
/// - `r/g/b` не являются целыми `0..=255`;
/// - `a` не является числом `0.0..=1.0`.
fn _parse_grid_color(color_str: &str) -> Result<library::screenshot::image_grid::GridColor, String> {

    let s = color_str.trim();

    // Проверяем внешние скобки.
    if !s.starts_with('(') || !s.ends_with(')') {
        return Err(format!(
            "Неверный формат цвета '{}': ожидалась строка вида '(r, g, b, a)'",
            color_str
        ));
    }   // if

    // Убираем внешние скобки.
    let inner = &s[1..s.len() - 1];

    // Делим по запятым.
    let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();

    if parts.len() != 4 {
        return Err(format!(
            "Неверный формат цвета '{}': ожидалось 4 компонента, получено {}",
            color_str,
            parts.len()
        ));
    }   // if

    // RGB как u8.
    let r: u8 = parts[0].parse().map_err(|_| {
        format!("Неверный red-компонент цвета '{}': '{}'", color_str, parts[0])
    })?;

    let g: u8 = parts[1].parse().map_err(|_| {
        format!("Неверный green-компонент цвета '{}': '{}'", color_str, parts[1])
    })?;

    let b: u8 = parts[2].parse().map_err(|_| {
        format!("Неверный blue-компонент цвета '{}': '{}'", color_str, parts[2])
    })?;

    // Alpha как float 0.0..=1.0.
    let a_f32: f32 = parts[3].parse().map_err(|_| {
        format!("Неверный alpha-компонент цвета '{}': '{}'", color_str, parts[3])
    })?;

    if !(0.0..=1.0).contains(&a_f32) {
        return Err(format!(
            "Alpha-компонент цвета '{}' вне диапазона [0.0..1.0]: {}",
            color_str, a_f32
        ));
    }   // if

    // Переводим alpha в диапазон 0..=255 с округлением.
    let a_u8 = (a_f32 * 255.0).round() as u8;

    Ok(library::screenshot::image_grid::GridColor(r, g, b, a_u8))
}   // _parse_grid_color()
