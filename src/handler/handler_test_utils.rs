//! handler_test_utils.rs

//----------------------------------------------------------------------------------------------
//                  Общие настройки тестов
//----------------------------------------------------------------------------------------------

use std::fs;
use crate::agent::request::report::Report;
use crate::agent::request::session;
use crate::glob::{initialize_glob, AgentError};

const SESSION_ID: &str = "1D927F";
const BROWSER: &str = "chrome";
const AI_URL: &str = "https://lmarena.ai";

/// Подстрока заголовка окна-ЦЕЛИ для поиска (для тестов захвата окна по title/HWND).
/// Должна совпадать по регистру (win32tool::find_window_by_title использует contains()).
pub(super) const WINDOW_NEEDLE: &str = "test";

pub(super) struct WindowRegion {
    pub(super) x: &'static str,
    pub(super) y: &'static str,
    pub(super) width: &'static str,
    pub(super) height: &'static str,
}   // WindowRegion

pub(super) const WINDOW_REGION: WindowRegion = WindowRegion {
    x: "100",
    y: "100",
    width: "500",
    height: "400",
};

//----------------------------------------------------------------------------------------------
//                  Вспомогательные функции
//----------------------------------------------------------------------------------------------

/// Описание: Вычисляет `window_title` по умолчанию из общих констант.
pub(super) fn window_title() -> String {
    format!("{} [{}]", AI_URL, SESSION_ID)
}   // window_title()

/// Описание: Best-effort очистка `work.log` и `error.log`.
pub(super) fn cleanup_logs() {
    let _ = fs::remove_file(&crate::glob::config().worklog_path);
    let _ = fs::remove_file(&crate::glob::config().errlog_path);
}   // cleanup_logs()

/// Описание: Инициализирует глобальный session-контекст через `session::init_session_context()`.
///
/// # Возвращаемое значение
/// Тип: Report: Репорт INIT (можно распечатать в тесте).
pub(super) fn init_session_smoke() -> Report {

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
