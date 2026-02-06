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

use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, SystemTime};
use crate::glob::config;

/// Корневой каталог логов (относительно текущего каталога запуска).
const LOG_DIRECTORY: &str = "log";

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
    _init_log_timestamp(ts)?;

    // Удалить устаревшие журналы
    _clean_up_logs()?;

    // Создать каталоги, если их нет (страховка, они есть).
    let run_dir = Path::new(LOG_DIRECTORY).join(_log_timestamp());
    fs::create_dir_all(&run_dir)?;

    // Создать файлы журналов.
    let work_path = run_dir.join(WORK_LOG_FILENAME);
    let comment_path = run_dir.join(COMMENT_LOG_FILENAME);
    let logic_path = run_dir.join(LOGIC_LOG_FILENAME);
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

fn _init_log_timestamp(ts: &str) -> Result<(), io::Error> {
    unsafe {
        let leaked: &'static str = Box::leak(ts.to_string().into_boxed_str());
        LOG_TIMESTAMP = Some(leaked);

        Ok(())
    }
}   // _init_log_timestamp()

fn _log_timestamp() -> &'static str {
    unsafe {
        LOG_TIMESTAMP.unwrap()
    }
}   // _log_timestamp()

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

/// Удаляет устаревшие каталоги логов в `log/` по политике retention.
///
/// # Политика
/// 1) Гарантированно сохраняет минимум `min_dirs_to_keep` самых “новых” каталогов (по времени `modified()`).
/// 2) Все остальные каталоги удаляет, если они старше `log_dirs_retention_days`.
/// 3) Каталог текущего запуска `log/<TS>` (если TS инициализирован) никогда не удаляется.
///
/// # Ошибки
/// Возвращает `io::Error`:
/// - если не удалось прочитать содержимое `log/`
/// - если не удалось удалить один или несколько каталогов (возвращается первая ошибка)
fn _clean_up_logs() -> Result<(), io::Error> {

    let min_dirs_to_keep = config().min_log_dirs_to_keep;
    let log_dirs_retention_days = config().log_dirs_retention_days;

    // Корневой каталог логов.
    let log_root = Path::new(LOG_DIRECTORY);
    if !log_root.exists() {
        return Ok(());
    }   // if

    // “Сейчас” и граница по возрасту.
    let now = SystemTime::now();
    let retention_secs = log_dirs_retention_days.saturating_mul(24 * 60 * 60);
    let retention = Duration::from_secs(retention_secs);

    // Если retention слишком большой (или SystemTime “сломался”) — просто не удаляем по возрасту.
    let cutoff = now.checked_sub(retention);

    // Текущий каталог запуска (если TS уже инициализирован).
    let current_run_dir = unsafe { LOG_TIMESTAMP }
        .map(|ts| log_root.join(ts));

    // Собрать каталоги.
    let mut dirs: Vec<(PathBuf, SystemTime)> = Vec::new();
    for entry in fs::read_dir(log_root)? {
        let entry = entry?;
        let ft = entry.file_type()?;
        if !ft.is_dir() {
            continue;
        }   // if

        let path = entry.path();

        // Не трогаем текущий каталог запуска.
        if current_run_dir.as_ref().is_some_and(|p| p == &path) {
            continue;
        }   // if

        // Берём modified; если не удалось — считаем “очень старым”.
        let mtime = entry.metadata()
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        dirs.push((path, mtime));
    }   // for

    // Сортировка: новые сверху.
    dirs.sort_by(|a, b| b.1.cmp(&a.1));

    // Защищаем минимум N самых новых каталогов.
    let protected_count = usize::try_from(min_dirs_to_keep).unwrap_or(usize::MAX);

    let mut first_err: Option<io::Error> = None;

    for (idx, (dir_path, mtime)) in dirs.iter().enumerate() {

        // 1) Минимум N новых — не удаляем.
        if idx < protected_count {
            continue;
        }   // if

        // 2) По возрасту: если cutoff не вычислился, не удаляем вообще.
        let Some(cutoff) = cutoff else {
            continue;
        };

        // 3) Старые — удаляем.
        if *mtime < cutoff {
            if let Err(e) = fs::remove_dir_all(dir_path) {
                if first_err.is_none() {
                    first_err = Some(e);
                }
            }   // if
        }   // if
    }   // for

    if let Some(e) = first_err {
        return Err(e);
    }   // if

    Ok(())
}   // clean_up_logs()
