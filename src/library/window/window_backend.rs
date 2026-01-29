
//--------------------------------------------------------------------------------------------------
//                  Внутренний интерфейс
//--------------------------------------------------------------------------------------------------

use std::ffi::CString;
use std::thread::sleep;
use std::time::Duration;
use windows::core::BOOL;
use windows::Win32::Foundation::{HWND, LPARAM, RECT};
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowRect, GetWindowTextLengthW, GetWindowTextW, IsIconic, IsWindowVisible, SetForegroundWindow, ShowWindow, SW_RESTORE};
use crate::library::window::{get_window_list, WindowInfo};

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
/// # Важно
/// - Функция НЕ умеет сама наводить фокус на нужное поле. Она работает с тем, что уже в фокусе.
/// - Функция ДЕСТРУКТИВНА к выделению: делает Ctrl+A (выделит всё).
/// - Функция ТРОГАЕТ clipboard:
///   - сначала кладет маркер,
///   - затем делает Ctrl+C (копирование),
///   - затем читает текст из clipboard.
///   Восстановление clipboard делается ВНЕ этой функции (внешним уровнем).
///
/// # Алгоритм работы
/// 1) Положить в clipboard “маркер”, чтобы отличить “Ctrl+C не сработал” от “поле пустое/старый буфер”.
/// 2) Ctrl+A (выделить всё).
/// 3) Ctrl+C (скопировать выделение в clipboard).
/// 4) (best effort) Снять выделение стрелкой вправо.
/// 5) Прочитать clipboard и сравнить с `text_expected` (после нормализации перевода строк/хвоста).
///
/// # Возвращаемое значение
/// `true` — если копия из поля совпала с `text_expected`.
/// `false` — при любом сбое или несовпадении.
///
/// # Примечания
/// - Сравнение включает нормализацию `\r\n -> \n` и обрезку хвостовых `\r/\n`,
///   т.к. браузер/clipboard часто меняют представление перевода строк.
pub(super) fn _verify_focused_textinput(text_expected: &str) -> bool {

    /// Нормализует текст для сравнения (best effort).
    fn _normalize(s: &str) -> String {

        // Нормализуем CRLF -> LF (частый кейс при Ctrl+C из браузера/контролов).
        let s = s.replace("\r\n", "\n");

        // Убираем только хвостовые переводы строк.
        // Это полезно, если поле ввода добавляет trailing newline при копировании.
        s.trim_end_matches(['\r', '\n']).to_string()

    }   // _normalize()

    // Маркер в clipboard, чтобы обнаружить ситуацию, когда Ctrl+C не переписал буфер.
    // (Например: фокус ушел, браузер не дал копировать, или копирование еще не успело примениться.)
    const CLIPBOARD_MARKER: &str = "__HB_VERIFY_CLIPBOARD_MARKER__";

    // 1) Пишем маркер в clipboard.
    if crate::library::clipboard::set_clipboard_text(CLIPBOARD_MARKER).is_err() {
        return false;
    }   // if

    // 2) Выделить всё.
    if crate::library::keyboard::send_ctrl_a().is_err() {
        return false;
    }   // if

    // Небольшая пауза: UI/браузеру нужно время применить выделение.
    sleep(Duration::from_millis(20));

    // 3) Скопировать выделение в clipboard.
    if crate::library::keyboard::send_ctrl_c().is_err() {
        return false;
    }   // if

    // Пауза: clipboard обновляется не мгновенно.
    sleep(Duration::from_millis(20));

    // 4) (best effort) Снять выделение, чтобы поле не оставалось “синим”.
    // Даже если не сработает — это не должно ломать верификацию.
    let _ = crate::library::keyboard::send_right_arrow();

    // 5) Читать clipboard и сравнивать.
    let copied = match crate::library::clipboard::get_clipboard_text() {
        Ok(s) => s,
        Err(_) => return false,
    };

    // Если остался маркер — значит Ctrl+C не переписал буфер. Верификация не пройдена.
    if copied == CLIPBOARD_MARKER {
        return false;
    }   // if

    // Сравнение после нормализации.
    _normalize(&copied) == _normalize(text_expected)

}   // _verify_focused_textinput_equals()

/// Находит окно по подстроке заголовка (needle) ровно в одном экземпляре.
///
/// # Логика
/// - Вызывает `get_window_list(Some(needle), include_invisible=false, include_empty_title=true)`.
/// - Если найдено:
///   - 0 окон => Err
///   - 1 окно => Ok(hwnd, title)
///   - >1 окон => Err (двусмысленность)
///
/// # Ошибки
/// Возвращает `Err(String)`, если окно не найдено или найдено более одного окна.
pub(super) fn _find_window_by_needle(needle: &str) -> Result<(HWND, String), String> {

    // Защита: пустая needle — программная/логическая ошибка вызова.
    if needle.trim().is_empty() {
        return Err("needle пустой: нечего искать".to_string());
    }   // if

    // Поиск делаем среди видимых окон. Невидимые обычно не имеют смысла для фокуса/вставки.
    // Пустые заголовки не матчнут needle, но оставляем include_empty_title=true для нейтральности.
    let list = get_window_list(Some(needle), false, true)?;

    if list.len() == 1 {

        // Возвращаем hwnd и title найденного окна.
        let w = &list[0];
        Ok((w.hwnd, w.title.clone()))

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

    // 4) Геометрия окна (в координатах виртуального рабочего стола).
    //
    // ВАЖНО:
    // - GetWindowRect возвращает outer rect (включая non-client рамки).
    // - Для minimized окон геометрия может быть не “ожидаемой” (зависит от политики DWM/OS),
    //   но для диагностики и большинства задач этого достаточно.
    let mut rect = RECT::default();
    unsafe {
        if !GetWindowRect(hwnd, &mut rect).is_ok() {
            return Err("GetWindowRect() failed".to_string());
        }   // if
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
        // Быстрый путь: окно уже в foreground.
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
