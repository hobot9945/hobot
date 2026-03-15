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
use crate::glob::{ask_step_permission, show_error_message, substring};
use crate::glob::log_control::{write_to_comment_log, write_to_work_log, writeln_to_work_log};
use crate::library::{keyboard, window};
use crate::library::window::{find_window_by_needle, paste_text_into_window_by_needle};

/// Главный объект агента: читает запросы, обрабатывает и доставляет отчёты в UI.
pub struct Agent {

    // Процессор одного запроса: unwrap протокола, маршрутизация, построение отчёта.
    request_processor: request::RequestProcessor,

    // Ридер Native Messaging: режет поток на цельные запросы <<<ai/<<<ext.
    request_reader: RequestReader,

    // -- Тестовые флаги ---

    // Остановиться после одного обработанного запроса.
    #[cfg(test)]
    do_only_once: bool,

    // Остановиться после одного обработанного запроса.
    #[cfg(test)]
    do_not_send_report_to_ai: bool,
}   // Agent

// Внешний интерфейс
impl Agent {

    /// Создает новый `Agent`.
    ///
    /// # Побочные эффекты
    /// - Пытается открыть work.log (truncate/append по конфигу).
    /// - При ошибке открытия work.log фиксирует критическую ошибку, но агент продолжает работу.
    pub fn new() -> Self {
        Self {
            request_processor: request::RequestProcessor::new(),
            request_reader: RequestReader::new(),
            #[cfg(test)]
            do_only_once: false,
            #[cfg(test)]
            do_not_send_report_to_ai: false,
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

        writeln_to_work_log("=== сессия начата ===");

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
        // Уведомление о завершении выполнения запроса надо посылать только после того как отчет
        // ушел к AI, иначе может быть преждевременно отпущен фокус поля ввода. Поэтому, сначала
        // помечаем сигнал к отправке, а отправляем после.
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
                                write_to_work_log(&raw_request);

                                // Если, вдруг, расширение не подняло флаг LOGGER_MODE, при посылке команд
                                is_completion_signal_to_be_sent = true;

                                // Идем в начало цикла.
                                drv = LoopDriver::Reset;
                                continue;
                            } else {
                                // Логируем директиву. Запросы приходят голенькими, добавляем
                                // переводы строк для красоты.
                                writeln_to_work_log(&raw_request);
                            }

                            // Идем к обработке запроса.
                            drv = LoopDriver::ProcessRequest;
                            continue;
                        },

                        // stdin закрыт расширением. Веб умер. Логировать ошибку, выйти. Отчета
                        // нет и выводить уже некуда, просто выходим.
                        Ok(None) => {
                            handle_error!("Неожиданное завершение работы расширения. Хобот завершает работу.");

                            // Завершаем работу Хобота.
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

                    match request_res {

                        // Уведомляем расширение о завершении обработки запроса. Только для AI-директив
                        // и запроса на инициализацию. И при успехе и при неудаче. Нужно освободить
                        // веб-интерфейс от необходимости удерживать фокус и вкладку.
                        Ok((request_source, request_type)) => {

                            // Шлём уведомление о завершении обработки директивы AI.
                            if request_source == RequestSource::Ai {
                                is_completion_signal_to_be_sent = true;
                            }   // if AiDirective

                            // Шлем уведомление о завершении обработки пакета инициализации.
                            if request_source == RequestSource::Extension {
                                if let Some(msg_type) = request_type
                                    && msg_type == glob::EXT_MSG_INIT_SESSION
                                {
                                    // В шаговом режиме проверяем разрешил ли пользователь начать сессию.
                                    if session::step_through().unwrap() &&
                                        !ask_step_permission("Поступил запрос на инициализацию сессии. Начинать сессию?")
                                    {
                                        // Отказано в открытии сессии. Шлем команду снятия флага занятости агента.
                                        if let Err(e) = Self::_send_directive_completion_signal() {
                                            handle_error!("Критическая ошибка: не прошла отправка сигнала завершения директивы.: {}", e);
                                        }   // if

                                        // Завершаем работу Хобота.
                                        drv = LoopDriver::Finish;
                                        continue;
                                    }

                                    // Это был пакет инициализации. Шлем уведомление.
                                    is_completion_signal_to_be_sent = true;
                                }
                            }

                            drv = LoopDriver::LogAndSendToAi;
                        },

                        Err(AgentError::Recoverable(msg)) => {
                            // Ошибка логики или парсинга, не требующая остановки агента.
                            handle_error!("Ошибка запроса:\n\t{}", msg);
                            drv = LoopDriver::LogAndSendToAi;
                        },

                        Err(AgentError::Critical(msg)) => {
                            // Критическая ошибка внутри process_request (например, сбой регулярок).
                            handle_error!("Критическая ошибка запроса: {}", msg);
                            drv = LoopDriver::LogAndSendToAi;
                        }
                    };

                    // drv уже установлен, еще перед отправкой уведомления о завершении обработки запроса.
                    continue;
                },

                // В случае нормальной обработки запросов и в случае легких ошибок логируем и отсылаем
                // AI отчеты, продолжаем работу.
                LoopDriver::LogAndSendToAi => {
                    if !self.request_processor.is_report_empty() {

                        // 1) work.log (полный отчёт)
                        let work_rep = report::work_report().unwrap();
                        if !work_rep.trim().is_empty() {
                            writeln_to_work_log(&work_rep);
                        }   // if work_rep

                        // 2) comment_log.md (компактный отчёт)
                        // По EXT-сообщениям comment_report должен быть пустым, поэтому просто пропускаем.
                        let comment_rep = report::comment_report().unwrap();
                        if !comment_rep.trim().is_empty() {
                            write_to_comment_log(&comment_rep);
                        }   // if comment_rep

                        // 3) Отправка отчёта в UI (только work_report). Исключение - для отчета о завершении
                        //    работы по запросу расширения. Окно AI уже закрыто, отчет слать некуда.
                        if !self.request_processor.is_hobot_completion_requested {

                            #[cfg(not(test))]
                            // Если работа, не тест.
                            Self::_send_report_to_ai();

                            #[cfg(test)]
                            // Если тест, но отправка в AI не заглушена (тесты с проверкой записи в поле ввода AI)
                            if !self.do_not_send_report_to_ai {
                                Self::_send_report_to_ai();
                            }
                        }

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
                                    glob::EXT_MSG_COMPLETION);

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

//--------------------------------------------------------------------------------------------------
//                                      Внутренний интерфейс
//--------------------------------------------------------------------------------------------------
impl Agent {

    /// Посылает сообщение в окно AI (clipboard + Ctrl+V).
    ///
    /// # Алгоритм работы
    /// - Посылает сообщение в окно AI, текст берет из `REPORT.work_report`.
    /// - Посылает изображения в окно AI, берет их из `REPORT.image_list`.
    /// - Если не смогла послать, выдает модальное окно ошибки.
    ///
    /// # Побочные эффекты
    /// - Генерирует события клавиатуры (Ctrl+V), после использования clipboard, восстанавливает текстовое
    ///   содержимое, но другое, например изображение или файл, будет утеряно.
    /// - Опустошает `REPORT.image_list`.
    fn _send_report_to_ai() {

        // 1. Получаем заголовок окна AI.
        let window_title = match session::window_title() {
            Ok(title) => title,
            Err(e) => {
                show_error_message("Критическая ошибка Хобота",
                                   &format!("Сессия не инициализирована: {}", e));
                return;
            }
        };

        // 2. Отправляем изображения.
        let images = match report::take_images() {
            Ok(imgs) => imgs,
            Err(e) => {
                show_error_message("Ошибка Хобота",
                                   &format!("Не удалось получить изображения отчёта: {}", e));
                return;
            }
        };

        // Обычно одно изображение, но в report, на всякий случай, содержит список образов.
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

        // 3. Отправляем текст, если он есть.
        let text = match report::work_report() {
            Ok(t) => t,
            Err(e) => {
                show_error_message("Ошибка Хобота",
                                   &format!("Не удалось получить текст отчёта: {}", e));
                return;
            }
        };

        // 4. Отправляем текст.
        let text = text.trim();
        if !text.is_empty() {
            if let Err(e) = paste_text_into_window_by_needle(&window_title, text) {
                show_error_message("Ошибка Хобота",
                                   &format!("Не удалось вставить отчёт в окно AI '{}':\n{}\nТекст: '{}'",
                                            window_title, e, &substring(text, 0, Some(100))));
                return;
            }
        }

        // Нажать Enter.
        if let Err(e) = window::press_enter_and_verify(&window_title, None) {
            show_error_message("Ошибка Хобота",
                               &format!("{}", e));
        }
    }   // send_report_to_ai()

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
            msg_type: glob::HBT_MSG_DIRECTIVE_COMPLETED.to_string(),
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
}   // impl Agent (private)
