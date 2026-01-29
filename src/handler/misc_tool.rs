//! misc_tool.rs

use crate::library::markdown_fence::wrap_in_fence;
use crate::library::misc_tool;

/// Регистрирует команды управления панелью задач.
pub fn handlers_map_init(handlers: &mut std::collections::HashMap<&'static str, crate::handler::HandlerFn>) {
    handlers.insert("show_taskbar", show_taskbar);
    handlers.insert("restore_taskbar_autohide_state", restore_taskbar_autohide_state);
    handlers.insert("show_taskbar_autohide_state", show_taskbar_autohide_state);
}   // handlers_map_init()


/// Отключает авто-скрытие панели задач (делает её видимой).
///
/// # Параметры
/// - Нет параметров.
pub fn show_taskbar(_: &Option<Vec<String>>) -> Result<String, String> {
    misc_tool::show_taskbar();
    Ok(wrap_in_fence("Панель задач зафиксирована (autohide выключен)."))
}   // turn_taskbar_autohide_off()


/// Восстанавливает исходное состояние авто-скрытия панели задач.
///
/// # Параметры
/// - Нет параметров.
pub fn restore_taskbar_autohide_state(_: &Option<Vec<String>>) -> Result<String, String> {
    misc_tool::restore_taskbar_autohide_state();
    Ok(wrap_in_fence("Состояние панели задач восстановлено."))
}   // restore_taskbar_autohide()

/// Проверяет, включен ли режим авто-скрытия панели задач.
///
/// # Параметры
/// - нет параметров.
///
/// # Возвращаемое значение
/// - "true", если авто-скрытие включено.
/// - "false", если панель зафиксирована.
pub fn show_taskbar_autohide_state(_: &Option<Vec<String>>) -> Result<String, String> {
    let state = if misc_tool::is_taskbar_autohide_enabled() { "включено"} else { "выключено" };
    Ok(wrap_in_fence(&format!("Автосокрытие панели задач {}", state)))
}   // is_taskbar_autohide_enabled()