//! test_extmsg_context_test
//!
//! Тесты для `ext_msg`.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Проверка парсинга `PROTOCOL_ERROR` (JSON от расширения).
//! - Проверка формирования отчета в `ReportContext` (формат `<<<hbt ... >>>hbt`).
//! - Проверка поведения при ошибках входных данных (невалидный JSON / отсутствие `error_message`).
//!
//! # ПРИМЕЧАНИЯ
//! Это unit-тесты (внутри крейта), поэтому используются `#[cfg(test)]` и модуль `tests`. [web:206]

#[cfg(test)]
mod tests {
    use crate::writln;

    use crate::agent::request::ext_msg::ExtensionMessageContext;
    use crate::agent::request::report;
    use crate::glob::error_control::AgentError;

    /// Проверяет, что отчет обернут в протокольные теги `<<<hbt SESSION_ID ... >>>hbt SESSION_ID`.
    ///
    /// # Параметры
    /// - `report`: Полный текст отчета из `ReportContext::as_text()`.
    /// - `session_id`: Ожидаемый идентификатор сессии в тегах.
    fn _assert_has_hbt_brackets(report: &str, session_id: &str) {
        let opening = format!("<<<hbt {}", session_id);
        let closing = format!(">>>hbt {}", session_id);

        assert!(
            report.starts_with(&opening),
            "Отчет должен начинаться с '{}'",
            opening
        );

        assert!(
            report.ends_with(&closing),
            "Отчет должен заканчиваться '{}'",
            closing
        );
    }   // _assert_has_hbt_brackets()

    /// Проверяет “счастливый путь”:
    /// - корректный JSON с `error_message` парсится;
    /// - строится отчет в формате `<<<hbt SESSION_ID ... >>>hbt SESSION_ID`;
    /// - полезная нагрузка попадает внутрь отчета.
    #[test]
    fn test_protocol_error_success_builds_report() {

        // 0) Идентификатор сессии (в реальности приходит из INIT).
        let session_id = "123456";

        // 1) Подготовка входных данных: JSON-тело EXT сообщения (без <<<ext ... >>>ext).
        // Тут нам важно только поле `error_message`, которое должно быть проброшено ИИ.
        let json_body = r#"{ "error_message": "DIRECTIVE_ID mismatch: expected 5, received 6" }"#;

        // 2) Обработка:
        // - ctx: обработчик EXT ошибок.
        // - report_ctx: контейнер результата, который агент потом отправит в чат ИИ.
        let mut ctx = ExtensionMessageContext::new();

        let res = ctx.handle_extension_message_request(json_body, session_id);
        assert!(res.is_ok(), "Ожидали Ok(()), получено: {:?}", res);

        // 3) Проверки отчета:
        // Проверяем не точное совпадение всего markdown, а “якорные” элементы протокола и смысла.
        let report = report::work_report().unwrap();

        _assert_has_hbt_brackets(&report, session_id);

        // Отчет должен явно сказать, что это проблема расширения, а не “ошибка команды”.
        assert!(report.contains("Ошибка протокола расширения"), "Должен быть заголовок отчета");

        // Главное: сообщение расширения должно попасть в тело.
        assert!(
            report.contains("DIRECTIVE_ID mismatch: expected 5, received 6"),
            "Должна быть полезная нагрузка error_message"
        );
    }   // test_protocol_error_success_builds_report()

    /// Проверяет поведение на невалидном JSON:
    /// - метод обязан вернуть `AgentError::Recoverable`;
    /// - это не критическая ошибка агента (агент не должен падать).
    #[test]
    fn test_protocol_error_invalid_json_is_recoverable() {

        // 0) Идентификатор сессии (в реальности приходит из INIT).
        let session_id = "123456";

        // Некорректный JSON: строка оборвана.
        let json_body = r#"{ "error_message": "#;

        let mut ctx = ExtensionMessageContext::new();

        let res = ctx.handle_extension_message_request(json_body, session_id);

        match res {
            Err(AgentError::Recoverable(_)) => { /* ok */ },
            other => panic!("Ожидали Err(Recoverable), получено: {:?}", other),
        }   // match
    }   // test_protocol_error_invalid_json_is_recoverable()

    /// Проверяет поведение на валидном JSON без обязательного поля `error_message`:
    /// - метод обязан вернуть `AgentError::Recoverable`;
    /// - это ошибка входных данных, а не “поломка агента”.
    #[test]
    fn test_protocol_error_missing_field_is_recoverable() {

        // 0) Идентификатор сессии (в реальности приходит из INIT).
        let session_id = "123456";

        // Валидный JSON, но нет нужного поля error_message.
        let json_body = r#"{ "nope": "value" }"#;

        let mut ctx = ExtensionMessageContext::new();

        let res = ctx.handle_extension_message_request(json_body, session_id);

        match res {
            Err(AgentError::Recoverable(_)) => { /* ok */ },
            other => panic!("Ожидали Err(Recoverable), получено: {:?}", other),
        }   // match
    }   // test_protocol_error_missing_field_is_recoverable()

    /// Дымовой тест для ручного просмотра отчета.
    ///
    /// # Примечания
    /// Для отображения вывода теста используй запуск: `cargo test -- --nocapture`. [web:235]
    #[test]
    fn just_a_run() {

        // --- Arrange ---
        // Идентификатор сессии (в реальности приходит из INIT).
        let session_id = "123456";

        // Имитируем типичное сообщение расширения PROTOCOL_ERROR.
        // Это сценарий, когда расширение поймало рассинхрон последовательности директив.
        let json_body = r#"{ "error_message": "DIRECTIVE_ID has gap. expected=5 received=7" }"#;

        let mut ctx = ExtensionMessageContext::new();

        // --- Act ---
        let res = ctx.handle_extension_message_request(json_body, session_id);
        assert!(res.is_ok(), "Ожидали Ok(()), получено: {:?}", res);

        // --- Print ---
        // Смотри вывод через `cargo test -- --nocapture`.
        let report = report::work_report().unwrap();

        _assert_has_hbt_brackets(&report, session_id);
        writln!("\n{}", report);
    }   // just_a_run()

}   // mod tests
