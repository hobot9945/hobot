//! test_shell_test.rs

#[cfg(test)]
mod tests {
    use crate::handler::shell::shell_cmd;

    /// Проверка успешного выполнения простой команды echo.
    #[test]
    fn test_shell_command_success() {
        let params = Some(vec!["echo hello_world".to_string()]);
        let result = shell_cmd(&params);

        assert!(result.is_ok(), "Команда должна была выполниться успешно");
        assert_eq!(result.unwrap(), "hello_world");
    }   // test_shell_command_success()

    /// Проверка реакции на отсутствие параметров.
    #[test]
    fn test_shell_command_no_params() {
        let result = shell_cmd(&None);
        assert!(result.is_err(), "Должна быть ошибка при отсутствии параметров");
    }   // test_shell_command_no_params()

    /// Проверка реакции на избыточное количество параметров.
    #[test]
    fn test_shell_command_too_many_params() {
        let params = Some(vec!["echo 1".into(), "echo 2".into()]);
        let result = shell_cmd(&params);
        assert!(result.is_err(), "Должна быть ошибка при > 1 параметра");
    }   // test_shell_command_too_many_params()

    /// Проверка вывода ошибки при запуске несуществующей команды.
    #[test]
    fn test_shell_command_invalid_cmd() {
        // В Windows cmd /C выдаст ошибку в stderr, если программа не найдена
        let params = Some(vec!["not_a_real_command_hobot_test".to_string()]);
        let result = shell_cmd(&params);

        assert!(result.is_err(), "Должна быть ошибка выполнения");
        // Проверяем, что в ошибке есть хоть какой-то текст
        let err_msg = result.unwrap_err();
        assert!(!err_msg.is_empty());
    }   // test_shell_command_invalid_cmd)
}   // mod tests

#[cfg(test)]
mod tests_1 {
    use crate::handler::shell::powershell_cmd;
    use super::*;
    #[test]
    fn just_a_run() {
        let cmd = r#"Start-Sleep -s 5"#.to_string();
        let params = Some(vec![cmd]);
        let result = powershell_cmd(&params);
        println!("{}", result.unwrap());
    }   // just_a_run()
}   // mod tests_1