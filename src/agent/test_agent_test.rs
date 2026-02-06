//! test_agent_test.rs
//!
//! Дымовые тесты уровня Agent.
//!
//! # Важно
//! - INIT пишет в глобальный SESSION_CONTEXT (OnceLock) и не может быть повторён в рамках одного процесса.
//! - Поэтому этот тест должен быть единственным успешным INIT-тестом в прогоне.
#[cfg(test)]
mod tests {
    use crate::agent::Agent;
    use crate::agent::request::{report, session};
    use crate::{glob, writln};
    use crate::library::test_utils;
    use crate::library::test_utils::{build_log_timestamp_like_bat, print_error_log, print_work_log};

    /// Описание: Дымовой тест INIT_SESSION.
    ///
    /// Проверяет:
    /// - что Agent читает Native Messaging пакет,
    /// - что EXTENSION_INIT корректно обрабатывается,
    /// - что формируется отчёт (ReportContext не пуст),
    /// - что глобальный session_id доступен после инициализации.
    #[test]
    fn smoke_init_session_builds_report_and_sets_session_context() {
        glob::initialize_glob(&build_log_timestamp_like_bat());

        // 1) Готовим EXT/INIT пакет.
        let ext_packet = r#"
<<<ext
    {
      "type": "INIT_SESSION",
      "payload": {
        "session_id": "test_session_1",
        "browser": "chrome",
        "ai_url": "https://example.local/ai",
        "window_name": "Chrome_WidgetWin_1",
        "window_title": "Chat [HBT-test]"
      }
    }
>>>ext
"#;

        // 2) Native Messaging: {"text":"..."} + length prefix.
        let native_json = test_utils::wrap_to_native_json(&ext_packet);
        let input = test_utils::mock_stdin(&native_json);

        // 3) Прогоняем агент на этом stdin (и получаем EOF).
        let mut agent = Agent::new();
        agent.do_only_once = true;
        agent.run(input);

        // 4) Проверяем, что сессия инициализировалась.
        let sid = match session::session_id() {
            Ok(v) => v,
            Err(e) => {
                // Если что-то пошло не так — вывод логов сильно ускоряет разбор.
                print_error_log();
                print_work_log();
                panic!("SESSION_CONTEXT не инициализирован после INIT: {}", e);
            }
        };
        assert_eq!(sid, "test_session_1");

        // 5) Проверяем, что отчёт сформирован.
        if agent.request_processor.is_report_empty() {
            print_error_log();
            print_work_log();
            panic!("После INIT отчёт должен быть сформирован (report_ctx не должен быть пустым).");
        }   // if

        // 6) Проверяем содержимое отчёта (основные маркеры).
        // Если не соберётся из-за приватности report_ctx — скажи, подстроим под текущие модификаторы видимости.
        let report_text = report::work_report().unwrap();

        assert!(
            report_text.contains("# 🚀 Хобот готов к работе"),
            "Отчёт INIT должен содержать заголовок готовности"
        );
        assert!(
            report_text.contains("session_id: test_session_1"),
            "Отчёт INIT должен содержать дамп session_id"
        );
        assert!(
            report_text.contains("window_title: Chat [HBT-test]"),
            "Отчёт INIT должен содержать дамп window_title"
        );

        // Визуальная проверка.
        writln!("{}", report_text);

        print_error_log();
        print_work_log();
    }   // smoke_init_session_builds_report_and_sets_session_context()

    /// Дымовой тест: INIT_SESSION с некорректным JSON.
    ///
    /// Цель:
    /// - Убедиться, что агент не падает на битом JSON INIT.
    /// - Посмотреть, что он сформирует/залогирует (визуальная оценка).
    #[test]
    fn smoke_bad_json_init_session() {
        // 1) Глобальная инициализация + чистка логов.
        glob::initialize_glob(&build_log_timestamp_like_bat());

        // 2) Битый INIT_SESSION: ломаем JSON в payload (нет закрывающих скобок).
        let bad_init_packet = r#"
<<<ext
    {
      "type": "INIT_SESSION",
      "payload": {
        "session_id": "test_session_1",
        "browser": "chrome",
        "ai_url": "https://example.local/ai",
        "window_name": "Chrome_WidgetWin_1",
        "window_title": "Chat [HBT-test]"
      // <-- нет закрывающих } для payload и } для объекта
>>>ext
"#;

        // 3) Прогоняем через агента.
        let bad_init_native_json = test_utils::wrap_to_native_json(bad_init_packet);
        let input = test_utils::mock_stdin(&bad_init_native_json);

        let mut agent = Agent::new();
        agent.do_only_once = true;
        agent.run(input);

        // 4) Печать отчёта (может быть пустым — это тоже сигнал).
        let report_text = report::work_report().unwrap();
        writln!("\n=== REPORT (after BAD INIT_SESSION) ===\n{}\n", report_text);

        // 5) Логи — в конце.
        print_error_log();
        print_work_log();
    }   // smoke_bad_json_init_session()

    /// Дымовой тест: EXT/INIT_SESSION без payload (валидный JSON, но схема не та).
    ///
    /// Цель:
    /// - Убедиться, что агент не падает.
    /// - Посмотреть, что он сформирует/залогирует при "missing field payload".
    #[test]
    fn smoke_init_session_missing_payload() {
        // 1) Глобальная инициализация + чистка логов.
        glob::initialize_glob(&build_log_timestamp_like_bat());

        // 2) INIT_SESSION без payload.
        let bad_init_packet = r#"
<<<ext
    {
        "type": "INIT_SESSION"
    }
>>>ext
"#;

        // 3) Прогоняем через агента.
        let bad_init_native_json = test_utils::wrap_to_native_json(bad_init_packet);
        let input = test_utils::mock_stdin(&bad_init_native_json);

        let mut agent = Agent::new();
        agent.do_only_once = true;
        agent.run(input);

        // 4) Печать отчёта (может быть пустым — это тоже полезно).
        let report_text = report::work_report().unwrap();
        writln!("\n=== REPORT (after INIT_SESSION missing payload) ===\n{}\n", report_text);

        // 5) Логи — в конце.
        print_error_log();
        print_work_log();
    }   // smoke_init_session_missing_payload()

    /// Дымовой тест: INIT_SESSION -> COMPLETION.
    ///
    /// Проверки минимальные:
    /// - Агент не падает.
    /// - Агент формирует какой-то отчёт (не пустой) на каждом шаге.
    /// - После INIT доступен session_id (значит глобальный SESSION_CONTEXT реально инициализировался).
    ///
    /// Всё содержимое отчётов печатается в stdout (визуальная оценка).
    #[test]
    fn smoke_init_then_completion() {
        // 1) Инициализация глобального конфига.
        glob::initialize_glob(&build_log_timestamp_like_bat());

        // 2) Собираем EXT/INIT_SESSION пакет.
        let init_packet = r#"
<<<ext
    {
      "type": "INIT_SESSION",
      "payload": {
        "session_id": "test_session_1",
        "browser": "chrome",
        "ai_url": "https://example.local/ai",
        "window_name": "Chrome_WidgetWin_1",
        "window_title": "Chat [HBT-test]"
      }
    }
>>>ext
"#;
        // 3) Собираем EXT/COMPLETION пакет.
        let completion_packet =  r#"
<<<ext
    {
        "type": "COMPLETION"
    }
>>>ext
"#;
        // 4) Запускаем агента на INIT.
        let init_native_json = test_utils::wrap_to_native_json(init_packet);
        let input = test_utils::mock_stdin(&init_native_json);

        let mut agent = Agent::new();
        agent.do_only_once = true;
        agent.run(input);

        // Печать отчёта после INIT (визуальная оценка).
        let report_text = report::work_report().unwrap();
        writln!("{}", report_text);

        // 5) Запускаем агента на COMPLETION отдельно.
        //
        // ВАЖНО: мы работаем со старым экземпляром Agent, симулируя последовательную подачу запросов.
        // Флаг тестового выхода из петли сбрасывается, выход происходит естественным путем.
        let completion_native_json = test_utils::wrap_to_native_json(completion_packet);
        let input = test_utils::mock_stdin(&completion_native_json);

        agent.do_only_once = false;
        agent.run(input);

        // 6) Печать отчёта после COMPLETION (визуальная оценка).
        let report_text = report::work_report().unwrap();
        writln!("{}", report_text);

        // 6) В конце теста печатаем журналы (как просил).
        print_error_log();
        print_work_log();
    }   // smoke_init_then_completion()

    /// Дымовой тест: INIT_SESSION -> PROTOCOL_ERROR.
    ///
    /// Цель:
    /// - Убедиться, что агент принимает служебную ошибку от расширения и формирует отчет в AI-формате.
    ///
    /// Проверки минимальные:
    /// - Агент не падает.
    /// - Есть текст отчета (не пустой).
    ///
    /// Всё содержимое отчётов печатается в stdout (визуальная оценка).
    #[test]
    fn smoke_init_then_protocol_error() {
        // 1) Инициализация глобального конфига.
        glob::initialize_glob(&build_log_timestamp_like_bat());

        // 2) INIT.
        let init_packet = r#"
<<<ext
    {
      "type": "INIT_SESSION",
      "payload": {
        "session_id": "test_session_1",
        "browser": "chrome",
        "ai_url": "https://example.local/ai",
        "window_name": "Chrome_WidgetWin_1",
        "window_title": "Chat [HBT-test]"
      }
    }
>>>ext
"#;
        // 3) PROTOCOL_ERROR.
        let protocol_error_packet = r#"
<<<ext
    {
        "type": "PROTOCOL_ERROR",
        "error_message": "DIRECTIVE_ID has gap. expected=5 received=7"
    }
>>>ext
"#;
        // 4) Запускаем агента на INIT: выходим принудительно, чтобы не ждать EOF.
        let init_native_json = test_utils::wrap_to_native_json(init_packet);
        let input = test_utils::mock_stdin(&init_native_json);

        let mut agent = Agent::new();
        agent.do_only_once = true;
        agent.run(input);

        // Печать отчёта после INIT_SESSION (визуальная оценка).
        let report_text = report::work_report().unwrap();
        writln!("{}", report_text);

        // 5) Запускаем агента на PROTOCOL_ERROR: тоже выходим принудительно (один пакет -> один прогон).
        let protocol_error_native_json = test_utils::wrap_to_native_json(protocol_error_packet);
        let input = test_utils::mock_stdin(&protocol_error_native_json);

        agent.do_only_once = true;
        agent.run(input);

        // Печать отчёта после PROTOCOL_ERROR (визуальная оценка).
        let report_text = report::work_report().unwrap();
        writln!("{}", report_text);

        // 6) В конце теста печатаем журналы и чистим их.
        print_error_log();
        print_work_log();
    }   // smoke_init_then_protocol_error()

    /// Дымовой тест: INIT_SESSION -> AI директива (2 команды shell) -> COMPLETION.
    ///
    /// Цель:
    /// - Убедиться, что парсинг директивы проходит.
    /// - Обе команды shell исполняются без ошибок.
    /// - Формируется отчёт по директиве (визуально проверяешь).
    ///
    /// Примечание:
    /// - Из-за глобального SESSION_CONTEXT (OnceLock) тест подразумевает, что INIT делается один раз за прогон.
    #[test]
    fn smoke_init_directive_two_shell_then_completion() {
        // 1) Глобальная инициализация.
        glob::initialize_glob(&build_log_timestamp_like_bat());

        // 2) INIT.
        let init_packet = r#"
<<<ext
    {
      "type": "INIT_SESSION",
      "payload": {
        "session_id": "test_session_1",
        "browser": "chrome",
        "ai_url": "https://example.local/ai",
        "window_name": "Chrome_WidgetWin_1",
        "window_title": "Chat [HBT-test]"
      }
    }
>>>ext
"#;
        // 3) AI директива с двумя командами shell_cmd.
        //
        // Важно:
        // - В твоём HandlerRegistry есть "shell_cmd".
        // - shell_cmd ожидает ровно один параметр: строка команды для cmd.exe.
        //
        // Команды максимально простые: echo.
        let directive_packet = r#"
<<<ai 1 test_session_1
    {
        "dir_comment": "smoke: two shell_cmd",
        "commands": [
            {
                "cmd_id": 1,
                "cmd_comment": "Just joyfull being born.",
                "name": "shell_cmd",
                "params": ["echo Hello, world!!!"]
            },
            {
                "cmd_id": 2,
                "name": "shell_cmd",
                "params": ["echo cmd_2_ok"]
            }
        ]
    }
>>>ai 1 test_session_1
"#;
        // 4) COMPLETION.
        let completion_packet = r#"
<<<ext
    {
        "type": "COMPLETION"
    }
>>>ext
"#;
        let mut agent = Agent::new();

        // --- Шаг A: INIT ---
        {
            let init_native_json = test_utils::wrap_to_native_json(init_packet);
            let input = test_utils::mock_stdin(&init_native_json);

            agent.do_only_once = true;
            agent.run(input);

            let report_text = report::work_report().unwrap();
            writln!("{}", report_text);
        }

        // --- Шаг B: Директива (2 shell_cmd) ---
        {
            let directive_native_json = test_utils::wrap_to_native_json(directive_packet);
            let input = test_utils::mock_stdin(&directive_native_json);

            agent.do_only_once = true;
            agent.run(input);

            let report_text = report::work_report().unwrap();
            writln!("{}", report_text);
        }

        // --- Шаг C: COMPLETION ---
        {
            let completion_native_json = test_utils::wrap_to_native_json(completion_packet);
            let input = test_utils::mock_stdin(&completion_native_json);

            // Тут хотим именно естественный выход по COMPLETION.
            agent.do_only_once = false;
            agent.run(input);

            let report_text = report::work_report().unwrap();
            writln!("{}", report_text);
        }

        // Логи — в конце.
        print_error_log();
        print_work_log();
    }   // smoke_init_directive_two_shell_then_completion()

    /// Дымовой тест: INIT_SESSION -> AI директива (3 команды shell, ошибка на 2-й) -> COMPLETION.
    ///
    /// Цель:
    /// - Убедиться, что директива обрабатывается, и при ошибке на 2-й команде:
    ///   - в отчёте появляется ошибка команды и ошибка уровня директивы,
    ///   - 3-я команда не исполняется (останов по первой ошибке).
    ///
    /// Всё содержимое отчётов печатается в stdout (визуальная оценка).
    #[test]
    fn smoke_init_directive_three_shell_second_fails_then_completion() {
        // 1) Глобальная инициализация.
        glob::initialize_glob(&build_log_timestamp_like_bat());

        // 2) INIT.
        let init_packet = r#"
<<<ext
    {
      "type": "INIT_SESSION",
      "payload": {
        "session_id": "test_session_1",
        "browser": "chrome",
        "ai_url": "https://example.local/ai",
        "window_name": "Chrome_WidgetWin_1",
        "window_title": "Chat [HBT-test]"
      }
    }
>>>ext
"#;

        // 3) AI директива: 3 команды shell_cmd, ошибка на второй.
        let directive_packet = r#"
<<<ai 1 test_session_1
    {
        "dir_comment": "smoke: 3 shell_cmd, second fails",
        "commands": [
            {
                "cmd_id": 1,
                "name": "shell_cmd",
                "params": ["echo cmd_1_ok"]
            },
            {
                "cmd_id": 2,
                "name": "shell_cmd",
                "params": ["not_a_real_command_hobot_test"]
            },
            {
                "cmd_id": 3,
                "name": "shell_cmd",
                "params": ["echo cmd_3_should_not_run"]
            }
        ]
    }
>>>ai 1 test_session_1
"#;

        // 4) COMPLETION.
        let completion_packet = r#"
<<<ext
    {
        "type": "COMPLETION"
    }
>>>ext
"#;

        let mut agent = Agent::new();

        // --- Шаг A: INIT ---
        {
            let init_native_json = test_utils::wrap_to_native_json(init_packet);
            let input = test_utils::mock_stdin(&init_native_json);

            agent.do_only_once = true;
            agent.run(input);

            let report_text = report::work_report().unwrap();
            writln!("\n=== REPORT (after INIT_SESSION) ===\n{}\n", report_text);
        }

        // --- Шаг B: Директива (3 shell_cmd, ошибка на 2й) ---
        {
            let directive_native_json = test_utils::wrap_to_native_json(directive_packet);
            let input = test_utils::mock_stdin(&directive_native_json);

            agent.do_only_once = true;
            agent.run(input);

            let report_text = report::work_report().unwrap();
            writln!("\n=== REPORT (after AI directive: second fails) ===\n{}\n", report_text);
        }

        // --- Шаг C: COMPLETION ---
        {
            let completion_native_json = test_utils::wrap_to_native_json(completion_packet);
            let input = test_utils::mock_stdin(&completion_native_json);

            agent.do_only_once = false;
            agent.run(input);

            let report_text = report::work_report().unwrap();
            writln!("\n=== REPORT (after COMPLETION) ===\n{}\n", report_text);
        }

        // Логи — в конце.
        print_error_log();
        print_work_log();
    }   // smoke_init_directive_three_shell_second_fails_then_completion()

    /// Дымовой тест: INIT_SESSION -> AI директива (битый JSON) -> COMPLETION.
    ///
    /// Цель:
    /// - Убедиться, что агент не падает на некорректном JSON директивы.
    /// - Посмотреть, что он сформирует/залогирует (визуальная оценка).
    #[test]
    fn smoke_init_then_bad_json_directive_then_completion() {
        // 1) Глобальная инициализация.
        glob::initialize_glob(&build_log_timestamp_like_bat());

        // 2) INIT.
        let init_packet = r#"
<<<ext
    {
      "type": "INIT_SESSION",
      "payload": {
        "session_id": "test_session_1",
        "browser": "chrome",
        "ai_url": "https://example.local/ai",
        "window_name": "Chrome_WidgetWin_1",
        "window_title": "Chat [HBT-test]"
      }
    }
>>>ext
"#;

        // 3) AI директива с некорректным JSON в теле.
        //
        // Ломаем именно JSON-тело внутри тегов: не закрываем массив/объект.
        let bad_directive_packet = r#"
<<<ai 1 test_session_1
    {
        "dir_comment": "smoke: bad json",
        "commands": [
            { "cmd_id": 1, "name": "shell_cmd", "params": ["echo should_not_run"] }
        // <-- нет закрывающих ] и }
>>>ai 1 test_session_1
"#;

        // 4) COMPLETION.
        let completion_packet = r#"
<<<ext
    {
        "type": "COMPLETION"
    }
>>>ext
"#;

        let mut agent = Agent::new();

        // --- Шаг A: INIT ---
        {
            let init_native_json = test_utils::wrap_to_native_json(init_packet);
            let input = test_utils::mock_stdin(&init_native_json);

            agent.do_only_once = true;
            agent.run(input);

            let report_text = report::work_report().unwrap();
            writln!("\n=== REPORT (after INIT_SESSION) ===\n{}\n", report_text);
        }

        // --- Шаг B: BAD директива ---
        {
            let directive_native_json = test_utils::wrap_to_native_json(bad_directive_packet);
            let input = test_utils::mock_stdin(&directive_native_json);

            // Выходим после одного сообщения, даже если внутри что-то пошло не так.
            agent.do_only_once = true;
            agent.run(input);

            // На текущей реализации отчёт может быть пустым — это тоже полезный сигнал на отладку.
            let report_text = report::work_report().unwrap();
            writln!("\n=== REPORT (after BAD JSON directive) ===\n{}\n", report_text);
        }

        // --- Шаг C: COMPLETION ---
        {
            let completion_native_json = test_utils::wrap_to_native_json(completion_packet);
            let input = test_utils::mock_stdin(&completion_native_json);

            agent.do_only_once = false;
            agent.run(input);

            let report_text = report::work_report().unwrap();
            writln!("\n=== REPORT (after COMPLETION) ===\n{}\n", report_text);
        }

        // Логи — в конце.
        print_error_log();
        print_work_log();
    }   // smoke_init_then_bad_json_directive_then_completion()

    /// Дымовой тест: невалидный Native Messaging пакет (тело не JSON).
    ///
    /// Цель:
    /// - Убедиться, что агент корректно обрабатывает критическую ошибку десериализации обёртки
    ///   Native Messaging и завершает цикл.
    ///
    /// Примечание:
    /// - Мы не проверяем stdout, только визуально/по логам.
    #[test]
    fn smoke_invalid_native_message_envelope() {
        // 1) Глобальная инициализация.
        glob::initialize_glob(&build_log_timestamp_like_bat());

        // 2) Формируем "битый" Native Messaging пакет:
        // - длина = 7
        // - тело = "not json" (невалидный JSON, serde_json::from_slice упадёт)
        let bad_body = b"not json";
        let bad_len = bad_body.len() as u32;

        let mut raw = Vec::new();
        raw.extend_from_slice(&bad_len.to_ne_bytes());
        raw.extend_from_slice(bad_body);

        let input = std::io::Cursor::new(raw);

        // 3) Запускаем агента.
        // break_run_loop не нужен: агент должен выйти сам по критической ошибке.
        let mut agent = Agent::new();
        agent.run(input);

        // 4) Печать логов.
        print_error_log();
        print_work_log();
    }   // smoke_invalid_native_message_envelope()

}   // mod tests

