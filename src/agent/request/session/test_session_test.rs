//! test_session_test
//!
//! Тесты для `session.rs`.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Проверка инициализации сессии через `init_session_context()`.
//! - Проверка доступа к полям сессии через `session_id()/browser()/ai_url()/window_name()/window_title()`.
//! - Проверка валидации сессии (`validate_session()`).
//! - Проверка формирования отчета в `ReportContext` (формат `<<<hbt ... >>>hbt`).
//! - Проверка поведения на ошибках входных данных (невалидный JSON / отсутствие обязательных полей).
//!
//! # ПРИМЕЧАНИЯ
//! Это unit-тесты (внутри крейта), поэтому используются `#[cfg(test)]` и модуль `tests`. [web:206]

#[cfg(test)]
mod tests {
    use crate::writln;

    use crate::agent::request::{report, session};
    use crate::glob::error_control::AgentError;

    /// Единый SessionID для всех тестов модуля.
    ///
    /// ВАЖНО: Сессия хранится в глобальном `OnceLock` и не сбрасывается между тестами,
    /// поэтому все тесты должны использовать один и тот же session_id.
    const TEST_SESSION_ID: &str = "sess_TEST_123";

    /// Проверяет, что отчет обернут в протокольные теги `<<<hbt ... >>>hbt`.
    ///
    /// # Параметры
    /// - `report`: Полный текст отчета из `ReportContext::as_text()`.
    fn _assert_has_hbt_brackets(report: &str) {
        let report = report.trim();

        let opening = format!("<<<hbt {}", TEST_SESSION_ID);
        let closing = format!(">>>hbt {}", TEST_SESSION_ID);

        assert!(report.starts_with(&opening), "Отчет должен начинаться с <<<hbt SESSION_ID");
        assert!(report.ends_with(&closing), "Отчет должен заканчиваться >>>hbt SESSION_ID");
    }   // _assert_has_hbt_brackets()

    /// Возвращает валидный JSON INIT с единым SessionID.
    fn _init_json() -> String {
        format!(r#"
{{
    "type": "INIT",
    "payload": {{
        "session_id": "{sess}",
        "browser": "firefox",
        "ai_url": "https://example.com",
        "window_name": "AI Chat [HBT-123]"
    }}
}}"#, sess = TEST_SESSION_ID)
    }   // _init_json()

    /// Проверяет “счастливый путь” INIT:
    /// - корректный JSON парсится;
    /// - данные сохраняются в глобальном контексте `session`;
    /// - `validate_session()` начинает строго сверять session_id;
    /// - формируется отчет в `ReportContext`.
    ///
    /// # ВАЖНО
    /// Поскольку `OnceLock` не сбрасывается между тестами, этот тест не должен падать,
    /// даже если сессия уже инициализирована другим тестом с тем же SessionID.
    #[test]
    fn test_init_success_saves_data_and_builds_report() {

        let init_json = _init_json();

        // Если сессия уже была инициализирована ранее тем же SessionID, это будет Critical.
        // Чтобы тест был стабильным, допускаем 2 исхода:
        // - Ok(()) если это первая инициализация.
        // - Err(Critical(...)) если кто-то уже успел проинициализировать сессию в рамках процесса.
        let res = session::init_session_context(&init_json);

        match res {
            Ok(()) => {
                // Должен быть сформирован отчет.
                let report = report::text().unwrap();
                _assert_has_hbt_brackets(&report);

                assert!(report.contains("# 🚀 Хобот готов к работе."),
                        "Должен быть заголовок \"# 🚀 Хобот готов к работе.\"");
                assert!(report.contains("**Контекст сессии:**"), "Должен быть заголовок \"**Контекст сессии:**\"");
                assert!(report.contains(TEST_SESSION_ID), "Должен быть session_id");
                assert!(report.contains("https://example.com"), "Должен быть ai_url");
                assert!(report.contains("AI Chat [HBT-123]"), "Должен быть window_name");
            },

            Err(AgentError::Critical(_)) => {
                // Сессия уже инициализирована. Тогда отчет мог быть пустым (мы не строили новый),
                // но глобальные геттеры обязаны работать.
            },

            other => panic!("Неожиданный результат INIT: {:?}", other),
        }   // match

        // После INIT (или если уже был INIT) сверка сессии должна быть строгой.
        assert!(session::validate_session(TEST_SESSION_ID).is_ok(), "Сессия должна валидироваться как своя");
        assert!(
            matches!(session::validate_session("sess_ALIEN"), Err(AgentError::Recoverable(_))),
            "Чужая сессия должна отвергаться как Recoverable"
        );

        // Проверяем доступ к полям.
        assert_eq!(session::session_id().unwrap(), TEST_SESSION_ID);
        assert_eq!(session::browser().unwrap(), "firefox");
        assert_eq!(session::ai_url().unwrap(), "https://example.com");
        assert_eq!(session::window_title().unwrap(), "AI Chat [HBT-123]");
    }   // test_init_success_saves_data_and_builds_report()

    /// Проверяет, что невалидный JSON приводит к `AgentError::Recoverable`.
    #[test]
    fn test_init_invalid_json_is_recoverable() {
        let init_json = r#"{ "type": "INIT", "payload": "#;

        let res = session::init_session_context(init_json);

        match res {
            Err(AgentError::Recoverable(_)) => { /* ok */ },
            other => panic!("Ожидали Err(Recoverable), получено: {:?}", other),
        }   // match
    }   // test_init_invalid_json_is_recoverable()

    /// Проверяет, что отсутствие обязательного поля payload приводит к `AgentError::Recoverable`.
    ///
    /// Здесь JSON валидный, но структура не соответствует `InitEnvelope { payload: SessionContext }`.
    #[test]
    fn test_init_missing_payload_is_recoverable() {
        let init_json = r#"{ "type": "INIT" }"#;

        let res = session::init_session_context(init_json);

        match res {
            Err(AgentError::Recoverable(_)) => { /* ok */ },
            other => panic!("Ожидали Err(Recoverable), получено: {:?}", other),
        }   // match
    }   // test_init_missing_payload_is_recoverable()

    /// Дымовой тест генерации отчёта INIT, предназначенный для ручного просмотра.
    ///
    /// # Примечания
    /// Для отображения вывода теста используй запуск: `cargo test -- --nocapture`. [web:36]
    #[test]
    fn just_a_run() {

        let init_json = _init_json();

        let _ = session::init_session_context(&init_json);

        if !report::is_empty().unwrap() {
            _assert_has_hbt_brackets(&report::text().unwrap());
            writln!("\n{}", report::text().unwrap());
        }
    }   // just_a_run()

}   // mod tests
