//! keyboard_and_text.rs — Хэндлеры команд клавиатуры и вставки текста.
//!
//! Модуль предоставляет обработчики для вставки текста в произвольное окно
//! через clipboard + Ctrl+V с верификацией (Ctrl+A/Ctrl+C).
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Регистрация команд клавиатуры/текста в реестре хэндлеров.
//! - Вставка текста в окно по needle (подстроке заголовка) или по HWND.
//!
//! # ПРИМЕЧАНИЯ
//! - Вставка использует системный clipboard и пытается восстановить его (best effort, только текст).
//! - Верификация вставки зависит от того, что фокус ввода стоит в текстовом поле целевого окна.

use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use windows::Win32::Foundation::HWND;
use crate::glob::ask_execution_permission;
use crate::handler;
use crate::library::markdown_fence::wrap_in_fence;
use crate::library::{keyboard, window};

/// Описание: Регистрирует обработчики команд keyboard/text в карту хэндлеров.
///
/// # Параметры
/// - `handlers_map`: Карта, в которую добавляются хэндлеры.
///
/// # Побочные эффекты
/// - Модифицирует переданную карту.
pub fn handlers_map_init(handlers_map: &mut HashMap<&str, handler::HandlerFn>) {
    handlers_map.insert("paste_text_into_window_by_title", paste_text_into_window_by_title);
    handlers_map.insert("paste_text_into_window_by_hwnd", paste_text_into_window_by_hwnd);
    handlers_map.insert("write_text_into_logic_log", write_text_into_logic_log);
    handlers_map.insert("press_vk", press_vk);
    handlers_map.insert("press_key", press_key);
    handlers_map.insert("press_n_keys", press_n_keys);
}   // handlers_map_init()

/// Описание: Вставляет текст в окно, найденное по подстроке заголовка (needle).
///
/// # Параметры
/// - `params`: `["<needle>", "<text>"]`
///   - `<needle>`: подстрока заголовка окна (contains).
///   - `<text>`: текст для вставки.
///
/// # Возвращаемое значение
/// Тип: String: Markdown-блок с сообщением о выполнении и полными данными окна.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное число параметров;
/// - окно не найдено (или найдено более одного);
/// - не удалось вставить текст или подтвердить вставку.
///
/// # Побочные эффекты
/// - Фокусирует целевое окно (best effort).
/// - Временно перезаписывает системный буфер обмена.
fn paste_text_into_window_by_title(params: &Option<Vec<String>>) -> Result<String, String> {

    if !ask_execution_permission("вставка текста в окно по титулу") {
        return Err("Отказано в доступе: Пользователь запретил выполнение команды.".to_string());
    }   // if

    // 1) Валидация параметров.
    handler::check_param_count(params, 2)?;
    let needle: String = handler::check_param_type(params, 0)?;
    let text: String = handler::check_param_type(params, 1)?;

    // 2) Вставка по needle (внутри: find+focus -> Ctrl+V -> verify -> restore clipboard).
    let wi = window::paste_text_into_window_by_needle(&needle, &text)?;

    // 3) Отчет.
    let hwnd_hex = _hwnd_to_hex(wi.hwnd);

    let out = format!(
        "Текст вставлен.\nneedle='{}'\nhwnd={}\ntitle='{}'\npos: [{}, {}]\nsize: {}x{}\nlen={}",
        needle,
        hwnd_hex,
        wi.title,
        wi.x,
        wi.y,
        wi.width,
        wi.height,
        text.len()
    );

    Ok(wrap_in_fence(&out))
}   // paste_text_into_window_by_title()

/// Описание: Вставляет текст в окно по HWND.
///
/// # Параметры
/// - `params`: `["<hwnd>", "<text>"]`
///   - `<hwnd>`: HWND в десятичном виде или hex с префиксом `0x`.
///   - `<text>`: текст для вставки.
///
/// # Возвращаемое значение
/// Тип: String: Markdown-блок с сообщением о выполнении и полными данными окна.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное число параметров;
/// - HWND не удалось распарсить;
/// - не удалось сфокусировать окно/вставить/подтвердить вставку.
///
/// # Побочные эффекты
/// - Фокусирует целевое окно (best effort).
/// - Временно перезаписывает системный буфер обмена.
fn paste_text_into_window_by_hwnd(params: &Option<Vec<String>>) -> Result<String, String> {

    if !ask_execution_permission("вставка текста в окно по hwnd") {
        return Err("Отказано в доступе: Пользователь запретил выполнение команды.".to_string());
    }   // if

    // 1) Валидация параметров.
    handler::check_param_count(params, 2)?;
    let hwnd_in: String = handler::check_param_type(params, 0)?;
    let text: String = handler::check_param_type(params, 1)?;

    // 2) Парсинг HWND.
    let hwnd: HWND = window::parse_hwnd(&hwnd_in)?;

    // 3) Вставка (внутри: focus -> Ctrl+V -> verify -> restore clipboard).
    let wi = window::paste_text_into_window_by_hwnd(hwnd, &text)?;
    let hwnd_hex = _hwnd_to_hex(wi.hwnd);

    // 4) Отчет.
    let out = format!(
        "Текст вставлен.\nhwnd_in='{}'\nhwnd={}\ntitle='{}'\npos: [{}, {}]\nsize: {}x{}\nlen={}",
        hwnd_in,
        hwnd_hex,
        wi.title,
        wi.x,
        wi.y,
        wi.width,
        wi.height,
        text.len()
    );

    Ok(wrap_in_fence(&out))
}   // paste_text_into_window_by_hwnd()

/// Описание: Записывает переданный текст в логический журнал logic_log.md.
///
/// ВАЖНО:
/// - Текст пишется "как есть", без добавления переводов строк.
/// - Любое форматирование (Markdown, секции, списки, fenced-блоки) — ответственность AI.
///
/// # Параметры
/// - `params`: `["<text>"]`
///
/// # Возвращаемое значение
/// Type: String: Markdown-блок с подтверждением записи (для отчёта по директиве).
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное число параметров (ожидается 1).
///
/// # Побочные эффекты
/// - Дописывает в `log/<TS>/logic_log.md` через `glob::log_control`.
fn write_text_into_logic_log(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Валидация параметров.
    handler::check_param_count(params, 1)?;
    let text: String = handler::check_param_type(params, 0)?;

    // 2) Запись в лог "байт-в-байт" (в смысле: без модификации строки).
    //    Переводы строк, секции и прочее формирует AI.
    crate::glob::log_control::write_to_logic_log(&text);

    // 3) Отчет.
    let out = format!(
        "Текст записан в logic_log.md.\nbytes={}",
        text.as_bytes().len()
    );

    Ok(wrap_in_fence(&out))

}   // write_text_into_logic_log()

/// Описание: Нажимает одну клавишу или комбинацию "модификаторы + клавиша" по VK-кодам.
///
/// # Формат параметров
/// - `params`: массив из **1..N строк**
///   - если 1 строка: одиночное нажатие (press)
///   - если >1 строки: первые N-1 — модификаторы, последняя — основная клавиша
///
/// # Формат VK-кода
/// Поддерживается:
/// - hex: `"0x11"`, `"0X56"`
/// - decimal: `"17"`, `"86"`
///
/// # Примеры
/// - `["0x0D"]` → Enter
/// - `["0x11","0x56"]` → Ctrl+V
/// - `["0x11","0x10","0x53"]` → Ctrl+Shift+S
///
/// # Возвращаемое значение
/// Markdown-блок с фактом нажатия (для логов/отчёта).
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - params отсутствуют или пустые;
/// - VK-код не парсится;
/// - SendInput не смог отправить события.
fn press_vk(params: &Option<Vec<String>>) -> Result<String, String> {

    if !ask_execution_permission("эмуляция нажатия произвольной клавиши/комбинации клавиш'") {
        return Err("Отказано в доступе: Пользователь запретил выполнение команды.".to_string());
    }

    /// Парсит VK-код из строки (hex/decimal).
    fn _parse_vk_code(raw: &str) -> Result<u16, String> {
        let s = raw.trim();

        if s.is_empty() {
            return Err("VK-код пустой".to_string());
        }   // if

        let val_u32 = if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
            u32::from_str_radix(hex, 16)
                .map_err(|e| format!("не удалось распарсить hex VK-код '{}': {}", s, e))?
        } else {
            s.parse::<u32>()
                .map_err(|e| format!("не удалось распарсить decimal VK-код '{}': {}", s, e))?
        };   // if

        let val_u16 = u16::try_from(val_u32)
            .map_err(|_| format!("VK-код '{}' не укладывается в u16", s))?;

        Ok(val_u16)
    }   // _parse_vk_code()

    let Some(v) = params.as_ref() else {
        return Err("Неверное число параметров: ожидалось 1..N, получено 0".to_string());
    };

    if v.is_empty() {
        return Err("Неверное число параметров: ожидалось 1..N, получено 0".to_string());
    }   // if

    // Парсим все VK-коды.
    let mut vk_codes: Vec<u16> = Vec::with_capacity(v.len());
    for raw in v {
        vk_codes.push(_parse_vk_code(raw)?);
    }   // for

    // 1 VK => одиночное нажатие.
    if vk_codes.len() == 1 {
        keyboard::send_vk_press(vk_codes[0])?;

        let out = format!("Нажата клавиша: vk=0x{:X} ({})", vk_codes[0], vk_codes[0]);
        return Ok(wrap_in_fence(&out));
    }   // if

    // N VK => модификаторы + основная.
    let key = *vk_codes.last().expect("программная ошибка: vk_codes пуст");
    let modifiers = &vk_codes[..vk_codes.len() - 1];

    keyboard::send_vk_combo(modifiers, key)?;

    // Дать UI время отработать нажатие клавиши.
    sleep(Duration::from_millis(20));

    // Формируем отчет.
    let mods_hex: Vec<String> = modifiers.iter()
        .map(|v| format!("0x{:X}", v))
        .collect();

    let out = format!(
        "Нажата комбинация: modifiers=[{}], key=0x{:X}",
        mods_hex.join(", "),
        key
    );

    Ok(wrap_in_fence(&out))
}   // press_vk()

/// Описание: Нажимает “избранную” клавишу или комбинацию клавиш по строковому имени.
///
/// Это сокращённый и более безопасный вариант, чем `press_vk`.
///
/// # Параметры
/// - `params`: `["<key>"]`, где `<key>` — строка вида:
///   - `"Ctrl+V"`
///   - `"Ctrl+C"`
///   - `"Ctrl+A"`
///   - `"Enter"`
///   - `"Right"`
///
/// # Нормализация входа
/// Перед распознаванием строка нормализуется:
/// - `trim()`
/// - `to_ascii_lowercase()`
/// - удаление пробелов `' '`
///
/// То есть `" Ctrl + V "` будет распознано как `"ctrl+v"`.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное число параметров (ожидается 1),
/// - комбинация не поддерживается,
/// - не удалось отправить события через SendInput.
///
/// # Возвращаемое значение
/// Markdown-блок с подтверждением выполненного действия.
fn press_key(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Валидация параметров.
    handler::check_param_count(params, 1)?;
    let raw: String = handler::check_param_type(params, 0)?;

    // 2) Нормализация:
    // - trim
    // - lowercase
    // - убрать пробелы (чтобы "Ctrl + V" == "Ctrl+V")
    let token = raw.trim().to_ascii_lowercase().replace(' ', "");

    // 3) Диспетчеризация.
    match token.as_str() {

        "ctrl+v" => {
            if !ask_execution_permission("нажатие клавиши Ctrl+V") {
                return Err("Отказано в доступе: Пользователь запретил выполнение команды.".to_string());
            }

            keyboard::send_ctrl_v()?;
        },

        "ctrl+c" => {
            keyboard::send_ctrl_c()?;
        },

        "ctrl+a" => {
            keyboard::send_ctrl_a()?;
        },

        "enter" => {
            if !ask_execution_permission("нажатие клавиши Enter") {
                return Err("Отказано в доступе: Пользователь запретил выполнение команды.".to_string());
            }

            keyboard::send_enter()?;
        },

        "ctrl+enter" => {
            if !ask_execution_permission("нажатие клавиши Enter") {
                return Err("Отказано в доступе: Пользователь запретил выполнение команды.".to_string());
            }

            keyboard::send_ctrl_enter()?;
        },

        "alt+f4" => {
            keyboard::send_alt_f4()?;
        },

        "esc" => {
            keyboard::send_esc()?;
        },

        "backspace" => {
            if !ask_execution_permission("нажатие клавиши Backspace") {
                return Err("Отказано в доступе: Пользователь запретил выполнение команды.".to_string());
            }

            keyboard::send_vk_press(0x08)?;
        },

        "delete" => {
            if !ask_execution_permission("нажатие клавиши Delete") {
                return Err("Отказано в доступе: Пользователь запретил выполнение команды.".to_string());
            }

            keyboard::send_vk_press(0x2E)?;
        },

        "left" => {
            keyboard::send_vk_press(0x25)?;
        },

        "right" => {
            keyboard::send_vk_press(0x27)?;
        },

        "up" => {
            keyboard::send_vk_press(0x26)?;
        },

        "down" => {
            keyboard::send_vk_press(0x28)?;
        },

        "pageup" => {
            keyboard::send_vk_press(0x21)?;
        },

        "pagedown" => {
            keyboard::send_vk_press(0x22)?;
        },

        "tab" => {
            keyboard::send_vk_press(0x09)?;
        },

        "shift+tab" => {
            keyboard::send_shift_tab()?;
        },

        _ => {
            return Err(format!(
                "Неподдерживаемая клавиша/комбинация: '{}'. Разрешены: Ctrl+V, Ctrl+C, Ctrl+A, Enter, Right",
                raw
            ));
        }
    }   // match

    // Дать UI время отработать нажатие клавиши.
    sleep(Duration::from_millis(20));

    // 4) Отчет.
    let out = format!("Нажата клавиша/комбинация: '{}'", raw);
    Ok(wrap_in_fence(&out))
}   // press_key()

/// Описание: Нажимает одну и ту же клавишу несколько раз.
///
/// # Параметры
/// - `params`: `["<key>", "<count>"]`
///   - `<key>`: строковое имя клавиши;
///   - `<count>`: число нажатий (`usize`, должно быть >= 1).
///
/// # Нормализация входа
/// Перед распознаванием имя клавиши нормализуется:
/// - `trim()`
/// - `to_ascii_lowercase()`
/// - удаление пробелов `' '`
///
/// То есть `" Right_Arrow "` будет распознано как `"right_arrow"`.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - неверное число параметров;
/// - `<count>` не число или `<count> == 0`;
/// - клавиша не поддерживается;
/// - не удалось отправить события через SendInput.
///
/// # Возвращаемое значение
/// Markdown-блок с подтверждением выполненного действия.
fn press_n_keys(params: &Option<Vec<String>>) -> Result<String, String> {

    // 1) Валидация параметров.
    handler::check_param_count(params, 2)?;
    let raw_key: String = handler::check_param_type(params, 0)?;
    let count: usize = handler::check_param_type(params, 1)?;

    if count == 0 {
        return Err("Число нажатий должно быть >= 1".to_string());
    }   // if

    // 2) Нормализация имени клавиши.
    let token = raw_key.trim().to_ascii_lowercase().replace(' ', "");

    // 3) Для потенциально изменяющих клавиш запрашиваем разрешение один раз перед серией нажатий.
    match token.as_str() {
        "backspace" => {
            if !ask_execution_permission("многократное нажатие клавиши Backspace") {
                return Err("Отказано в доступе: Пользователь запретил выполнение команды.".to_string());
            }
        },

        "delete" => {
            if !ask_execution_permission("многократное нажатие клавиши Delete") {
                return Err("Отказано в доступе: Пользователь запретил выполнение команды.".to_string());
            }
        },

        "left" | "right" | "up" | "down" | "pageup" | "pagedown" | "tab" | "shift+tab" => {
            /* разрешение не требуется */
        },

        _ => {
            return Err(format!(
                "Неподдерживаемая клавиша для press_n_keys: '{}'. Разрешены: Right, Right_Arrow, Backspace, Delete",
                raw_key
            ));
        }
    }   // match

    // 4) Выполняем серию нажатий.
    for _ in 0..count {
        match token.as_str() {
            
            "backspace" => {
                keyboard::send_vk_press(0x08)?;
            },

            "delete" => {
                keyboard::send_vk_press(0x2E)?;
            },

            "left" => {
                keyboard::send_vk_press(0x25)?;
            },

            "right" => {
                keyboard::send_vk_press(0x27)?;
            },

            "up" => {
                keyboard::send_vk_press(0x26)?;
            },

            "down" => {
                keyboard::send_vk_press(0x28)?;
            },

            "pageup" => {
                keyboard::send_vk_press(0x21)?;
            },

            "pagedown" => {
                keyboard::send_vk_press(0x22)?;
            },

            "tab" => {
                keyboard::send_vk_press(0x09)?;
            },

            "shift+tab" => {
                keyboard::send_shift_tab()?;
            },

            _ => {
                return Err(format!(
                    "Неподдерживаемая клавиша для press_n_keys: '{}'. Разрешены: Right, Right_Arrow, Backspace, Delete",
                    raw_key
                ));
            }
        }   // match

        // Дать UI время отработать одно нажатие.
        sleep(Duration::from_millis(20));
    }   // for

    // 5) Отчет.
    let out = format!(
        "Нажата клавиша: '{}', count={}",
        raw_key,
        count
    );
    Ok(wrap_in_fence(&out))
}   // press_n_keys()

//--------------------------------------------------------------------------------------------------
//                  Внутренние утилиты
//--------------------------------------------------------------------------------------------------

/// Форматирует HWND в hex-виде для отчётов.
fn _hwnd_to_hex(hwnd: HWND) -> String {
    format!("0x{:X}", hwnd.0 as usize)
}   // _hwnd_to_hex()