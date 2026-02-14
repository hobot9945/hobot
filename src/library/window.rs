//! window — Win32-утилиты поиска, фокусировки и инвентаризации окон.
//!
//! # ОПИСАНИЕ
//! Модуль содержит набор функций для работы с внешними top-level окнами Windows:
//! - получение заголовка окна (Win32 title bar text) через `GetWindowTextW`;
//! - перечисление видимых окон через `EnumWindows`;
//! - поиск окна по подстроке заголовка (needle);
//! - попытка вывести окно в foreground и дать ему фокус (best effort).
//!
//! Модуль используется агентом для:
//! - фокусировки вкладки/окна AI перед вставкой отчёта;
//! - поиска и фокусировки произвольных окон по запросу AI;
//! - диагностики: получение списка видимых окон.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! 1. Перечисление окон:
//!    - `EnumWindows` + фильтр `IsWindowVisible`.
//!    - Опциональная фильтрация по `title.contains(needle)`.
//!
//! 2. Фокусировка окна:
//!    - восстановление свернутого окна (minimized) перед попыткой фокусировки;
//!    - вызов `SetForegroundWindow`;
//!    - проверка результата через `GetForegroundWindow` (polling с таймаутом).
//!
//! 3. Упаковка результата:
//!    - формирование структуры `WindowInfo` (hwnd, title и признаки состояния).
//!
//! # ИНВАРИАНТЫ
//! - Callback `EnumWindows` **не** делает “ранний выход” через `FALSE`, чтобы не ломать трактовку
//!   результата в `windows-rs`.
//! - Окна фильтруются по `IsWindowVisible`, чтобы не цеплять служебные/скрытые HWND.
//! - Фокусировка выполняется в цикле ретраев, т.к. окна (особенно браузеры) часто не готовы
//!   к foreground сразу после создания/обновления страницы.
//!
//! # ПРИМЕЧАНИЯ
//! - Успех `SetForegroundWindow` может блокироваться политикой Windows. Поэтому всегда есть верификация
//!   через `GetForegroundWindow` и повторы по таймеру.
//! - Заголовок окна может быть пустым — это нормальный случай для некоторых системных окон.mod test_window_test;

pub mod window_backend;
#[cfg(test)]
mod test_window_test;

use std::ffi::{c_void, CString};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use windows::core::{w, BOOL};
use windows::Win32::Foundation::{HWND, LPARAM, RECT};
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetForegroundWindow, GetGUIThreadInfo,
                                              GetWindowRect, GetWindowTextLengthW, GetWindowTextW,
                                              GetWindowThreadProcessId, IsChild, IsIconic,
                                              IsWindowVisible, IsZoomed, SetForegroundWindow,
                                              ShowWindow, GUITHREADINFO, SW_RESTORE, SW_SHOWMAXIMIZED,
                                              GUI_INMENUMODE, GUI_POPUPMENUMODE};
use crate::handle_log;
use crate::library::keyboard;
use crate::library::window::window_backend::{_FindWindowCtx, __find_by_needle_enum_windows_callback,
                                             _find_window_by_needle, _focus_window, _get_window_info,
                                             _get_window_title, _manual_attach_thread_input};

/// В течение какого времени повторяем попытки найти/сфокусировать окно.
const TRYING_PERIOD_MS: u64 = 10_000;

/// Ожидание между попытками.
const RETRY_PERIOD_MS: u64 = 100;

/// Сводная информация о top-level окне Windows.
///
/// Значения — снимок состояния на момент вызова и могут устареть сразу после возврата.
///
/// # Назначение
/// Используется для:
/// - диагностики (список видимых окон),
/// - выдачи человеку понятного статуса (foreground/minimized),
/// - дальнейших действий по hwnd.
pub(crate) struct WindowInfo {
    pub(crate) hwnd: HWND,            // Хэндл окна (HWND).
    pub(crate) title: String,         // Заголовок окна (Win32 title bar text).
    pub(crate) is_foreground: bool,   // Признак: окно сейчас в foreground.
    pub(crate) is_minimized: bool,    // Признак: окно свернуто (minimized).
    pub(crate) x: i32,                // X левого верхнего угла окна (виртуальный рабочий стол).
    pub(crate) y: i32,                // Y левого верхнего угла окна (виртуальный рабочий стол).
    pub(crate) width: u32,            // Ширина окна (в пикселях).
    pub(crate) height: u32,           // Высота окна (в пикселях).
}   // WindowInfo

/// Возвращает список top-level окон с фильтрацией по заголовку и видимости.
///
/// # Параметры
/// - `needle`: Опциональная подстрока для фильтрации по заголовку (contains).
/// - `include_invisible`: Включать невидимые окна.
/// - `include_empty_title`: Включать окна с пустым заголовком.
///
/// # Алгоритм работы
/// - Перечисляет окна через `EnumWindows`.
/// - Фильтрует окна согласно параметрам:
///   - по видимости (`IsWindowVisible`) если `include_invisible=false`,
///   - по пустому заголовку если `include_empty_title=false`,
///   - по `needle` если задан.
/// - Для каждого hwnd собирает `WindowInfo`.
///
/// # Возвращаемое значение
/// `Vec<WindowInfo>`: список окон (порядок — порядок `EnumWindows`).
///
/// # Ошибки
/// Возвращает `Err(String)`, если `EnumWindows` завершился с ошибкой.
pub(crate) fn get_window_list(needle: Option<&str>, include_invisible: bool, include_empty_title: bool)
    -> Result<Vec<WindowInfo>, String>
{
    // Контекст перечисления: callback использует его для фильтрации и накопления hwnd.
    let mut ctx = _FindWindowCtx {
        needle,
        include_invisible,
        include_empty_title,
        hwnd_vec: Vec::new(),
    };

    // EnumWindows заполнит ctx.hwnd_vec.
    unsafe {
        let lparam = LPARAM((&mut ctx as *mut _FindWindowCtx) as isize);
        EnumWindows(Some(__find_by_needle_enum_windows_callback), lparam)
            .map_err(|e| format!("{}, {}: EnumWindows() failed: {}", file!(), line!(), e))?;
    }   // unsafe

    // Превращаем HWND-ы в WindowInfo.
    let mut out: Vec<WindowInfo> = Vec::with_capacity(ctx.hwnd_vec.len());
    for hwnd in ctx.hwnd_vec {
        out.push(_get_window_info(hwnd)?);
    }   // for

    Ok(out)
}   // get_window_list()

/// Вставляет содержимое буфера обмена (Ctrl+V) в окно, найденное по `needle`. Содержимое может
/// быть любым, в том числе образом или файлом. Может быть и текстом, но для текста предусмотрены
/// специальные, более надежные (с контролем исполнения) функции.
///
/// # Алгоритм работы
/// - Находит окно по подстроке заголовка (needle).
/// - Пытается сфокусировать найденное окно.
/// - Отправляет комбинацию Ctrl+V в текущий фокус.
///
/// # Параметры
/// - `needle`: Подстрока заголовка окна (contains).
///
/// # Ошибки
/// Возвращает `Err(String)`, если окно не найдено/не удалось сфокусировать/не удалось послать Ctrl+V.
///
/// # Побочные эффекты
/// - Меняет foreground окно (фокус).
/// - Генерирует события клавиатуры (Ctrl+V).
pub(crate) fn paste_clipboard_into_window_by_needle(needle: &str) -> Result<(), String> {

    // 1) Найти окно и гарантировать фокус.
    let (_hwnd, _title) = find_window_by_needle_and_focus(needle)?;

    // 2) Вставить содержимое clipboard в текущий фокус.
    keyboard::send_ctrl_v()?;

    // 3) Небольшая пауза против гонок: UI может обработать Ctrl+V не мгновенно.
    sleep(Duration::from_millis(20));

    Ok(())
}   // paste_text_into_window_by_needle()

/// Кладёт `text` в буфер обмена и вставляет его (Ctrl+V) в окно, найденное по `needle`,
/// затем подтверждает вставку через Ctrl+A/Ctrl+C.
///
/// # Параметры
/// - `needle`: Подстрока заголовка окна (contains).
/// - `text`: Текст для вставки.
///
/// # Возвращаемое значение
/// `(HWND, String)`:
/// - `HWND`: найденный hwnd (куда вставляли),
/// - `String`: заголовок окна (как был прочитан на момент успешной фокусировки).
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - окно не найдено / найдено более одного / не удалось сфокусировать,
/// - не удалось вставить текст или подтвердить вставку.
///
/// # Побочные эффекты
/// - Фокусирует целевое окно (best effort).
/// - Временно перезаписывает системный буфер обмена.
pub(crate) fn paste_text_into_window_by_needle(needle: &str, text: &str) -> Result<(HWND, String), String> {

    // 1) Найти окно и сфокусировать его.
    let (hwnd, win_title) = find_window_by_needle_and_focus(needle)?;

    // 2) Вставить текст в текущее поле ввода (внутри этого окна) с верификацией.
    _paste_text_into_window_and_verify(hwnd, text, &win_title)?;

    Ok((hwnd, win_title))
}   // paste_text_into_window_by_needle()

/// Кладёт `text` в буфер обмена и вставляет его (Ctrl+V) в окно по `hwnd`,
/// затем подтверждает вставку через Ctrl+A/Ctrl+C.
///
/// # Параметры
/// - `hwnd`: HWND целевого окна.
/// - `text`: Текст для вставки.
///
/// # Возвращаемое значение
/// `(HWND, String)`:
/// - `HWND`: hwnd (куда вставляли),
/// - `String`: заголовок окна (как был прочитан на момент успешной фокусировки).
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - не удалось сфокусировать окно,
/// - не удалось вставить текст или подтвердить вставку.
///
/// # Побочные эффекты
/// - Фокусирует целевое окно (best effort).
/// - Временно перезаписывает системный буфер обмена.
pub(crate) fn paste_text_into_window_by_hwnd(hwnd: HWND, text: &str) -> Result<(HWND, String), String> {

    // 1) Сфокусировать окно.
    let (hwnd, win_title) = focus_window(hwnd)?;

    // 2) Контекст для ошибок.
    let hwnd_dbg = format!("hwnd=0x{:X}", hwnd.0 as usize);

    // 3) Вставить текст в текущее поле ввода (внутри этого окна) с верификацией.
    _paste_text_into_window_and_verify(hwnd, text, &hwnd_dbg)?;

    Ok((hwnd, win_title))
}   // paste_text_into_window_by_hwnd()

/// Находит окно по `needle` и пытается сфокусировать его.
///
/// # Алгоритм работы
/// - Ищет окно по `needle` (с ретраями).
/// - Пытается сфокусировать найденный hwnd (с ретраями).
///
/// # Параметры
/// - `needle`: Подстрока заголовка окна.
///
/// # Возвращаемое значение
/// `(HWND, String)`:
/// - `HWND`: найденный hwnd,
/// - `String`: заголовок окна (как был прочитан на момент фокусировки).
///
/// # Ошибки
/// Возвращает `Err(String)`, если окно не найдено или фокусировка не удалась.
pub(crate) fn find_window_by_needle_and_focus(needle: &str) -> Result<(HWND, String), String> {

    // 1) Найти окно (с ретраями)
    let (hwnd, win_title) = find_window_by_needle(needle)?;

    // 2) Сфокусировать найденное окно (с ретраями внутри focus_window)
    let _ = focus_window(hwnd)?;

    // Вообще не удалось сфокусироваться. Уходим с ошибкой.
    Ok((hwnd, win_title))
}   // find_window_by_needle_and_focus()

/// Находит окно по подстроке заголовка (needle) с ретраями по таймеру.
///
/// # Зачем отдельная функция
/// Позволяет получить `HWND` для последующего использования (скриншот окна, фокусировка и т.п.).
///
/// # Параметры
/// - `needle`: Подстрока заголовка окна.
///
/// # Возвращаемое значение
/// `(HWND, String)` — hwnd и заголовок найденного окна.
///
/// # Ошибки
/// Возвращает `Err(String)`, если окно не найдено за `TRYING_PERIOD_MS`.
pub(crate) fn find_window_by_needle(needle: &str) -> Result<(HWND, String), String> {

    let mut find_res = Err(format!("{}, {}: программная ошибка - ни одного цикла поиска окна.",
                                   file!(), line!()));

    // Цикл ретраев нужен, потому что окно может появиться не сразу (особенно браузер/веб).
    for _ in 0..TRYING_PERIOD_MS / RETRY_PERIOD_MS {

        // Пытаемся один проход перечисления окон.
        match _find_window_by_needle(needle) {
            Ok(wnd) => {

                // Успех: выходим.
                find_res = Ok(wnd);
                break;
            },
            Err(e) => {

                // Не нашли: ждём и пробуем снова.
                sleep(Duration::from_millis(RETRY_PERIOD_MS));
                find_res = Err(e);
            }
        }
    }

    find_res
}   // find_window_by_needle()

/// Пытается вывести окно в foreground и дать ему фокус (best effort) с ретраями.
///
/// # Алгоритм работы
/// - Читает заголовок окна (для сообщений об ошибках).
/// - (Опционально) временно присоединяет ввод текущего потока к потоку окна (AttachThreadInput),
///   чтобы повысить шанс успеха `SetForegroundWindow`.
/// - В цикле ретраев вызывает `_focus_window(hwnd)`.
/// - После попыток обязательно выполняет detach.
///
/// # Параметры
/// - `hwnd`: Хэндл окна.
///
/// # Возвращаемое значение
/// `(HWND, String)`:
/// - `HWND`: тот же hwnd (для удобства чейнинга),
/// - `String`: заголовок окна (как был прочитан в начале фокусировки).
///
/// # Ошибки
/// Возвращает `Err(String)`, если окно не стало foreground за `TRYING_PERIOD_MS`.
///
/// # Побочные эффекты
/// - Меняет состояние окна (разворачивание minimized).
/// - Меняет foreground окно.
/// - Временно меняет режим маршрутизации ввода потоков (AttachThreadInput).
pub(crate) fn focus_window(hwnd: HWND) -> Result<(HWND, String), String> {

    // Заголовок используем для диагностики (даже если он пустой).
    let win_title = _get_window_title(hwnd);

    // Подготовка AttachThreadInput: повышаем шанс фокусировки для чужого потока окна.
    let mut need_attach;
    let mut current_thread_id;
    let mut window_thread_id;
    unsafe {
        current_thread_id = GetCurrentThreadId();
        window_thread_id = GetWindowThreadProcessId(hwnd, None);
        need_attach = current_thread_id != window_thread_id;

        if need_attach {
            _manual_attach_thread_input(current_thread_id, window_thread_id, true);
        }   // if
    }   // unsafe

    // Цикл ретраев: окно может быть не готово к foreground немедленно.
    let mut focus_res = Err(format!("{}, {}: не удалось сфокусировать окно '{}' за отведенное время.",
                                    file!(), line!(), win_title));
    for _ in 0..TRYING_PERIOD_MS / RETRY_PERIOD_MS {
        match _focus_window(hwnd) {
            // Удалось. Запоминаем успех и выходим из цикла.
            Ok(()) => {
                focus_res = Ok((hwnd, win_title));
                break;
            },
            // Не удалось. Выжидаем и повторяем.
            Err(e) => {
                sleep(Duration::from_millis(RETRY_PERIOD_MS));
                focus_res = Err(e);
            }
        }   // match
    }   // for

    // Важно: detach делаем всегда, даже если фокусировка не удалась.
    unsafe {
        if need_attach {
            _manual_attach_thread_input(current_thread_id, window_thread_id, false);
        }   // if
    }   // unsafe

    // Дать GUI успокоиться после отключения attach. Без этого, например, не ловятся pop-up окна
    // (правая кнопка мыши). Минимальная задержка, при которой работало на моей машине - 80мс.
    sleep(Duration::from_millis(300));

    focus_res
}   // focus_window()

/// Возвращает информацию об окне, которое сейчас находится в foreground.
///
/// # Алгоритм работы
/// - Вызывает `GetForegroundWindow()`.
/// - Если HWND == NULL — возвращает ошибку.
/// - Собирает расширенную информацию через `_get_window_info(hwnd)`.
///
/// # Возвращаемое значение
/// `WindowInfo`: hwnd, title и признаки состояния.
///
/// # Ошибки
/// Возвращает `Err(String)`, если foreground окно отсутствует (NULL).
pub(crate) fn get_foreground_window_info() -> Result<WindowInfo, String> {

    // 1) Получить HWND окна в foreground (может быть NULL).
    let hwnd = unsafe { GetForegroundWindow() };

    // 2) NULL означает, что foreground окно не определено.
    if hwnd.0.is_null() {
        return Err("GetForegroundWindow вернул NULL (foreground окно отсутствует)".to_string());
    }   // if

    // 3) Собрать расширенную информацию.
    let win_info = _get_window_info(hwnd);

    win_info
}   // get_foreground_window_info()

/// Описание: Парсит HWND из строки.
///
/// Поддерживает:
/// - hex: `"0x..."` / `"0X..."`
/// - decimal: `"123456"`
///
/// # Зачем это в library/window
/// HWND — это “оконная” сущность. Парсер нужен в нескольких хэндлерах:
/// - `capture_window_by_hwnd`
/// - `focus_window_by_hwnd`
/// - любые будущие команды, работающие с HWND.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - строка не является числом (hex/decimal),
/// - значение не укладывается в `isize` (формат HWND в windows-rs).
pub(crate) fn parse_hwnd(hwnd_str: &str) -> Result<HWND, String> {

    // 1) Убираем пробелы по краям — команды часто приходят “чуть грязными”.
    // Например: " 0x1234 " или "1234\r\n".
    let s = hwnd_str.trim();

    // 2) Парсим в u64.
    // - Если есть префикс 0x/0X — считаем, что это hex.
    // - Иначе — decimal.
    //
    // Почему u64:
    // - это удобный промежуточный тип для parse,
    // - позволяет корректно обработать как hex, так и decimal,
    // - после парсинга мы уже приводим к isize (формат HWND в windows-rs).
    let val_u64 = if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {

        // 2.1) Hex формат: "0x1A2B3C"
        // strip_prefix возвращает только часть после "0x".
        u64::from_str_radix(hex, 16)
            .map_err(|e| format!("HWND: не удалось распарсить hex '{}': {}", s, e))?

    } else {

        // 2.2) Decimal формат: "123456"
        s.parse::<u64>()
            .map_err(|e| format!("HWND: не удалось распарсить decimal '{}': {}", s, e))?

    };   // if hex/decimal

    // 3) Приводим к isize.
    //
    // HWND в windows-rs представлен как HWND(isize).
    // На 64-bit Windows это обычно ок, но мы всё равно проверяем переполнение.
    let val_isize = isize::try_from(val_u64)
        .map_err(|_| format!("HWND: значение '{}' не укладывается в isize", s))?;

    // 4) Формируем HWND.
    Ok(HWND(val_isize as *mut c_void))
}   // parse_hwnd()

//--------------------------------------------------------------------------------------------------
//                  Внутренний интерфейс
//--------------------------------------------------------------------------------------------------

/// Кладёт `text` в буфер обмена и вставляет его (Ctrl+V) в текущее сфокусированное окно,
/// затем пытается подтвердить вставку через Ctrl+A/Ctrl+C.
///
/// # Важно
/// Функция НЕ ищет окно и НЕ ставит фокус сама.
/// Предполагается, что окно уже сфокусировано, и фокус ввода стоит в нужном поле.
///
/// # Механика вставки/копирования буфера.
/// При эмуляции нажатий клавиш они ставятся в очередь и выполняются асинхронно, но строго последовательно.
/// Во время выполнения клавишных операций содержимое буфера менять нельзя, чтобы не нарушить их работу.
///
/// Алгоритм.
/// 1) сохранить clipboard в локальной переменной (String).
/// 1) положить текст в clipboard,
/// 2) выполнить Ctrl+V (вставка). Очень медленно уходит на исполнение, а мы продолжаем работу.
/// 3) запустить проверку. По результату либо вернуть ошибку, либо Ok(()), но в обоих случаях
///    предварительно восстановить clipboard. К моменту возврата из функции проверки все клавиши
///    отработали и можно восстанавливать старый текст в буфере.
///
/// # Параметры
/// - `foreground_hwnd`: хэндлер окна переднего плана.
/// - `text`: Текст для вставки.
/// - `err_msg_context`: Контекст для текста ошибки (например, title или hwnd=0x...).
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - не удалось записать текст в clipboard,
/// - не удалось отправить Ctrl+V,
/// - не удалось подтвердить вставку.
///
/// # Побочные эффекты
/// - Временно перезаписывает системный буфер обмена.
/// - Верификация временно перезаписывает системный буфер обмена (Ctrl+C).
/// - Пытается восстановить clipboard, но ТОЛЬКО в части текста:
///   если в clipboard было изображение/файлы/HTML, восстановить “как было” через arboard нельзя.
/// - Генерирует события клавиатуры (Ctrl+V).
fn _paste_text_into_window_and_verify(foreground_hwnd: HWND, text: &str, err_msg_context: &str)
    -> Result<(), String>
{
    // Убрать, если есть, pop-up поверх нашего поля ввода.
    _check_and_close_popup(foreground_hwnd)?;

    // 0) Сохраняем текущий clipboard (только текст).
    //
    // Важно:
    // - Если в буфере был НЕ текст (например, файлы/картинка), arboard может вернуть Err.
    // - В этом случае мы НЕ сможем восстановить clipboard “как было”.
    let prev_clip_text: Option<String> = crate::library::clipboard::get_clipboard_text().ok();

    /// Локальная best-effort функция восстановления clipboard (только текст).
    fn _restore_clipboard_text(prev_clip_text: &Option<String>) {
        if let Some(prev) = prev_clip_text.as_deref() {
            let _ = crate::library::clipboard::set_clipboard_text(prev);
        }   // if
    }   // _restore_clipboard_text()

    // 1) Кладём текст для вставки в clipboard.
    if let Err(e) = crate::library::clipboard::set_clipboard_text(text) {
        _restore_clipboard_text(&prev_clip_text);
        return Err(e);
    }   // if

    // 2) Вставляем текст.
    if let Err(e) = keyboard::send_ctrl_v() {
        _restore_clipboard_text(&prev_clip_text);
        return Err(e);
    }   // if
return Ok(());
    // 4) Верификация (внутри: Ctrl+A -> Ctrl+C -> read clipboard -> compare).
    if window_backend::_verify_focused_textinput(text) {

        // Успех: восстанавливаем clipboard и выходим.
        _restore_clipboard_text(&prev_clip_text);
        return Ok(());
    }   // if

    // Верификация не прошла. Восстанавливаем карман, выходим с ошибкой.
    _restore_clipboard_text(&prev_clip_text);

    Err(format!(
        "не удалось подтвердить вставку текста в поле ввода окна ({})",
        err_msg_context
    ))
}   // _paste_text_into_window()

/// Проверяет наличие активного pop-up и пытается закрыть его (ESC) с ожиданием результата.
///
/// # Алгоритм
/// 1. Проверяет `_is_popup_active`. Если `false` — выходит сразу.
/// 2. Отправляет нажатие `ESC`.
/// 3. В цикле (до 500мс) проверяет, исчезли ли признаки pop-up.
///
/// # Параметры
/// - `foreground_hwnd`: HWND главного окна.
///
/// # Ошибки
/// Возвращает `Err(String)` только при сбое отправки клавиш или WinAPI.
fn _check_and_close_popup(foreground_hwnd: HWND) -> Result<(), String> {

    // 1. Быстрая проверка
    if !_is_popup_active(foreground_hwnd)? {
        return Ok(());
    }   // if

    // 2. Отправка ESC
    keyboard::send_esc()?;

    // 3. Ожидание закрытия (Polling)
    // 10 попыток по 50 мс = 500 мс макс.
    for _ in 0..10 {
        sleep(Duration::from_millis(50));

        if !_is_popup_active(foreground_hwnd)? {
            // Успех: состояние нормализовалось
            return Ok(());
        }   // if
    }   // for

    Ok(())
}   // _check_and_close_popup_wait()

/// Вспомогательная внутренняя функция для проверки состояния GUI потока.
/// Проверяет, перехвачен ли фокус ввода или захват мыши всплывающим окном (pop-up) или меню.
///
/// Используется для диагностики состояния "паразитных" окон (контекстные меню, комбобоксы),
/// которые блокируют ввод в главное окно.
///
/// # Параметры
/// - `foreground_hwnd`: HWND окна, которое ожидается активным (главное окно).
///
/// # Возвращаемое значение
/// - `true`: Обнаружены признаки активного меню или захвата фокуса другим окном.
/// - `false`: Фокус и захват мыши принадлежат `foreground_hwnd` (или отсутствуют).
///
/// # Ошибки
/// Возвращает `Err(String)` при сбое WinAPI (например, `GetGUIThreadInfo`).
fn _is_popup_active(foreground_hwnd: HWND) -> Result<bool, String> {
    unsafe {
        // Защита от NULL
        if foreground_hwnd.0.is_null() {
            return Ok(false);
        }   // if

        // Получаем ID потока
        let thread_id = GetWindowThreadProcessId(foreground_hwnd, None);
        if thread_id == 0 {
            // Окно могло быть закрыто в процессе
            return Ok(false);
        }   // if

        let mut gui_info = GUITHREADINFO::default();
        gui_info.cbSize = size_of::<GUITHREADINFO>() as u32;

        if GetGUIThreadInfo(thread_id, &mut gui_info).is_err() {
            return Err("GetGUIThreadInfo failed".to_string());
        }   // if

        // 1. Флаги меню (стандартные Win32 меню)
        let in_menu_flag = (gui_info.flags & (GUI_INMENUMODE | GUI_POPUPMENUMODE)).0 != 0;

        // 2. Фокус ввода "угнан" (находится не в главном окне)
        let focus_hijacked = !gui_info.hwndFocus.0.is_null() && gui_info.hwndFocus != foreground_hwnd;

        // 3. Мышь захвачена другим окном (критично для кастомных меню, например в Chrome)
        let mouse_captured = !gui_info.hwndCapture.0.is_null() && gui_info.hwndCapture != foreground_hwnd;

        Ok(in_menu_flag || focus_hijacked || mouse_captured)
    }   // unsafe
}   // _is_popup_active()

