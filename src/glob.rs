//! glob.rs
//!
//! Глобальные константы, синглтоны и утилиты общего назначения.
//!
//! # Архитектура
//! Модуль предоставляет централизованный доступ к:
//! - Конфигурации (через внутренний модуль `Config`).
//! - Системе обработки ошибок (`ErrorControl`).
//!
//! # Инварианты
//! - `initialize_glob()` должен быть вызван в `main` первым, до любого использования `config()` или логирования.

use std::cmp::min;
use std::io::Write;
use std::sync::Mutex;
use std::sync::OnceLock;
use windows::core::PCWSTR;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, IDYES, MB_DEFBUTTON1, MB_ICONERROR,
                                              MB_ICONWARNING, MB_OK, MB_SETFOREGROUND,
                                              MB_SYSTEMMODAL, MB_YESNO};
use crate::glob::config::AppConfig;
pub(crate) use crate::glob::error_control::AgentError;

// Внутренние модули
mod config;
pub mod error_control;
pub mod stdout_sink;

/// Константы.

/// Указание макросам печати, таким как prln!() и wrln!() дополнять сообщение именем файла и номера
/// строки вызова макроса.
///
/// # Объяснение
/// Бывает что забытый макрос печатает ненужные сообщения при работе программы, но его крайне трудно
/// найти. В этом случае выставляем этот флаг и макрос показывает точку в программе где он находится.
#[allow(dead_code)]
pub const PRLN_PRINTS_FILE_LINE: bool = false;

// --- Типы сообщений между подсистемами ---

/// Расширение посылает агенту пакет инициализации.
pub const EXT_MSG_TYPE_INIT_SESSION: &str = "INIT_SESSION";

/// Расширение посылает агенту текст ошибки для проксирования к AI
pub const EXT_MSG_TYPE_PROTOCOL_ERROR: &str = "PROTOCOL_ERROR";

/// Расширение требует остановку агента.
pub const EXT_MSG_TYPE_COMPLETION: &str = "COMPLETION";

/// Хобот посылает расширению сигнал о завершении исполнения директивы.
pub const HBT_MSG_TYPE_DIRECTIVE_COMPLETED: &str = "DIRECTIVE_COMPLETED";

// --- Константы протокола обмена расширение-агент ---

/// Маркеры транспортного протокола во входящем потоке.
pub const PROTOCOL_TAG_AI_OPEN: &str = "<<<ai";
pub const PROTOCOL_TAG_AI_CLOSE: &str = ">>>ai";
pub const PROTOCOL_TAG_EXT_OPEN: &str = "<<<ext";
pub const PROTOCOL_TAG_EXT_CLOSE: &str = ">>>ext";

// --- Глобальные переменные ---

/// Предоставляет доступ к глобальной конфигурации.
/// Делегирует вызов внутреннему модулю `Config`.
///
/// # Паника
/// Если конфигурация не инициализирована (ошибка этапа разработки).
///
/// # Возвращаемое значение
/// &AppConfig: Ссылка на конфигурацию.
pub fn config() -> &'static AppConfig {
    config::get()
}   // config()

/// Инициализирует глобальные компоненты приложения.
/// Загружает конфигурацию и устанавливает её в глобальный синглтон.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - Не удалось загрузить или создать файл конфигурации.
/// - Глобальная конфигурация уже была инициализирована ранее.
pub fn initialize_glob() -> Result<(), String> {

    // 1. Инициализируем конфигурацию через внутренний модуль
    config::init()?;

    // Здесь можно добавить инициализацию других компонентов (например, ErrorControl), если потребуется

    Ok(())
}   // initialize_glob()

// --- Глобальные функции ---
/// Отображает системное модальное окно с ошибкой и единственной кнопкой OK.
///
/// Окно будет отображаться поверх всех других окон (`MB_SYSTEMMODAL` + `MB_TOPMOST`),
/// блокируя выполнение потока агента до нажатия кнопки.
///
/// # Алгоритм работы
/// 1. Конвертирует строки Rust (UTF-8) в вектор `u16` с нуль-терминатором (формат Windows Unicode).
/// 2. Вызывает `MessageBoxW` с флагами ошибки и модальности.
///
/// # Параметры
/// - `title`: Заголовок окна.
/// - `message`: Текст сообщения об ошибке.
pub fn show_error_message(title: &str, message: &str) {

    // Преобразуем &str в UTF-16 (Vec<u16>) с нуль-терминатором на конце,
    // так как WinAPI ожидает C-style wide strings.
    let title_w: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
    let message_w: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        // MB_SYSTEMMODAL - поверх всех окон.
        // MB_SETFOREGROUND - попытаться вынести на передний план.
        // MB_ICONERROR - красный крестик и звук ошибки.
        // MB_OK - одна кнопка.
        MessageBoxW(
            None,
            PCWSTR(message_w.as_ptr()),
            PCWSTR(title_w.as_ptr()),
            MB_OK | MB_ICONERROR | MB_SYSTEMMODAL | MB_SETFOREGROUND
        );
    }   // unsafe
}   // show_error_message()

/// Запрашивает у пользователя разрешение на выполнение потенциально опасного действия.
///
/// Отображает модальное окно с кнопками "Да" и "Нет". Окно блокирует работу агента до ответа.
///
/// # Параметры
/// - `action_description`: Описание действия, которое требует подтверждения (например, текст команды).
///
/// # Возвращаемое значение
/// - `true`: Пользователь разрешил действие (нажал "Да").
/// - `false`: Пользователь запретил действие (нажал "Нет").
pub fn ask_user_permission(action_description: &str) -> bool {
    let title = "Хобот: Требуется подтверждение (Read-Only Mode)";
    let message = format!(
        "Включен режим ограничения изменений (os_read_only).\n\n\
         Попытка выполнить команду, не входящую в белый список:\n\n\
         {}\n\n\
         Разрешить выполнение?",
        action_description
    );

    // Преобразуем строки в UTF-16 для WinAPI
    let title_w: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
    let message_w: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        // MB_YESNO - кнопки Да/Нет.
        // MB_ICONWARNING - иконка предупреждения.
        // MB_DEFBUTTON2 - по умолчанию активна кнопка "Нет" (защита от случайного нажатия Enter).
        // MB_SYSTEMMODAL - поверх всех окон.
        let result = MessageBoxW(
            None,
            PCWSTR(message_w.as_ptr()),
            PCWSTR(title_w.as_ptr()),
            MB_YESNO | MB_ICONWARNING | MB_SYSTEMMODAL | MB_DEFBUTTON1 | MB_SETFOREGROUND
        );

        result == IDYES
    }   // unsafe
}   // ask_user_permission()

/// Возвращает подстроку (слайс) из исходной строки на основе индексов символов.
///
/// Функция находит байтовые границы указанных символов, используя однократный проход
/// итератора для поиска обеих границ.
///
/// # Алгоритм работы
/// 1. Определяет общее количество символов для валидации входных данных.
/// 2. Вычисляет целевой индекс окончания.
/// 3. Проверяет корректность параметров через `assert!`.
/// 4. Создает итератор по байтовым индексам.
/// 5. Сдвигает итератор до позиции `start` и запоминает байт начала.
/// 6. Продолжает сдвигать **тот же самый** итератор до позиции `end` для поиска байта конца.
/// 7. Возвращает слайс.
///
/// # Паника (Panics)
/// - Если `start > end`.
/// - Если индексы выходят за пределы длины строки (в символах).
///
/// # Возвращаемое значение
/// Тип: `&str`: Ссылка на подстроку.
///
/// ## Параметры
/// - `s`: Исходная строка.
/// - `start`: Индекс первого символа.
/// - `end`: Опциональный индекс конца (exclusive).
pub fn substring(s: &str, start: usize, end: Option<usize>) -> &str {
    // 1. Предварительная валидация (требует полного прохода для подсчета длины).
    let char_count = s.chars().count();
    let mut end_idx = end.unwrap_or(char_count);

    assert!(start <= end_idx, "Начальный индекс ({}) больше конечного ({})", start, end_idx);

    // 1. Если end больше длины строки, то уменьшаем end_idx.
    end_idx = min(char_count, end_idx);

    // 2. Создаем итератор. Добавляем s.len() в конец, чтобы корректно обработать срез до конца строки.
    let mut iter = s.char_indices()
        .map(|(i, _)| i)
        .chain(std::iter::once(s.len()));

    // 3. Находим start_byte.
    // Метод .nth(start) потребляет элементы от 0 до start включительно.
    // Итератор останавливается на позиции, следующей за start.
    let start_byte = iter.nth(start).expect("Ошибка поиска start_byte");

    // 4. Находим end_byte, продолжая движение тем же итератором.
    let end_byte = if start == end_idx {
        // Если индексы равны, срез пустой, смещения совпадают.
        start_byte
    } else {
        // Нам нужно пройти (end - start) шагов.
        // Но так как .nth(0) возвращает *следующий* элемент, мы отнимаем 1.
        let delta = end_idx - start;
        iter.nth(delta - 1).expect("Ошибка поиска end_byte")
    };

    &s[start_byte..end_byte]
}   // substring()

/// Отправляет текстовое сообщение в стандартный поток вывода (stdout),
/// оборачивая его в формат протокола Native Messaging.
///
/// Функция подготавливает данные для расширения браузера, упаковывая их в JSON-объект,
/// который затем передается в системный поток.
///
/// # Алгоритм работы
/// 1. Определение локальной структуры `NativeWrapper` для сериализации.
/// 2. Создание экземпляра обертки с переданным текстом (wrapper).
/// 3. Сериализация структуры в JSON-строку.
/// 4. Вызов низкоуровневой функции `_send_raw_to_stdout` для физической отправки байт.
///
/// # Ошибки (Errors)
/// Возвращает `AgentError::Critical` в случаях:
/// - Сбоя сериализации структуры в JSON (ошибка библиотеки `serde`).
/// - Ошибки записи в поток `stdout` (внутри вызываемой подфункции).
///
/// # Возвращаемое значение
/// Тип: `Result<(), AgentError>`: Пустой результат в случае успешной отправки.
///
/// ## Параметры
/// - `msg`: Текст сообщения, которое необходимо доставить расширению браузера.
pub fn send_to_stdout(msg: &str) -> Result<(), AgentError> {

    // Обертка Native Messaging
    #[derive(serde::Serialize)]
    struct NativeWrapper {
        text: String,
    }   // NativeWrapper

    let wrapper = NativeWrapper { text: msg.to_string() };

    let final_json = serde_json::to_string(&wrapper).map_err(|e| {
        AgentError::Critical(format!("{}, {}: Ошибка сериализации обертки NativeMsg: {}", file!(), line!(), e))
    })?;

    // Делегирование низкоуровневой отправки (запись длины и тела сообщения).
    _send_raw_to_stdout(&final_json)
}

/// Отправляет произвольную строку в stdout по протоколу Native Messaging.
///
/// Функция не анализирует содержимое строки и не добавляет никаких маркеров протокола.
/// Она просто вычисляет длину, добавляет префикс и пишет в поток.
///
/// # Параметры
/// - `final_json`: Строка для отправки.
///
/// # Ошибки
/// Возвращает `AgentError::Critical`, если не удалось выполнить операцию записи в поток.
fn _send_raw_to_stdout(final_json: &str) -> Result<(), AgentError> {
    let len = final_json.len() as u32;
    let mut out = std::io::stdout();

    out.write_all(&len.to_ne_bytes())
        .map_err(|e| AgentError::Critical(
            format!("{}, {}: Ошибка записи длины в stdout: {}", file!(), line!(), e)))?;

    out.write_all(final_json.as_bytes())
        .map_err(|e| AgentError::Critical(
            format!("{}, {}: Ошибка записи тела в stdout: {}", file!(), line!(), e)))?;

    out.flush()
        .map_err(|e| AgentError::Critical(
            format!("{}, {}: Ошибка сброса буфера stdout: {}", file!(), line!(), e)))?;

    Ok(())
}   // send_raw_to_stdout()