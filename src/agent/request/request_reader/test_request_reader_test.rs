//! test_request_reader_test
//!
//! Дымовые тесты для `RequestReader`.
//!
//! # Цель
//! Тесты “ручные”: их удобно запускать по одному и смотреть вывод.
//! Проверяем только выделение границ `<<<ai ... >>>ai` / `<<<ext ... >>>ext` при batching
//! (несколько запросов подряд в одном `text`).
//!
//! # ГРАНИЦЫ
//! - Не валидируем JSON внутри директив/EXT, это задача `RequestProcessor`.
//! - Не тестируем “запрос пришел частями между native-message пакетами”: в текущей модели
//!   это не поддерживается и не требуется.
#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{wrln, writln};
    use crate::agent::request::request_reader::RequestReader;
    use crate::library::test_utils;

    /// Собирает “stdin” из нескольких Native Messaging пакетов.
    ///
    /// # Зачем
    /// `RequestReader` читает `Read` и многократно вызывает `read_raw_request()`,
    /// поэтому подсовываем один Cursor, содержащий подряд несколько пакетов.
    fn _mock_stdin_stream(native_json_msgs: &[String]) -> Cursor<Vec<u8>> {
        let mut all: Vec<u8> = Vec::new();

        for msg in native_json_msgs {
            // mock_stdin() формирует: [len u32][body bytes]
            let cur = test_utils::mock_stdin(msg);
            let bytes = cur.into_inner();

            all.extend_from_slice(&bytes);
        }   // for

        Cursor::new(all)
    }   // _mock_stdin_stream()

    #[test]
    fn smoke_single_ai_single_packet() {
        let ai = r#"<<<ai 1 AEBEEC
{"dir_comment":"smoke","commands":[]}
>>>ai 1 AEBEEC"#;

        let native_json = test_utils::wrap_to_native_json(ai);
        let mut input = _mock_stdin_stream(&[native_json]);

        let mut rdr = RequestReader::new();

        let req = rdr.read_next_request(&mut input).unwrap().unwrap();

        wrln!("Extracted:", req);
        assert_eq!(req, ai);
    }   // smoke_single_ai_single_packet()

    #[test]
    fn smoke_batched_ext_two_requests_one_packet() {
        // Два EXT запроса приехали одним native-message пакетом (batching).
        let ext1 = r#"<<<ext {"type":"INIT_SESSION","payload":{"session_id":"AEBEEC"}} >>>ext"#;
        let ext2 = r#"<<<ext {"type":"COMPLETION"} >>>ext"#;

        let batched = format!("{}\n{}", ext1, ext2);

        let native_json = test_utils::wrap_to_native_json(&batched);
        let mut input = _mock_stdin_stream(&[native_json]);

        let mut rdr = RequestReader::new();

        let req1 = rdr.read_next_request(&mut input).unwrap().unwrap();
        wrln!("Extracted #1:", req1);
        assert_eq!(req1, ext1);

        let req2 = rdr.read_next_request(&mut input).unwrap().unwrap();
        wrln!("Extracted #2:", req2);
        assert_eq!(req2, ext2);
    }   // smoke_batched_ext_two_requests_one_packet()

    #[test]
    fn smoke_batched_ai_two_requests_one_packet() {
        // Две AI директивы приехали одним `text`. Между ними может быть newline.
        let ai1 = r#"<<<ai 10 AEBEEC
{"dir_comment":"ai1","commands":[]}
>>>ai 10 AEBEEC"#;

        let ai2 = r#"<<<ai 11 AEBEEC
{"dir_comment":"ai2","commands":[]}
>>>ai 11 AEBEEC"#;

        let batched = format!("{}\n{}", ai1, ai2);

        let native_json = test_utils::wrap_to_native_json(&batched);
        let mut input = _mock_stdin_stream(&[native_json]);

        let mut rdr = RequestReader::new();

        let req1 = rdr.read_next_request(&mut input).unwrap().unwrap();
        writln!("--- Extracted #1 ---");
        writln!("{}", req1);
        writln!("--------------------");
        assert_eq!(req1, ai1);

        let req2 = rdr.read_next_request(&mut input).unwrap().unwrap();
        writln!("--- Extracted #2 ---");
        writln!("{}", req2);
        writln!("--------------------");
        assert_eq!(req2, ai2);
    }   // smoke_batched_ai_two_requests_one_packet()

    #[test]
    fn smoke_batched_ai_then_ext_no_separator() {
        // Самый противный вариант batching: второй запрос сразу “впритык” после первого.
        let ai = r#"<<<ai 20 AEBEEC
{"dir_comment":"ai","commands":[]}
>>>ai 20 AEBEEC"#;

        let ext = r#"<<<ext {"type":"COMPLETION"} >>>ext"#;

        let batched = format!("{}{}", ai, ext);

        let native_json = test_utils::wrap_to_native_json(&batched);
        let mut input = _mock_stdin_stream(&[native_json]);

        let mut rdr = RequestReader::new();

        let req1 = rdr.read_next_request(&mut input).unwrap().unwrap();
        wrln!("Extracted #1:", req1);
        assert_eq!(req1, ai);

        let req2 = rdr.read_next_request(&mut input).unwrap().unwrap();
        wrln!("Extracted #2:", req2);
        assert_eq!(req2, ext);
    }   // smoke_batched_ai_then_ext_no_separator()

    #[test]
    fn smoke_garbage_before_first_marker_is_dropped() {
        // Мусор до первого маркера должен быть выкинут.
        let garbage = " \n\nSOME_GARBAGE_TEXT\n\n";
        let ext = r#"<<<ext {"type":"COMPLETION"} >>>ext"#;

        let mixed = format!("{}{}", garbage, ext);

        let native_json = test_utils::wrap_to_native_json(&mixed);
        let mut input = _mock_stdin_stream(&[native_json]);

        let mut rdr = RequestReader::new();

        let req = rdr.read_next_request(&mut input).unwrap().unwrap();

        wrln!("Extracted:", req);
        assert_eq!(req, ext);
    }   // smoke_garbage_before_first_marker_is_dropped()
}   // mod tests
