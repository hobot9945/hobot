//! test_screenshot_test — Дымовые тесты хэндлеров `handler::screenshot`.
//!
//! # ОПИСАНИЕ
//! Тесты проверяют end-to-end поведение команд скриншотов на уровне хэндлеров:
//! - снятие скриншота;
//! - копирование в буфер обмена (“карман”);
//! - фокусировка окна AI и вставка (Ctrl+V) в поле ввода.
//!
//! # ПРИМЕЧАНИЯ
//! - Инициализация сессии выполняется напрямую через `session::init_session_context()`.
//! - Тесты помечены `#[ignore]` (требуют реального окна и вмешиваются в clipboard/фокус).

#[cfg(test)]
mod tests {
    use crate::agent::request::report;
    use crate::handler::handler_test_utils::{init_session_smoke, WINDOW_NEEDLE, WINDOW_REGION};
    // Хэндлеры, которые тестируем (доступны из дочернего модуля).
    use crate::handler::screenshot::{
        capture_monitor,
        capture_region,
        capture_virtual_screen,
        capture_window_by_hwnd,
        capture_window_by_title,
        get_monitor_layout,
    };

    use crate::library::window;

    //----------------------------------------------------------------------------------------------
    //                  тесты
    //----------------------------------------------------------------------------------------------

    /// Описание: Дымовой тест хэндлера `get_monitor_layout`.
    ///
    /// # Побочные эффекты
    /// - Не трогает clipboard и не меняет фокус окна.
    #[ignore]
    #[test]
    fn smoke_get_monitor_layout() {

        let init_report = init_session_smoke();
        println!("\n=== INIT REPORT ===\n{}\n===================\n", report::text().unwrap());

        // Команда без параметров.
        let res = get_monitor_layout(&None)
            .expect("get_monitor_layout() failed");

        println!("\n=== HANDLER RESULT ===\n{}\n======================\n", res);

        // Минимальная sanity-проверка, что вернулась таблица.
        assert!(res.contains("| logical | physical |"), "ожидалась таблица Markdown (заголовок не найден)");
    }   // smoke_get_monitor_layout()

    /// Описание: Дымовой тест хэндлера `capture_monitor`.
    ///
    /// # Побочные эффекты
    /// - Перезаписывает системный буфер обмена.
    /// - Переводит фокус на окно AI.
    /// - Генерирует Ctrl+V.
    #[ignore]
    #[test]
    fn smoke_screenshot_monitor() {

        let init_report = init_session_smoke();
        println!("\n=== INIT REPORT ===\n{}\n===================\n", report::text().unwrap());

        // Параметр: логический индекс монитора (0 = первый).
        let params = Some(vec!["1".to_string()]);

        // Прямой вызов тестируемого хэндлера.
        let res = capture_monitor(&params)
            .expect("capture_monitor() failed");

        println!("\n=== HANDLER RESULT ===\n{}\n======================\n", res);
    }   // smoke_screenshot_monitor()

    /// Описание: Дымовой тест хэндлера `capture_virtual_screen`.
    ///
    /// # Побочные эффекты
    /// - Перезаписывает системный буфер обмена.
    /// - Переводит фокус на окно AI.
    /// - Генерирует Ctrl+V.
    #[ignore]
    #[test]
    fn smoke_screenshot_all_monitors() {

        let init_report = init_session_smoke();
        println!("\n=== INIT REPORT ===\n{}\n===================\n", report::text().unwrap());

        // Команда без параметров.
        let res = capture_virtual_screen(&None)
            .expect("capture_virtual_screen() failed");

        println!("\n=== HANDLER RESULT ===\n{}\n======================\n", res);
    }   // smoke_screenshot_all_monitors()

    /// Описание: Дымовой тест хэндлера `capture_region` (x,y,w,h -> clipboard -> Ctrl+V в окно AI).
    ///
    /// # Побочные эффекты
    /// - Перезаписывает системный буфер обмена.
    /// - Переводит фокус на окно AI.
    /// - Генерирует Ctrl+V.
    #[ignore]
    #[test]
    fn smoke_capture_region() {

        let init_report = init_session_smoke();
        println!("\n=== INIT REPORT ===\n{}\n===================\n", report::text().unwrap());

        // Параметры: x, y, width, height.
        let params = Some(vec![
            WINDOW_REGION.x.to_string(),
            WINDOW_REGION.y.to_string(),
            WINDOW_REGION.width.to_string(),
            WINDOW_REGION.height.to_string(),
        ]);

        let res = capture_region(&params)
            .expect("capture_region() failed");

        println!("\n=== HANDLER RESULT ===\n{}\n======================\n", res);
    }   // smoke_capture_region()

    /// Описание: Дымовой тест хэндлера `capture_window_by_title` (needle -> screenshot -> clipboard -> Ctrl+V).
    ///
    /// # Побочные эффекты
    /// - Перезаписывает системный буфер обмена.
    /// - Фокусирует найденное окно-ЦЕЛЬ.
    /// - Переводит фокус на окно AI.
    /// - Генерирует Ctrl+V.
    #[ignore]
    #[test]
    fn smoke_capture_window_by_title() {

        let init_report = init_session_smoke();
        println!("\n=== INIT REPORT ===\n{}\n===================\n", report::text().unwrap());

        // Параметр: needle окна-ЦЕЛИ.
        let params = Some(vec![WINDOW_NEEDLE.to_string()]);

        let res = capture_window_by_title(&params)
            .expect("capture_window_by_title() failed");

        println!("\n=== HANDLER RESULT ===\n{}\n======================\n", res);
    }   // smoke_capture_window_by_title()

    /// Описание: Дымовой тест хэндлера `capture_window_by_hwnd` (hwnd -> screenshot -> clipboard -> Ctrl+V).
    ///
    /// # Побочные эффекты
    /// - Перезаписывает системный буфер обмена.
    /// - Переводит фокус на окно AI.
    /// - Генерирует Ctrl+V.
    #[ignore]
    #[test]
    fn smoke_capture_window_by_hwnd() {

        let init_report = init_session_smoke();
        println!("\n=== INIT REPORT ===\n{}\n===================\n", report::text().unwrap());

        // 1) Найти окно-ЦЕЛЬ и взять его HWND.
        let (hwnd, win_title) = window::find_window_by_needle(WINDOW_NEEDLE)
            .unwrap_or_else(|e| panic!("окно не найдено: {}", e));

        // 2) Превратить HWND в строковый параметр (hex), который принимает хэндлер.
        let hwnd_str = format!("0x{:X}", hwnd.0 as usize);

        // 3) Вызвать хэндлер.
        let params = Some(vec![hwnd_str.clone()]);

        let res = capture_window_by_hwnd(&params)
            .expect("capture_window_by_hwnd() failed");

        println!("\n=== TARGET WINDOW ===\nHWND={}\nTITLE={}\n=====================\n", hwnd_str, win_title);
        println!("\n=== HANDLER RESULT ===\n{}\n======================\n", res);
    }   // smoke_capture_window_by_hwnd()

}   // tests