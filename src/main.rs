//! test_main

#[allow(unused_imports)]

use std::io;
use windows::Win32::UI::HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, DPI_AWARENESS_CONTEXT_SYSTEM_AWARE};
use windows::Win32::UI::WindowsAndMessaging::SetProcessDPIAware;
use crate::agent::Agent;
use crate::glob::initialize_glob;

mod agent;
mod glob;
mod handler;
pub mod library;
#[cfg(test)]
mod test_main_test;

fn main() {

    init_dpi_awareness_best_effort();

    // Инициализировать глобальные переменные. Если случились ошибки, завершаем работу. Сообщения
    // об ошибках будут выданы там внутри.
    if let Err(_) = initialize_glob() {
        return;
    };

    // В реальном запуске используем стандартные потоки ввода-вывода
    let stdin = io::stdin();

    // Создать и запустить Agent
    let mut agent = Agent::new();
    let _ = agent.run(stdin);
}

/// Включает DPI awareness для процесса (best effort).
/// Нужно, чтобы при выборе масштаба в настройках экрана отличного от 100%, координаты курсора
/// показывались правильно.
///
/// Важно вызвать ДО любых операций, завязанных на координаты/размеры экрана
/// (xcap, GetCursorInfo/GetCursorPos, GDI и т.п.).
fn init_dpi_awareness_best_effort() {
    unsafe {
        // Лучший вариант для Win10/11: Per Monitor v2.
        if SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2).is_ok() {
            return;
        }   // if

        // Фолбэк: Per Monitor v1.
        if SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE).is_ok() {
            return;
        }   // if

        // Фолбэк: System aware.
        if SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_SYSTEM_AWARE).is_ok() {
            return;
        }   // if

        // Самый старый фолбэк.
        let _ = SetProcessDPIAware();
    }   // unsafe
}   // init_dpi_awareness_best_effort()