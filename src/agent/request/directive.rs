//! directive.rs
//!
//! Процессор директив ИИ.
//!
//! Этот модуль реализует “чёрный ящик” обработки директивы для `Request`: снаружи виден только тип 
//! `Directive` и его внешний интерфейс.
//!
//! Директива в терминах протокола объекта `Directive` — это:
//! - Извлеченные из заголовка транспортных тегов: `DIRECTIVE_ID` и `SESSION_ID`.
//! - JSON-тело: `dir_comment` + массив `commands[]`, где у команды есть `cmd_id`, `name`,
//!   опциональные `params`, `cmd_comment`, `allowed_timeout`.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Принять заголовок и JSON-тело директивы и привести их к типизированному виду.
//! - Сохранить директиву во внутренний контекст (не раскрывая его наружу).
//! - Подготовить подсистему исполнения (инициализировать `CommandProcessor`).
//! - Исполнить команды строго последовательно (через `CommandProcessor`).
//! - Сформировать отчёт в `Report`.
//!
//! # ГРАНИЦЫ ИНКАПСУЛЯЦИИ
//! - `Command` и `DirectiveContext` — приватные типы: они существуют только для внутренней
//!   реализации и тестов-подмодулей.
//! - Внешний код не должен иметь доступа к `dir_id`, `session_id`, списку команд и т.п.
//!
//! # ИНВАРИАНТЫ
//! - После успешного `parse_directive_body()` директива считается “готовой к исполнению”:
//!   внутренний контекст заполнен, `CommandProcessor` инициализирован.
//! - `execute_and_build_report()` не принимает входных данных директивы: он работает только
//!   по уже распарсенному внутреннему состоянию.
//! - После `clear()` объект возвращается в нулевое состояние и готов к повторному использованию.

mod command_processor;

use serde::Deserialize;

use crate::agent::request::directive::command_processor::CommandProcessor;
use crate::glob::error_control::AgentError;

/// Описание одной команды из массива `commands[]`.
///
/// Тип приватный: используется только внутри `directive` и его подмодулей.
#[derive(Debug, Deserialize, Clone)]
struct Command {
    #[serde(default)]
    cmd_comment: Option<String>,         // Опциональный комментарий команды.
    cmd_id: u32,                         // Идентификатор команды из протокола (cmd_id).
    name: String,                        // Имя команды (ключ для поиска хандлера).
    #[serde(default)]
    params: Option<Vec<String>>,         // Опциональные параметры команды.
    #[serde(default)]
    allowed_timeout: Option<u64>,        // Опциональный таймаут выполнения (мс).
}   // Command

/// Внутренний контекст директивы.
///
/// Содержит полное состояние директивы в типизированном виде.
/// Тип приватный: наружу не экспортируется.
#[derive(Debug, Default)]
struct DirectiveContext {
    session_id: String,                  // Идентификатор сессии из заголовка `<<<ai ...`.
    dir_id: u32,                         // Номер директивы из заголовка `<<<ai ...`.
    dir_comment: Option<String>,         // Опциональный комментарий директивы.
    commands: Vec<Command>,              // Список команд к исполнению.
}   // DirectiveContext

/// Процессор директивы ИИ.
///
/// Это единственный публичный тип модуля: внешний код работает только через его методы.
#[derive(Debug)]
pub struct DirectiveProcessor {
    dir_ctx: DirectiveContext,           // Внутренний контекст директивы (приватен).
    cmd_prc: CommandProcessor,           // Исполнитель команд и генератор отчёта.
}   // DirectiveProcessor

impl DirectiveProcessor {

    /// Описание: Создает новый экземпляр процессора директивы в нулевом состоянии.
    ///
    /// # Возвращаемое значение
    /// Тип: Self: Новый `DirectiveProcessor`.
    pub fn new() -> Self {
        Self {
            dir_ctx: DirectiveContext::default(),
            cmd_prc: CommandProcessor::new(),
        }
    }   // new()

    /// Описание: Парсит директиву (заголовок + JSON тело) и подготавливает её к исполнению.
    ///
    /// # Алгоритм работы
    /// - Сохраняет `directive_id` и `session_id` во внутренний контекст.
    /// - Валидирует непустое тело.
    /// - Десериализует JSON обёртку директивы:
    ///   `{ "dir_comment": "...", "commands": [ ... ] }`.
    /// - Сохраняет `dir_comment` и `commands` в контекст.
    /// - Инициализирует `CommandProcessor` метаданными директивы (dir_id, total_cmd).
    ///
    /// # Ошибки
    /// Возвращает `AgentError::Recoverable`, если JSON некорректен или пуст.
    ///
    /// # Побочные эффекты
    /// - Перезаписывает внутреннее состояние текущей директивы.
    pub fn process_directive(
        &mut self,
        json_body: &str,
        directive_id: u32,
        session_id: String,
    ) -> Result<(), AgentError> {

        // 1. Заголовок директивы (из транспортных тегов)
        self.dir_ctx.dir_id = directive_id;
        self.dir_ctx.session_id = session_id;

        // 2. JSON-тело директивы
        if json_body.trim().is_empty() {
            return Err(AgentError::Recoverable("пустое тело директивы.".to_string()));
        }   // if

        // Обёртка директивы по протоколу: комментарий директивы + команды.
        #[derive(Deserialize)]
        struct CommandsWrapper {
            #[serde(default)]
            dir_comment: Option<String>,
            commands: Vec<Command>,
        }   // CommandsWrapper

        let wrapper = serde_json::from_str::<CommandsWrapper>(json_body).map_err(|e| {
            AgentError::Recoverable(format!(r#"oшибка в JSON директивы.
JSON:
    {}

Ожидался объект {{'commands': []}}.

Детали: {}"#, json_body, e))
        })?;

        self.dir_ctx.dir_comment = wrapper.dir_comment;
        self.dir_ctx.commands = wrapper.commands;

        // 3. Исполнение команд. Err возвращается только при критических сбоях (например, отказ 
        // HandlerRegistry). Ошибки исполнения команд ложатся в отчет, наружу не выходят.
        // Ошибка не должна предотвращать формирование отчета.
        let res = self.cmd_prc
            .process_commands(&self.dir_ctx.commands, self.dir_ctx.dir_id)
            .map_err(AgentError::Critical);

        // 4. Формируем отчет исполнения директивы.
        self.cmd_prc.build_report(&self.dir_ctx);

        res
    }   // parse_directive_body()

    /// Описание: Сбрасывает состояние процессора директивы для повторного использования.
    ///
    /// # Побочные эффекты
    /// - Очищает внутренний контекст директивы.
    /// - Сбрасывает состояние `CommandProcessor`.
    pub fn clear(&mut self) {
        self.dir_ctx.dir_id = 0;
        self.dir_ctx.session_id.clear();
        self.dir_ctx.dir_comment = None;
        self.dir_ctx.commands.clear();

        self.cmd_prc.clear();
    }   // clear()
}   // impl DirectiveProcessor
