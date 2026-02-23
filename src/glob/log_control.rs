//! log_control.rs — Централизованное управление журналами (comment/logic).
//!
//! # ОПИСАНИЕ
//! Модуль:
//! - хранит таймстамп запуска (LOG_TIMESTAMP),
//! - вычисляет каталог запуска логов `log\<TS>\`,
//! - открывает и удерживает открытыми:
//!   - comment_log.md
//!   - logic_log.md
//!
//! # ИНВАРИАНТЫ
//! - `init()` должен быть вызван ровно один раз при старте (из `glob::initialize_glob()`).
//! - После `init()` файлы остаются открытыми до завершения процесса.
//! - Ошибки инициализации пробрасываются наружу: panic делает вызывающий код.

use std::cmp::max;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, SystemTime};
use chrono::NaiveDateTime;
use crate::glob::config;
use crate::{handle_log, wrln};

/// Корневой каталог логов (относительно текущего каталога запуска).
const LOG_DIRECTORY: &str = "log";

/// Корневой каталог логов (относительно текущего каталога запуска).
const LOG_ARCHIVE_DIRECTORY: &str = "log\\archive";

/// Имя файла рабочего журнала.
const WORK_LOG_FILENAME: &str = "work.log";

/// Имя файла журнала комментариев (Markdown).
const COMMENT_LOG_FILENAME: &str = "comment_log.md";

/// Имя файла журнала логики (Markdown).
const LOGIC_LOG_FILENAME: &str = "logic_log.md";

/// Открытый файл work.log.
static WORK_LOG: OnceLock<Mutex<File>> = OnceLock::new();

/// Открытый файл comment_log.md.
static COMMENT_LOG: OnceLock<Mutex<File>> = OnceLock::new();

/// Открытый файл logic_log.md.
static LOGIC_LOG: OnceLock<Mutex<File>> = OnceLock::new();

/// Таймстамп запуска (строка из аргумента командной строки).
///
/// ВАЖНО: значение делаем `'static` через leak, чтобы ссылка жила весь процесс.
///
/// # Safety
/// - Инициализировать строго один раз в самом начале процесса.
/// - После инициализации только читать.
static mut LOG_TIMESTAMP: Option<&'static str> = None;
fn _init_log_timestamp(ts: &str) {
    unsafe {
        let leaked: &'static str = Box::leak(ts.to_string().into_boxed_str());
        LOG_TIMESTAMP = Some(leaked);
    }
}   // _init_log_timestamp()
fn _log_timestamp() -> &'static str {
    unsafe { LOG_TIMESTAMP.unwrap() }
}   // _log_timestamp()

/// Инициализирует модуль логов.
///
/// # Параметры
/// - `ts`: Таймстамп запуска (например `2026-02-05_15.46.52`).
///
/// # Ошибки
/// Возвращает `io::Error`, если:
/// - не удалось создать каталог `log\<TS>\`
/// - не удалось открыть файлы
/// - модуль уже был инициализирован
pub(crate) fn init(ts: &str) -> Result<(), io::Error> {

    // Принять временную метку журналов.
    _init_log_timestamp(ts);

    // Создать каталоги, если их нет.
    let log_dir = Path::new(LOG_DIRECTORY).join(_log_timestamp());
    fs::create_dir_all(&log_dir)?;
    let arc_log_dir = Path::new(LOG_ARCHIVE_DIRECTORY);
    fs::create_dir_all(&arc_log_dir)?;

    // Создать файлы журналов.
    let work_path = log_dir.join(WORK_LOG_FILENAME);
    let comment_path = log_dir.join(COMMENT_LOG_FILENAME);
    let logic_path = log_dir.join(LOGIC_LOG_FILENAME);
    let work_file = _open_file(&work_path)?;
    let comment_file = _open_file(&comment_path)?;
    let logic_file = _open_file(&logic_path)?;

    // Сохранить в статических переменных.
    WORK_LOG
        .set(Mutex::new(work_file))
        .map_err(|_| io::Error::new(io::ErrorKind::AlreadyExists, "WORK_LOG уже инициализирован"))?;
    COMMENT_LOG
        .set(Mutex::new(comment_file))
        .map_err(|_| io::Error::new(io::ErrorKind::AlreadyExists, "COMMENT_LOG уже инициализирован"))?;
    LOGIC_LOG
        .set(Mutex::new(logic_file))
        .map_err(|_| io::Error::new(io::ErrorKind::AlreadyExists, "LOGIC_LOG уже инициализирован"))?;

    // Перенести старые журналы в архив
    _archive_old_logs();

    // Удалить устаревшие журналы
    _clean_up_logs();

    Ok(())
}   // init()

/// Дописывает текст в work.log (как есть).
pub(crate) fn write_to_work_log(text: &str) {
    _write_to_log_or_panic(&WORK_LOG, text, WORK_LOG_FILENAME);
}   // write_to_work_log()

/// Дописывает строку в work.log (добавляет `\n`).
pub(crate) fn writeln_to_work_log(line: &str) {
    _write_to_log_or_panic(&WORK_LOG, &format!("{}\n", line), WORK_LOG_FILENAME);
}   // writeln_to_work_log()

/// Дописывает текст в comment_log.md (как есть).
pub(crate) fn write_to_comment_log(text: &str) {
    _write_to_log_or_panic(&COMMENT_LOG, text, COMMENT_LOG_FILENAME);
}   // write_to_comment_log()

/// Дописывает строку в comment_log.md (добавляет `\n`).
pub(crate) fn writeln_to_comment_log(line: &str) {
    _write_to_log_or_panic(&COMMENT_LOG, &format!("{}\n", line), COMMENT_LOG_FILENAME);
}   // writeln_to_comment_log()

/// Дописывает текст в logic_log.md (как есть).
pub(crate) fn write_to_logic_log(text: &str) {
    _write_to_log_or_panic(&LOGIC_LOG, text, LOGIC_LOG_FILENAME);
}   // write_to_logic_log()

/// Дописывает строку в logic_log.md (добавляет `\n`).
pub(crate) fn writeln_to_logic_log(line: &str) {
    _write_to_log_or_panic(&LOGIC_LOG, &format!("{}\n", line), LOGIC_LOG_FILENAME);
}   // writeln_to_logic_log()

/// Временной штамп каталогов журналов, используется тестами для доступа к журналам.
#[cfg(test)]
pub(crate) fn log_timestamp() -> &'static str {
    _log_timestamp()
}   // _log_timestamp()

//--------------------------------------------------------------------------------------------------
//                                 Внутренний интерфейс
//--------------------------------------------------------------------------------------------------

fn _open_file(path: &PathBuf) -> Result<File, io::Error> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
}   // _open_truncate_file()

fn _write_to_log_or_panic(
    target: &'static OnceLock<Mutex<File>>,
    text: &str,
    log_name: &str
) {
    _write_to_log(target, text).unwrap_or_else(|e| {
        panic!("Критическая ошибка: запись в '{}' не удалась: {}", log_name, e);
    });
}   // _write_to_log_or_panic()

fn _write_to_log(target: &'static OnceLock<Mutex<File>>, text: &str) -> Result<(), io::Error> {
    let lock = target.get().ok_or_else(|| {
        io::Error::new(io::ErrorKind::Other, "log_control не инициализирован (init() не вызывался)")
    })?;

    let mut file = lock.lock().map_err(|_| {
        io::Error::new(io::ErrorKind::Other, "log_control: mutex poisoned")
    })?;

    file.write_all(text.as_bytes())?;
    file.flush()?;

    Ok(())
}   // _write_to_log()

/// Удаляет устаревшие каталоги логов в `log/archive/` по политике retention.
///
/// # Политика
/// 1) Гарантированно сохраняет минимум `min_dirs_to_keep` самых “новых” каталогов.
/// 2) Все остальные каталоги удаляет, если они старше `log_dirs_retention_days`.
///
/// # Ошибки
/// Best effort. При ошибках в журнал ошибок помещается сообщение.
fn _clean_up_logs() {

    // Корневой каталог логов.
    let log_root = Path::new(LOG_ARCHIVE_DIRECTORY);
    if !log_root.exists() {
        handle_log!("не найден каталог архива журналов {}", log_root.display());
        return;
    }   // if

    // Собрать каталоги.
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut dir_iter = fs::read_dir(log_root)
        .map_err(|e| {
            handle_log!("ошибка при работе с каталогом архива логов: {}", e);
            return;
        })
        .unwrap();
    for entry in dir_iter {

        // Берем только каталоги.
        if let Err(e) = entry {
            // Не смогли прочесть каталог. Пропускаем его.
            handle_log!("{}", e);
            continue;
        }
        let path = entry.unwrap().path();
        if !path.is_dir() {
            // Это не каталог, пропускаем.
            continue;
        }   // if

        // Помещаем имена каталогов в вектор.
        if let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) {
            if is_valid_timestamp_name(dir_name) {
                dirs.push(path);
            } else {
                handle_log!("пропущен каталог (не соответствует формату даты): {}", path.display());
            }
        } else {
            handle_log!("каталог журнала имеет не-UTF8 имя: {}", path.display());
            continue;
        }  // if
    }   // for


    // Сортировка: новые в начале.
    dirs.sort_by(|a, b| b.cmp(&a));

    // Защищаем минимум N самых новых каталогов.
    let protected_count = config().min_log_dirs_to_keep as usize;

    // Рассчитываем таймстамп отсечки.
    let now = chrono::Local::now();
    let cut_off =
        now.checked_sub_days(chrono::Days::new(config().log_dirs_retention_days));
    let cut_off = if cut_off.is_some() {
        // Рассчитываем таймстамп. Все что меньше, подлежит удалению.
        cut_off.unwrap().format("%Y-%m-%d_%H.%M.%S").to_string()
    } else {
        // Не удалось вычислить крайнюю дату. Очистка отменяется.
        handle_log!("не удалось вычислить таймстамп отсечки.");
        return;
    };
    let cut_off = cut_off.as_str();

    // Проходим по массиву, удаляем устаревшие.
    for (i, path) in dirs.iter().enumerate() {

        // 1) Минимум N новых — не удаляем.
        if i < protected_count {
            continue;
        }   // if

        // 2) Старые — удаляем
        let dir_name = path.file_name().unwrap().to_str().unwrap();
        if dir_name < cut_off {
            if let Err(e) = fs::remove_dir_all(path) {
                handle_log!("ошибка удаления каталога журналов: {}", e);
                continue;
            }   // if
        }   // if
    }   // for
}   // clean_up_logs()

/// Описание: Перемещает старые рабочие каталоги журналов в архив.
///
/// Функция оставляет в корневой папке логов только `config().unarchived_dirs`
/// самых свежих каталогов (включая только что созданный каталог текущей сессии).
/// Остальные валидные каталоги переносятся в `LOG_ARCHIVE_DIRECTORY`.
///
/// # Алгоритм работы
/// 1. Проверяет наличие корневого и архивного каталогов.
/// 2. Сканирует корневой каталог логов (`LOG_DIRECTORY`).
/// 3. Отбирает только те каталоги, имена которых соответствуют формату таймстампа.
/// 4. Сортирует список каталогов по убыванию (от самых свежих к самым старым).
/// 5. Пропускает заданное конфигурацией число свежих каталогов.
/// 6. Перемещает оставшиеся (более старые) каталоги в папку архива.
///
/// # Побочные эффекты
/// - Читает директорию `log/`.
/// - Перемещает папки на диске.
/// - В случае ошибок доступа пишет предупреждения в системный журнал (`handle_log!`).
fn _archive_old_logs() {

    // 1. Подготавливаем пути к корневому каталогу и архиву.
    let log_root = Path::new(LOG_DIRECTORY);
    let archive_root = Path::new(LOG_ARCHIVE_DIRECTORY);

    // Если хотя бы одного из каталогов нет, архивировать нечего или некуда.
    // (Хотя они должны быть созданы ранее в `init()`).
    if !log_root.exists() || !archive_root.exists() {
        return;
    }   // if

    // Вектор для сбора всех найденных каталогов с правильными именами таймстампов.
    let mut dirs: Vec<PathBuf> = Vec::new();

    // 2. Открываем итератор по корневому каталогу логов.
    let dir_iter = match fs::read_dir(log_root) {
        Ok(iter) => iter,
        Err(e) => {
            // Ошибка чтения каталога (например, нет прав). Логируем и выходим.
            handle_log!("ошибка чтения корневого каталога логов: {}", e);
            return;
        }
    };  // match

    // 3. Сканируем содержимое каталога.
    for entry in dir_iter {

        // Проверяем доступность элемента файловой системы.
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                handle_log!("ошибка при доступе к элементу логов: {}", e);
                continue;
            }
        };  // match

        let path = entry.path();

        // Нас интересуют только каталоги (файлы типа work.log или подкаталог archive пропускаем).
        if !path.is_dir() {
            continue;
        }   // if

        // Извлекаем имя каталога в виде строки. Если имя содержит не-UTF8 символы, пропускаем.
        let dir_name = match path.file_name().and_then(|s| s.to_str()) {
            Some(name) => name,
            None => continue,
        };  // match

        // Проверяем, является ли имя каталога валидным таймстампом.
        // Это отсеет папку "archive" и любые другие посторонние каталоги.
        if !is_valid_timestamp_name(dir_name) {
            continue;
        }   // if

        // Добавляем валидный путь в список для дальнейшей сортировки.
        dirs.push(path);
    }   // for entry

    // 4. Сортируем каталоги по убыванию имени.
    // Поскольку формат `YYYY-MM-DD_HH.MM.SS` лексикографически сортируется корректно,
    // обратная сортировка (b.cmp(a)) поместит самые свежие даты в начало списка.
    dirs.sort_by(|a, b| b.cmp(a));

    // 5. Получаем лимит неархивируемых каталогов из глобальной конфигурации.
    // Преобразуем u64 в usize для безопасного сравнения с индексами вектора.
    let unarchived_limit = config().unarchived_dirs as usize;

    // 6. Проходим по отсортированному списку и перемещаем старые каталоги в архив.
    for (i, path) in dirs.iter().enumerate() {

        // Пропускаем первые `unarchived_limit` каталогов (самые свежие).
        if i < unarchived_limit {
            continue;
        }   // if

        // Получаем имя каталога, который нужно переместить (оно гарантированно есть и валидно).
        let dir_name = path.file_name().unwrap();

        // Защита текущего каталога. Файлы внутри него уже открыты.
        let dir_name_str = dir_name.to_str().unwrap();
        if dir_name_str == _log_timestamp() {
            continue;
        }   // if

        // Формируем целевой путь в каталоге архива.
        let target_path = archive_root.join(dir_name);

        // Пытаемся переместить каталог.
        // При перемещении внутри одного логического диска это быстрая операция смены указателя.
        if let Err(e) = fs::rename(path, &target_path) {
            // Если каталог заблокирован другим процессом или нет прав — просто логируем ошибку.
            // Попытка архивации повторится при следующем запуске агента.
            handle_log!("ошибка перемещения каталога '{}' в архив: {}", dir_name.to_string_lossy(), e);
        }   // if
    }   // for (i, path)
}   // _archive_old_logs()

/// Описание: Проверяет, строго ли соответствует имя файла/каталога шаблону "%Y-%m-%d_%H.%M.%S".
///
/// # Алгоритм работы
/// 1. Проверяет длину строки (шаблон `YYYY-MM-DD_HH.MM.SS` занимает ровно 19 символов).
/// 2. Использует `chrono` для проверки корректности расстановки разделителей и
///    валидности самой даты/времени.
///
/// # Параметры
/// - `name`: Имя файла или каталога для проверки.
///
/// # Возвращаемое значение
/// Тип: `bool`: `true`, если строка является валидной датой в указанном формате.
fn is_valid_timestamp_name(name: &str) -> bool {

    // 1) Длина строго 19 символов. Отсекает имена с суффиксами/префиксами
    // (например, "2026-02-05_15.46.52_old").
    if name.len() != 19 {
        return false;
    }   // if

    // 2) Парсинг даты. Отсеет неверные разделители и несуществующие даты (например, 32 февраля).
    NaiveDateTime::parse_from_str(name, "%Y-%m-%d_%H.%M.%S").is_ok()

}   // is_valid_timestamp_name()