//! shell.rs — Модуль системных команд (Shell Commands).
//!
//! ОПИСАНИЕ:
//! Модуль предоставляет реализации обработчиков (хэндлеров) для выполнения
//! произвольных команд в системных оболочках операционной системы Windows.
//!
//! Хэндлеры этого модуля принимают строку команды от ядра агента (Agent),
//! запускают соответствующий процесс (cmd.exe или powershell.exe),
//! ожидают завершения и возвращают текстовый результат (stdout) или описание
//! ошибки (stderr) для включения в отчет для ИИ.
//!
//! ОТВЕТСТВЕННОСТЬ:
//! 1. Регистрация команд:
//!    - `shell_cmd`: Выполнение команд через интерпретатор cmd.exe.
//!    - `powershell_cmd`: Выполнение команд и скриптов через PowerShell.
//!
//! 2. Валидация и безопасность:
//!    - Проверка количества параметров перед запуском.
//!    - Проверка на допустимость выполнения в режиме `os_read_only` (Белый список + запрос пользователя).
//!    - Использование `std::process::Command` с флагом `.raw_arg` для передачи
//!      сложных командных строк (с кавычками, спецсимволами) без искажения
//!      синтаксиса команд, генерируемых ИИ.
//!
//! 3. Обработка результатов:
//!    - При успешном выполнении: возврат содержимого стандартного вывода (stdout).
//!    - При ошибке выполнения: возврат содержимого стандартного потока ошибок (stderr)
//!      или кода возврата процесса.
//!    - Преобразование байтовых массивов вывода в UTF-8 строку с потерей (lossy),
//!      чтобы избежать паники на некорректных кодировках.

mod test_shell_test;

use std::collections::HashMap;
use std::os::windows::process::CommandExt;
use std::process::Command;
use crate::handler::{check_param_count, check_param_type, HandlerFn};
use crate::library;
use crate::agent::request::session;
use crate::glob;

/// Регистрирует обработчики команд системной оболочки в предоставленную карту.
///
/// # Аргументы
///
/// * `handlers_map` - Изменяемая ссылка на `HashMap`, в которую будут добавлены
///   функции-обработчики этого модуля. Ключ — строковое имя команды для AI,
///   значение — указатель на функцию типа `HandlerFn`.
///
/// # Возвращаемое значение
///
/// Функция ничего не возвращает, напрямую модифицируя переданную карту.
pub fn handlers_map_init(handlers_map: &mut HashMap<&str, HandlerFn>) {
    handlers_map.insert("shell_cmd", shell_cmd);
    handlers_map.insert("powershell_cmd", powershell_cmd);
}   // handlers_map_init()

/// Хэндлер для выполнения произвольных shell-команд.
///
/// # Arguments
/// * `params` - Вектор, содержащий ровно один элемент: полную строку команды для исполнения.
///
/// # Windows 10
/// Выполнение идет через `cmd /C`, что позволяет запускать встроенные команды (echo, dir и др.).
/// Использование `raw_arg` позволяет передавать сложные конструкции с любыми типами кавычек внутри
/// команды.
///
/// # Errors
/// Возвращает ошибку, если:
/// - Количество параметров не равно 1.
/// - Сработала защита `os_read_only` (пользователь отклонил действие).
/// - Процесс не удалось запустить.
/// - Команда завершилась с ошибкой (возвращается текст из stderr).
pub fn shell_cmd(params: &Option<Vec<String>>) -> Result<String, String> {
    // Валидация: нам нужна ровно одна строка в векторе строк.
    check_param_count(&params, 1)?;
    let command_line: String = check_param_type(&params, 0)?;

    // Проверка ограничений безопасности (Read Only Mode)
    _check_os_readonly(&command_line)?;

    // raw_arg() не занимается экранированием параметров как args(), что мешает работе.
    let output = Command::new("cmd")
        .raw_arg(format!("/C \"{}\"", command_line))
        .output()
        .map_err(|e| format!("Критическая ошибка запуска: {}", e))?;

    if output.status.success() {
        // Команда завершилась с кодом 0 (успех).
        // Декодируем stdout из cp866/UTF-8 в String.
        let text = _decode_process_output(&output.stdout);
        // Оборачиваем в Markdown fenced block и возвращаем как Ok.
        Ok(library::markdown_fence::wrap_in_fence(&text))
    } else {
        // Команда завершилась с ненулевым кодом (ошибка).
        // Декодируем stderr и убираем пробелы по краям.
        let err_text = _decode_process_output(&output.stderr).trim().to_string();
        // Если stderr пустой — подставляем код возврата как текст ошибки.
        let err_payload = if err_text.is_empty() {
            format!("Код возврата: {:?}", output.status.code())
        } else {
            err_text
        };
        // Оборачиваем в fence и возвращаем как Err.
        Err(library::markdown_fence::wrap_in_fence(&err_payload))
    }
}   // shell_cmd()

/// Хэндлер для выполнения команд PowerShell.
///
/// # Arguments
/// * `params` - Ссылка на вектор, содержащий ровно один элемент: строку скрипта или команды PowerShell.
///
/// # Windows 10
/// Вызов идет напрямую через `powershell -Command "..."`. Использование `raw_arg` позволяет
/// передавать сложные конструкции с любыми типами кавычек внутри команды.
///
/// # Errors
/// Возвращает ошибку, если:
/// - Количество параметров не равно 1.
/// - Сработала защита `os_read_only` (пользователь отклонил действие).
/// - Процесс PowerShell не найден или не запустился.
/// - Команда вернула ненулевой код (текст ошибки берется из stderr).
pub fn powershell_cmd(params: &Option<Vec<String>>) -> Result<String, String> {
    // Валидация аналогична shell_cmd
    check_param_count(&params, 1)?;
    let command_line: String = check_param_type(&params, 0)?;

    // Проверка ограничений безопасности (Read Only Mode)
    _check_os_readonly(&command_line)?;

    // Используем raw_arg для сохранения целостности внутренних кавычек команды [web:1][web:3].
    // Оборачиваем command_line в кавычки для корректного восприятия параметра -Command.
    let output = Command::new("powershell")
        .raw_arg(format!("-Command \"{}\"", command_line))
        .output()
        .map_err(|e| format!("Критическая ошибка запуска PowerShell: {}", e))?;

    if output.status.success() {
        let text = _decode_process_output(&output.stdout);
        Ok(library::markdown_fence::wrap_in_fence(&text))
    } else {
        let err_text = _decode_process_output(&output.stderr).trim().to_string();
        let err_payload = if err_text.is_empty() {
            format!("PowerShell вернул код: {:?}", output.status.code())
        } else {
            err_text
        };
        Err(library::markdown_fence::wrap_in_fence(&err_payload))
    }
}   // powershell_cmd()

/// Декодирует байтовый вывод процесса в UTF-8.
///
/// # Алгоритм работы
/// 1. Если байты — валидный UTF-8, возвращает как есть.
/// 2. Иначе — декодирует из cp866 (OEM Russian, стандартная кодовая страница cmd.exe).
///
/// # Параметры
/// - `bytes`: Сырой вывод процесса (stdout или stderr).
///
/// # Возвращаемое значение
/// Тип: String: Декодированная строка.
fn _decode_process_output(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(s) => s.to_string(),
        Err(_) => {
            let (decoded, _, _) = encoding_rs::IBM866.decode(bytes);
            decoded.into_owned()
        }
    }
}   // _decode_process_output()

/// Проверяет допустимость команды в режиме os_read_only.
///
/// Если включен режим `os_read_only`, команда проверяется по "белому списку".
/// Если команда подозрительна (содержит перенаправления или не входит в список),
/// запрашивается подтверждение пользователя через GUI.
///
/// # Параметры
/// - `cmd`: Строка команды.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - Пользователь отклонил выполнение.
/// - Не удалось получить контекст сессии.
fn _check_os_readonly(cmd: &str) -> Result<(), String> {

    // Если ограничений нет — выходим сразу.
    if !session::os_readonly().map_err(|e| e.to_string())? {
        return Ok(());
    }   // if

    // --- Логика Белого списка ---

    let cmd_lower = cmd.trim().to_lowercase();
    let is_suspicious_char = cmd.contains('>') || cmd.contains('|');

    // Список безопасных команд (только чтение/информация)
    let allowed_prefixes = [
        "dir", "tree", "type", "echo", "cd", "chdir",
        "whoami", "hostname", "ipconfig", "systeminfo",
        "tasklist", "find", "findstr", "where", "start-sleep"
    ];

    // Команда считается безопасной, если:
    // 1. Не содержит спецсимволов перенаправления.
    // 2. Начинается с одного из разрешенных префиксов.
    let is_safe = !is_suspicious_char && allowed_prefixes.iter().any(|&prefix| {
        cmd_lower == prefix || cmd_lower.starts_with(&format!("{} ", prefix))
    });

    if is_safe {
        return Ok(());
    }   // if is_safe

    // --- Запрос подтверждения ---

    // Если мы здесь — команда подозрительная. Спрашиваем хозяина.
    if glob::ask_execution_permission(cmd) {
        Ok(())
    } else {
        Err("Отказано в доступе: Пользователь запретил выполнение команды.".to_string())
    }
}   // _check_os_readonly()