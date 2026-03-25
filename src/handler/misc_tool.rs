//! misc_tool.rs
//!
//! Хандлеры составных служебных сценариев.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Выполнение многозвенных действий, использующих несколько библиотечных подсистем.
//! - Координация shell, поиска окон, UI Automation и мыши.
//! - Формирование fenced-отчётов для AI.

use std::collections::HashMap;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{PostMessageW, ShowWindow, SW_MINIMIZE, WM_CLOSE};
use crate::agent::request::session;
use crate::handler::{check_param_count, HandlerFn};
use crate::library::markdown_fence::wrap_in_fence;
use crate::library::{automation, misc_tool as lib_misc_tool, mouse};
use crate::library::window::{find_window_by_needle, find_window_by_needle_and_focus,
                             focus_window_with_retries, get_window_list, WindowInfo};

/// Регистрирует команды управления панелью задач.
pub(super) fn handlers_map_init(handlers: &mut HashMap<&'static str, HandlerFn>) {
    handlers.insert("show_taskbar", show_taskbar);
    handlers.insert("restore_taskbar_autohide_state", restore_taskbar_autohide_state);
    handlers.insert("show_taskbar_autohide_state", show_taskbar_autohide_state);
    handlers.insert("drag_file_from_explorer_to_ai", drag_file_from_explorer_to_ai);
}   // handlers_map_init()


/// Отключает авто-скрытие панели задач (делает её видимой).
///
/// # Параметры
/// - Нет параметров.
pub fn show_taskbar(_: &Option<Vec<String>>) -> Result<String, String> {
    lib_misc_tool::show_taskbar();
    Ok(wrap_in_fence("Панель задач зафиксирована (autohide выключен)."))
}   // turn_taskbar_autohide_off()


/// Восстанавливает исходное состояние авто-скрытия панели задач.
///
/// # Параметры
/// - Нет параметров.
pub fn restore_taskbar_autohide_state(_: &Option<Vec<String>>) -> Result<String, String> {
    lib_misc_tool::restore_taskbar_autohide_state();
    Ok(wrap_in_fence("Состояние панели задач восстановлено."))
}   // restore_taskbar_autohide()

/// Проверяет, включен ли режим авто-скрытия панели задач.
///
/// # Параметры
/// - нет параметров.
///
/// # Возвращаемое значение
/// - "true", если авто-скрытие включено.
/// - "false", если панель зафиксирована.
pub fn show_taskbar_autohide_state(_: &Option<Vec<String>>) -> Result<String, String> {
    let state = if lib_misc_tool::is_taskbar_autohide_enabled() { "включено"} else { "выключено" };
    Ok(wrap_in_fence(&format!("Автосокрытие панели задач {}", state)))
}   // is_taskbar_autohide_enabled()


/// Пауза после старта `search-ms`, чтобы Explorer успел начать строить окно.
const SEARCH_WINDOW_INITIAL_DELAY_MS: u64 = 250;

/// Пауза между попытками ожидания окна поиска.
const SEARCH_WINDOW_RETRY_DELAY_MS: u64 = 200;

/// Число попыток ожидания окна поиска.
///
/// Суммарное ожидание:
/// `SEARCH_WINDOW_RETRY_COUNT * SEARCH_WINDOW_RETRY_DELAY_MS`.
const SEARCH_WINDOW_RETRY_COUNT: usize = 20;

/// Пауза после захвата файла перед переводом фокуса в окно AI.
const DRAG_START_DELAY_MS: u64 = 220;

/// Пауза после сворачивания окна эксплорера.
const FOLD_WINDOW_DELAY_MS: u64 = 180;

/// Пауза после фокусировки окна AI перед переносом курсора к точке сброса.
const AI_FOCUS_SETTLE_DELAY_MS: u64 = 180;

/// Guard для защиты от “залипания” ЛКМ при аварийном выходе из хандлера.
///
/// # Назначение
/// Если после `left_button_down()` произойдет ошибка до явного `left_button_up()`,
/// guard в `Drop` попробует отпустить кнопку мыши best effort.
///
/// # Важно
/// - После успешного `left_button_up()` нужно вызвать `disarm()`.
/// - Guard не логирует ошибки и не паникует в `Drop`.
struct _MouseLeftButtonGuard {
    is_armed: bool,
}   // _MouseLeftButtonGuard

impl _MouseLeftButtonGuard {
    /// Создать неактивный guard.
    fn new() -> Self {
        Self { is_armed: false }
    }   // new()

    /// Пометить, что ЛКМ была нажата и при ошибке её нужно отпустить в `Drop`.
    fn arm(&mut self) {
        self.is_armed = true;
    }   // arm()

    /// Пометить, что ЛКМ уже отпущена штатно.
    fn disarm(&mut self) {
        self.is_armed = false;
    }   // disarm()
}   // impl _MouseLeftButtonGuard

impl Drop for _MouseLeftButtonGuard {
    fn drop(&mut self) {
        if self.is_armed {
            // Best effort: нельзя оставлять систему с логически “зажатой” кнопкой мыши.
            let _ = mouse::left_button_up();
        }   // if
    }   // drop()
}   // impl Drop for _MouseLeftButtonGuard

/// Перетащить файл из окна поиска Explorer в окно AI.
///
/// # Параметры
/// - `params[0]`: Полный путь к файлу.
///
/// # Алгоритм работы
/// 1. Проверяет корректность входного параметра и существование файла.
/// 2. Открывает окно `search-ms`.
/// 3. Если штатный поиск search-окна не удался, закрывает все окна с таким `file_name` в заголовке
///    и повторяет открытие один раз.
/// 4. Находит геометрию файла в окне Explorer через UI Automation.
/// 5. Подводит мышь к файлу и зажимает левую кнопку.
/// 6. Находит и фокусирует окно AI по `SessionContext.window_title`.
/// 7. Переносит курсор в центр окна AI и отпускает кнопку.
/// 8. Закрывает окно поиска Explorer.
/// 9. Возвращает fenced-отчёт.
///
/// # Возвращаемое значение
/// Тип: `Result<String, String>`
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - параметр невалиден;
/// - файл не существует;
/// - не удалось открыть окно поиска;
/// - не удалось найти файл в окне поиска;
/// - не удалось найти или сфокусировать окно AI;
/// - не удалось выполнить drag-and-drop.
fn drag_file_from_explorer_to_ai(params: &Option<Vec<String>>) -> Result<String, String> {
    // 1. Проверяем число параметров.
    check_param_count(params, 1)?;

    // 2. Извлекаем строку полного пути к файлу.
    let file_path = params
        .as_ref()
        .and_then(|v| v.get(0))
        .ok_or_else(|| "Параметр полного пути к файлу не найден.".to_string())?;

    // 3. Проверяем существование файла.
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("Файл не найден: '{}'", file_path));
    }   // if

    // 4. Извлекаем имя файла.
    let file_name = path.file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format!("Не удалось извлечь имя файла из пути '{}'.", file_path))?;

    // 5. Извлекаем родительский каталог.
    let parent_dir = path.parent()
        .and_then(|p| p.to_str())
        .ok_or_else(|| format!("Не удалось извлечь каталог из пути '{}'.", file_path))?;

    // 6. Открываем окно поиска Explorer. В штатном случае это дает одно уникальное окно.
    //    Если уже открыты дубликаты окон для того же файла, закроем все похожие окна снова откроем наше.
    let explorer_window = _open_search_window_for_file(file_name, parent_dir)?;

    // 6.1. Без стабилизации окна после открытия поиск файла показывает неверные координаты.
    sleep(Duration::from_millis(1000));

    // 7. Находим геометрию файла внутри search-окна Explorer.
    let file_geometry =
        automation::find_file_rect_in_explorer(explorer_window.hwnd, file_name)?;

    // 8. Guard для аварийного отпускания ЛКМ при ошибке после left_button_down().
    let mut mouse_guard = _MouseLeftButtonGuard::new();

    // 9. Подводим курсор к центру файла.
    mouse::move_cursor_to_position(file_geometry.center.x, file_geometry.center.y)
        .map_err(|e| format!("Не удалось подвести курсор к файлу '{}': {}", file_name, e))?;

    // 11. Ищем и фокусируем окно AI.
    let ai_window_needle = session::window_title()
        .map_err(|e| format!("Не удалось получить window_title из SessionContext: {}", e))?;
    let ai_window = find_window_by_needle_and_focus(&ai_window_needle)
        .map_err(|e| format!("Не удалось найти и сфокусировать окно AI '{}': {}", ai_window_needle, e))?;

    // 12. Даём системе короткое время зафиксировать foreground.
    sleep(Duration::from_millis(AI_FOCUS_SETTLE_DELAY_MS));

    // 13. Вычисляем точку сброса (центр окна AI).
    let drop_point = _window_center(&ai_window);

    // 14. Нажимаем ЛКМ и взводим guard.
    mouse::left_button_down()
        .map_err(|e| format!("Не удалось зажать левую кнопку мыши на файле '{}': {}", file_name, e))?;
    mouse_guard.arm();

    // 15. Даём Explorer время перейти в состояние drag.
    sleep(Duration::from_millis(DRAG_START_DELAY_MS));

    // 15.1. Сворачиваем окно Explorer после начала drag.
    //
    // Это нужно, чтобы search-окно гарантированно не перекрывало окно AI и не мешало точке сброса.
    // Сворачивание делаем только после:
    // - позиционирования курсора на файле,
    // - нажатия ЛКМ,
    // - небольшой паузы на переход Explorer в состояние drag-and-drop.
    unsafe {
        let _ = ShowWindow(explorer_window.hwnd, SW_MINIMIZE);
    }   // unsafe

    // Даём системе короткое время обработать сворачивание окна.
    sleep(Duration::from_millis(FOLD_WINDOW_DELAY_MS));

    // 16. Переносим курсор к точке сброса.
    //     Поскольку ЛКМ уже зажата, это продолжение drag-and-drop.
    mouse::move_cursor_to_position(drop_point.0, drop_point.1)
        .map_err(|e| format!("Не удалось перенести файл '{}' к окну AI: {}", file_name, e))?;

    // 17. Короткая пауза перед завершением drop.
    sleep(Duration::from_millis(120));

    // 18. Отпускаем ЛКМ штатно и обезвреживаем guard.
    mouse::left_button_up()
        .map_err(|e| format!("Не удалось отпустить левую кнопку мыши для drop файла '{}': {}", file_name, e))?;
    mouse_guard.disarm();

    // 19. Закрываем search-окно Explorer best effort.
    let _ = _close_window(explorer_window.hwnd);

    // 20. Формируем fenced-отчёт.
    let out = format!(
        "Файл перетащен в окно AI.\n\
         file='{}'\n\
         source_window='{}'\n\
         target_window='{}'\n\
         drag_from=({}, {})\n\
         drop_to=({}, {})",
        file_name,
        explorer_window.title,
        ai_window.title,
        file_geometry.center.x,
        file_geometry.center.y,
        drop_point.0,
        drop_point.1
    );

    Ok(wrap_in_fence(&out))
}   // drag_file_from_explorer_to_ai()

#[cfg(test)]
mod test {
    use std::thread::sleep;
    use std::time::Duration;
    use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_MINIMIZE};
    use crate::handler::misc_tool::{_MouseLeftButtonGuard, _close_window, _open_search_window_for_file,
                                    _window_center, AI_FOCUS_SETTLE_DELAY_MS, DRAG_START_DELAY_MS,
                                    FOLD_WINDOW_DELAY_MS};
    use crate::library::{automation, mouse};
    use crate::library::window::find_window_by_needle_and_focus;

    #[test]
    fn smoke() {

        let file_name = "user_guide.pdf";

        // 6. Открываем окно поиска Explorer. В штатном случае это дает одно уникальное окно.
        //    Если уже открыты дубликаты окон для того же файла, закроем все похожие окна снова откроем наше.
        let explorer_window = _open_search_window_for_file(file_name,
                                                           "C:\\hobot\\doc\\tech_spec").unwrap();

        // Без стабилизации окна после открытия поиск файла показывает неверные координаты.
        sleep(Duration::from_millis(1000));

        // 7. Находим геометрию файла внутри search-окна Explorer.
        let file_geometry =
            automation::find_file_rect_in_explorer(explorer_window.hwnd, file_name).unwrap();

        // 13. Ищем и фокусируем окно AI.
        let ai_window_needle = "rena";
        let ai_window = find_window_by_needle_and_focus(&ai_window_needle)
            .map_err(|e| format!("Не удалось найти и сфокусировать окно AI '{}': {}", ai_window_needle, e)).unwrap();

        // 15. Вычисляем центр окна AI.
        let drop_point = _window_center(&ai_window);

        // 5. Даём системе короткое время зафиксировать foreground.
        sleep(Duration::from_millis(AI_FOCUS_SETTLE_DELAY_MS));

        // 9. Guard для аварийного отпускания ЛКМ при ошибке после left_button_down().
        let mut mouse_guard = _MouseLeftButtonGuard::new();

        // 10. Подводим курсор к центру файла.
        mouse::move_cursor_to_position(file_geometry.center.x, file_geometry.center.y)
            .map_err(|e| format!("Не удалось подвести курсор к файлу '{}': {}", file_name, e)).unwrap();

        // 11. Нажимаем ЛКМ и взводим guard.
        mouse::left_button_down()
            .map_err(|e| format!("Не удалось зажать левую кнопку мыши на файле '{}': {}", file_name, e)).unwrap();
        mouse_guard.arm();

        // 12. Даём Explorer время перейти в состояние drag.
        sleep(Duration::from_millis(DRAG_START_DELAY_MS));

        // 15.1. Сворачиваем окно Explorer после начала drag. Это нужно, чтобы search-окно
        // гарантированно не перекрывало окно AI и не мешало точке сброса. Попытка сфокусировать
        // окно AI, чтобы освободить поле сброса, приводит к потере драга мышки.
        // Сворачивание делаем только после:
        // - позиционирования курсора на файле,
        // - нажатия ЛКМ,
        // - небольшой паузы на переход Explorer в состояние drag-and-drop.
        unsafe {
            let _ = ShowWindow(explorer_window.hwnd, SW_MINIMIZE);
        }   // unsafe

        // Даём системе короткое время обработать сворачивание окна.
        sleep(Duration::from_millis(FOLD_WINDOW_DELAY_MS));

        // 16. Переносим курсор к точке сброса.
        //     Поскольку ЛКМ уже зажата, это продолжение drag-and-drop.
        mouse::move_cursor_to_position(drop_point.0, drop_point.1)
            .map_err(|e| format!("Не удалось перенести файл '{}' к окну AI: {}", file_name, e)).unwrap();

        // 17. Короткая пауза перед завершением drop.
        sleep(Duration::from_millis(120));

        // 18. Отпускаем ЛКМ штатно и обезвреживаем guard.
        mouse::left_button_up()
            .map_err(|e| format!("Не удалось отпустить левую кнопку мыши для drop файла '{}': {}", file_name, e)).unwrap();
        // mouse_guard.disarm();

        // 19. Закрываем search-окно Explorer best effort.
        let _ = _close_window(explorer_window.hwnd);
    }
}

/// Открыть окно поиска Explorer для файла и вернуть его `WindowInfo`.
///
/// # Алгоритм работы
/// 1. Пытается открыть `search-ms`.
/// 2. Пытается дождаться окна поиска по `file_name`.
/// 3. Если это не удалось (типично — из-за уже открытых дубликатов), закрывает все окна
///    с таким `file_name` в заголовке, открывает поиск заново и повторяет попытку один раз.
/// 4. После успешного ожидания фокусирует найденное окно и возвращает свежий `WindowInfo`.
///
/// # Параметры
/// - `file_name`: Имя файла.
/// - `parent_dir`: Каталог, в котором нужно искать файл.
///
/// # Возвращаемое значение
/// Тип: `Result<WindowInfo, String>`
fn _open_search_window_for_file(file_name: &str, parent_dir: &str) -> Result<WindowInfo, String> {
    // 1. Первая попытка: штатный сценарий без cleanup.
    _start_search_ms_for_file(file_name, parent_dir)?;

    // 2. Даём Explorer стартовую паузу на создание окна.
    sleep(Duration::from_millis(SEARCH_WINDOW_INITIAL_DELAY_MS));

    // 3. Пытаемся дождаться окна.
    match _wait_search_window(file_name) {
        Ok(info) => {
            // После нахождения окна явно фокусируем его через библиотечную функцию.
            focus_window_with_retries(info.hwnd)
        }

        Err(first_err) => {
            // 4. Recovery path: закрываем старые окна и пробуем один раз заново.
            let _closed_count = _close_windows_by_needle(file_name)?;

            // 5. Даём GUI время обработать WM_CLOSE.
            sleep(Duration::from_millis(180));

            // 6. Повторный запуск поиска.
            _start_search_ms_for_file(file_name, parent_dir)?;

            // 7. Даём Explorer стартовую паузу.
            sleep(Duration::from_millis(SEARCH_WINDOW_INITIAL_DELAY_MS));

            // 8. Вторая и последняя попытка.
            let info = _wait_search_window(file_name).map_err(|second_err| {
                format!(
                    "Не удалось получить уникальное окно поиска для файла '{}'. \
                     Первая ошибка: {}. Повторная ошибка: {}",
                    file_name,
                    first_err,
                    second_err
                )
            })?;

            // 9. После второго запуска также явно фокусируем найденное окно.
            focus_window_with_retries(info.hwnd)
        }
    }   // match
}   // _open_search_window_for_file()

/// Запустить `search-ms` для файла в заданном каталоге.
///
/// # Параметры
/// - `file_name`: Имя файла.
/// - `parent_dir`: Каталог поиска.
///
/// # Ошибки
/// Возвращает `Err(String)`, если команда `cmd /C start ...` завершилась ошибкой.
fn _start_search_ms_for_file(file_name: &str, parent_dir: &str) -> Result<(), String> {
    // Формируем URI поиска.
    let search_url = format!(
        "search-ms:query={}&crumb=location:{}",
        file_name,
        parent_dir
    );

    // Важно: `start` нужен для открытия search-ms через оболочку Windows.
    let command_line = format!("start \"\" \"{}\"", search_url);

    _run_cmd(&command_line)
}   // _start_search_ms_for_file()

/// Дождаться появления окна поиска Explorer по `file_name`.
///
/// # Алгоритм работы
/// - Использует лёгкий поиск без фокусировки.
/// - Между попытками делает паузу.
///
/// # Параметры
/// - `file_name`: Подстрока заголовка окна поиска.
///
/// # Возвращаемое значение
/// Тип: `Result<WindowInfo, String>`
///
/// # Ошибки
/// Возвращает `Err(String)`, если окно не найдено за отведённое число попыток.
fn _wait_search_window(file_name: &str) -> Result<WindowInfo, String> {
    match find_window_by_needle(file_name) {
        Ok(info) => {
            return Ok(info);
        }

        Err(_) => {
            Err(format!("{}, {}: Окно поиска Explorer по needle '{}' не появилось.",
                file!(), line!(), file_name))
        }
    }   // match
}   // _wait_search_window()

/// Закрыть все видимые окна, заголовок которых содержит `needle`.
///
/// # Параметры
/// - `needle`: Подстрока заголовка окна.
///
/// # Возвращаемое значение
/// Тип: `Result<usize, String>` — число окон, которым был отправлен `WM_CLOSE`.
///
/// # Примечание
/// Предполагается, что `needle` достаточно редкий и соответствует имени файла, а не общему слову.
fn _close_windows_by_needle(needle: &str) -> Result<usize, String> {
    // Получаем список видимых окон, содержащих needle в заголовке.
    let list = get_window_list(Some(needle), false, true)?;

    let mut closed_count = 0usize;

    for wnd in list {
        // Отправляем WM_CLOSE мягко, как при нажатии на крестик.
        _close_window(wnd.hwnd)?;
        closed_count += 1;
    }   // for

    Ok(closed_count)
}   // _close_windows_by_needle()

/// Закрыть окно по `HWND` через `WM_CLOSE`.
///
/// # Параметры
/// - `hwnd`: Дескриптор окна.
///
/// # Ошибки
/// Возвращает `Err(String)`, если `PostMessageW()` не смог поставить сообщение.
fn _close_window(hwnd: HWND) -> Result<(), String> {
    unsafe {
        PostMessageW(Some(hwnd), WM_CLOSE, WPARAM(0), LPARAM(0))
            .map_err(|e| format!(
                "Не удалось отправить WM_CLOSE в окно 0x{:X}: {}",
                hwnd.0 as usize,
                e
            ))?;
    }   // unsafe

    Ok(())
}   // _close_window()

/// Выполнить командную строку через `cmd /C`, передавая команду в сыром виде.
///
/// # Описание
/// Используется `raw_arg()`, потому что `args()` вмешивается в экранирование и ломает
/// команды вида `start "" "search-ms:..."`.
///
/// # Параметры
/// - `command_line`: Текст команды без префикса `cmd /C`.
///
/// # Возвращаемое значение
/// Тип: `Result<(), String>`
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - не удалось запустить `cmd`;
/// - команда завершилась с ненулевым кодом возврата.
///
/// # Примечание
/// В stdout/stderr ничего не выводится вручную.
/// Возвращаем только текст ошибки, пригодный для отчёта хандлера.
fn _run_cmd(command_line: &str) -> Result<(), String> {
    // raw_arg() не занимается экранированием параметров как args(), что мешает работе.
    let output = Command::new("cmd")
        .raw_arg(format!("/C \"{}\"", command_line))
        .output()
        .map_err(|e| format!("Критическая ошибка запуска: {}", e))?;

    if output.status.success() {
        // Команда завершилась штатно.
        Ok(())
    } else {
        // Команда завершилась с ошибкой.
        // Декодируем stderr тем же способом, что и shell-хандлеры.
        let err_text = _decode_process_output(&output.stderr).trim().to_string();

        // Если stderr пустой — подставляем код возврата.
        let err_payload = if err_text.is_empty() {
            format!("Код возврата: {:?}", output.status.code())
        } else {
            err_text
        };

        Err(err_payload)
    }   // if
}   // _run_cmd()

/// Декодировать байтовый вывод дочернего процесса в `String`.
///
/// # Алгоритм работы
/// - Сначала пытается декодировать как UTF-8.
/// - Если UTF-8 не подходит, пытается декодировать как CP866.
///
/// # Параметры
/// - `bytes`: Сырые байты stdout/stderr дочернего процесса.
///
/// # Возвращаемое значение
/// Тип: `String`
fn _decode_process_output(bytes: &[u8]) -> String {
    // 1. Быстрый путь: корректный UTF-8.
    if let Ok(text) = String::from_utf8(bytes.to_vec()) {
        return text;
    }   // if

    // 2. Fallback: CP866 — типичная OEM-кодировка консоли Windows.
    let (cow, _, _) = encoding_rs::IBM866.decode(bytes);
    cow.into_owned()
}   // _decode_process_output()

/// Вычислить центр окна по его геометрии.
///
/// # Параметры
/// - `wnd`: Информация об окне.
///
/// # Возвращаемое значение
/// Тип: `(i32, i32)` — экранные координаты центра окна.
fn _window_center(wnd: &WindowInfo) -> (i32, i32) {
    let center_x = wnd.x + (wnd.width as i32 / 2);
    let center_y = wnd.y + (wnd.height as i32 / 2);

    (center_x, center_y)
}   // _window_center()
