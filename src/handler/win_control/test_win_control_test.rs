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
    use std::fs;

    use crate::agent::request::report::Report;
    use crate::agent::request::session;
    use crate::glob::error_control::AgentError;
    use crate::glob::initialize_glob;
    // Хэндлеры, которые тестируем (доступны из дочернего модуля).
    use crate::handler::win_control::{
        find_window_info,
        get_foreground_window_info,
    };

    //----------------------------------------------------------------------------------------------
    //                  Общие настройки тестов (легкое управление)
    //----------------------------------------------------------------------------------------------

    const SESSION_ID: &str = "1D927F";
    const BROWSER: &str = "chrome";
    const AI_URL: &str = "https://arena.ai";

    /// Подстрока заголовка окна-ЦЕЛИ для поиска (для тестов захвата окна по title/HWND).
    /// Должна совпадать по регистру (win32tool::find_window_by_title использует contains()).
    const WINDOW_NEEDLE: &str = "test";

    struct WindowRegion {
        x: &'static str,
        y: &'static str,
        width: &'static str,
        height: &'static str,
    }   // WindowRegion

    const WINDOW_REGION: WindowRegion = WindowRegion {
        x: "100",
        y: "100",
        width: "500",
        height: "400",
    };

    //----------------------------------------------------------------------------------------------
    //                  Вспомогательные функции
    //----------------------------------------------------------------------------------------------

    /// Описание: Вычисляет `window_title` по умолчанию из общих констант.
    fn window_title() -> String {
        format!("{} [{}]", AI_URL, SESSION_ID)
    }   // window_title()

    /// Описание: Best-effort очистка `work.log` и `error.log`.
    fn cleanup_logs() {
        let _ = fs::remove_file(&crate::glob::config().worklog_path);
        let _ = fs::remove_file(&crate::glob::config().errlog_path);
    }   // cleanup_logs()

    /// Описание: Инициализирует глобальный session-контекст через `session::init_session_context()`.
    ///
    /// # Возвращаемое значение
    /// Тип: Report: Репорт INIT (можно распечатать в тесте).
    fn init_session_smoke() -> Report {

        // Конфиг нужен для логов. Повторная инициализация допустима для ручных тестов.
        if let Err(e) = initialize_glob() {
            if !e.contains("Повтор") && !e.contains("повтор") {
                panic!("Failed to initialize glob: {}", e);
            }
        }   // if

        cleanup_logs();

        // ВАЖНО: `session::init_session_context()` принимает JSON-ТЕЛО (без <<<ext/>>>ext).
        let init_json_body = format!(
            r#"{{
    "type": "INIT_SESSION",
    "payload": {{
        "session_id": "{session_id}",
        "browser": "{browser}",
        "ai_url": "{ai_url}",
        "window_title": "{window_title}"
    }}
}}"#,
            session_id = SESSION_ID,
            browser = BROWSER,
            ai_url = AI_URL,
            window_title = window_title()
        );

        let mut report = Report::new();

        match session::init_session_context(&init_json_body, &mut report) {
            Ok(()) => { /* ok */ }

            Err(AgentError::Critical(msg)) if msg.contains("повторная инициализация") => {
                panic!("SESSION_CONTEXT уже инициализирован (перезапусти тестовый процесс): {}", msg);
            }

            Err(e) => {
                panic!("init_session_context() failed: {}", e);
            }
        }   // match

        report
    }   // init_session_smoke()

    //----------------------------------------------------------------------------------------------
    //                  тесты
    //----------------------------------------------------------------------------------------------

    /// Описание: Дымовой тест хэндлера `get_foreground_window_info_handler`.
    ///
    /// # Побочные эффекты
    /// - Нет.
    #[ignore]
    #[test]
    fn smoke_get_foreground_window_info() {

        // INIT не обязателен, но оставляем единый сценарий запуска.
        let _ = init_session_smoke();

        // Команда без параметров.
        let res = get_foreground_window_info(&None)
            .expect("get_foreground_window_info_handler() failed");

        println!("\n=== HANDLER RESULT ===\n{}\n======================\n", res);
    }   // smoke_get_foreground_window_info()

    /// Описание: Дымовой тест хэндлера `find_window_info` (needle -> HWND + полный title).
    ///
    /// # Побочные эффекты
    /// - Нет.
    #[ignore]
    #[test]
    fn smoke_find_window_info() {

        // Команда без необходимости INIT, но оставляем единый сценарий запуска тестов.
        let _ = init_session_smoke();

        // Параметр: needle.
        let params = Some(vec![WINDOW_NEEDLE.to_string()]);

        let res = find_window_info(&params)
            .expect("find_window_info() failed");

        println!("\n=== HANDLER RESULT ===\n{}\n======================\n", res);
    }   // smoke_find_window_info()
}   // tests