//! error_control.rs
//!
//! Модуль централизованного управления ошибками.
//!
//! Функционал:
//! - Логирование в файл (если доступен).
//! - Дублирование критических ошибок в stdout (для расширения).

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Mutex, OnceLock};
use crate::glob;
use crate::glob::stdout_sink::send_critical_to_stdout;

/// Глобальный контроллер ошибок. OnceLock обеспечивает ленивую/однократную инициализацию.
static ERROR_CONTROL: OnceLock<Mutex<ErrorControl>> = OnceLock::new();

/// Если функция возвращает Result, она возвращает ошибку типа AgentError: Result<T, AgentError>.
#[derive(Debug)]
pub enum AgentError {
    Recoverable(String), // Ошибка выполнения (продолжаем работу)
    Critical(String),    // Авария (прерываем цикл или паникуем)
}

impl std::fmt::Display for AgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentError::Recoverable(s) => write!(f, "{}", s),
            AgentError::Critical(s) => write!(f, "{}", s),
        }
    }
}   // impl Display for AgentError

/// Централизованный контроллер ошибок.
///
/// Реализует стратегию "best effort" для файлового логирования:
/// если файл недоступен, ошибки пишутся только в stdout.
pub struct ErrorControl {
    /// Дескриптор файла журнала ошибок. None, если файл не удалось открыть.
    error_log: Option<File>,
    
    /// Здесь формируется сообщение для посылки AI.
    msg_for_ai: String
}   // ErrorControl

impl ErrorControl {

    /// Создает новый экземпляр контроллера ошибок.
    ///
    /// Пытается открыть файл по пути из конфига (`cfg.errlog_path`) в режиме append.
    /// Если открытие не удалось (нет прав, плохой путь), логирование в файл будет отключено,
    /// но агент продолжит работу.
    pub fn new() -> Self {
        let cfg = crate::glob::config();

        // Игнорируем ошибку открытия, деградируем до работы без файла
        let file = if glob::config().are_logs_cleared_at_start {
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&cfg.errlog_path)
                .ok()
        } else {
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&cfg.errlog_path)
                .ok()
        };

        Self {
            error_log: file,
            msg_for_ai: String::new()
        }
    }   // new()

    /// Регистрирует информационное сообщение (технический лог).
    ///
    /// Сообщение записывается только в журнал ошибок (если доступен).
    /// В stdout сообщение не отправляется.
    ///
    /// Функция безопасна для вызова из любого потока.
    ///
    /// # Параметры
    /// - `msg`: Текст сообщения для записи.
    ///
    /// # Побочные эффекты
    /// - Пишет строку в файл журнала ошибок (best effort).
    pub fn handle_log(msg: &str) {
        let error_control = ERROR_CONTROL.get_or_init(|| Mutex::new(ErrorControl::new()));

        // Если мьютекс "отравлен" (poisoned) из-за паники в другом потоке,
        // мы всё равно забираем данные, так как нам важно просто записать сообщение.
        let mut error_control =
            error_control.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        error_control._handle_log(msg);
    }   // handle_log()

    /// Регистрирует некритическую ошибку.
    ///
    /// Ошибка записывается в лог (если доступен).
    /// Работа приложения не прерывается.
    ///
    /// Функция безопасна для вызова из любого потока.
    pub fn handle_error(msg: &str) {
        let error_control = ERROR_CONTROL.get_or_init(|| Mutex::new(ErrorControl::new()));

        // Если мьютекс "отравлен" (poisoned) из-за паники в другом потоке,
        // мы всё равно забираем данные, так как нам важно просто записать ошибку.
        let mut error_control =
            error_control.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        error_control._handle_error(msg);
    }   // process_error()

    /// Регистрирует критическую ошибку.
    ///
    /// Ошибка пишется в лог и немедленно отправляется в stdout для расширения.
    /// Используется для ситуаций, требующих внимания пользователя.
    pub fn handle_critical(msg: &str) {
        let error_control = ERROR_CONTROL.get_or_init(|| Mutex::new(ErrorControl::new()));

        let mut error_control =
            error_control.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        error_control._handle_critical(msg);
    }   // process_critical()

    /// Взять сообщение для AI.
    pub fn msg_for_ai() -> String {
        let error_control = ERROR_CONTROL.get_or_init(|| Mutex::new(ErrorControl::new()));

        // Если мьютекс "отравлен" (poisoned) из-за паники в другом потоке,
        // мы всё равно забираем данные, так как нам важно просто записать сообщение.
        let mut error_control =
            error_control.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        error_control.msg_for_ai.clone()
    }
    
    /// Очистка сообщения для AI.
    pub fn clear() {
        let error_control = ERROR_CONTROL.get_or_init(|| Mutex::new(ErrorControl::new()));

        // Если мьютекс "отравлен" (poisoned) из-за паники в другом потоке,
        // мы всё равно забираем данные, так как нам важно просто записать сообщение.
        let mut error_control =
            error_control.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        error_control.msg_for_ai.clear();
    }
    
    /// Очищено ли сообщение для AI.
    pub fn is_empty() -> bool {
        let error_control = ERROR_CONTROL.get_or_init(|| Mutex::new(ErrorControl::new()));

        // Если мьютекс "отравлен" (poisoned) из-за паники в другом потоке,
        // мы всё равно забираем данные, так как нам важно просто записать сообщение.
        let mut error_control =
            error_control.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        error_control.msg_for_ai.is_empty()
    }
}

impl ErrorControl {

    /// Обработка информационного сообщения.
    ///
    /// Действия:
    /// 1. Пишет строку `ИНФО: ...` в файл (если доступен).
    ///
    /// # Параметры
    /// - `msg`: Текст сообщения для записи.
    ///
    /// # Побочные эффекты
    /// - Пишет строку в файл журнала ошибок (best effort).
    fn _handle_log(&mut self, msg: &str) {
        
        let msg = format!("ИНФО: {}", msg);
        
        // Пишем в файл (если есть)
        if let Some(file) = self.error_log.as_mut() {
            // Игнорируем ошибки записи
            let _ = writeln!(file, "{}", &msg);
        }
    }   // handle_log()

    /// Обработка некритической ошибки.
    ///
    /// Действия:
    /// 1. Пишет строку `ОШИБКА: ...` в файл (если доступен).
    ///
    /// # Параметры
    /// - `msg`: Текст ошибки для записи.
    fn _handle_error(&mut self, msg: &str) {

        let msg = format!("ОШИБКА: {}", msg);

        // Пишем в файл (если есть)
        if let Some(file) = self.error_log.as_mut() {
            // Игнорируем ошибки записи
            let _ = writeln!(file, "{}", &msg);
        }
    }   // process_error()

    /// Обработка критической ошибки.
    ///
    /// Действия:
    /// 1. Пишет `АВАРИЯ: ...` в файл (если доступен).
    /// 2. Отправляет уведомление в stdout (для расширения).
    ///
    /// # Параметры
    /// - `msg`: Текст ошибки для записи и отправки.
    fn _handle_critical(&mut self, msg: &str) {

        let msg = format!("АВАРИЯ: {}", msg);

        // 1. Пишем в файл
        if let Some(file) = self.error_log.as_mut() {
            let _ = writeln!(file, "{}", &msg);
        }

        // 2. Уведомляем расширение
        send_critical_to_stdout(&msg);

        // Добавляем сообщение для AI
        self.msg_for_ai.push_str(&format!("<<<hbt\n{}\n>>>hbt", &msg));
    }   // process_critical()
}   // impl ErrorControl