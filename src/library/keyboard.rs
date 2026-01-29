//! keyboard.rs

use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;
use crate::library::keyboard::keyboard_backend::{send_key_combo, send_key_press};

mod keyboard_backend;

/// Описание: Нажимает и отпускает клавишу по её VK-коду (press).
///
/// # Параметры
/// - `vk`: виртуальный код клавиши (VK_*), например:
///   - Ctrl = 0x11
///   - Enter = 0x0D
///
/// # Ошибки
/// Возвращает `Err(String)`, если SendInput не смог отправить события.
pub fn send_vk_press(vk: u16) -> Result<(), String> {
    send_key_press(VIRTUAL_KEY(vk))
}   // send_vk_press()

/// Описание: Отправляет комбинацию вида "модификаторы + основная клавиша" по VK-кодам.
///
/// Примеры:
/// - modifiers=[0x11], key=0x56  -> Ctrl+V
/// - modifiers=[0x11,0x10], key=0x53 -> Ctrl+Shift+S
///
/// # Параметры
/// - `modifiers`: VK-коды модификаторов (Ctrl/Shift/Alt/Win и т.п.)
/// - `key`: VK-код основной клавиши
///
/// # Ошибки
/// Возвращает `Err(String)`, если SendInput не смог отправить события.
pub(crate) fn send_vk_combo(modifiers: &[u16], key: u16) -> Result<(), String> {

    // Переводим u16 -> VIRTUAL_KEY. Нужен Vec, потому что send_key_combo принимает срез.
    let mods: Vec<VIRTUAL_KEY> = modifiers.iter().map(|&v| VIRTUAL_KEY(v)).collect();

    send_key_combo(&mods, VIRTUAL_KEY(key))

}   // send_vk_combo()

/// Описание: Отправляет нажатие Enter (press) в текущий фокус.
///
/// Использует виртуальный код VK_RETURN (0x0D), что достаточно для обычных окон
/// с текстовым вводом.
///
/// # Ошибки
/// Возвращает `Err(String)`, если SendInput не смог отправить события.
pub(crate) fn send_enter() -> Result<(), String> {
    // VK_RETURN = 0x0D
    let vk_return = VIRTUAL_KEY(0x0D);
    send_key_press(vk_return)
}   // send_enter()

/// Описание: Отправляет комбинацию Ctrl+V в текущий фокус.
///
/// # Ошибки
/// Возвращает `Err(String)`, если SendInput вернул 0 или отправил не все события.
pub(crate) fn send_ctrl_v() -> Result<(), String> {
    // VK_CONTROL = 0x11, 'V' = 0x56
    let vk_ctrl = VIRTUAL_KEY(0x11);
    let vk_v = VIRTUAL_KEY(0x56);

    send_key_combo(&[vk_ctrl], vk_v)
}   // send_ctrl_v()

/// Описание: Отправляет комбинацию Ctrl+A в текущий фокус (выделить всё).
///
/// # Ошибки
/// Возвращает `Err(String)`, если SendInput вернул 0 или отправил не все события.
pub(crate) fn send_ctrl_a() -> Result<(), String> {
    // VK_CONTROL = 0x11, 'A' = 0x41
    let vk_ctrl = VIRTUAL_KEY(0x11);
    let vk_a = VIRTUAL_KEY(0x41);

    send_key_combo(&[vk_ctrl], vk_a)
}   // send_ctrl_a()

/// Описание: Отправляет комбинацию Ctrl+C в текущий фокус (копировать).
///
/// # Ошибки
/// Возвращает `Err(String)`, если SendInput вернул 0 или отправил не все события.
pub(crate) fn send_ctrl_c() -> Result<(), String> {
    // VK_CONTROL = 0x11, 'C' = 0x43
    let vk_ctrl = VIRTUAL_KEY(0x11);
    let vk_c = VIRTUAL_KEY(0x43);

    send_key_combo(&[vk_ctrl], vk_c)
}   // send_ctrl_c()

/// Описание: Отправляет нажатие стрелки вправо (Right Arrow).
///
/// Обычно используется для снятия выделения после Ctrl+A.
/// Курсор при этом встанет в конец выделенного текста.
///
/// # Ошибки
/// Возвращает `Err(String)`, если SendInput не смог отправить события.
pub(crate) fn send_right_arrow() -> Result<(), String> {
    // VK_RIGHT = 0x27
    let vk_right = VIRTUAL_KEY(0x27);
    send_key_press(vk_right)
}   // send_right_arrow()