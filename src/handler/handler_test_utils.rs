//! handler_test_utils.rs
#![cfg(test)]
//----------------------------------------------------------------------------------------------
//                  Общие настройки тестов
//----------------------------------------------------------------------------------------------

use crate::agent::request::session;
use crate::glob;
use crate::glob::{initialize_glob, AgentError};
use crate::library::test_utils::{build_log_timestamp_like_bat, get_current_working_dir_no_tail};

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

/// Описание: Инициализирует глобальный session-контекст через `session::init_session_context()`.
///
/// # Побочные эффекты
/// Изменяет `REPORT`.
pub(super) fn init_session_smoke() {

    // Конфиг нужен для логов.
    initialize_glob(&get_current_working_dir_no_tail(), &build_log_timestamp_like_bat());

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

    match session::init_session_context(&init_json_body) {
        Ok(()) => { /* ok */ }

        Err(AgentError::Critical(msg)) if msg.contains("повторная инициализация") => {
            panic!("SESSION_CONTEXT уже инициализирован (перезапусти тестовый процесс): {}", msg);
        }

        Err(e) => {
            panic!("init_session_context() failed: {}", e);
        }
    }   // match
}   // init_session_smoke()

