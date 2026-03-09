//! window_backend.rs — Внутренние Win32-примитивы для `library::window`.
//!
//! # ОПИСАНИЕ
//! Модуль содержит низкоуровневые функции (WinAPI), вынесенные из `library::window`,
//! чтобы разделить публичный API и “грязную” платформенную реализацию.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! 1) Перечисление и поиск top-level окон:
//!    - контекст и callback для `EnumWindows`,
//!    - поиск окна по подстроке заголовка (needle) с проверкой однозначности.
//!
//! 2) Сбор информации по окну:
//!    - получение заголовка через `GetWindowTextW`,
//!    - геометрия через `DwmGetWindowAttribute`,
//!    - признаки foreground/minimized.
//!
//! 3) Примитив фокусировки окна (одна попытка):
//!    - `SetForegroundWindow` + верификация через `GetForegroundWindow`,
//!    - восстановление свернутого окна (SW_RESTORE).
//!
//! 4) Best-effort поддержка фокусировки “чужого” окна:
//!    - ручная загрузка `AttachThreadInput` из `user32.dll`.
//!
//! 5) Верификация вставки текста в текущем фокусе:
//!    - запуск Ctrl+A / Ctrl+C,
//!    - ожидание “стабилизации” clipboard по длине,
//!    - сравнение хвоста буфера с ожидаемым текстом (игнорируя whitespace, с лимитом длины).
//!
//! # ИНВАРИАНТЫ
//! - Функции модуля не предназначены для прямого вызова извне `library::window` (internal backend).
//! - Callback `EnumWindows` не делает ранний выход через `FALSE`, чтобы не ломать трактовку результата.
//!
//! # ПРИМЕЧАНИЯ
//! - Логика “публичных” ретраев/таймаутов и политики ошибок находится в `library::window`.
//! - Верификация текстового ввода опирается на эвристики (тайминги/ожидание clipboard),
//!   т.к. момент применения Ctrl+V / Ctrl+C из реального UI точно определить нельзя.

use std::ffi::CString;
use std::thread::sleep;
use std::time::Duration;
use windows::core::BOOL;
use windows::Win32::Foundation::{HWND, LPARAM, RECT};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS};
use windows::Win32::System::DataExchange::{GetClipboardOwner, GetClipboardSequenceNumber};
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextLengthW,
                                              GetWindowTextW, IsIconic, IsWindowVisible,
                                              SetForegroundWindow, ShowWindow, SW_RESTORE};
use crate::library::window::{get_foreground_window_info, get_window_list, WindowInfo};
use crate::{handle_log, wrln};
use crate::glob::substring;
use crate::library::clipboard;

/// Контекст перечисления окон для callback `EnumWindows`.
///
/// # Поля
/// - `needle`:
///   - `Some("...")` => добавляем в список только окна, у которых title содержит needle;
///   - `None` => добавляем окна согласно флагам include_*.
/// - `include_invisible`:
///   - `true` => включать невидимые окна (IsWindowVisible == false),
///   - `false` => включать только видимые.
/// - `include_empty_title`:
///   - `true` => включать окна с пустым заголовком,
///   - `false` => исключать окна с пустым заголовком.
/// - `hwnd_vec`: накопленный список найденных HWND.
pub(super) struct _FindWindowCtx<'a> {
    pub(super) needle: Option<&'a str>,      // Подстрока фильтрации по заголовку (опционально).
    pub(super) include_invisible: bool,      // Включать невидимые окна?
    pub(super) include_empty_title: bool,    // Включать окна с пустым заголовком?
    pub(super) hwnd_vec: Vec<HWND>,          // Результат: список hwnd.
}   // _FindWindowCtx

/// Описание: Проверяет, что в текущем текстовом поле (в текущем фокусе) лежит `text_expected`.
///
/// # Механика вставки/копирования буфера.
/// При эмуляции нажатий клавиш они ставятся в очередь и выполняются асинхронно, но строго последовательно.
/// Во время выполнения клавишных операций содержимое буфера менять нельзя, чтобы не нарушить их работу.
/// Например, если до завершения выполнения вставки Ctrl+V (которая работает крайне медленно) перезаписать
/// буфер, то будет вставлено новое значение. Момент же окончания работы клавиш вообще и вставки
/// в частности, нам неизвестен. Приходится полагаться на таймауты.
///
/// # Важно
/// - Функция НЕ умеет сама наводить фокус на нужное поле. Она работает с тем, что уже в фокусе.
/// - Функция ДЕСТРУКТИВНА к выделению: делает Ctrl+A (выделит всё).
/// - Функция НЕ ИЗМЕНЯЕТ clipboard, только периодически читает его содержимое, чтобы понять на каком
///   этапе мы находимся.
///
/// # Алгоритм работы
/// Перед входом в буфер был положен нужный текст (кстати, тот же что и в `text_expected`) и была
/// нажата комбинация Ctrl+V. Нажатие встало в очередь, но вставка отработает очень нескоро.
///
/// 1) Ctrl+A (выделить всё). Комбинация ставится в очередь вслед за командой вставки, а мы идем дальше.
/// 2) Ctrl+C (скопировать выделение в clipboard). Ставится в очередь вслед за Ctrl+A.
/// 3) Ждем когда закончится операция вставки и, по крайней мере, начнет работать копирование в буфер.
///    Здесь опираемся на номер изменения clipboard (GetClipboardSequenceNumber()). Как только он
///    изменился, считаем что началось копирование из поля в буфер обмена.
/// 4) В цикле читаем clipboard и смотрим, пуст ли он, и, если не пустой, перестала ли расти длина
///    текста в нем. Когда перестала, считаем что буфер прочтен полностью.
/// 5) (best effort) Снять выделение стрелкой вправо.
/// 6) Приступаем к сравнению: просматриваем `text_expected` и copied с конца, отбрасывая пробельные
///    символы. Длина сравнения ограничена, чтобы не навредить производительности.
///
/// # Параметры
/// - `text_expected`: Текст для проверки.
///
/// # Возвращаемое значение
/// `true` — если копия из поля совпала с `text_expected`.
/// `false` — при любом сбое или несовпадении.
///
/// # Примечания
/// - Сравнение включает нормализацию `\r\n -> \n` и обрезку хвостовых `\r/\n`,
///   т.к. браузер/clipboard часто меняют представление перевода строк.
pub(super) fn _verify_focused_textinput(text_expected: &str) -> bool {

    // Взять номер изменения clipboard.
    let mut init_clip_seq: u32;
    unsafe {
        init_clip_seq = GetClipboardSequenceNumber();
    }

    // 1) Выделить всё.
    if crate::library::keyboard::send_ctrl_a().is_err() {
        return false;
    }   // if

    // 2) Скопировать выделение в clipboard.
    if crate::library::keyboard::send_ctrl_c().is_err() {
        return false;
    }   // if

    // 3) В течение разумного времени проверяем номер clipboard. Если он изменился, то считаем,
    // что началось копирование из поля в clipboard.
    'wait_for_clipboard_change: {
        for i in 0..500 {
            unsafe {
                let cur_clip_seq = GetClipboardSequenceNumber();
// let win = get_foreground_window_info().unwrap();
// let text = clipboard::get_clipboard_text().unwrap_or("не определен".to_string());
// handle_log!("i={}, cur_seq='{}', init_seq='{}', hwnd='{:?}', title='{}', text=:\n'{}'",
//     i, cur_clip_seq, init_clip_seq, win.hwnd, win.title, substring(&text, 0, Some(50)));
                if cur_clip_seq != init_clip_seq {
                    // Номер изменился, заканчиваем ожидание.
                    break 'wait_for_clipboard_change;
                }
                sleep(Duration::from_millis(10));
            }
        }

        // Таймаут прошел, клипборд не обновился. Считаем что вставка ушла в пустоту.
        return false;
    }

    // 4) Читать clipboard. Считаем, что заполнение буфера обмена происходит медленно, поэтому ждем,
    // пока длина текста перестанет изменяться. Это касается пустого буфера. Если буфер не
    // заполняется, это приводит к завершению ожидания.
    let mut copied = String::new();
    let mut clip_last_len: usize = 0;
    let mut clip_last_mut_i = 0;
    const FINISH_CRIT: i32 = 10;     // 10 циклов клипборд не изменялся - принят полностью.
    for i in 0..200 {
        sleep(Duration::from_millis(10));
        copied = match clipboard::get_clipboard_text() {
            Ok(s) => s,
            Err(_) => continue,
        };
        if copied.len() != clip_last_len {
            clip_last_len = copied.len();
            clip_last_mut_i = i;
        } else if i - clip_last_mut_i >= FINISH_CRIT {
            // Длина текста в буфере обмена стабилизировалась, выходим.
            break;
        }
    }

    // 5) (best effort) Снять выделение, чтобы поле не оставалось “синим” (точнее, запустить снятие).
    let _ = crate::library::keyboard::send_right_arrow();

    // 6) Сравнение после нормализации.
    const VERIFY_TAIL_CHARS: usize = 100;

    // true если совпал expected с хвостом copied (с конца, без whitespace), либо совпали последние
    // 100 значимых символов.
    _tail_matches_expected_ignore_ws(&copied, text_expected, VERIFY_TAIL_CHARS)
}   // _verify_focused_textinput()

/// Описание: Проверяет совпадение хвоста `copied` с хвостом `text_expected`,
/// сравнивая строки с конца и игнорируя все пробельные символы.
///
/// # Правило
/// - Идём с конца `text_expected`, пропуская `char::is_whitespace()`.
/// - Для каждого значимого символа expected берём следующий значимый символ из `copied` и сравниваем.
/// - При первом несовпадении возвращаем `false`.
/// - Если `text_expected` длинный, достаточно совпадения последних `max_chars` значимых символов.
///
/// # Параметры
/// - `copied`: проверяемый текст.
/// - `text_expected`: образец
/// - `max_chars`: максимальное число проверяемых (с конца) символов.
///
/// # Возвращаемое значение
/// - `true`: если совпали все значимые символы expected (если их < max_chars),
///          либо совпали последние max_chars значимых символов expected.
/// - `false`: если `copied` закончился раньше или найдено несовпадение.
fn _tail_matches_expected_ignore_ws(copied: &str, text_expected: &str, max_chars: usize) -> bool {

    // Итераторы “с конца”.
    let mut exp_it = text_expected.chars().rev();
    let mut cop_it = copied.chars().rev();

    let mut matched: usize = 0;

    // Берём очередной “значимый” символ expected.
    while let Some(e_ch) = exp_it.find(|c| !c.is_whitespace()) {

        // Берём очередной “значимый” символ copied.
        let Some(c_ch) = cop_it.find(|c| !c.is_whitespace()) else {
            return false;
        };

        // Сравнение.
        if c_ch != e_ch {
            return false;
        }   // if

        matched += 1;

        // Достигли лимита — считаем, что проверка успешна.
        if matched >= max_chars {
            return true;
        }   // if

    }   // while

    // expected закончился раньше лимита — значит весь expected совпал.
    true

}   // _tail_matches_expected_ignore_ws()

#[cfg(test)]
#[test]
fn test_empty_str() {
    assert!(_tail_matches_expected_ignore_ws("", "", 100));
}

/// Находит окно по подстроке заголовка (needle) ровно в одном экземпляре.
///
/// # Логика
/// - Вызывает `get_window_list(Some(needle), include_invisible=false, include_empty_title=true)`.
/// - Если найдено:
///   - 0 окон => Err
///   - 1 окно => Ok(WindowInfo)
///   - >1 окон => Err (двусмысленность)
///
/// # Возвращаемое значение
/// Тип: `WindowInfo`: Информация о найденном окне.
///
/// # Ошибки
/// Возвращает `Err(String)`, если окно не найдено или найдено более одного окна.
pub(super) fn _find_window_by_needle(needle: &str) -> Result<WindowInfo, String> {

    // Защита: пустая needle — программная/логическая ошибка вызова.
    if needle.trim().is_empty() {
        return Err("needle пустой: нечего искать".to_string());
    }   // if

    // Поиск делаем среди видимых окон. Невидимые обычно не имеют смысла для фокуса/вставки.
    // Пустые заголовки не матчнут needle, но оставляем include_empty_title=true для нейтральности.
    let mut list = get_window_list(Some(needle), false, true)?;

    if list.len() == 1 {

        // Возвращаем WindowInfo найденного окна (изымаем из вектора).
        Ok(list.remove(0))

    } else if list.is_empty() {

        Err(format!(
            "{}, {}: окно не найдено по подстроке в заголовке: '{}'",
            file!(), line!(), needle
        ))

    } else {

        Err(format!(
            "{}, {}: найдено по подстроке '{}' более одного ({}) окна",
            file!(), line!(), needle, list.len()
        ))

    }   // if
}   // _find_window_by_needle()

/// Описание: Возвращает расширенную информацию об окне.
///
/// # Параметры
/// - `hwnd`: Хэндл окна.
///
/// # Ошибки
/// Возвращает `Err(String)`, если hwnd == NULL (защита от программной ошибки).
pub(super) fn _get_window_info(hwnd: HWND) -> Result<WindowInfo, String> {

    if hwnd.0.is_null() {
        return Err("hwnd == NULL".to_string());
    }   // if

    // 1) Заголовок окна (может быть пустым — это нормально).
    let title = _get_window_title(hwnd);

    // 2) Признак foreground.
    let fg_hwnd = unsafe { GetForegroundWindow() };
    let is_foreground = !fg_hwnd.0.is_null() && fg_hwnd == hwnd;

    // 3) Признак minimized.
    let is_minimized = unsafe { IsIconic(hwnd).as_bool() };

    // 4) Геометрия окна в координатах виртуального рабочего стола, без теней.
    let mut rect = RECT::default();
    unsafe {
        DwmGetWindowAttribute(
            hwnd,
            DWMWA_EXTENDED_FRAME_BOUNDS,
            &mut rect as *mut _ as *mut _,
            size_of::<RECT>() as u32,
        ).map_err(|e| format!(
            "{}, {}: DwmGetWindowAttribute(DWMWA_EXTENDED_FRAME_BOUNDS) failed: {}", file!(), line!(), e
        ))?;
    }   // unsafe

    let w_i32 = rect.right - rect.left;
    let h_i32 = rect.bottom - rect.top;

    // Защита от мусорной геометрии/переполнений.
    if w_i32 < 0 || h_i32 < 0 {
        return Err(format!(
            "некорректный RECT: left={}, top={}, right={}, bottom={}",
            rect.left, rect.top, rect.right, rect.bottom
        ));
    }   // if

    let width = u32::try_from(w_i32)
        .map_err(|_| format!("width не укладывается в u32: {}", w_i32))?;

    let height = u32::try_from(h_i32)
        .map_err(|_| format!("height не укладывается в u32: {}", h_i32))?;

    Ok(WindowInfo {
        hwnd,
        title,
        is_foreground,
        is_minimized,
        x: rect.left,
        y: rect.top,
        width,
        height,
    })
}   // _get_window_info()

/// Читает заголовок окна (Win32 title bar text) как `String`.
///
/// # Алгоритм работы
/// - Получает длину заголовка через `GetWindowTextLengthW`.
/// - Выделяет буфер UTF-16 длиной `len + 1` (под завершающий `\0`).
/// - Читает текст через `GetWindowTextW`.
/// - Конвертирует в `String` через `from_utf16_lossy`.
///
/// # Возвращаемое значение
/// Заголовок окна. Может быть пустым.
///
/// # Побочные эффекты
/// - Нет.
pub(super) fn _get_window_title(hwnd: HWND) -> String {
    unsafe {
        // 1) Длина заголовка в UTF-16 код-юнитах (без завершающего '\0').
        let len = GetWindowTextLengthW(hwnd);
        if len <= 0 {
            return String::new();
        }   // if

        // 2) Буфер +1 под '\0', который пишет GetWindowTextW.
        let mut buf: Vec<u16> = vec![0u16; (len as usize) + 1];

        // 3) copied — количество записанных символов без '\0'.
        let copied = GetWindowTextW(hwnd, &mut buf);
        if copied <= 0 {
            return String::new();
        }   // if

        // 4) Конвертация UTF-16 -> String.
        String::from_utf16_lossy(&buf[..(copied as usize)])
    }   // unsafe
}   // _get_window_title()

/// Одна попытка фокусировки окна (без циклических ретраев).
///
/// # Алгоритм работы
/// - Если окно уже foreground — успех.
/// - Если окно свернуто — восстанавливает (SW_RESTORE) и ждёт небольшую паузу.
/// - Вызывает `SetForegroundWindow`.
/// - Проверяет, что `GetForegroundWindow() == hwnd`.
///
/// # Ошибки
/// Возвращает `Err(String)`, если `SetForegroundWindow` не сработал или окно не стало foreground.
pub(super) fn _focus_window(hwnd: HWND) -> Result<(), String> {
    unsafe {
        // // Быстрый путь: окно уже в foreground.
        if GetForegroundWindow() == hwnd {
            return Ok(());
        }   // if

        // Если окно свернуто — восстановить. Это повышает шанс корректной фокусировки.
        if IsIconic(hwnd).as_bool() {
            let _ = ShowWindow(hwnd, SW_RESTORE);

            // Ждём, чтобы окно успело развернуться и обработать изменения.
            sleep(Duration::from_millis(200));
        }   // if

        // Попытаться перевести окно в foreground (может быть запрещено политикой Windows).
        let ok = SetForegroundWindow(hwnd);
        if !ok.as_bool() {
            return Err("SetForegroundWindow вернул FALSE (Windows мог запретить перевод в foreground)".to_string());
        }   // if
    }   // unsafe

    unsafe {
        // Верификация: foreground действительно поменялся на нужный hwnd.
        if GetForegroundWindow() == hwnd {
            Ok(())
        } else {
            Err(format!("окно \"{}\" не получило фокус", hwnd.0 as isize))
        }   // if
    }   // unsafe
}   // _focus_window()

/// Callback `EnumWindows`: собирает hwnd окон, фильтруя по `needle` и флагам include_*.
///
/// # ВАЖНО
/// Callback всегда возвращает TRUE, чтобы `EnumWindows` завершался штатно.
/// Ранний выход через FALSE в `windows-rs` может интерпретироваться как ошибка перечисления.
pub(super) unsafe extern "system" fn __find_by_needle_enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {

    // Восстанавливаем указатель на контекст из LPARAM.
    let ctx_ptr = lparam.0 as *mut _FindWindowCtx;

    #[cfg(debug_assertions)]
    if ctx_ptr.is_null() {
        // В debug это сигнализирует о программной ошибке.
        return BOOL(0);
    }   // if

    // UNSAFE: разыменование указателя контекста.
    let ctx = &mut *ctx_ptr;

    // 1) Фильтр по видимости.
    if !ctx.include_invisible {
        if !IsWindowVisible(hwnd).as_bool() {
            return BOOL(1);
        }   // if
    }   // if

    // 2) Заголовок нужен если:
    // - задан needle (нужно title.contains),
    // - или запрещены пустые заголовки (нужно проверить title.is_empty()).
    let need_title_check = ctx.needle.is_some() || !ctx.include_empty_title;

    if need_title_check {

        // Берём заголовок окна (может быть пустым).
        let title = _get_window_title(hwnd);

        // 2.1) Отсекаем пустые заголовки, если они запрещены.
        if !ctx.include_empty_title && title.is_empty() {
            return BOOL(1);
        }   // if

        // 2.2) Фильтрация по needle.
        if let Some(needle) = ctx.needle {
            if !title.contains(needle) {
                return BOOL(1);
            }   // if
        }   // if let Some

    }   // if need_title_check

    // 3) Окно удовлетворило фильтрам — добавляем в список.
    ctx.hwnd_vec.push(hwnd);

    // Продолжаем перечисление.
    BOOL(1)
}   // __find_by_needle_enum_windows_callback()

// Определение типа функции AttachThreadInput
type AttachThreadInputFn = unsafe extern "system" fn(u32, u32, bool) -> bool;

/// Ручная загрузка AttachThreadInput для обхода конфликтов версий.
/// Позволяет прицепиться к потоку окна для управления его фокусом.
pub(super) unsafe fn _manual_attach_thread_input(id_attach: u32, id_attach_to: u32, attach: bool) {
    // 1. Готовим имена библиотеки и функции в формате C-строк (с нулем на конце)
    let lib_name = CString::new("user32.dll").unwrap();
    let func_name = CString::new("AttachThreadInput").unwrap();

    // 2. LoadLibraryA: Просим Windows загрузить системную библиотеку user32.dll в память.
    // Если она уже загружена (а она загружена всегда), мы просто получаем её адрес (h_module).
    if let Ok(h_module) = LoadLibraryA(windows::core::PCSTR(lib_name.as_ptr() as *const u8)) {

        // 3. GetProcAddress: Ищем внутри этой библиотеки адрес нужной нам функции по имени.
        // Windows возвращает нам просто адрес в памяти (число).
        if let Some(addr) = GetProcAddress(h_module, windows::core::PCSTR(func_name.as_ptr() as *const u8)) {

            // 4. std::mem::transmute: Самая опасная магия.
            // Мы говорим Rust'у: "Смотри, вот этот адрес памяти `addr` — это на самом деле функция,
            // которая принимает (u32, u32, bool). Верь мне!"
            // Мы превращаем адрес в вызываемую функцию `func`.
            let func: AttachThreadInputFn = std::mem::transmute(addr);

            // 5. Вызываем функцию!
            func(id_attach, id_attach_to, attach);
        }   // if
    }   // if
}   // manual_attach_thread_input()
