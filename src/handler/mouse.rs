//! mouse.rs — Хэндлеры команд управления мышью.
//!
//! # ОПИСАНИЕ
//! Модуль реализует команды протокола для управления курсором и кликами мыши.
//!
//! # КООРДИНАТЫ
//! Все координаты — в пикселях виртуального рабочего стола (могут быть отрицательными).
//!
//! # КОМАНДЫ (имена для AI)
//! - `get_mouse_position`: params=[]
//! - `mouse_move_to`:      params=["x","y"]
//! - `mouse_left_click`:   params=[] или ["x","y"]
//! - `mouse_left_dblclick`: params=[] или ["x","y"]
//! - `mouse_right_click`:  params=[] или ["x","y"]
//! - `mouse_right_dblclick`: params=[] или ["x","y"]
//!
//! # ВАЖНО
//! Для того чтобы команды реально появились в реестре, нужно вызвать
//! `mouse::handlers_map_init()` из `HandlerRegistry::new()` (в handler.rs).

use std::collections::HashMap;

use crate::handler::{check_param_count, check_param_type, HandlerFn};
use crate::library::markdown_fence::wrap_in_fence;
use crate::library::mouse;

/// Регистрирует mouse-команды в карту хэндлеров.
///
/// # Параметры
/// - `handlers_map`: Карта `command_name -> handler_fn`.
pub fn handlers_map_init(handlers_map: &mut HashMap<&str, HandlerFn>) {
    handlers_map.insert("get_mouse_position", get_mouse_position);
    handlers_map.insert("mouse_move_to", mouse_move_to);

    handlers_map.insert("mouse_left_click", mouse_left_click);
    handlers_map.insert("mouse_left_dblclick", mouse_left_dblclick);

    handlers_map.insert("mouse_right_click", mouse_right_click);
    handlers_map.insert("mouse_right_dblclick", mouse_right_dblclick);

    handlers_map.insert("mouse_scroll", mouse_scroll);
    handlers_map.insert("mouse_drag", mouse_drag);
}   // handlers_map_init()

/// Возвращает текущую позицию курсора мыши.
///
/// # Параметры
/// `params=[]`
///
/// # Возвращаемое значение
/// Тип: String: Координаты курсора в JSON (внутри markdown fence).
///
/// Пример:
/// `{"x": 123, "y": 456}`
///
/// # Ошибки
/// - Неверное число параметров (разрешено только 0).
/// - Ошибка WinAPI при чтении позиции курсора (пробрасывается из library::mouse).
fn get_mouse_position(params: &Option<Vec<String>>) -> Result<String, String> {

    // Параметры не требуются.
    check_param_count(params, 0).map_err(|e| wrap_in_fence(&e))?;

    let (x, y) = mouse::get_cursor_position()
        .map_err(|e| wrap_in_fence(&e))?;

    Ok(wrap_in_fence(&format!(r#"{{"x":{},"y":{}}}"#, x, y)))
}   // get_mouse_position()

/// Плавно перемещает курсор в точку (x, y).
///
/// # Параметры
/// `params=["x","y"]`
///
/// # Ошибки
/// - Неверное число параметров.
/// - Неверный тип параметров.
/// - Ошибка WinAPI при перемещении (пробрасывается из library::mouse).
fn mouse_move_to(params: &Option<Vec<String>>) -> Result<String, String> {

    // Тут движение без координат бессмысленно, поэтому требуем ровно 2 параметра.
    check_param_count(params, 2).map_err(|e| wrap_in_fence(&e))?;

    let x: i32 = check_param_type(params, 0).map_err(|e| wrap_in_fence(&e))?;
    let y: i32 = check_param_type(params, 1).map_err(|e| wrap_in_fence(&e))?;

    mouse::move_cursor_to_position(x, y)
        .map_err(|e| wrap_in_fence(&e))?;

    Ok(wrap_in_fence("OK"))   // можно заменить на более информативный текст позже
}   // mouse_move_to()

/// Одиночный левый клик.
///
/// # Параметры
/// - `params=[]`            -> клик в текущей позиции курсора.
/// - `params=["x","y"]`     -> плавно переместиться в (x, y), затем кликнуть.
///
/// # Ошибки
/// - Неверное число параметров (разрешено только 0 или 2).
fn mouse_left_click(params: &Option<Vec<String>>) -> Result<String, String> {
    let pos = _parse_optional_xy(params).map_err(|e| wrap_in_fence(&e))?;

    mouse::left_click(pos)
        .map_err(|e| wrap_in_fence(&e))?;

    Ok(wrap_in_fence("OK"))
}   // mouse_left_click()

/// Двойной левый клик.
///
/// # Параметры
/// - `params=[]`            -> двойной клик в текущей позиции.
/// - `params=["x","y"]`     -> плавно переместиться в (x, y), затем двойной клик.
fn mouse_left_dblclick(params: &Option<Vec<String>>) -> Result<String, String> {
    let pos = _parse_optional_xy(params).map_err(|e| wrap_in_fence(&e))?;

    mouse::left_double_click(pos)
        .map_err(|e| wrap_in_fence(&e))?;

    Ok(wrap_in_fence("OK"))
}   // mouse_left_dblclick()

/// Одиночный правый клик (контекстное меню).
///
/// # Параметры
/// - `params=[]`            -> клик в текущей позиции.
/// - `params=["x","y"]`     -> плавно переместиться в (x, y), затем кликнуть.
fn mouse_right_click(params: &Option<Vec<String>>) -> Result<String, String> {
    let pos = _parse_optional_xy(params).map_err(|e| wrap_in_fence(&e))?;

    mouse::right_click(pos)
        .map_err(|e| wrap_in_fence(&e))?;

    Ok(wrap_in_fence("OK"))
}   // mouse_right_click()

/// Двойной правый клик (редко используется).
///
/// # Параметры
/// - `params=[]`            -> двойной клик в текущей позиции.
/// - `params=["x","y"]`     -> плавно переместиться в (x, y), затем двойной клик.
fn mouse_right_dblclick(params: &Option<Vec<String>>) -> Result<String, String> {
    let pos = _parse_optional_xy(params).map_err(|e| wrap_in_fence(&e))?;

    mouse::right_double_click(pos)
        .map_err(|e| wrap_in_fence(&e))?;

    Ok(wrap_in_fence("OK"))
}   // mouse_right_dblclick()

/// Прокрутка колеса мыши (вертикальный скролл).
///
/// # Параметры
/// - `params=["<lines>"]`
///   Скролл в текущей позиции курсора.
/// - `params=["<x>","<y>","<lines>"]`
///   Плавно подвести курсор в (x,y), затем проскроллить.
///
/// # Направление
/// - `lines > 0` — вверх
/// - `lines < 0` — вниз
fn mouse_scroll(params: &Option<Vec<String>>) -> Result<String, String> {

    let (pos, lines) = _parse_scroll_args(params)
        .map_err(|e| wrap_in_fence(&e))?;

    mouse::scroll(pos, lines)
        .map_err(|e| wrap_in_fence(&e))?;

    Ok(wrap_in_fence("OK"))
}   // mouse_scroll()

/// Перетаскивание объекта мышью (drag-and-drop).
///
/// # Параметры
/// `params=["<x_from>","<y_from>","<x_to>","<y_to>"]`
///
/// Алгоритм внутри library::mouse::drag():
/// - плавно подвести курсор в from
/// - зажать ЛКМ
/// - протянуть до to
/// - отпустить ЛКМ
fn mouse_drag(params: &Option<Vec<String>>) -> Result<String, String> {

    // Требуем ровно 4 параметра.
    check_param_count(params, 4).map_err(|e| wrap_in_fence(&e))?;

    let x_from: i32 = check_param_type(params, 0).map_err(|e| wrap_in_fence(&e))?;
    let y_from: i32 = check_param_type(params, 1).map_err(|e| wrap_in_fence(&e))?;
    let x_to: i32 = check_param_type(params, 2).map_err(|e| wrap_in_fence(&e))?;
    let y_to: i32 = check_param_type(params, 3).map_err(|e| wrap_in_fence(&e))?;

    mouse::drag((x_from, y_from), (x_to, y_to))
        .map_err(|e| wrap_in_fence(&e))?;

    Ok(wrap_in_fence("OK"))
}   // mouse_drag()


//--------------------------------------------------------------------------------------------------
//                  Внутренний интерфейс
//--------------------------------------------------------------------------------------------------

/// Парсит позицию курсора как `Option<(x,y)>` из params.
///
/// # Формат
/// - `None` или `Some(vec![])` => `Ok(None)`
/// - `Some(vec![x, y])`        => `Ok(Some((x,y)))`
///
/// # Зачем именно так
/// Для кликов удобный контракт:
/// - нет параметров -> действие в текущей позиции,
/// - 2 параметра -> сначала плавно перемещаемся, потом выполняем действие.
///
/// # Ошибки
/// Возвращает `Err(String)`, если количество параметров не 0 и не 2, либо x/y не парсятся в i32.
fn _parse_optional_xy(params: &Option<Vec<String>>) -> Result<Option<(i32, i32)>, String> {

    // 1) Вообще нет params => None (текущая позиция).
    let Some(v) = params.as_ref() else {
        return Ok(None);
    };

    // 2) Явно пустой список => None.
    if v.is_empty() {
        return Ok(None);
    }   // if

    // 3) Разрешаем только 2 параметра.
    if v.len() != 2 {
        return Err(format!(
            "Неверное число параметров: ожидалось 0 или 2, получено {}",
            v.len()
        ));
    }   // if

    // 4) Парсим x/y.
    let x: i32 = check_param_type(params, 0)?;
    let y: i32 = check_param_type(params, 1)?;

    Ok(Some((x, y)))
}   // _parse_optional_xy()

/// Парсит параметры команды mouse_scroll.
///
/// # Формат
/// - `None` / `[]` -> ошибка (нужен хотя бы lines)
/// - `[lines]` -> (None, lines)
/// - `[x, y, lines]` -> (Some((x,y)), lines)
fn _parse_scroll_args(params: &Option<Vec<String>>) -> Result<(Option<(i32, i32)>, i32), String> {

    let Some(v) = params.as_ref() else {
        return Err("Неверное число параметров: ожидалось 1 или 3, получено 0".to_string());
    };

    match v.len() {
        1 => {
            let lines: i32 = check_param_type(params, 0)?;
            Ok((None, lines))
        },

        3 => {
            let x: i32 = check_param_type(params, 0)?;
            let y: i32 = check_param_type(params, 1)?;
            let lines: i32 = check_param_type(params, 2)?;
            Ok((Some((x, y)), lines))
        },

        n => Err(format!("Неверное число параметров: ожидалось 1 или 3, получено {}", n)),
    }   // match
}   // _parse_scroll_args()
