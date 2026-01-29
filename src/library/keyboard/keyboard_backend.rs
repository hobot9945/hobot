//! keyboard.rs
//!
//! ОПИСАНИЕ:
//! Низкоуровневая эмуляция клавиатуры через WinAPI.
//!
//! ОТВЕТСТВЕННОСТЬ:
//! - Отправка одиночных нажатий и комбинаций клавиш в текущий foreground/focused ввод через SendInput.
//!
//! ПРИМЕЧАНИЯ:
//! - Реализация использует виртуальные коды клавиш (VK). Для "обычных окон" этого достаточно.
//! - Если когда-нибудь понадобится "как физическая клавиатура независимо от layout", это делается через scancode
//!   и флаг KEYEVENTF_SCANCODE (см. KEYBDINPUT). [web:454]

use windows::core::Error as WinError;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput,
    INPUT,
    INPUT_0,
    INPUT_KEYBOARD,
    KEYBDINPUT,
    KEYEVENTF_KEYUP,
    VIRTUAL_KEY,
};

/// Описание: Нажимает и отпускает клавишу (press) по виртуальному коду.
///
/// # Параметры
/// - `vk`: Виртуальный код клавиши.
///
/// # Ошибки
/// Возвращает `Err(String)`, если SendInput не смог отправить события.
pub(crate) fn send_key_press(vk: VIRTUAL_KEY) -> Result<(), String> {
    let inputs = [_make_key_input(vk, false), _make_key_input(vk, true)];
    _send_inputs(&inputs)
}   // send_key_press()

/// Описание: Нажимает клавишу (key down) по виртуальному коду.
///
/// # Параметры
/// - `vk`: Виртуальный код клавиши.
///
/// # Ошибки
/// Возвращает `Err(String)`, если SendInput не смог отправить событие.
pub(crate) fn send_key_down(vk: VIRTUAL_KEY) -> Result<(), String> {
    let inputs = [_make_key_input(vk, false)];
    _send_inputs(&inputs)
}   // send_key_down()

/// Описание: Отпускает клавишу (key up) по виртуальному коду.
///
/// # Параметры
/// - `vk`: Виртуальный код клавиши.
///
/// # Ошибки
/// Возвращает `Err(String)`, если SendInput не смог отправить событие.
pub(crate) fn send_key_up(vk: VIRTUAL_KEY) -> Result<(), String> {
    let inputs = [_make_key_input(vk, true)];
    _send_inputs(&inputs)
}   // send_key_up()

/// Описание: Отправляет комбинацию вида "модификаторы + основная клавиша".
///
/// Пример: Ctrl+V, Alt+F4, Ctrl+Shift+S и т.п.
///
/// # Алгоритм работы
/// - Нажимаем все модификаторы в указанном порядке.
/// - Нажимаем основную клавишу.
/// - Отпускаем основную клавишу.
/// - Отпускаем модификаторы в обратном порядке (чтобы стек модификаторов корректно размотался).
///
/// # Параметры
/// - `modifiers`: Срез виртуальных кодов модификаторов (Ctrl/Shift/Alt/Win и т.п.).
/// - `key`: Виртуальный код основной клавиши.
///
/// # Ошибки
/// Возвращает `Err(String)`, если SendInput не смог отправить события.
pub(crate) fn send_key_combo(modifiers: &[VIRTUAL_KEY], key: VIRTUAL_KEY) -> Result<(), String> {
    let mut inputs: Vec<INPUT> = Vec::with_capacity(modifiers.len() * 2 + 2);

    for &m in modifiers {
        inputs.push(_make_key_input(m, false));
    }   // for

    inputs.push(_make_key_input(key, false));
    inputs.push(_make_key_input(key, true));

    for &m in modifiers.iter().rev() {
        inputs.push(_make_key_input(m, true));
    }   // for

    _send_inputs(&inputs)
}   // send_key_combo()

//--------------------------------------------------------------------------------------------------
//                  Внутренний интерфейс
//--------------------------------------------------------------------------------------------------

/// Описание: Универсальная отправка массива INPUT через SendInput.
///
/// # Параметры
/// - `inputs`: Массив событий клавиатуры/мыши.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - SendInput вернул 0 (ошибка, читаем GetLastError).
/// - SendInput отправил не все события.
fn _send_inputs(inputs: &[INPUT]) -> Result<(), String> {
    let sent = unsafe { SendInput(inputs, size_of::<INPUT>() as i32) };

    if sent == 0 {
        let win32 = unsafe { GetLastError() };
        let e = WinError::from(win32);
        return Err(format!("SendInput вернул 0: {}", e));
    }   // if

    if sent as usize != inputs.len() {
        return Err(format!("SendInput отправил не все события: {}/{}", sent, inputs.len()));
    }   // if

    Ok(())
}   // send_inputs()

/// Описание: Строит INPUT для клавиатурного события по виртуальному коду.
///
/// # Параметры
/// - `vk`: Виртуальный код клавиши.
/// - `is_key_up`: true => отпускание клавиши; false => нажатие клавиши.
///
/// # Возвращаемое значение
/// Type: INPUT: Структура события для SendInput.
fn _make_key_input(vk: VIRTUAL_KEY, is_key_up: bool) -> INPUT {
    let flags = if is_key_up { KEYEVENTF_KEYUP } else { Default::default() };

    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}   // make_key_input()
