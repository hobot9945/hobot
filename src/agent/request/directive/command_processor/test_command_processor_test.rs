//! test_command_processor_test.rs
//!
//! Тесты для `command_processor.rs`.

#[cfg(test)]
mod tests {
    #[allow(unused_imports)] use crate::{wrln, writln};
    use crate::agent::request::directive::{Command, DirectiveContext};
    use crate::agent::request::directive::command_processor::CommandProcessor;
    use crate::agent::request::report;

    /// Проверяет, что `process_commands()` останавливает выполнение директивы на первой ошибке,
    /// и результаты команд записываются только до проблемной команды включительно.
    ///
    /// # Сценарий
    /// 1. Команда #1: валидная (`shell_cmd`) и должна выполниться успешно.
    /// 2. Команда #2: невалидная (хандлер не найден) и должна завершить обработку директивы.
    /// 3. Команда #3: не должна быть выполнена.
    ///
    /// # Ожидания
    /// - `cmd_results` содержит ровно 2 элемента.
    /// - `dir_err_msg` установлен и указывает на `cmd_id=2`.
    /// - `completed_cmd == 1` (успешно только первая команда).
    #[test]
    fn test_stops_on_second_command_error_cmd_results_len_2() {

        // --- Arrange: готовим входные данные как будто они пришли из директивы AI ---

        // 1) Первая команда: корректный хандлер "shell_cmd".
        //    Ожидаем успех и непустой stdout.
        //
        // 2) Вторая команда: намеренно несуществующий хандлер.
        //    Это должно привести к ошибке "Неизвестная команда" и остановить директиву.
        //
        // 3) Третья команда: существует в списке, но НЕ должна быть исполнена, так как стоп после #2.
        let commands = vec![
            Command {
                cmd_id: 1,
                cmd_comment: None,
                name: "shell_cmd".to_string(),
                params: Some(vec!["echo hello, world".to_string()]),
                allowed_timeout: None,
            },
            Command {
                cmd_id: 2,
                cmd_comment: None,
                name: "no_command".to_string(), // намеренно несуществующая команда-ХАНДЛЕР
                params: Some(vec!["echo good bye, world".to_string()]),
                allowed_timeout: None,
            },
            Command {
                cmd_id: 3,
                cmd_comment: None,
                name: "shell_cmd".to_string(),
                params: Some(vec!["echo SHOULD_NOT_RUN".to_string()]),
                allowed_timeout: None,
            },
        ];

        // Контекст директивы нужен, чтобы корректно инициализировать метаданные процессора:
        // `dir_id`, `total_cmd` и т.п.
        let dir_ctx = DirectiveContext {
            session_id: "sess_01".to_string(),
            dir_id: 1,
            dir_comment: None,
            commands: commands.clone(),
        };

        // Создаем процессор, инициируем его контекстом директивы.
        let mut cp = CommandProcessor::new();

        // --- Act: выполняем команды ---
        let res = cp.process_commands(&commands, dir_ctx.dir_id);

        // --- Assert: проверяем, что модуль повел себя как требуется ---

        // По текущей договоренности: ошибки выполнения команд не должны превращаться в Err(...) метода,
        // метод возвращает Err только на "ошибки разработки/инфы".
        assert!(
            res.is_ok(),
            "process_commands() не должен возвращать Err на ошибке команды-хандлера"
        );

        // Должно быть зафиксировано, что директива остановлена на 2-й команде.
        assert!(cp.dir_res.dir_err_msg.is_some(), "Ожидали, что dir_err_msg будет установлен");
        let dir_err = cp.dir_res.dir_err_msg.as_ref().unwrap();
        assert!(dir_err.contains("cmd_id=2"), "Ожидали останов на cmd_id=2, msg={}", dir_err);

        // Главное: результаты есть только для первых двух команд, третья не выполнялась.
        assert_eq!(
            cp.dir_res.cmd_results.len(),
            2,
            "Должны быть результаты только для первых двух команд"
        );

        // Метаданные: всего в директиве было 3 команды, успешно выполнена только первая.
        assert_eq!(cp.dir_res.total_cmd, 3, "total_cmd должен отражать количество команд директивы");
        assert_eq!(cp.dir_res.completed_cmd, 1, "Должна быть успешно выполнена только первая команда");

        // --- Проверки содержимого результатов ---

        // Команда #1: успех
        assert_eq!(cp.dir_res.cmd_results[0].id, 1);
        assert_eq!(cp.dir_res.cmd_results[0].name, "shell_cmd");
        assert!(cp.dir_res.cmd_results[0].cmd_err_msg.is_none(), "Успешная команда не должна иметь cmd_err_msg");
        assert!(
            cp.dir_res.cmd_results[0].cmd_result.as_deref().unwrap_or("").contains("hello, world"),
            "Ожидали stdout от echo, получено: {:?}",
            cp.dir_res.cmd_results[0].cmd_result
        );

        // Команда #2: ошибка "неизвестная команда", результат отсутствует
        assert_eq!(cp.dir_res.cmd_results[1].id, 2);
        assert_eq!(cp.dir_res.cmd_results[1].name, "no_command");
        assert!(cp.dir_res.cmd_results[1].cmd_result.is_none(), "При ошибке должен отсутствовать result");
        assert!(cp.dir_res.cmd_results[1].cmd_err_msg.is_some(), "При ошибке должен быть cmd_err_msg");
        let cmd2_err = cp.dir_res.cmd_results[1].cmd_err_msg.as_ref().unwrap();
        assert!(
            cmd2_err.contains("Неизвестная команда"),
            "Не тот текст ошибки: {}",
            cmd2_err
        );
        // wrln!(cp);
    }   // test_stops_on_second_command_error_cmd_results_len_2()

    /// Проверяет, что `process_commands()` останавливает выполнение директивы,
    /// если хандлер найден, но его выполнение завершилось ошибкой.
    ///
    /// # Сценарий
    /// 1. Команда #1: `shell_cmd` с `echo` — успешна.
    /// 2. Команда #2: `shell_cmd` с несуществующей командой — ошибка выполнения `cmd.exe`.
    /// 3. Команда #3: не должна быть выполнена.
    ///
    /// # Ожидания
    /// - `cmd_results` содержит ровно 2 элемента.
    /// - `dir_err_msg` установлен и указывает на `cmd_id=2`.
    /// - `completed_cmd == 1`.
    #[test]
    fn test_stops_on_second_command_shell_handler_error_cmd_results_len_2() {

        // --- Arrange ---
        let commands = vec![
            Command {
                cmd_id: 1,
                cmd_comment: None,
                name: "shell_cmd".to_string(),
                params: Some(vec!["echo hello, world".to_string()]),
                allowed_timeout: None,
            },
            Command {
                cmd_id: 2,
                cmd_comment: None,
                name: "shell_cmd".to_string(),
                // cmd.exe должен свалиться: "abcde" как команда не существует.
                params: Some(vec!["abcde good bye, world".to_string()]),
                allowed_timeout: None,
            },
            Command {
                cmd_id: 3,
                cmd_comment: None,
                name: "shell_cmd".to_string(),
                // Не должно выполниться, если стоп на cmd_id=2.
                params: Some(vec!["echo SHOULD_NOT_RUN".to_string()]),
                allowed_timeout: None,
            },
        ];

        let dir_ctx = DirectiveContext {
            session_id: "sess_01".to_string(),
            dir_id: 1,
            dir_comment: None,
            commands: commands.clone(),
        };

        let mut cp = CommandProcessor::new();

        // --- Act ---
        let res = cp.process_commands(&commands, dir_ctx.dir_id);

        // --- Assert ---
        assert!(res.is_ok(), "process_commands() не должен возвращать Err на ошибке выполнения хандлера");

        assert!(cp.dir_res.dir_err_msg.is_some(), "Ожидали, что dir_err_msg будет установлен");
        let dir_err = cp.dir_res.dir_err_msg.as_ref().unwrap();
        assert!(dir_err.contains("cmd_id=2"), "Ожидали останов на cmd_id=2, msg={}", dir_err);

        assert_eq!(cp.dir_res.cmd_results.len(), 2, "Должны быть результаты только для первых двух команд");
        assert_eq!(cp.dir_res.completed_cmd, 1, "Должна быть успешно выполнена только первая команда");

        // Команда #1: успех
        assert_eq!(cp.dir_res.cmd_results[0].id, 1);
        assert!(cp.dir_res.cmd_results[0].cmd_err_msg.is_none());

        // Команда #2: ошибка выполнения shell_cmd
        assert_eq!(cp.dir_res.cmd_results[1].id, 2);
        assert!(cp.dir_res.cmd_results[1].cmd_result.is_none(), "При ошибке выполнения result должен отсутствовать");
        assert!(
            cp.dir_res.cmd_results[1].cmd_err_msg.is_some(),
            "При ошибке выполнения должен быть cmd_err_msg"
        );
        // wrln!(cp);
    }   // test_stops_on_second_command_shell_handler_error_cmd_results_len_2()

    /// Дымовой тест генерации отчёта (`build_report()`), предназначенный для ручного просмотра.
    ///
    /// # Сценарий
    /// 1. Команда #1: `shell_cmd` с `echo` — должна выполниться успешно.
    /// 2. Команда #2: `shell_cmd` с несуществующей командой — должна завершиться ошибкой `cmd.exe`.
    /// 3. Команда #3: не должна быть выполнена, так как выполнение директивы останавливается на первой ошибке.
    /// 4. Генерируется Markdown-отчёт через `build_report()` и печатается в stdout.
    ///
    /// # Ожидания
    /// - `process_commands()` не возвращает `Err(...)` (ошибки команд фиксируются внутри `CommandProcessor`).
    /// - В отчёте видно:
    ///   * успешную команду #1 с выводом,
    ///   * ошибку команды #2,
    ///   * отсутствие выполнения команды #3.
    ///
    /// # Примечания
    /// Для отображения вывода теста используй запуск: `cargo test -- --nocapture`.
    #[test]
    fn just_a_run() {

        // --- Arrange ---
        // Специально собранный список команд, чтобы:
        //  1-я команда прошла,
        //  2-я команда упала,
        //  3-я команда не выполнялась (проверка stop-on-first-error).
        let commands = vec![
            Command {
                cmd_id: 1,
                cmd_comment: None,
                name: "shell_cmd".to_string(),
                params: Some(vec!["echo hello_from_hobot".to_string()]),
                allowed_timeout: None,
            },
            Command {
                cmd_id: 2,
                cmd_comment: None,
                name: "shell_cmd".to_string(),
                params: Some(vec!["not_a_real_command_hobot_test".to_string()]),
                allowed_timeout: None,
            },
            Command {
                cmd_id: 3,
                cmd_comment: None,
                name: "shell_cmd".to_string(),
                params: Some(vec!["echo SHOULD_NOT_RUN".to_string()]),
                allowed_timeout: None,
            },
        ];

        // Контекст директивы нужен для корректного заполнения метаданных отчёта (dir_id/total_cmd).
        let dir_ctx = DirectiveContext {
            session_id: "sess_01".to_string(),
            dir_id: 77,
            dir_comment: None,
            commands: commands.clone(),
        };

        let mut cp = CommandProcessor::new();

        // --- Act ---
        let _ = cp.process_commands(&commands, dir_ctx.dir_id);

        cp.build_report(&dir_ctx);

        // --- Print ---
        // Смотри вывод через `cargo test -- --nocapture`.
        writln!("\n{}", report::text().unwrap());
    }   // just_a_run()

}   // mod tests
