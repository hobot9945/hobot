//! agent.rs — Главный цикл агента (оркестрация запросов).
//!
//! ОПИСАНИЕ:
//! Модуль содержит тип `Agent`, который:
//! - читает сообщения Native Messaging из входного потока,
//! - выделяет из потока отдельные запросы (AI/EXT),
//! - запускает обработку запроса и формирует отчёт,
//! - вставляет отчёт в окно AI через Win32 (clipboard + Ctrl+V),
//! - отправляет расширению сигнал DIRECTIVE_COMPLETED,
//! - ведёт work.log для отладки.
//!
//! ОТВЕТСТВЕННОСТЬ:
//! 1. Основной цикл обработки (LoopDriver) + управление состояниями.
//! 2. Логирование сырых запросов и отчётов в work.log.
//! 3. Пробрасывание критических ошибок в ErrorControl и, по возможности, в UI.
//!
//! ИНВАРИАНТЫ:
//! - Перед обработкой каждого запроса сбрасываются контексты `RequestProcessor` и `ErrorControl`.
//! - Агент не пишет «просто текст» в stdout: только Native Messaging обёртка через glob::send_to_stdout().
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use crate::agent::request::{report, session, RequestSource};
use request::request_reader::RequestReader;

pub(crate) mod request;
mod test_agent_test;

use crate::{glob, handle_error, handle_log, library};
use crate::glob::error_control::AgentError;
use crate::glob::{show_error_message, substring};
use crate::library::window::{find_window_by_needle, paste_text_into_window_by_needle};

/// Главный объект агента: читает запросы, обрабатывает и доставляет отчёты в UI.
pub struct Agent {

    // Процессор одного запроса: unwrap протокола, маршрутизация, построение отчёта.
    request_processor: request::RequestProcessor,

    // Ридер Native Messaging: режет поток на цельные запросы <<<ai/<<<ext.
    request_reader: RequestReader,

    // Лог рабочей сессии. None если файл открыть не удалось (критика уже зафиксирована).
    work_log: Option<File>,

    // Тестовый флаг: остановиться после одного обработанного запроса.
    #[cfg(test)]
    do_only_once: bool,
}   // Agent

// Внешний интерфейс
impl Agent {

    /// Создает новый `Agent`.
    ///
    /// # Побочные эффекты
    /// - Пытается открыть work.log (truncate/append по конфигу).
    /// - При ошибке открытия work.log фиксирует критическую ошибку, но агент продолжает работу.
    pub fn new() -> Self {
        let work_log = match Self::_open_work_log() {
            Ok(work_log) => Some(work_log),
            Err(e) => {
                let cfg = crate::glob::config();
                let path = PathBuf::from(&cfg.worklog_path);
                handle_error!("Критическая ошибка: не удалось открыть work.log ({}): {}", path.display(), e);
                None
            }
        };

        Self {
            request_processor: request::RequestProcessor::new(),
            request_reader: RequestReader::new(),
            work_log,
            #[cfg(test)]
            do_only_once: false,
        }
    }   // new()

    /// Основной цикл приема и обработки запросов.
    ///
    /// # Алгоритм работы
    /// - Последовательно читает запросы из `input`.
    /// - Для каждого запроса: сбрасывает контексты → парсит/обрабатывает → пишет отчёт в UI.
    /// - Завершает работу по EXT-сообщению COMPLETION или при закрытии входного потока.
    ///
    /// # Параметры
    /// - `input`: Источник байт Native Messaging (stdin в проде, Cursor в тестах).
    ///
    /// # Побочные эффекты
    /// - Пишет в work.log.
    /// - Вставляет отчёты в окно AI.
    /// - Шлёт сообщения в stdout (Native Messaging) для расширения.
    pub fn run(&mut self, mut input: impl Read) {

        self._writeln_to_worklog("=== сессия начата ===");

        // Управляющая переменная цикла.
        #[derive(Debug)]
        enum LoopDriver {
            /// Сброс всех состояний перед приемом нового запроса.
            Reset,

            /// Получение из входного потока текста запроса в запросных скобках <<<ai или <<<ext.
            GetRawRequest,

            /// Обработать запрос. Для AI-директив исполняются все команды, по необходимости. Минимально,
            /// запрос просто парсится и для него генерируется отчет.
            ProcessRequest,

            /// Логируем и отсылаем AI отчеты.
            LogAndSendToAi,

            /// Перейти к следующему запросу.
            DoNextRequest,

            /// Завершить цикл.
            Finish,
        }

        // Основной цикл 
        let mut drv = LoopDriver::Reset;
        let mut raw_request = "".to_string();
        let mut is_completion_signal_to_be_sent = false;
        loop {
            match drv {

                // Сброс состояния контекстов перед новой итерацией.
                LoopDriver::Reset => {
                    self.request_processor.clear();
                    is_completion_signal_to_be_sent = false;
                    drv = LoopDriver::GetRawRequest;
                },

                // Читаем сырой текст запроса. Ошибки, если они случатся, в основном, будут критическими.
                // Но на всякий случай, обрабатываем и восстановимую ошибку.
                LoopDriver::GetRawRequest => {

                    match self.request_reader.read_next_request(&mut input) {

                        // Текст запроса принят.
                        Ok(Some(text)) => {

                            // Сохраняем запрос для дальнейшей обработки.
                            raw_request = text;

                            // Поднят флаг только логирования, пишем сырой запрос в журнал, на всякий
                            // случай говорим расширению, что команда исполнена, идем в начало цикла.
                            if glob::config().is_log_only {

                                // Логируем директиву без перевода строки, чтобы отображать пришедший
                                // текст символ в символ.
                                self._write_to_worklog(&raw_request);

                                // Если, вдруг, расширение не подняло флаг LOGGER_MODE, при посылке команд
                                is_completion_signal_to_be_sent = true;

                                // Идем в начало цикла.
                                drv = LoopDriver::Reset;
                                continue;
                            } else {
                                // Логируем директиву. Запросы приходят голенькими, добавляем
                                // переводы строк для красоты.
                                self._writeln_to_worklog(&raw_request);
                            }

                            // Идем к обработке запроса.
                            drv = LoopDriver::ProcessRequest;
                            continue;
                        },

                        // stdin закрыт расширением. Веб умер. Логировать ошибку, выйти. Отчета
                        // нет и выводить уже некуда, просто выходим.
                        Ok(None) => {
                            handle_error!("Неожиданное завершение работы расширения. Хобот завершает работу.");

                            drv = LoopDriver::Finish;
                            continue;
                        }

                        // Ошибки при получении сырого текста - это будут ошибки разработки.
                        Err(e @ AgentError::Critical(_)) | Err(e @ AgentError::Recoverable(_)) => {
                            let msg = match e {
                                AgentError::Critical(m) => format!("Критическая ошибка: {}", m),
                                AgentError::Recoverable(m) => format!("Ошибка: {}", m),
                            };

                            // Ошибки - в журнал, AI.
                            handle_error!("{}", msg);

                            // Продолжаем с самого начала.
                            drv = LoopDriver::Reset;
                            continue;
                        }
                    };
                },

                // Парсинг сырого текста и обработка запросов.
                LoopDriver::ProcessRequest => {

                    // Парсинг и маршрутизация запроса (Директивы, INIT, PROTOCOL_ERROR, COMPLETION)
                    let request_res =
                        self.request_processor.process_request(&raw_request);

                    match &request_res {
                        Ok(_) => {
                            drv = LoopDriver::LogAndSendToAi;
                        },

                        Err(AgentError::Recoverable(msg)) => {
                            // Ошибка логики или парсинга, не требующая остановки агента.
                            handle_error!("Ошибка обработки запроса:\n\t{}", msg);
                            drv = LoopDriver::LogAndSendToAi;
                        },

                        Err(AgentError::Critical(msg)) => {
                            // Критическая ошибка внутри process_request (например, сбой регулярок).
                            handle_error!("Критическая ошибка обработки: {}", msg);
                            drv = LoopDriver::LogAndSendToAi;
                        }
                    };

                    // Уведомляем расширение о завершении обработки запроса. Только для AI-директив
                    // и запроса на инициализацию. И при успехе и при неудаче. Нужно освободить
                    // веб-интерфейс от необходимости удерживать фокус и вкладку.
                    if let Ok((request_source, request_type)) = request_res {

                        // Шлём уведомление о завершении обработки директивы AI.
                        if request_source == RequestSource::Ai {
                            is_completion_signal_to_be_sent = true;
                        }   // if AiDirective

                        // Шлем уведомление о завершении обработки пакета инициализации.
                        if request_source == RequestSource::Extension {
                            if let Some(msg_type) = request_type
                                && msg_type == glob::EXT_MSG_TYPE_INIT_SESSION
                            {
                                // Это был пакет инициализации. Шлем уведомление.
                                is_completion_signal_to_be_sent = true;
                            }
                        }
                    }

                    // drv уже установлен, еще перед отправкой уведомления о завершении обработки запроса.
                    continue;
                },

                // В случае нормальной обработки запросов и в случае легких ошибок логируем и отсылаем
                // AI отчеты, продолжаем работу.
                LoopDriver::LogAndSendToAi => {
                    if !self.request_processor.is_report_empty() {
                        self._writeln_to_worklog(&report::text().unwrap());
                        send_report_to_ai();

                        if is_completion_signal_to_be_sent {
                            if let Err(e) = Self::_send_directive_completion_signal() {
                                handle_error!("Критическая ошибка: не прошла отправка сигнала завершения директивы.: {}", e);
                            }   // if
                        }
                    }

                    drv = LoopDriver::DoNextRequest;
                    continue;
                },

                LoopDriver::DoNextRequest => {
                    #[cfg(test)]
                    if self.do_only_once {

                        // Поднят отладочный флаг обработки только одного запроса. Выходим.
                        drv = LoopDriver::Finish;
                        continue;
                    }

                    if self.request_processor.is_hobot_completion_requested {
                        // Затребовано завершение работы. Пишем в журнал, отчитываемся AI, выходим.
                        handle_log!("агент завершает работу по команде расширения ({}).",
                                    glob::EXT_MSG_TYPE_COMPLETION);

                        drv = LoopDriver::Finish;
                        continue;
                    }

                    drv = LoopDriver::Reset;
                    continue;
                },

                // Выход. Просто, выход.
                LoopDriver::Finish => {
                    break;
                },
            }
        }
    }
}

/// Посылает сообщение в окно AI (clipboard + Ctrl+V).
///
/// # Алгоритм работы
/// - Посылает сообщение в окно AI, текст берет из `REPORT.text`.
/// - Посылает изображения в окно AI, берет их из `REPORT.image_list`.
/// - Если не смогла послать, выдает модальное окно ошибки.
///
/// # Побочные эффекты
/// - Генерирует события клавиатуры (Ctrl+V), после использования clipboard, восстанавливает текстовое
///   содержимое, но другое, например изображение или файл, будет утеряно.
/// - Опустошает `REPORT.image_list`.
pub(crate) fn send_report_to_ai() {

    // 1. Получаем заголовок окна AI.
    let window_title = match session::window_title() {
        Ok(title) => title,
        Err(e) => {
            show_error_message("Критическая ошибка Хобота",
                               &format!("Сессия не инициализирована: {}", e));
            return;
        }
    };

    // 2. Отправляем текст, если он есть.
    let text = match report::text() {
        Ok(t) => t,
        Err(e) => {
            show_error_message("Ошибка Хобота",
                               &format!("Не удалось получить текст отчёта: {}", e));
            return;
        }
    };

    // 3. Отправляем текст.
    let text = text.trim();
    if !text.is_empty() {
        if let Err(e) = paste_text_into_window_by_needle(&window_title, text) {
            show_error_message("Ошибка Хобота",
                               &format!("Не удалось вставить отчёт в окно AI '{}':\n{}\nТекст: '{}'",
                                        window_title, e, &substring(text, 0, Some(100))));
            return;
        }
    }

    // 4. Отправляем изображения.
    let images = match report::take_images() {
        Ok(imgs) => imgs,
        Err(e) => {
            show_error_message("Ошибка Хобота",
                               &format!("Не удалось получить изображения отчёта: {}", e));
            return;
        }
    };

    for img in images {
        // Кладём изображение в clipboard.
        if let Err(e) = library::clipboard::set_clipboard_image(img) {
            show_error_message("Ошибка Хобота",
                               &format!("Не удалось поместить изображение в clipboard: {}", e));
            continue;
        }

        // Вставляем через Ctrl+V.
        if let Err(e) = library::window::paste_clipboard_into_window_by_needle(&window_title) {
            show_error_message("Ошибка Хобота",
                               &format!("Не удалось вставить изображение в окно AI '{}': {}",
                                        window_title, e));
            continue;
        }
    }   // for img
}   // send_report_to_ai()

//--------------------------------------------------------------------------------------------------
//                                      Внутренний интерфейс
//--------------------------------------------------------------------------------------------------
impl Agent {

    /// Отправляет расширению сигнал о завершении обработки директивы/инициализации.
    ///
    /// # Протокол
    /// Формирует Native Messaging пакет:
    /// ```json
    /// <<<hbt
    ///     {
    ///         "type": "DIRECTIVE_COMPLETED"
    ///     }
    /// >>>hbt
    /// ```
    ///
    /// # Когда вызывается
    /// - После **успешной** обработки AI-директивы.
    /// - После **успешной** обработки EXT INIT_SESSION.
    /// - При **аварийном** завершении директивы (чтобы освободить UI).
    ///
    /// # Алгоритм работы
    /// 1. Сериализует `CompletionSignal { msg_type: "DIRECTIVE_COMPLETED" }`.
    /// 2. Оборачивает в `<<<hbt ... >>>hbt` (без DIR_ID/SESSION_ID).
    /// 3. Отправляет через `glob::send_to_stdout()`.
    ///
    /// # Ошибки
    /// Возвращает `Err(String)` только при сбое сериализации/stdout.
    ///
    /// **ВАЖНО**: Ошибка **критична** — расширение не узнает о завершении,
    /// UI останется "занятым". Вызывающий код должен обработать.
    ///
    /// # Побочные эффекты
    /// - Пишет в stdout (Native Messaging).
    fn _send_directive_completion_signal() -> Result<(), String>{

        // Локальная структура сигнала
        #[derive(serde::Serialize)]
        struct CompletionSignal {
            #[serde(rename = "type")]
            msg_type: String,
        }   // CompletionSignal

        let signal = CompletionSignal {
            msg_type: glob::HBT_MSG_TYPE_DIRECTIVE_COMPLETED.to_string(),
        };

        // Сериализация полезной нагрузки
        let json_payload = serde_json::to_string(&signal).map_err(|e| {
            format!("ошибка сериализации сигнала: {}", e)
        })?;

        // Оборачивание в протокольные теги Хобота
        let hbt_message = format!("<<<hbt\n{}\n>>>hbt", json_payload);

        // Отправка через глобальный транспорт
        glob::send_to_stdout(&hbt_message).map_err(|e| {
            format!("ошибка отправки в stdout: {}", e)
        })?;

        Ok(())
    }   // send_completion_signal()

    /// Записывает строку БЕЗ перевода строки в work.log.
    ///
    /// # Параметры
    /// - `text`: Текст для записи (сырой запрос в режиме log_only).
    ///
    /// # Поведение
    /// - Ничего не делает, если `self.work_log = None`.
    /// - Иначе `write!(file, "{}", text)` — дописывает в конец без `\n`.
    ///
    /// # Использование
    /// Только в режиме `log_only`: для вывода директив “символ в символ”, как они приходят.
    ///
    /// # Ошибки
    /// Игнорирует `io::Error` записи (best effort).
    ///
    /// # Побочные эффекты
    /// - Дописывает в work.log (если открыт).
    fn _write_to_worklog(&mut self, text: &str) {
        // 3. Логирование входящего сырого сообщения
        if let Some(file) = self.work_log.as_mut() {
            let _ = write!(file, "{}", text);
        }
    }

    /// Записывает строку с переводом строки в work.log.
    ///
    /// # Параметры
    /// - `text`: Текст для записи (сырой запрос/отчёт).
    ///
    /// # Поведение
    /// - Ничего не делает, если `self.work_log = None` (файл не открыт).
    /// - Иначе `writeln!(file, "{}", text)` — дописывает в конец с `\n`.
    ///
    /// # Ошибки
    /// Игнорирует `io::Error` записи (best effort логирование).
    ///
    /// # Побочные эффекты
    /// - Дописывает в work.log (если открыт).
    fn _writeln_to_worklog(&mut self, text: &str) {
        // 3. Логирование входящего сырого сообщения
        if let Some(file) = self.work_log.as_mut() {
            let _ = writeln!(file, "{}", text);
        }
    }

    /// Открывает файл work.log в режиме записи с учетом настройки `are_logs_cleared_at_start`.
    ///
    /// # Поведение
    /// - Если `are_logs_cleared_at_start = true`: создает/перезаписывает (`truncate(true)`).
    /// - Если `false`: дописывает в конец (`append(true)`).
    ///
    /// # Ошибки
    /// Возвращает `io::Error` при неудаче открытия/создания файла (например, нет прав).
    ///
    /// # Побочные эффекты
    /// - Создает файл, если его не было.
    /// - Очищает/дописывает по настройке конфига.
    fn _open_work_log() -> Result<File, io::Error> {
        let cfg = crate::glob::config();
        let path = PathBuf::from(&cfg.worklog_path);

        if glob::config().are_logs_cleared_at_start {
            OpenOptions::new()
                .create(true)    // Создаёт файл, если его нет
                .write(true)     // Режим записи
                .truncate(true)  // Очищает файл при открытии (сбрасывает размер на 0)
                .open(&path)
        } else {
            OpenOptions::new()
                .create(true)    // Создаёт файл, если его нет
                .write(true)     // Режим записи
                .append(true)    // Устанавливает режим добавления в конец.
                .open(&path)
        }
    }   // _open_work_log()
}   // impl Agent (private)
