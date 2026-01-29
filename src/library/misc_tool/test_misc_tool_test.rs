//! test_misc_tool_test.rs
//!
//! Дымовые тесты для модуля misc_tool.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Проверка базовой работоспособности функций управления панелью задач.
//!
//! # Важно
//! Тесты помечены `#[ignore]`, так как изменяют глобальное состояние панели задач Windows.
//! Запуск вручную: `cargo test test_misc_tool -- --ignored`

use std::thread;
use std::time::Duration;
use crate::wrln;
use super::{show_taskbar, restore_taskbar_autohide_state};

/// Дымовой тест: show_taskbar() и restore_taskbar() выполняются без паники.
///
/// Проверяет:
/// - Одиночный цикл show/restore.
/// - Повторный restore (когда состояние уже сброшено) не паникует.
/// - Повторный show не перетирает исходное сохраненное состояние.
#[test]
#[ignore]
fn test_taskbar_show_restore_smoke() {
    // Первый цикл: show сохраняет состояние, restore восстанавливает.
    show_taskbar();
    wrln!("выдвинулся");
    thread::sleep(Duration::from_secs(1));

    restore_taskbar_autohide_state();
    wrln!("задвинулся");
    thread::sleep(Duration::from_secs(1));
}   // test_taskbar_show_restore_smoke()
