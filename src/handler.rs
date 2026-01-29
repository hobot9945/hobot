//! handler — Центральный реестр команд и утилиты валидации.
//!
//! Модуль служит диспетчером, связывающим текстовые имена команд (из протокола ИИ)
//! с конкретными функциями-обработчиками (хэндлерами), реализованными в подмодулях
//! (shell, screenshot, mouse, keyboard).
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Реестр команд (`HandlerRegistry`): хранение карты `Command Name -> Handler Function`.
//! - Вспомогательные утилиты валидации параметров (`check_param_count`, `check_param_type`).
//! - Определение общего сигнатурного типа `HandlerFn`.

mod shell;
mod screenshot;
mod mouse;
mod win_control;
mod handler_test_utils;
mod misc_tool;
mod keyboard_and_text;

use std::collections::HashMap;
use std::str::FromStr;

/// Тип функции-обработчика команды.
///
/// Принимает ссылку на `Option<Vec<String>>` с параметрами команды.
/// Возвращает `Result` с текстовым результатом или сообщением об ошибке.
pub type HandlerFn = fn(&Option<Vec<String>>) -> Result<String, String>;

/// Реестр команд: связывает имена команд с функциями-обработчиками.
///
/// Используется для диспетчеризации входящих директив.
pub struct HandlerRegistry {
    handlers: HashMap<&'static str, HandlerFn>,
}   // HandlerRegistry

impl HandlerRegistry {

    /// Описание: Создаёт новый реестр и наполняет его обработчиками из подмодулей.
    ///
    /// # Алгоритм работы
    /// - Создаёт пустую `HashMap`.
    /// - Вызывает функции регистрации из каждого подмодуля (shell, screenshot, ...).
    ///
    /// # Возвращаемое значение
    /// Тип: Self: Инициализированный реестр команд.
    pub fn new() -> Self {
        let mut registry = Self {
            handlers: HashMap::new(),
        };

        // Регистрируем команды из модуля shell
        shell::handlers_map_init(&mut registry.handlers);
        win_control::handlers_map_init(&mut registry.handlers);
        screenshot::handlers_map_init(&mut registry.handlers);
        misc_tool::handlers_map_init(&mut registry.handlers);
        mouse::handlers_map_init(&mut registry.handlers);
        keyboard_and_text::handlers_map_init(&mut registry.handlers);

        registry
    }   // new()

    /// Описание: Возвращает ссылку на карту обработчиков.
    ///
    /// # Возвращаемое значение
    /// Тип: &HashMap<&'static str, HandlerFn>: Ссылка на внутреннюю карту.
    pub fn handlers(&self) -> &HashMap<&'static str, HandlerFn> {
        &self.handlers
    }   // handler()
}   // impl HandlerRegistry

/// Описание: Проверяет, совпадает ли число параметров с ожидаемым.
///
/// # Параметры
/// - `params`: Опциональный вектор строк из структуры Command.
/// - `expected`: Ожидаемое число параметров.
///
/// # Ошибки
/// Возвращает `Err(String)`, если фактическое число не равно `expected`.
pub fn check_param_count(params: &Option<Vec<String>>, expected: usize) -> Result<(), String> {
    let actual = params.as_ref().map_or(0, |v| v.len());
    if actual != expected {
        return Err(format!("Неверное число параметров: ожидалось {}, получено {}", expected, actual));
    }   // if
    Ok(())
}   // check_param_count()

/// Описание: Извлекает параметр по индексу и приводит к заданному типу `T`.
///
/// # Параметры
/// - `params`: Опциональный вектор строк из структуры Command.
/// - `index`: Индекс параметра в векторе (начиная с 0).
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - индекс вне диапазона;
/// - парсинг строки в тип `T` не удался.
pub fn check_param_type<T: FromStr>(params: &Option<Vec<String>>, index: usize) -> Result<T, String> {
    let raw_val = params
        .as_ref()
        .and_then(|v| v.get(index))
        .ok_or_else(|| format!("Параметр по индексу {} не найден", index))?;

    raw_val.parse::<T>().map_err(|_| {
        format!("Ошибка приведения параметра '{}' (индекс {}) к нужному типу", raw_val, index)
    })
}   // check_param_type()