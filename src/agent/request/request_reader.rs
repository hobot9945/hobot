//! request_reader.rs
//!
//! Выделение цельных запросов из потока Native Messaging.
//!
//! # ЗАЧЕМ ЭТО НУЖНО
//! Native Messaging доставляет данные “пакетами” (длина + JSON), а в `{"text": ...}`
//! может лежать:
//! - один полный запрос,
//! - несколько запросов подряд (в тестах).
//!
//! Поэтому нужен слой, который:
//! 1) накапливает текст, если в одном пакете было несколько запросов,
//! 2) отбрасывает мусор до маркеров протокола,
//! 3) вырезает ровно один цельный `<<<ai ... >>>ai` или `<<<ext ... >>>ext` запрос за вызов.
//!
//! # ГРАНИЦЫ
//! - Внутрь JSON тела директивы/EXT сообщения модуль не заглядывает.
//! - Валидность заголовков/структуры будет проверять `RequestProcessor::_unwrap_brackets()`.
//! - Между Native Messaging пакетами “частей запроса” нет: один пакет содержит только целые запросы.

mod test_request_reader_test;

use std::io;
use std::io::Read;
use serde::Deserialize;
use crate::glob;
use crate::glob::error_control::AgentError;

/// Ридер запросов поверх Native Messaging.
///
/// Держит внутренний буфер только для случая “пачка запросов в одном пакете”:
/// после извлечения первого запроса остаток остается в буфере и будет выдан следующим вызовом.
pub struct RequestReader {
    input_buffer: String,   // Накопленный хвост после вырезания очередного запроса.
}   // RequestReader

impl RequestReader {

    /// Описание: Создает новый RequestReader.
    ///
    /// # Возвращаемое значение
    /// Type: Self: Пустой ридер (буфер пуст).
    pub fn new() -> Self {
        Self {
            input_buffer: String::new(),
        }
    }   // new()

    /// Описание: Возвращает следующий цельный запрос `<<<ai ... >>>ai` или `<<<ext ... >>>ext`.
    ///
    /// # Алгоритм работы
    /// - Если в буфере уже есть цельный запрос (остаток после batching) — возвращаем его.
    /// - Иначе читаем следующий Native Messaging пакет и дописываем его `text` в буфер.
    /// - Дальше снова пытаемся извлечь один запрос из буфера.
    ///
    /// # Ошибки
    /// - Ошибки чтения Native Messaging пакета (делегируются из `read_raw_request()`).
    ///
    /// # Возвращаемое значение
    /// Type: Result<Option<String>, AgentError>
    /// - Ok(Some(String)): найден и извлечен ровно один полный запрос (включая теги).
    /// - Ok(None): stdin штатно закрыт.
    pub fn read_next_request(&mut self, input: &mut impl Read) -> Result<Option<String>, AgentError> {

        loop {
            if let Some(req) = self._try_extract_next_request() {
                return Ok(Some(req));
            }   // if

            match Self::_read_raw_request(input)? {
                Some(chunk) => {
                    self.input_buffer.push_str(&chunk);
                },

                None => {
                    // Поток закрыт: остаток в буфере считаем мусором и просто игнорируем.
                    // (Если захочешь сделать это критикой — скажи, поменяем на Err(Critical).)
                    self.input_buffer.clear();
                    return Ok(None);
                }
            }   // match
        }   // loop
    }   // read_next_request()
}   // impl RequestReader

// Внутренний интерфейс.
impl RequestReader {

    /// Читает из входного потока один пакет Native Messaging. Это директива AI или пакет расширения,
    /// но метод не заглядывает в принятый текст.
    ///
    /// # Алгоритм работы
    /// 1. Читает первые 4 байта (длина сообщения, native-endian).
    /// 2. Если поток закрыт (EOF), возвращает `Ok(None)`.
    /// 3. Выделяет буфер нужного размера и читает тело сообщения.
    /// 4. Десериализует JSON-обертку `{"text": "..."}`.
    ///
    /// # Ошибки
    /// Возвращает `Err(AgentError)` в следующих случаях:
    /// - `AgentError::Critical`: Ошибки ввода-вывода (например, обрыв канала stdin).
    ///
    /// # Возвращаемое значение
    /// `Result<Option<String>, AgentError>`: Текст сообщения или `None` при штатном закрытии потока.
    pub fn _read_raw_request(input: &mut impl Read) -> Result<Option<String>, AgentError> {
        // Обертка Native Messaging (внутренняя структура)
        #[derive(Deserialize)]
        struct InputMessage {
            text: String,
        }   // struct InputMessage

        let mut len_bytes = [0u8; 4];

        // Чтение длины сообщения
        if let Err(e) = input.read_exact(&mut len_bytes) {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                return Ok(None);
            }   // if EOF
            return Err(AgentError::Critical(format!("Ошибка чтения заголовка (длины): {}", e)));
        }   // if

        let len = u32::from_ne_bytes(len_bytes) as usize;
        let mut buf = vec![0u8; len];

        // Чтение тела сообщения
        if let Err(e) = input.read_exact(&mut buf) {
            return Err(AgentError::Critical(format!("Ошибка чтения тела сообщения (ожидалось {} байт): {}", len, e)));
        }   // if

        // Десериализация обертки
        let msg: InputMessage = serde_json::from_slice(&buf)
            .map_err(|e| AgentError::Critical(format!("Ошибка десериализации Native Messaging обертки: {}", e)))?;

        Ok(Some(msg.text))
    }   // read_raw_request()

    /// Пытается извлечь один полный запрос из `input_buffer`.
    ///
    /// Возвращает Some только если смогли выделить границу запроса.
    fn _try_extract_next_request(&mut self) -> Option<String> {

        // Удаляем мусор до ближайшего `<<<ai` / `<<<ext`.
        // Если маркеров нет — чистим буфер полностью: “частей запроса” между пакетами не бывает.
        Self::_drop_garbage_prefix(&mut self.input_buffer);

        if self.input_buffer.starts_with(glob::PROTOCOL_TAG_EXT_OPEN) {
            return Self::_extract_ext_request(&mut self.input_buffer);
        }   // if

        if self.input_buffer.starts_with(glob::PROTOCOL_TAG_AI_OPEN) {
            return Self::_extract_ai_request(&mut self.input_buffer);
        }   // if

        None
    }   // _try_extract_next_request()

    /// Извлекает `<<<ext ... >>>ext`.
    fn _extract_ext_request(buf: &mut String) -> Option<String> {
        let close_marker = glob::PROTOCOL_TAG_EXT_CLOSE;

        let close_pos = buf.find(close_marker)?;
        let end_pos = close_pos + close_marker.len();

        // Вырезаем “как есть”. Внутренние парсеры сами триммят и валидируют.
        let req = buf[..end_pos].trim().to_string();
        buf.drain(..end_pos);

        Self::_trim_leading_ws(buf);

        Some(req)
    }   // _extract_ext_request()

    /// Извлекает `<<<ai ... >>>ai ...`.
    ///
    /// Важно: мы НЕ парсим метаданные закрывающего тега.
    /// Границу запроса определяем так:
    /// 1) нашли `>>>ai`,
    /// 2) дальше ищем ближайший следующий `<<<ai`/`<<<ext` (если он есть),
    /// 3) считаем что всё до него — один запрос.
    fn _extract_ai_request(buf: &mut String) -> Option<String> {
        let close_marker = glob::PROTOCOL_TAG_AI_CLOSE;

        let close_pos = buf.find(close_marker)?;

        // Ищем начало следующего запроса после `>>>ai`.
        // Это позволяет корректно резать пачку без разбора `>>>ai DIR_ID SESSION_ID`.
        let search_from = close_pos + close_marker.len();
        let next_ai = buf[search_from..].find(glob::PROTOCOL_TAG_AI_OPEN).map(|i| search_from + i);
        let next_ext = buf[search_from..].find(glob::PROTOCOL_TAG_EXT_OPEN).map(|i| search_from + i);

        let end_pos = match (next_ai, next_ext) {
            (Some(a), Some(e)) => std::cmp::min(a, e),
            (Some(a), None) => a,
            (None, Some(e)) => e,
            (None, None) => buf.len(),
        };

        let req = buf[..end_pos].trim().to_string();
        buf.drain(..end_pos);

        Self::_trim_leading_ws(buf);

        Some(req)
    }   // _extract_ai_request()

    /// Удаляет ведущие whitespace в буфере.
    ///
    /// Это нужно, чтобы следующий поиск маркера работал стабильно.
    fn _trim_leading_ws(buf: &mut String) {
        if buf.is_empty() {
            return;
        }   // if
        
        if buf.as_bytes().first().is_some_and(|b| !b.is_ascii_whitespace()) {
            return;
        }   // if
        
        *buf = buf.trim_start().to_string();
    }   // _trim_leading_ws()

    /// Удаляет мусор до первого `<<<ai` / `<<<ext`.
    ///
    /// Так как “частей запроса” между пакетами не бывает, если маркеров в буфере нет —
    /// буфер можно очистить полностью.
    fn _drop_garbage_prefix(buf: &mut String) {
        let ai_pos = buf.find(glob::PROTOCOL_TAG_AI_OPEN);
        let ext_pos = buf.find(glob::PROTOCOL_TAG_EXT_OPEN);

        let first_pos = match (ai_pos, ext_pos) {
            (Some(a), Some(e)) => Some(std::cmp::min(a, e)),
            (Some(a), None) => Some(a),
            (None, Some(e)) => Some(e),
            (None, None) => None,
        };

        match first_pos {
            Some(0) => { /* уже на маркере */ }

            Some(pos) => {
                buf.drain(..pos);
            }

            None => {
                buf.clear();
            }
        }   // match
    }   // _drop_garbage_prefix()

}   // impl RequestReader (private)
