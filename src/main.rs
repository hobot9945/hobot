//! test_main

use std::fs;
#[allow(unused_imports)]

use std::io;
use std::path::PathBuf;
use opencv::core::get_version_string;
use windows::Win32::UI::HiDpi::{SetProcessDpiAwarenessContext,
                                DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE,
                                DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
                                DPI_AWARENESS_CONTEXT_SYSTEM_AWARE};
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

    // Получить из строки параметров каталог исполнения и валидировать его.
    let exec_dir = get_exec_directory_from_args();

    // Получить таймстамп запуска.
    let ts = std::env::args().nth(2).unwrap_or_else(|| {
        panic!("Критическая ошибка: второй параметр вызова hobot.exe не таймстамп запуска.\n\
Ожидался аргумент TS, например: 2026-02-05_15.46.52\n");
    });

    init_dpi_awareness_best_effort();

    // Инициализировать глобальные переменные. В случае ошибок, возбуждается паника.
    initialize_glob(&exec_dir, &ts);

    // В реальном запуске используем стандартные потоки ввода-вывода
    let stdin = io::stdin();

    // Создать и запустить Agent
    let mut agent = Agent::new();
    let _ = agent.run(stdin);

    // ПРИНУДИТЕЛЬНЫЙ ВЫХОД
    // Убивает все фоновые потоки (clipboard, etc) и возвращает управление батнику.
    // std::process::exit(0);
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

/// Получить каталог исполнения из argv[1] и валидировать.
fn get_exec_directory_from_args() -> String {
    let dir_path = std::env::args().nth(1).unwrap_or_else(|| {
        panic!("Критическая ошибка: первый параметр вызова hobot.exe не передан (ожидался каталог исполнения).");
    });

    // Извлечение метаданных (права доступа, время создания, размер и т.д.)
    let path = PathBuf::from(&dir_path);
    let meta = fs::metadata(&path).unwrap_or_else(|e| {
        panic!("Критическая ошибка: каталог исполнения '{}' недоступен: {}", path.display(), e);
    });

    // Метаданные выделены, проверить каталог ли это?
    if !meta.is_dir() {
        panic!(
            "Критическая ошибка: '{}' существует, но это не каталог.",
            path.display()
        );
    }

    dir_path
}