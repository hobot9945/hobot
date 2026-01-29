#[cfg(test)]
mod tests {
    use std::{fs};

    use crate::agent::Agent;
    use crate::glob::initialize_glob;
    use crate::library::test_utils::{mock_stdin, print_error_log, print_work_log, wrap_to_native_json};

    //----------------------------------------------------------------------------------------------
    //                  Общие настройки тестов (легкое управление)
    // todo при вставке текста в поле ввода AI теряются переводы строк.
    //----------------------------------------------------------------------------------------------

    const SESSION_ID: &str = "6BF260";
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

    /// Описание: Вычисляет `window_title` по умолчанию из общих констант.
    fn window_title() -> String {
        format!("{} [{}]", AI_URL, SESSION_ID)
    }   // window_title()

    /// Описание: Best-effort очистка `work.log` и `error.log`.
    fn cleanup_logs() {
        let _ = fs::remove_file(&crate::glob::config().worklog_path);
        let _ = fs::remove_file(&crate::glob::config().errlog_path);
    }   // cleanup_logs()

    /// Описание: EXT INIT_SESSION пакет.
    fn build_init_session_packet() -> String {
        let win_title = window_title();

        format!(r##"
<<<ext
    {{
        "type": "INIT_SESSION",
        "payload":
        {{
            "session_id": "{session_id}",
            "browser": "{browser}",
            "ai_url": "{ai_url}",
            "window_title": "{window_title}"
        }}
    }}
>>>ext
"##,
                session_id = SESSION_ID,
                browser = BROWSER,
                ai_url = AI_URL,
                window_title = win_title
        )
    }   // build_init_session_ext()

    /// Описание: EXT COMPLETION пакет.
    fn build_completion_packet() -> &'static str {
        r##"
<<<ext
    {
        "type": "COMPLETION"
    }
>>>ext
"##
    }   // build_completion_ext()

    /// Описание: Прогоняет агент на одном EXT-пакете `INIT_SESSION`.
    ///
    /// # Алгоритм работы
    /// - Инициализирует `glob`.
    /// - Чистит `work.log`/`error.log` (best effort).
    /// - Отправляет в `Agent::run()` один EXT запрос INIT_SESSION через `mock_stdin`.
    /// - Печатает логи для анализа.
    ///
    /// # Побочные эффекты
    /// - Удаляет `work.log`/`error.log` (если они существуют).
    /// - Создает/перезаписывает логи в зависимости от конфига.
    /// - Пишет диагностический вывод в stdout (через `print_*_log()`).
    #[ignore]
    #[test]
    fn run_init_packet() {
        initialize_glob().expect("Failed to initialize glob");
        cleanup_logs();

        // EXT INIT_SESSION
        let directive = build_init_session_packet();

        let directive = wrap_to_native_json(&directive);
        let _ = Agent::new().run(mock_stdin(&directive));

        print_error_log();
        print_work_log();
    }   // run_init_packet()

    /// Описание: Прогоняет агент на одной AI-директиве БЕЗ предварительного `INIT_SESSION`.
    ///
    /// # Цель теста
    /// Проверить, что агент корректно отрабатывает сценарий “директива пришла без инициализации сессии”
    /// (ожидается ошибка валидации сессии/контекста, запись в error.log и/или отчёт в work.log).
    ///
    /// # Алгоритм работы
    /// - Инициализирует `glob`.
    /// - Чистит `work.log`/`error.log` (best effort).
    /// - Отправляет в `Agent::run()` одну AI директиву.
    /// - Печатает логи для анализа.
    ///
    /// # Побочные эффекты
    /// - Удаляет `work.log`/`error.log` (если они существуют).
    /// - Создает/перезаписывает логи в зависимости от конфига.
    /// - Пишет диагностический вывод в stdout (через `print_*_log()`).
    #[ignore]
    #[test]
    fn run_directive() {
        initialize_glob().expect("Failed to initialize glob");
        cleanup_logs();

        // AI директива без INIT_SESSION (session_id берём из общих констант для удобства).
        let directive = format!(r##"
<<<ai 123 {session_id}
    {{
        "dir_comment": "Комментарий директивы",
        "commands": [
            {{
                "cmd_comment": "Комментарий команды",
                "cmd_id": 1,
                "name": "shell_cmd",
                "params": [
                    "echo hello"
                ]
            }},
            {{
                "cmd_comment": "Пример несуществующей команды",
                "cmd_id": 2,
                "name": "no_commmand",
                "params": [
                    "echo hello"
                ]
            }}
        ]
    }}
>>>ai 123 {session_id}
"##, session_id = SESSION_ID);

        let directive = wrap_to_native_json(&directive);
        let _ = Agent::new().run(mock_stdin(&directive));

        print_error_log();
        print_work_log();
    }   // just_a_run_directive()

    /// Описание: Прогоняет агент на связке `INIT_SESSION` + `COMPLETION`.
    ///
    /// # Алгоритм работы
    /// - Инициализирует `glob`.
    /// - Чистит `work.log`/`error.log` (best effort).
    /// - Отправляет в `Agent::run()` пакет, содержащий:
    ///   - EXT `INIT_SESSION` (чтобы инициализировать session-контекст),
    ///   - EXT `COMPLETION` (штатное завершение работы агента).
    /// - Печатает логи для анализа.
    ///
    /// # Побочные эффекты
    /// - Удаляет `work.log`/`error.log` (если они существуют).
    /// - Создает/перезаписывает логи в зависимости от конфига.
    /// - Пишет диагностический вывод в stdout (через `print_*_log()`).
    #[ignore]
    #[test]
    fn run_completion() {
        initialize_glob().expect("Failed to initialize glob");
        cleanup_logs();

        // EXT INIT_SESSION + EXT COMPLETION
        let directive = format!("{}{}", build_init_session_packet(), build_completion_packet());

        let directive = wrap_to_native_json(&directive);
        let _ = Agent::new().run(mock_stdin(&directive));

        print_error_log();
        print_work_log();
    }   // just_a_run_complete()}   // tests

    /// Описание: Прогоняет агент на связке `INIT_SESSION` + `AI директива`.
    ///
    /// # Алгоритм работы
    /// - Инициализирует `glob`.
    /// - Чистит `work.log`/`error.log` (best effort).
    /// - Отправляет в `Agent::run()` один пакет, содержащий:
    ///   - EXT `INIT_SESSION`,
    ///   - AI директиву с двумя командами:
    ///     1) валидная `shell_cmd`,
    ///     2) несуществующая команда (`no_commmand`) для проверки ошибки уровня команды/директивы.
    /// - Печатает логи для анализа.
    ///
    /// # Побочные эффекты
    /// - Удаляет `work.log`/`error.log` (если они существуют).
    /// - Создает/перезаписывает логи в зависимости от конфига.
    /// - Пишет диагностический вывод в stdout (через `print_*_log()`).
    #[ignore]
    #[test]
    fn run_init_and_directive() {
        initialize_glob().expect("Failed to initialize glob");
        cleanup_logs();

        // EXT INIT_SESSION + AI директива
        let init = build_init_session_packet();

        // ВАЖНО: session_id в тегах AI должен совпадать с INIT_SESSION.
        let ai = format!(r##"
<<<ai 123 {session_id}
    {{
        "dir_comment": "Комментарий директивы",
        "commands": [
            {{
                "cmd_comment": "Комментарий команды",
                "cmd_id": 1,
                "name": "shell_cmd",
                "params": [
                    "echo hello"
                ]
            }},
            {{
                "cmd_comment": "Пример несуществующей команды",
                "cmd_id": 2,
                "name": "no_commmand",
                "params": [
                    "echo hello"
                ]
            }}
        ]
    }}
>>>ai 123 {session_id}
"##, session_id = SESSION_ID);

        let directive = format!("{}{}", init, ai);

        let directive = wrap_to_native_json(&directive);
        let _ = Agent::new().run(mock_stdin(&directive));

        print_error_log();
        print_work_log();
    }   // run_init_and_directive()

    /// Описание: Дымовой тест команды `get_monitor_layout` — получает раскладку мониторов
    /// (logical -> physical + геометрия) и вставляет отчёт в поле ввода AI.
    ///
    /// # Предусловия
    /// - Окно AI с заголовком `window_title` реально существует.
    /// - Поле ввода AI в этом окне готово принимать вставку (в идеале — курсор уже стоит в поле).
    /// - В `config.toml` не включен `is_log_only`, иначе выполнение команд не произойдет.
    ///
    /// # Алгоритм работы
    /// - Инициализирует `glob`.
    /// - Чистит `work.log`/`error.log` (best effort).
    /// - Посылает INIT_SESSION.
    /// - Посылает AI-директиву #1 с командой `get_monitor_layout` (без параметров).
    /// - Посылает COMPLETION для штатного завершения цикла `Agent::run()`.
    /// - Печатает логи для ручного анализа.
    ///
    /// # Побочные эффекты
    /// - Переводит фокус на окно AI (по `window_title`) и вставляет текст отчёта (Ctrl+V),
    ///   так как агент всегда отправляет отчёты в UI.
    /// - Создает/перезаписывает `work.log` и `error.log`.
    #[ignore]
    #[test]
    fn get_monitor_layout_to_ai_input() {
        initialize_glob().expect("Failed to initialize glob");
        cleanup_logs();

        let init = build_init_session_packet();

        // ВАЖНО: номер директивы = 1, session_id должен совпадать с INIT_SESSION.
        let ai = format!(r##"
<<<ai 1 {session_id}
    {{
        "dir_comment": "smoke: get_monitor_layout -> paste into AI input",
        "commands": [
            {{
                "cmd_comment": "Получить раскладку мониторов (logical->physical + геометрия)",
                "cmd_id": 1,
                "name": "get_monitor_layout"
            }}
        ]
    }}
>>>ai 1 {session_id}
"##, session_id = SESSION_ID);

        let directive = format!("{}{}{}", init, ai, build_completion_packet());

        let directive = wrap_to_native_json(&directive);
        let _ = Agent::new().run(mock_stdin(&directive));

        print_error_log();
        print_work_log();
    }   // smoke_get_monitor_layout_to_ai_input()

    /// Описание: Дымовой тест команды `screenshot_all_monitors` — делает полный скриншот всех мониторов
    /// и вставляет его в поле ввода AI (через clipboard + Ctrl+V).
    ///
    /// # Предусловия
    /// - Окно AI с заголовком `window_title` реально существует.
    /// - Поле ввода AI в этом окне готово принимать вставку (в идеале — курсор уже стоит в поле).
    /// - В `config.toml` не включен `is_log_only`, иначе выполнение команд не произойдет.
    ///
    /// # Алгоритм работы
    /// - Инициализирует `glob`.
    /// - Чистит `work.log`/`error.log` (best effort).
    /// - Посылает INIT_SESSION.
    /// - Посылает AI-директиву #1 с командой `screenshot_all_monitors`.
    /// - Посылает COMPLETION для штатного завершения цикла `Agent::run()`.
    /// - Печатает логи для ручного анализа.
    ///
    /// # Побочные эффекты
    /// - Перезаписывает системный буфер обмена (сначала изображением, затем текстовым отчётом).
    /// - Переводит фокус на окно AI (по `window_title`).
    /// - Генерирует события клавиатуры (Ctrl+V).
    /// - Создает/перезаписывает `work.log` и `error.log`.
    #[ignore]
    #[test]
    fn capture_virtual_screen_to_ai_input() {
        initialize_glob().expect("Failed to initialize glob");
        cleanup_logs();

        let init = build_init_session_packet();

        // ВАЖНО: номер директивы = 1, session_id должен совпадать с INIT_SESSION.
        let ai = format!(r##"
<<<ai 1 {session_id}
    {{
        "dir_comment": "smoke: screenshot_all_monitors -> paste into AI input",
        "commands": [
            {{
                "cmd_comment": "Снять скриншот всех мониторов и вставить в окно AI",
                "cmd_id": 1,
                "name": "capture_virtual_screen"
            }}
        ]
    }}
>>>ai 1 {session_id}
"##, session_id = SESSION_ID);

        let directive = format!("{}{}{}", init, ai, build_completion_packet());

        let directive = wrap_to_native_json(&directive);
        let _ = Agent::new().run(mock_stdin(&directive));

        print_error_log();
        print_work_log();
    }   // smoke_screenshot_all_monitors_to_ai_input()

    /// Описание: Дымовой тест команды `screenshot_monitor` — делает скриншот монитора по логическому
    /// индексу и вставляет его в поле ввода AI (через clipboard + Ctrl+V).
    ///
    /// # Предусловия
    /// - Окно AI с заголовком `window_title` реально существует.
    /// - Поле ввод�� AI в этом окне готово принимать вставку (в идеале — курсор уже стоит в поле).
    /// - В `config.toml` не включен `is_log_only`, иначе выполнение команд не произойдет.
    ///
    /// # Алгоритм работы
    /// - Инициализирует `glob`.
    /// - Чистит `work.log`/`error.log` (best effort).
    /// - Посылает INIT_SESSION.
    /// - Посылает AI-директиву #1 с командой `screenshot_monitor` и параметром "0" (первый монитор).
    /// - Посылает COMPLETION для штатного завершения цикла `Agent::run()`.
    /// - Печатает логи для ручного анализа.
    ///
    /// # Побочные эффекты
    /// - Перезаписывает системный буфер обмена (сначала изображением, затем текстовым отчётом).
    /// - Переводит фокус на окно AI (по `window_title`).
    /// - Генерирует события клавиатуры (Ctrl+V).
    /// - Создает/перезаписывает `work.log` и `error.log`.
    #[ignore]
    #[test]
    fn capture_monitor_to_ai_input() {
        initialize_glob().expect("Failed to initialize glob");
        cleanup_logs();

        let init = build_init_session_packet();

        // ВАЖНО: номер директивы = 1, session_id должен совпадать с INIT_SESSION.
        let ai = format!(r##"
<<<ai 1 {session_id}
    {{
        "dir_comment": "smoke: screenshot_monitor -> paste into AI input",
        "commands": [
            {{
                "cmd_comment": "Снять скриншот монитора 0 и вставить в окно AI",
                "cmd_id": 1,
                "name": "capture_monitor",
                "params": ["1"]
            }}
        ]
    }}
>>>ai 1 {session_id}
"##, session_id = SESSION_ID);

        let directive = format!("{}{}{}", init, ai, build_completion_packet());

        let directive = wrap_to_native_json(&directive);
        let _ = Agent::new().run(mock_stdin(&directive));

        print_error_log();
        print_work_log();
    }   // smoke_screenshot_monitor_to_ai_input()

    /// Описание: Дымовой тест команды `get_foreground_window_info` — возвращает HWND и заголовок
    /// текущего foreground-окна и вставляет отчёт в поле ввода AI.
    ///
    /// # Предусловия
    /// - Окно AI с заголовком `window_title` реально существует.
    /// - Поле ввода AI в этом окне готово принимать вставку.
    ///
    /// # Побочные эффекты
    /// - Переводит фокус на окно AI и вставляет текст отчёта (Ctrl+V).
    /// - Создает/перезаписывает `work.log` и `error.log`.
    #[ignore]
    #[test]
    fn get_foreground_window_info_to_ai_input() {
        initialize_glob().expect("Failed to initialize glob");
        cleanup_logs();

        let init = build_init_session_packet();

        let ai = format!(r##"
<<<ai 1 {session_id}
    {{
        "dir_comment": "smoke: get_foreground_window_info -> paste into AI input",
        "commands": [
            {{
                "cmd_comment": "Получить HWND и заголовок foreground-окна",
                "cmd_id": 1,
                "name": "get_foreground_window_info"
            }}
        ]
    }}
>>>ai 1 {session_id}
"##, session_id = SESSION_ID);

        let directive = format!("{}{}{}", init, ai, build_completion_packet());

        let directive = wrap_to_native_json(&directive);
        let _ = Agent::new().run(mock_stdin(&directive));

        print_error_log();
        print_work_log();
    }   // smoke_get_foreground_window_info_to_ai_input()

    /// Описание: Дымовой тест команды `find_window_info` — ищет окно по подстроке заголовка (needle)
    /// и вставляет отчёт с HWND и полным заголовком в поле ввода AI.
    ///
    /// # Предусловия
    /// - Окно AI с заголовком `window_title` реально существует.
    /// - Окно-цель с подстрокой `WINDOW_NEEDLE` в заголовке существует.
    /// - Поле ввода AI готово принимать вставку.
    ///
    /// # Побочные эффекты
    /// - Переводит фокус на окно AI и вставляет текст отчёта (Ctrl+V).
    /// - Создает/перезаписывает `work.log` и `error.log`.
    #[ignore]
    #[test]
    fn find_window_info_to_ai_input() {
        initialize_glob().expect("Failed to initialize glob");
        cleanup_logs();

        let init = build_init_session_packet();

        let ai = format!(r##"
<<<ai 1 {session_id}
    {{
        "dir_comment": "smoke: find_window_info -> paste into AI input",
        "commands": [
            {{
                "cmd_comment": "Найти окно по needle и вернуть HWND + title",
                "cmd_id": 1,
                "name": "find_window_info",
                "params": ["{needle}"]
            }}
        ]
    }}
>>>ai 1 {session_id}
"##, session_id = SESSION_ID, needle = WINDOW_NEEDLE);

        let directive = format!("{}{}{}", init, ai, build_completion_packet());

        let directive = wrap_to_native_json(&directive);
        let _ = Agent::new().run(mock_stdin(&directive));

        print_error_log();
        print_work_log();
    }   // smoke_find_window_info_to_ai_input()

    /// Описание: Дымовой тест команды `capture_region` — снимает скриншот прямоугольной области
    /// виртуального рабочего стола и вставляет его в поле ввода AI.
    ///
    /// # Предусловия
    /// - Окно AI с заголовком `window_title` реально существует.
    /// - Поле ввода AI готово принимать вставку изображения.
    ///
    /// # Побочные эффекты
    /// - Перезаписывает системный буфер обмена.
    /// - Переводит фокус на окно AI.
    /// - Генерирует Ctrl+V.
    /// - Создает/перезаписывает `work.log` и `error.log`.
    #[ignore]
    #[test]
    fn capture_region_to_ai_input() {
        initialize_glob().expect("Failed to initialize glob");
        cleanup_logs();

        let init = build_init_session_packet();

        let ai = format!(r##"
<<<ai 1 {session_id}
    {{
        "dir_comment": "smoke: capture_region -> paste into AI input",
        "commands": [
            {{
                "cmd_comment": "Снять скриншот области и вставить в окно AI",
                "cmd_id": 1,
                "name": "capture_region",
                "params": ["{x}", "{y}", "{width}", "{height}"]
            }}
        ]
    }}
>>>ai 1 {session_id}
"##,
                         session_id = SESSION_ID,
                         x = WINDOW_REGION.x,
                         y = WINDOW_REGION.y,
                         width = WINDOW_REGION.width,
                         height = WINDOW_REGION.height
        );

        let directive = format!("{}{}{}", init, ai, build_completion_packet());

        let directive = wrap_to_native_json(&directive);
        let _ = Agent::new().run(mock_stdin(&directive));

        print_error_log();
        print_work_log();
    }   // smoke_capture_region_to_ai_input()

    /// Описание: Дымовой тест команды `capture_window_by_title` — ищет окно по подстроке заголовка,
    /// снимает скриншот и вставляет его в поле ввода AI.
    ///
    /// # Предусловия
    /// - Окно AI с заголовком `window_title` реально существует.
    /// - Окно-цель с подстрокой `WINDOW_NEEDLE` в заголовке существует.
    /// - Поле ввода AI готово принимать вставку изображения.
    ///
    /// # Побочные эффекты
    /// - Фокусирует окно-цель (для корректного скриншота).
    /// - Перезаписывает системный буфер обмена.
    /// - Переводит фокус на окно AI.
    /// - Генерирует Ctrl+V.
    /// - Создает/перезаписывает `work.log` и `error.log`.
    #[ignore]
    #[test]
    fn capture_window_by_title_to_ai_input() {
        initialize_glob().expect("Failed to initialize glob");
        cleanup_logs();

        let init = build_init_session_packet();

        let ai = format!(r##"
<<<ai 1 {session_id}
    {{
        "dir_comment": "smoke: capture_window_by_title -> paste into AI input",
        "commands": [
            {{
                "cmd_comment": "Снять скриншот окна по needle и вставить в окно AI",
                "cmd_id": 1,
                "name": "capture_window_by_title",
                "params": ["{needle}"]
            }}
        ]
    }}
>>>ai 1 {session_id}
"##, session_id = SESSION_ID, needle = WINDOW_NEEDLE);

        let directive = format!("{}{}{}", init, ai, build_completion_packet());

        let directive = wrap_to_native_json(&directive);
        let _ = Agent::new().run(mock_stdin(&directive));

        print_error_log();
        print_work_log();
    }   // smoke_capture_window_by_title_to_ai_input()

    /// Описание: Дымовой тест команды `capture_window_by_hwnd` — снимает скриншот окна по HWND
    /// и вставляет его в поле ввода AI.
    ///
    /// # Предусловия
    /// - Окно AI с заголовком `window_title` реально существует.
    /// - Окно-цель с подстрокой `WINDOW_NEEDLE` в заголовке существует (тест сначала находит его HWND).
    /// - Поле ввода AI готово принимать вставку изображения.
    ///
    /// # Алгоритм работы
    /// - Сначала выполняет `find_window_info` для получения HWND окна-цели.
    /// - Затем выполняет `capture_window_by_hwnd` с полученным HWND.
    ///
    /// # Побочные эффекты
    /// - Перезаписывает системный буфер обмена.
    /// - Переводит фокус на окно AI.
    /// - Генерирует Ctrl+V.
    /// - Создает/перезаписывает `work.log` и `error.log`.
    #[ignore]
    #[test]
    fn capture_window_by_hwnd_to_ai_input() {
        initialize_glob().expect("Failed to initialize glob");
        cleanup_logs();

        // 1) Сначала найти HWND окна-цели напрямую через win32tool.
        use crate::library::window;
        let (hwnd, _win_title) = window::find_window_by_needle(WINDOW_NEEDLE)
            .unwrap_or_else(|e| panic!("окно '{}' не найдено: {}", WINDOW_NEEDLE, e));
        let hwnd_str = format!("0x{:X}", hwnd.0 as usize);

        let init = build_init_session_packet();

        let ai = format!(r##"
<<<ai 1 {session_id}
    {{
        "dir_comment": "smoke: capture_window_by_hwnd -> paste into AI input",
        "commands": [
            {{
                "cmd_comment": "Снять скриншот окна по HWND и вставить в окно AI",
                "cmd_id": 1,
                "name": "capture_window_by_hwnd",
                "params": ["{hwnd}"]
            }}
        ]
    }}
>>>ai 1 {session_id}
"##, session_id = SESSION_ID, hwnd = hwnd_str);

        let directive = format!("{}{}{}", init, ai, build_completion_packet());

        let directive = wrap_to_native_json(&directive);
        let _ = Agent::new().run(mock_stdin(&directive));

        print_error_log();
        print_work_log();
    }   // smoke_capture_window_by_hwnd_to_ai_input()

    /// Описание: Дымовой тест команды `get_window_list` — получает список top-level окон
    /// и вставляет отчёт в поле ввода AI.
    ///
    /// # Предусловия
    /// - Окно AI с заголовком `window_title` реально существует.
    /// - Поле ввода AI в этом окне готово принимать вставку (в идеале — курсор уже стоит в поле).
    /// - В `config.toml` не включен `is_log_only`, иначе выполнение команд не произойдет.
    ///
    /// # Алгоритм работы
    /// - Инициализирует `glob`.
    /// - Чистит `work.log`/`error.log` (best effort).
    /// - Посылает INIT_SESSION.
    /// - Посылает AI-директиву #1 с командой `get_window_list` (без параметров).
    /// - Посылает COMPLETION для штатного завершения цикла `Agent::run()`.
    /// - Печатает логи для ручного анализа.
    ///
    /// # Побочные эффекты
    /// - Переводит фокус на окно AI (по `window_title`) и вставляет текст отчёта (Ctrl+V),
    ///   так как агент всегда отправляет отчёты в UI.
    /// - Создает/перезаписывает `work.log` и `error.log`.
    #[ignore]
    #[test]
    fn get_window_list_to_ai_input() {
        initialize_glob().expect("Failed to initialize glob");
        cleanup_logs();

        let init = build_init_session_packet();

        // ВАЖНО: номер директивы = 1, session_id должен совпадать с INIT_SESSION.
        let ai = format!(r##"
<<<ai 1 {session_id}
    {{
        "dir_comment": "smoke: get_window_list -> paste into AI input",
        "commands": [
            {{
                "cmd_comment": "Получить список окон (дефолтные фильтры хэндлера)",
                "cmd_id": 1,
                "name": "get_window_list"
            }}
        ]
    }}
>>>ai 1 {session_id}
"##, session_id = SESSION_ID);

        let directive = format!("{}{}{}", init, ai, build_completion_packet());

        let directive = wrap_to_native_json(&directive);
        let _ = Agent::new().run(mock_stdin(&directive));

        print_error_log();
        print_work_log();
    }   // get_window_list_to_ai_input()

    /// Описание: Дымовой тест команды `paste_text_into_window_by_title` — вставляет текст
    /// в окно-цель по needle (подстроке заголовка) через clipboard + Ctrl+V с верификацией.
    ///
    /// # Предусловия
    /// - Команда `paste_text_into_window_by_title` зарегистрирована в HandlerRegistry.
    /// - Окно-цель с подстрокой `WINDOW_NEEDLE` в заголовке реально существует
    ///   и оно единственное (иначе find-by-needle вернёт ошибку неоднозначности).
    /// - В окне-цели фокус ввода стоит в текстовом поле, которое:
    ///   - готово принимать Ctrl+V,
    ///   - и после вставки содержит РОВНО `payload` (иначе верификация не пройдёт).
    /// - Окно AI с заголовком `window_title()` реально существует (агент будет вставлять туда отчёт).
    ///
    /// # Побочные эффекты
    /// - Перезаписывает системный буфер обмена (вставка + верификация + отчёты агента).
    /// - Фокусирует окно-цель, затем окно AI.
    /// - Создает/перезаписывает `work.log` и `error.log`.
    #[ignore]
    #[test]
    fn paste_text_into_window_by_title_to_target_window() {
        initialize_glob().expect("Failed to initialize glob");
        cleanup_logs();

        let init = build_init_session_packet();

        // Текст для вставки. Лучше без кавычек/переносов, чтобы не усложнять JSON.
        let payload = format!("HBOT_SMOKE_PASTE_TEXT [{}]", SESSION_ID);

        // ВАЖНО: номер директивы = 1, session_id должен совпадать с INIT_SESSION.
        let ai = format!(r##"
<<<ai 1 {session_id}
    {{
        "dir_comment": "smoke: paste_text_into_window_by_title -> paste into target window",
        "commands": [
            {{
                "cmd_comment": "Вставить текст в окно по needle (с верификацией)",
                "cmd_id": 1,
                "name": "paste_text_into_window_by_title",
                "params": ["{needle}", "{text}"]
            }}
        ]
    }}
>>>ai 1 {session_id}
"##,
                         session_id = SESSION_ID,
                         needle = window_title(),
                         text = payload
        );

        let directive = format!("{}{}{}", init, ai, build_completion_packet());

        let directive = wrap_to_native_json(&directive);
        let _ = Agent::new().run(mock_stdin(&directive));

        print_error_log();
        print_work_log();
    }   // paste_text_into_window_by_title_to_target_window()
}   // mod tests