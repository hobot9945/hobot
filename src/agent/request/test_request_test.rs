//! test_request_test
//!
//! Тесты для модуля обработки запросов (RequestProcessor).


/// Проверяет чтение сырых данных (Native Messaging), парсинг протокольных скобок и валидацию сессии.
#[cfg(test)]
mod _unwrap_brackets_tests {
    use crate::glob::error_control::AgentError;
    use crate::agent::request::{report, RequestProcessor, RequestSource};
    use crate::{glob, writln};
    use crate::agent::request::request_reader::RequestReader;
    use crate::library::test_utils::{mock_stdin, wrap_to_native_json};

    /// ВАЖНО: Сессия хранится в глобальном `OnceLock` и не сбрасывается между тестами,
    /// поэтому все тесты должны использовать один и тот же session_id.
    const TEST_SESSION_ID: &str = "sess_TEST_123";

    /// Общая часть для всех тестов - инициализация контекста сессии.
    fn _init_session(request_prc: &mut RequestProcessor) {

        let init_msg = format!(r#"
<<<ext
    {{
        "type": "INIT_SESSION",
        "payload": {{
            "session_id": "{sess}",
            "browser": "firefox",
            "ai_url": "https://example.com",
            "window_name": "Firefox",
            "window_title": "AI Chat [HBT-123]"
        }}
    }}
>>>ext"#, sess = TEST_SESSION_ID);
        request_prc.process_request(&init_msg).expect("Ошибка инициализации контекста сессии");
    }

    /// Проверяет полный цикл чтения и распаковки корректной директивы AI.
    ///
    /// # Сценарий
    /// 1. Создается валидная директива `<<<ai 10 sess_01 ... >>>ai 10 sess_01`.
    /// 2. Сообщение упаковывается в формат Native Messaging.
    /// 3. `read_raw_request` считывает сообщение из эмулятора stdin.
    /// 4. `_unwrap_brackets` парсит текст, извлекает тип и тело, заполняет контекст.
    ///
    /// # Ожидания
    /// - Чтение успешно.
    /// - Тип запроса: `RequestType::AiDirective`.
    /// - ID директивы (10) и SessionID (sess_01) корректно попали в `directive_ctx`.
    #[test]
    fn test_read_and_unwrap_ai_directive_success() {

        // 1. Подготовка данных
        // Валидная директива: ID=10, Session=sess_01, Тело={...}
        let raw_body = format!(r#"<<<ai 10 {sess}
    {{
        "commands": []
    }}
>>>ai 10 {sess}"#, sess = TEST_SESSION_ID);

        // Упаковываем в Native Messaging JSON
        let native_msg = wrap_to_native_json(&raw_body);
        let mut input = mock_stdin(&native_msg);

        // 2. Чтение сырого запроса (read_raw_request)
        let mut rdr = RequestReader::new();

        let read_result = rdr.read_next_request(&mut input);
        assert!(read_result.is_ok(), "Ошибка чтения сырого запроса");

        let raw_text = read_result.unwrap().expect("Ожидался текст, получен None (EOF)");
        assert_eq!(raw_text, raw_body, r#"Прочитанный текст не совпадает с отправленным:
        raw_text: {},
        raw_body: {}"#, raw_text, raw_body);

        // 3. Распаковка скобок (_unwrap_brackets)
        let mut request_prc = RequestProcessor::new();

        // Инициализируем сессию.
        _init_session(&mut request_prc);

        let unwrap_result = request_prc._unwrap_brackets(&raw_text);
        assert!(unwrap_result.is_ok(), "Ошибка распаковки валидной директивы: {:?}", unwrap_result.err());
        let (req_type, json_body, dir_header) = unwrap_result.unwrap();

        // 4. Проверки
        assert_eq!(req_type, RequestSource::Ai);
        assert_eq!(json_body, "{\n        \"commands\": []\n    }", "Тело JSON извлечено неверно");

        // Проверяем, что контекст заполнился метаданными
        assert_eq!(dir_header, Some((10, TEST_SESSION_ID.to_string())));
    }   // test_read_and_unwrap_ai_directive_success()

    /// Проверяет распаковку служебного сообщения расширения (`<<<ext ... >>>ext`).
    ///
    /// # Ожидания
    /// - Тип запроса: `RequestType::ExtensionInfo`.
    /// - Тело JSON извлекается без искажений.
    #[test]
    fn test_read_and_unwrap_ext_message_success() {
        let raw_body = r#"<<<ext {"type": "PING"} >>>ext"#;
        let native_msg = wrap_to_native_json(raw_body);
        let mut input = mock_stdin(&native_msg);

        let mut rdr = RequestReader::new();
        let raw_text = rdr.read_next_request(&mut input).unwrap().unwrap();

        let mut processor = RequestProcessor::new();
        let (req_type, json_body, dir_header) =
            processor._unwrap_brackets(&raw_text)
                .expect("Ошибка распаковки EXT");

        assert_eq!(req_type, RequestSource::Extension);
        assert_eq!(json_body, r#"{"type": "PING"}"#);
        let _ = dir_header; // dir_header не используется для EXT
    }   // test_read_and_unwrap_ext_message_success()

    /// Проверяет детектирование ошибки протокола: несовпадение ID директивы.
    ///
    /// # Сценарий
    /// - Открывающий тег: ID=10.
    /// - Закрывающий тег: ID=11.
    ///
    /// # Ожидания
    /// - Возвращается `AgentError::Recoverable`.
    #[test]
    fn test_unwrap_ai_mismatch_ids() {
        // Разные ID в начале и конце
        let raw_body = format!(r#"<<<ai 10 {sess} ... >>>ai 11 {sess}"#, sess = TEST_SESSION_ID);
        let mut request_prc = RequestProcessor::new();

        // Инициализируем сессию один раз через штатный процессинг INIT.
        let init_msg = format!(r#"
<<<ext
    {{
        "type": "INIT_SESSION",
        "payload": {{
            "session_id": "{sess}",
            "browser": "chrome",
            "ai_url": "https://ai",
            "window_name": "win",
            "window_title": "title"
        }}
    }}
>>>ext"#, sess = TEST_SESSION_ID);
        request_prc.process_request(&init_msg).expect("Init failed");

        let res = request_prc._unwrap_brackets(&raw_body);
        assert!(matches!(res, Err(AgentError::Recoverable(_))), "Должна быть ошибка несовпадения ID");
    }   // test_unwrap_ai_mismatch_ids()

    /// Проверяет детектирование ошибки протокола: несовпадение Session ID.
    ///
    /// # Сценарий
    /// - Открывающий тег: sess_1.
    /// - Закрывающий тег: sess_2.
    ///
    /// # Ожидания
    /// - Возвращается `AgentError::Recoverable`.
    #[test]
    fn test_unwrap_ai_mismatch_session() {
        // Разные сессии
        let raw_body = format!(r#"<<<ai 10 {sess_owner} ... >>>ai 10 sess_ALIEN"#, sess_owner = TEST_SESSION_ID);
        let mut request_prc = RequestProcessor::new();

        // Инициализируем сессию.
        _init_session(&mut request_prc);

        let res = request_prc._unwrap_brackets(&raw_body);
        assert!(matches!(res, Err(AgentError::Recoverable(_))), "Должна быть ошибка несовпадения SessionID");
    }   // test_unwrap_ai_mismatch_session()

    /// Проверяет механизм защиты от подмены сессии (Session Hijacking Protection).
    ///
    /// # Сценарий
    /// 1. Агент инициализируется сессией "sess_OWNER".
    /// 2. Приходит директива с сессией "sess_ALIEN".
    ///
    /// # Ожидания
    /// - Директива отвергается с ошибкой `AgentError::Recoverable`.
    /// - В тексте ошибки упоминается "Неверный SessionID".
    #[test]
    fn test_unwrap_session_validation_fail() {
        // Сценарий: Контекст уже инициализирован сессией "sess_OWNER",
        // а приходит директива с "sess_ALIEN".

        let mut request_prc = RequestProcessor::new();


        // Инициализируем сессию.
        _init_session(&mut request_prc);

        // Попытка распаковать свою директиву
        let raw_ok = format!(r#"<<<ai 5 {sess} {{}} >>>ai 5 {sess}"#, sess = TEST_SESSION_ID);
        let res = request_prc._unwrap_brackets(&raw_ok);
        assert!(res.is_ok(), "Своя директива (session_id = sess_OWNER) отвергнута как чужая.");

        // Попытка распаковать чужую директиву. Контекст уже инициализирован сессией "sess_OWNER",
        // а приходит директива с "sess_ALIEN".
        let raw_bad = r#"<<<ai 5 sess_ALIEN {} >>>ai 5 sess_ALIEN"#;
        let res = request_prc._unwrap_brackets(raw_bad);

        // Должна быть ошибка валидации сессии
        match res {
            Err(AgentError::Recoverable(msg)) => {
                assert!(
                    msg.contains("неверный SessionID") || msg.contains("несовпадение session_id"),
                    "Сообщение ошибки не соответствует ожиданиям: {}",
                    msg
                );
            },
            _ => panic!("Ожидалась ошибка валидации сессии"),
        }   // match
    }   // test_unwrap_session_validation_fail()

    /// Проверяет прием и обработку запросов от расширения и AI.
    #[cfg(test)]
    mod process_request_tests {
        #[allow(unused_imports)] use crate::wrln;
        use crate::agent::request::{report, RequestProcessor};
        use crate::agent::request::test_request_test::_unwrap_brackets_tests::_init_session;

        /// Единый SessionID для всех тестов модуля.
        ///
        /// ВАЖНО: Сессия хранится в глобальном `OnceLock` и не сбрасывается между тестами,
        /// поэтому все тесты должны использовать один и тот же session_id.
        const TEST_SESSION_ID: &str = "sess_TEST_123";

        /// Проверяет обработку сигнала COMPLETION от расширения.
        ///
        /// # Сценарий
        /// 1. Формируется сообщение расширения типа `COMPLETION`.
        /// 2. Вызывается `process_request`.
        ///
        /// # Ожидания
        /// - Метод возвращает `Ok(())`.
        /// - Флаг `is_hobot_completion_requested` устанавливается в `true`.
        #[test]
        fn test_extension_completion_signal() {
            // 1. Подготовка сообщения COMPLETION
            let raw_ext_msg = r#"<<<ext { "type": "COMPLETION" } >>>ext"#;

            let mut request_prc = RequestProcessor::new();

            // Инициализируем сессию.
            _init_session(&mut request_prc);

            // 2. Обработка запроса
            let result = request_prc.process_request(raw_ext_msg);

            // 3. Проверки
            assert!(result.is_ok(), "Обработка сигнала COMPLETION завершилась с ошибкой: {:?}", result);
            assert!(request_prc.is_hobot_completion_requested, "Флаг is_hobot_completion_requested не установлен");
        }   // test_process_completion_signal()

        /// Проверяет обработку сообщения инициализации (INIT).
        ///
        /// # Сценарий
        /// 1. Формируется сообщение расширения типа `INIT` с payload.
        /// 2. Вызывается `process_request`.
        ///
        /// # Ожидания
        /// - `init_ctx` успешно инициализируется переданными данными.
        /// - SessionID сохраняется корректно.
        #[test]
        fn test_extension_init_signal() {

            // 2. Обработка запроса
            let mut request_prc = RequestProcessor::new();

            // Инициализируем сессию.
            _init_session(&mut request_prc);

            // Проверяем, что сформирован сервисный отчет INIT
            assert!(!report::is_empty().unwrap(), "Отчет по инициализации сессии не сформирован (Report пуст)");
            let report = report::text().unwrap();

            // Проверяем, что глобальная сессия реально инициализирована
            let sid = crate::agent::request::session::session_id()
                .expect("SESSION_CONTEXT не инициализирован после INIT");
            assert_eq!(sid, TEST_SESSION_ID, "SessionID сохранен неверно");
        }   // test_process_init_signal()

        /// Проверяет обработку ошибки протокола (PROTOCOL_ERROR).
        ///
        /// # Сценарий
        /// 1. Приходит сообщение `PROTOCOL_ERROR` от расширения в текстовом формате.
        /// 2. Вызывается `process_request`.
        ///
        /// # Ожидания
        /// - Обработка проходит успешно.
        /// - Сообщение об ошибке попадает в `extension_error_ctx`.
        #[test]
        fn test_extension_protocol_error_signal() {
            // 1. Подготовка сообщения PROTOCOL_ERROR
            let raw_ext_msg = r#"
<<<ext
    {
        "type": "PROTOCOL_ERROR",
        "error_message": "Sequence mismatch: expected 5, received 6"
    }
>>>ext"#;

            // 2. Обработка запроса
            let mut request_prc = RequestProcessor::new();

            // Инициализируем сессию.
            _init_session(&mut request_prc);

            let result = request_prc.process_request(raw_ext_msg);

            // 3. Проверки
            assert!(result.is_ok(), "Обработка PROTOCOL_ERROR завершилась с ошибкой: {:?}", result);

            // Проверяем, что сформирован отчет для отправки в AI.
            assert!(!report::is_empty().unwrap(), "Отчет не сформирован (ReportContext пуст)");

            let report = report::text().unwrap();
            let report = report.trim();

            assert!(report.starts_with(&format!("<<<hbt {}", TEST_SESSION_ID)),
                    "Отчет должен начинаться с <<<hbt SESSION_ID");
            assert!(report.contains("Ошибка протокола расширения"), "Должен быть заголовок отчета");
            assert!(
                report.contains("Sequence mismatch: expected 5, received 6"),
                "Должна быть полезная нагрузка error_message"
            );
            assert!(report.ends_with(&format!(">>>hbt {}", TEST_SESSION_ID)),
                    "Отчет должен заканчиваться >>>hbt SESSION_ID");
        }   // test_process_protocol_error_signal()
    }   // mod process_request_tests

    /// Дымовой тест обработки сообщения COMPLETION.
    ///
    /// # Примечания
    /// Для отображения вывода теста используй запуск: `cargo test -- --nocapture`.
    #[test]
    fn just_a_run() {

        // ВАЖНО: Для формирования completion-отчета требуется инициализированная сессия,
        // так как отчет должен быть в тегах `<<<hbt SESSION_ID ... >>>hbt SESSION_ID`.
        let init_msg = format!(r#"
<<<ext
    {{
        "type": "{init}",
        "payload": {{
            "session_id": "{sess}",
            "browser": "firefox",
            "ai_url": "https://example.com",
            "window_name": "Firefox",
            "window_title": "AI Chat [HBT-123]"
        }}
    }}
>>>ext"#, init = glob::EXT_MSG_TYPE_INIT_SESSION, sess = TEST_SESSION_ID);

        let mut request_prc = RequestProcessor::new();

        // 2. Обработка запроса
        let mut request_prc = RequestProcessor::new();

        let init_res = request_prc.process_request(&init_msg);
        assert!(init_res.is_ok(), "Ошибка инициализации SESSION_CONTEXT: {:?}", init_res);

        // Эмулируем COMPLETION от расширения.
        let completion_msg = r#"<<<ext { "type": "COMPLETION" } >>>ext"#;
        let completion_res = request_prc.process_request(completion_msg);
        assert!(completion_res.is_ok(), "Ошибка при обработке запроса COMPLETION: {:?}", completion_res);
        assert!(
            request_prc.is_hobot_completion_requested,
            "Флаг is_hobot_completion_requested должен быть установлен после COMPLETION"
        );

        // Если отчет строится в process_request (через _build_completion_report), выводим его руками.
        let report = report::text().unwrap();
        if !report.is_empty() {
            let report = report.trim();
            assert!(
                report.starts_with(&format!("<<<hbt {}", TEST_SESSION_ID)),
                "Отчет COMPLETION должен начинаться с <<<hbt SESSION_ID"
            );
            assert!(
                report.ends_with(&format!(">>>hbt {}", TEST_SESSION_ID)),
                "Отчет COMPLETION должен заканчиваться >>>hbt SESSION_ID"
            );

            writln!("\n{}", report);
        }   // if
    }   // just_a_run()

}   // mod _unwrap_brackets_tests
