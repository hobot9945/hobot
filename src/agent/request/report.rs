//! report.rs
//!
//! Глобальный контекст отчета (Singleton).
//! Накапливает результаты (текст и изображения) выполнения директивы.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Хранение текущего текста отчета Markdown.
//! - Хранение списка захваченных изображений.
//! - Предоставление глобального доступа.
//!
//! # ИНИЦИАЛИЗАЦИЯ
//! В отличие от `SessionContext`, отчет не требует внешних данных для инициализации.
//! Используется ленивая инициализация при первом обращении.

use std::sync::{OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};
use xcap::image::RgbaImage;
use crate::glob::error_control::AgentError;

/// Глобальное хранилище отчета.
static REPORT: OnceLock<RwLock<Report>> = OnceLock::new();

/// Структура отчета.
struct Report {
    /// Отчет для журнала work.log
    work_report: String,

    /// Отчет для журнала comment_log.md
    comment_report: String,

    /// Образы для помещения в поле ввода AI
    image_list: Vec<RgbaImage>,
}   // Report

impl Report {
    fn new() -> Self {
        Self {
            work_report: String::new(),
            comment_report: String::new(),
            image_list: Vec::new(),
        }
    }   // new()
}   // impl Report

//--------------------------------------------------------------------------------------------------
//                  Публичный интерфейс
//--------------------------------------------------------------------------------------------------

/// Описание: Очищает отчет (текст и изображения).
///
/// Вызывается в начале обработки каждой директивы.
///
/// # Ошибки
/// Возвращает `AgentError::Critical`, если RwLock отравлен.
///
/// # Побочные эффекты
/// - Очищает глобальный буфер отчета.
pub fn clear() -> Result<(), AgentError> {
    let mut wg = _get_context_write_guard()?;
    wg.work_report.clear();
    wg.comment_report.clear();
    wg.image_list.clear();
    Ok(())
}   // clear()

/// Описание: Проверяет, есть ли что-то, что нужно поместить в журналы или в поле ввода AI.
///
/// # Возвращаемое значение
/// Тип: `bool`: `true`, если текст пуст и нет изображений.
///
/// # Ошибки
/// Возвращает `AgentError::Critical`, если RwLock отравлен.
pub fn is_report_empty() -> Result<bool, AgentError> {
    let rg = _get_context_read_guard()?;
    Ok(rg.work_report.is_empty() && rg.image_list.is_empty() && rg.comment_report.is_empty())
}   // is_empty()

/// Описание: Возвращает копию текста отчета.
///
/// # Возвращаемое значение
/// Тип: `String`: Клон текущего текста отчета.
///
/// # Ошибки
/// Возвращает `AgentError::Critical`, если RwLock отравлен.
pub fn work_report() -> Result<String, AgentError> {
    let rg = _get_context_read_guard()?;
    Ok(rg.work_report.clone())
}   // text()

/// Описание: Устанавливает текст отчета (полная перезапись).
///
/// Используется процессорами директив для формирования финального отчета.
///
/// # Параметры
/// - `new_text`: Новый текст отчета.
///
/// # Ошибки
/// Возвращает `AgentError::Critical`, если RwLock отравлен.
///
/// # Побочные эффекты
/// - Перезаписывает текст в глобальном буфере отчета.
pub fn set_work_report(s: &str) -> Result<(), AgentError> {
    let mut wg = _get_context_write_guard()?;
    wg.work_report.clear();
    wg.work_report.push_str(s);
    Ok(())
}   // set_text()

/// Описание: Возвращает копию текста отчета для comment_log.
///
/// # Возвращаемое значение
/// Тип: `String`: Клон текущего текста отчета.
///
/// # Ошибки
/// Возвращает `AgentError::Critical`, если RwLock отравлен.
pub fn comment_report() -> Result<String, AgentError> {
    let rg = _get_context_read_guard()?;
    Ok(rg.comment_report.clone())
}   // comment_report()

/// Описание: Устанавливает текст отчета для comment_log (полная перезапись).
///
/// # Параметры
/// - `s`: Новый текст отчета.
///
/// # Ошибки
/// Возвращает `AgentError::Critical`, если RwLock отравлен.
///
/// # Побочные эффекты
/// - Перезаписывает текст в глобальном буфере отчета.
pub fn set_comment_report(s: &str) -> Result<(), AgentError> {
    let mut wg = _get_context_write_guard()?;
    wg.comment_report.clear();
    wg.comment_report.push_str(s);
    Ok(())
}   // set_comment_report()

/// Описание: Добавляет изображение в список отчета.
///
/// # Параметры
/// - `img`: RGBA-изображение для добавления.
///
/// # Ошибки
/// Возвращает `AgentError::Critical`, если RwLock отравлен.
///
/// # Побочные эффекты
/// - Модифицирует глобальный буфер отчета.
pub fn add_image(img: RgbaImage) -> Result<(), AgentError> {
    let mut wg = _get_context_write_guard()?;
    wg.image_list.push(img);
    Ok(())
}   // add_image()

/// Описание: Забирает все изображения из отчета (drain).
///
/// Используется при отправке отчета, чтобы переместить владение без копирования.
///
/// # Возвращаемое значение
/// Тип: `Vec<RgbaImage>`: Вектор изображений (отчет становится пустым по изображениям).
///
/// # Ошибки
/// Возвращает `AgentError::Critical`, если RwLock отравлен.
///
/// # Побочные эффекты
/// - Очищает список изображений в глобальном буфере отчета.
pub fn take_images() -> Result<Vec<RgbaImage>, AgentError> {
    let mut wg = _get_context_write_guard()?;
    Ok(std::mem::take(&mut wg.image_list))
}   // take_images()

//--------------------------------------------------------------------------------------------------
//                  Внутренний интерфейс
//--------------------------------------------------------------------------------------------------

/// Описание: Внутренний хелпер для получения блокировки чтения.
///
/// # Ошибки
/// Возвращает `AgentError::Critical`, если RwLock отравлен.
fn _get_context_read_guard() -> Result<RwLockReadGuard<'static, Report>, AgentError> {
    REPORT
        .get_or_init(|| RwLock::new(Report::new()))
        .read()
        .map_err(|e| AgentError::Critical(
            format!("{}, {}: Report RwLock poisoned (read): {}", file!(), line!(), e)
        ))
}   // _get_context_read_guard()

/// Описание: Внутренний хелпер для получения блокировки записи.
///
/// # Ошибки
/// Возвращает `AgentError::Critical`, если RwLock отравлен.
fn _get_context_write_guard() -> Result<RwLockWriteGuard<'static, Report>, AgentError> {
    REPORT
        .get_or_init(|| RwLock::new(Report::new()))
        .write()
        .map_err(|e| AgentError::Critical(
            format!("{}, {}: Report RwLock poisoned (write): {}", file!(), line!(), e)
        ))
}   // _get_context_write_guard()