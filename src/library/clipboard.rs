//! clipboard.rs — Утилиты работы с буфером обмена.
//!
//! Модуль предоставляет унифицированный интерфейс для работы с системным буфером обмена
//! через библиотеку arboard.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Запись текста в буфер обмена.
//! - Запись RGBA-изображений в буфер обмена.
//!
//! # ИНВАРИАНТЫ
//! - Все операции используют единый механизм (arboard).
//! - Каждая запись полностью заменяет предыдущее содержимое clipboard.

use arboard::{Clipboard, ImageData};
use xcap::image::RgbaImage;

/// Описание: Читает текст из системного буфера обмена.
///
/// # Возвращаемое значение
/// Тип: Result<String, String>
/// - `Ok(text)` — текст из clipboard.
/// - `Err(msg)` — если не удалось открыть буфер обмена или в нём нет/не читается текст.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - не удалось инициализировать доступ к clipboard (`Clipboard::new`);
/// - `arboard` не смог извлечь текст (`get_text`) — например, в буфере сейчас картинка/файлы.
pub fn get_clipboard_text() -> Result<String, String> {

    // Инициализируем доступ к системному clipboard.
    let mut clipboard = Clipboard::new()
        .map_err(|e| format!("arboard: Clipboard::new() failed: {}", e))?;

    // Читаем текст.
    clipboard
        .get_text()
        .map_err(|e| format!("arboard: get_text() failed: {}", e))

}   // get_clipboard_text()

/// Описание: Записывает текст в системный буфер обмена.
///
/// # Параметры
/// - `text`: Текст для записи.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - не удалось инициализировать clipboard;
/// - не удалось записать текст.
///
/// # Побочные эффекты
/// - Полностью перезаписывает системный буфер обмена.
pub fn set_clipboard_text(text: &str) -> Result<(), String> {

    // Инициализируем доступ к системному clipboard.
    let mut clipboard = Clipboard::new()
        .map_err(|e| format!("arboard: Clipboard::new() failed: {}", e))?;

    // Записываем текст в clipboard.
    clipboard
        .set_text(text)
        .map_err(|e| format!("arboard: set_text() failed: {}", e))?;

    Ok(())
}   // set_clipboard_text()

/// Описание: Помещает RGBA-изображение в системный буфер обмена.
///
/// # Параметры
/// - `image`: RGBA-изображение для записи в clipboard.
///
/// # Ошибки
/// Возвращает `Err(String)`, если:
/// - не удалось инициализировать clipboard;
/// - не удалось записать изображение.
///
/// # Побочные эффекты
/// - Полностью перезаписывает системный буфер обмена.
pub fn set_clipboard_image(image: RgbaImage) -> Result<(), String> {

    // Извлекаем размеры изображения до того, как заберём владение данными.
    let width = image.width() as usize;
    let height = image.height() as usize;

    // Забираем сырые RGBA-байты (width * height * 4).
    // into_raw() потребляет image, возвращая Vec<u8>.
    let bytes = image.into_raw();    // Копирование в вектор. Владение надо забирать, карман хочет.

    // Формируем структуру данных для arboard.
    // bytes.into() конвертирует Vec<u8> в Cow<'static, [u8]>.
    let image_data = ImageData {
        width,
        height,
        bytes: bytes.into(),
    };

    // Инициализируем доступ к системному clipboard.
    let mut clipboard = Clipboard::new()
        .map_err(|e| format!("arboard: Clipboard::new() failed: {}", e))?;

    // Записываем изображение в clipboard.
    clipboard
        .set_image(image_data)
        .map_err(|e| format!("arboard: set_image() failed: {}", e))?;

    Ok(())
}   // set_clipboard_image()