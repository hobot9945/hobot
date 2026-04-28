//! command_processor — Исполнение команд директивы и построение отчёта.
//!
//! # ОПИСАНИЕ
//! Модуль содержит реализацию `CommandProcessor`, которая выполняет команды директивы и формирует
//! текстовый Markdown-отчёт для вставки в чат AI.
//!
//! Команды выполняются строго последовательно. При первой ошибке исполнения (неизвестная команда
//! или ошибка хэндлера) выполнение директивы останавливается, но отчёт формируется полностью:
//! - вывод/ошибка каждой выполненной команды попадает в отчёт;
//! - в отчёте директивы фиксируется причина остановки.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! 1. Исполнение списка команд в порядке директивы.
//! 2. Накопление результатов команд и статуса директивы.
//! 3. Формирование стабильного Markdown-отчёта (с adaptive fences, чтобы не ломать разметку).
//!
//! # ИНВАРИАНТЫ
//! - Внутри директивы команды выполняются строго последовательно.
//! - Отчеты по командам сохраняются и накапливаются.
//! - `Err(...)` из `process_commands()` возвращается только при инфраструктурных сбоях
//!   (в текущей реализации не используется, но контракт сохранён).
//!
//! # ГРАНИЦЫ ИНКАПСУЛЯЦИИ
//! - Внешний код (выше `directive`) не имеет доступа к структурам результатов.
//! - Единственный потребитель модуля — родительский `directive` (через `pub(super)` API).

mod test_command_processor_test;

use crate::agent::request::directive::{Command, DirectiveContext};
use crate::agent::request::{report, session};
use crate::glob::AgentError;
use crate::handler::HandlerRegistry;
use crate::{glob, library};

/// Результат выполнения одной команды.
///
/// Приватный тип: используется только внутри `command_processor` и тестов-подмодулей.
#[derive(Debug)]
struct CommandResult {
    id: u32,                               // ID команды из директивы.
    name: String,                          // Имя команды.
    cmd_err_msg: Option<String>,           // Сообщение об ошибке или None.
    cmd_result: Option<String>,            // Текстовый результат выполнения или None.
}   // CommandResult

/// Результат выполнения директивы.
///
/// Приватный тип: используется только внутри `command_processor` и тестов-подмодулей.
#[derive(Debug)]
struct DirectiveResult {
    dir_id: u32,                           // ID директивы.
    dir_err_msg: Option<String>,           // Ошибка уровня директивы.
    total_cmd: u32,                        // Общее количество команд в директиве.
    completed_cmd: u32,                    // Количество успешно выполненных команд.
    cmd_results: Vec<CommandResult>,       // Результаты команд.
}   // DirectiveResult

/// Исполнитель и сборщик отчёта по директиве.
///
/// Тип доступен только родительскому модулю `directive`.
#[derive(Debug)]
pub(super) struct CommandProcessor {
    handler_registry: HandlerRegistry,
    dir_res: DirectiveResult,
}   // CommandProcessor

impl CommandProcessor {

    /// Описание: Создает новый пустой процессор команд.
    ///
    /// # Возвращаемое значение
    /// Тип: Self: Новый `CommandProcessor` в нулевом состоянии.
    pub(super) fn new() -> Self {
        Self {
            handler_registry: HandlerRegistry::new(),
            dir_res: DirectiveResult {
                dir_id: 0,
                dir_err_msg: None,
                total_cmd: 0,
                completed_cmd: 0,
                cmd_results: Vec::new(),
            },
        }
    }   // new()

    /// Описание: Исполняет команды директивы последовательно и накапливает результаты.
    ///
    /// Метод получает список `Command` (уже распарсенных из JSON директивы) и:
    /// - создает реестр хэндлеров (`HandlerRegistry`);
    /// - для каждой команды ищет хэндлер по имени `cmd.name`;
    /// - вызывает хэндлер и сохраняет результат или ошибку в `dir_res.cmd_results`;
    /// - на первой ошибке (неизвестная команда или `Err` от хэндлера) останавливает цикл.
    ///
    /// # ВАЖНО: Ошибки команд — это данные отчёта
    /// Ошибки выполнения команд **не** возвращаются через `Err(...)`.
    /// Они сохраняются в `dir_res` и будут отражены в отчёте `build_report()`.
    ///
    /// `Err(String)` возвращается только для инфраструктурных/программных ошибок
    /// (например, если создание `HandlerRegistry` в будущем станет fallible).
    ///
    /// # Параметры
    /// - `commands`: Срез команд директивы в порядке исполнения.
    /// - `dir_id`: Идентификатор директивы (для отчёта).
    /// - `dir_comment`: Комментарий директивы.
    ///
    /// # Алгоритм работы
    /// - Записывает `dir_id` и общее число команд `total_cmd`.
    /// - Для каждой команды:
    ///   - пытается найти хэндлер по имени;
    ///   - если не найден:
    ///     - добавляет результат команды с ошибкой “Неизвестная команда”;
    ///     - записывает краткую ошибку директивы (`dir_err_msg`);
    ///     - прекращает исполнение директивы;
    ///   - если найден:
    ///     - вызывает хэндлер `h(&cmd.params)`;
    ///     - если `Ok(payload)`:
    ///       - сохраняет результат команды;
    ///     - если `Err(err_msg)`:
    ///       - сохраняет ошибку команды (как есть, может содержать fenced-блок);
    ///       - записывает краткую ошибку директивы (`dir_err_msg`);
    ///       - прекращает исполнение директивы.
    ///
    /// # Побочные эффекты
    /// - Заполняет `self.dir_res.cmd_results`.
    /// - Увеличивает `self.dir_res.completed_cmd` на каждую успешно выполненную команду.
    /// - В случае остановки директивы устанавливает `self.dir_res.dir_err_msg`.
    pub(super) fn process_commands(&mut self, commands: &[Command], dir_id: u32, dir_comment: &Option<String>)
        -> Result<(), AgentError>
    {
        // 1) Зафиксировать метаданные директивы (для отчёта).
        self.dir_res.dir_id = dir_id;
        self.dir_res.total_cmd = commands.len() as u32;

        // 2) В случае, если установлен пошаговый режим, запрашиваем разрешение на исполнение.
        let step_through = session::step_through()?;
        let mut step_description =
            format!("Директива {} {} {}", glob::PROTOCOL_TAG_AI_OPEN, dir_id, session::session_id()?);
        step_description = if dir_comment.is_none() {
            step_description
        } else {
            format!("{}\n\n// {}", step_description, dir_comment.as_ref().unwrap())
        };
        if step_through && !glob::ask_step_permission(&step_description) {
            self.dir_res.dir_err_msg = Some("Исполнение Директивы прервано пользователем.".to_string());
            self.dir_res.completed_cmd = 0;
            return Ok(());
        };

        // 3) Исполняем команды строго последовательно, как пришли в директиве.
        for cmd in commands {

            // 3.1) Найти хэндлер по имени команды.
            let handler = self.handler_registry.handlers().get(cmd.name.as_str());
            if handler.is_none() {

                // 3.1.1) Неизвестная команда: сохраняем ошибку на уровне команды...
                let err_msg = format!("Неизвестная команда: '{}'", cmd.name);

                // Ошибка команды — сохраняем как текст. Форматирование будет выполнено на этапе build_report().
                self._add_cmd_result(cmd.cmd_id, cmd.name.clone(), None, Some(err_msg));

                // 3.1.2) ...и фиксируем краткую причину остановки на уровне директивы.
                self.dir_res.dir_err_msg = Some(format!(
                    "Выполнение директивы остановлено: команда cmd_id={} не выполнена (неизвестная команда).",
                    cmd.cmd_id
                ));

                // 3.1.3) По протоколу: останов на первой ошибке.
                break;
            }   // if

            // 3.2) Хэндлер найден: исполняем команду.
            let h = handler.unwrap();
            let res = h(&cmd.params);
            match res {

                Ok(payload) => {
                    // 3.2.1) Успех: сохраняем вывод команды.
                    self._add_cmd_result(cmd.cmd_id, cmd.name.clone(), Some(payload), None);
                },

                Err(err_msg) => {
                    // 3.2.2) Ошибка хэндлера: сохраняем текст ошибки команды (может быть уже fenced-блоком).
                    self._add_cmd_result(cmd.cmd_id, cmd.name.clone(), None, Some(err_msg));

                    // 3.2.3) Фиксируем краткую причину остановки директивы.
                    self.dir_res.dir_err_msg = Some(format!(
                        "Выполнение директивы остановлено: команда cmd_id={} завершилась ошибкой.",
                        cmd.cmd_id
                    ));

                    // 3.2.4) Останов на первой ошибке.
                    break;
                }
            }   // match
        }   // for

        // 4) Ошибок инфраструктуры не было — возвращаем Ok. Ошибки команд уже сохранены в dir_res.
        Ok(())
    }   // process_commands()
    
    /// Описание: Формирует Markdown-отчёт по директиве на основе накопленных результатов.
    ///
    /// Отчёт включает:
    /// - транспортные теги `<<<hbt DIR_ID SESSION_ID` / `>>>hbt DIR_ID SESSION_ID`;
    /// - заголовок и общий статус директивы;
    /// - при частичном выполнении — секцию “Детали директивы”;
    /// - секции по каждой выполненной команде (вывод/ошибка).
    ///
    /// # Параметры
    /// - `dir_ctx`: Контекст директивы (используется для `session_id` и метаданных).
    ///
    /// # Алгоритм работы
    /// - Формирует opening/closing теги `<<<hbt ... >>>hbt`.
    /// - Генерирует заголовок и строку статуса:
    ///   - ✅ если `dir_err_msg = None` и `completed_cmd == total_cmd`;
    ///   - ⚠️ иначе.
    /// - Если директива завершилась частично — печатает `dir_err_msg` в fenced-блоке.
    /// - Для каждой команды печатает секцию:
    ///   - ✅ + “Вывод” для успешной команды;
    ///   - ⚠️ + “Текст ошибки” для ошибки;
    ///   - ⚠️ + “Вывод (частичный)” + “Текст ошибки” для смешанного случая.
    /// - Полезная нагрузка команд (`payload`/`err_msg`) вставляется в отчёт “как есть”.
    ///   Предполагается, что хэндлеры возвращают уже корректно оформленный Markdown
    ///   (обычно fenced-блок через `wrap_in_fence()`).
    /// - Для случая пустого вывода формируется fenced-блок с текстом `(пусто)`.
    ///
    /// # Побочные эффекты
    /// - Перезаписывает `REPORT`.
    pub(super) fn build_work_report(&mut self, dir_ctx: &DirectiveContext) {

        // ВАЖНО: transport-теги и session_id формируются здесь, чтобы внешний код не собирал отчёт вручную.
        let opening_bracket = format!("`<<<hbt {} {}`\n", self.dir_res.dir_id, dir_ctx.session_id);
        let closing_bracket = format!("`>>>hbt {} {}`\n", self.dir_res.dir_id, dir_ctx.session_id);

        let mut body = String::new();

        // 1) Заголовок и статус
        body.push_str(&format!("# 🏁 Отчет по директиве #{}\n", self.dir_res.dir_id));

        if self.dir_res.dir_err_msg.is_none() && self.dir_res.completed_cmd == self.dir_res.total_cmd {

            body.push_str(&format!(
                "> **Статус**: ✅ Выполнено ({}/{})\n\n",
                self.dir_res.completed_cmd, self.dir_res.total_cmd
            ));

        } else {

            body.push_str(&format!(
                "> **Статус**: ⚠️ Выполнено ({}/{})\n\n",
                self.dir_res.completed_cmd, self.dir_res.total_cmd
            ));

            if let Some(msg) = self.dir_res.dir_err_msg.as_ref() {
                body.push_str("**Детали директивы**:\n\n");
                library::markdown_fence::push_fenced_block(&mut body, msg);
                body.push('\n');
            }   // if

        }   // if

        // 2) Команды
        for (idx, r) in self.dir_res.cmd_results.iter().enumerate() {

            // Ищем команду в контексте директивы, у которой cmd_id совпадает с id результата
            let command = dir_ctx.commands.iter().find(|c| c.cmd_id == r.id);

            // Заголовок команды
            let human_idx = idx + 1;
            if let Some(cmd) = command && let Some(ref comment) = cmd.cmd_comment {

                // Есть комментарий команды. Вставляем в заголовок.
                body.push_str(&format!("### {}. `{}`\n", human_idx, comment));
                body.push_str(&format!("- **`{}` (ID: {})**\n", r.name, r.id));
            } else {
                // Нет комментария команды.
                body.push_str(&format!("### {}. `{}` (ID: {})\n", human_idx, r.name, r.id));
            }

            // Отчет по команде
            match (&r.cmd_result, &r.cmd_err_msg) {

                (Some(payload), None) => {

                    body.push_str("- **Результат**: ✅ OK\n");
                    body.push_str("- **Вывод**:\n\n");
                    body.push_str(payload);
                    body.push('\n');

                },

                (None, Some(err_msg)) => {

                    body.push_str("- **Результат**: ⚠️ ОШИБКА\n");
                    body.push_str("- **Текст ошибки**:\n\n");
                    body.push_str(err_msg);
                    body.push('\n');

                },

                (None, None) => {

                    body.push_str("- **Результат**: ✅ OK\n");
                    body.push_str("- **Вывод**:\n\n");
                    library::markdown_fence::push_fenced_block(&mut body, "(пусто)");
                    body.push('\n');

                },

                (Some(payload), Some(err_msg)) => {

                    body.push_str("- **Результат**: ⚠️ ОШИБКА\n");
                    body.push_str("- **Вывод (частичный)**:\n\n");
                    body.push_str(payload);
                    body.push('\n');

                    body.push_str("- **Текст ошибки**:\n\n");
                    body.push_str(err_msg);
                    body.push('\n');

                }
            }   // match
        }   // for

        let _ = report::set_work_report(&format!("{}{}{}", opening_bracket, body, closing_bracket));
    }   // build_work_report()

    /// Описание: Формирует компактный Markdown-отчёт по директиве для comment_log.md.
    ///
    /// Формат:
    /// - `## Директива <dir_id> <session_id>`
    /// - `<dir_comment>`
    /// - `- Статус: ... (x/y)`
    /// - `- Команды:` + список команд
    ///   - `### (<cmd_id>) <name> [<params_snip_60>]`
    ///     `<cmd_comment>`
    ///       - `Статус: ...`
    ///         `Ошибка: <snip_100>` (только при ошибке)
    ///
    /// # Побочные эффекты
    /// - Перезаписывает `REPORT.comment_report`.
    pub(super) fn build_comment_report(&mut self, dir_ctx: &DirectiveContext) {

        use std::collections::HashMap;

        const PARAMS_SNIP_LEN: usize = 80;
        const ERR_SNIP_LEN: usize = 100;
        const COMMENT_WRAP_LEN: usize = 100;

        // 1) Заголовок директивы: как в транспортных скобках <<<ai DIR_ID SESSION_ID
        let mut out = String::new();
        out.push_str(&format!("## Директива {} {}\n", self.dir_res.dir_id, dir_ctx.session_id));

        // 2) Комментарий директивы (текстом, без булетов)
        let dir_comment = dir_ctx.dir_comment.as_deref().unwrap_or("(без комментария)");
        Self::_push_wrapped_text(&mut out, 0, dir_comment.trim(), COMMENT_WRAP_LEN);
        out.push_str("\n\n");

        // 3) Статус директивы
        let is_dir_ok =
            self.dir_res.dir_err_msg.is_none() &&
                self.dir_res.completed_cmd == self.dir_res.total_cmd;

        let dir_status = if is_dir_ok { "✅ OK" } else { "⚠️ ОШИБКА" };

        out.push_str(&format!(
            "- Статус: {} ({}/{})\n",
            dir_status,
            self.dir_res.completed_cmd,
            self.dir_res.total_cmd
        ));

        if let Some(msg) = self.dir_res.dir_err_msg.as_deref() {
            let reason = Self::_err_snippet_one_line(msg, 200);   // 200 тут уместнее, чем 100
            out.push_str("  - Причина остановки: ");
            out.push_str(&reason);
            out.push('\n');
        }   // if

        // 4) Индекс результатов по cmd_id
        let mut res_map: HashMap<u32, &CommandResult> = HashMap::new();
        for r in &self.dir_res.cmd_results {
            res_map.insert(r.id, r);
        }   // for

        // 5) Команды
        out.push_str("- Команды:\n");

        for cmd in &dir_ctx.commands {

            // 5.1) Заголовок команды (id, name, params preview)
            let params_preview = Self::_params_snippet(&cmd.params, PARAMS_SNIP_LEN);
            out.push_str(&format!("  - ### ({}) {} [{}]\n", cmd.cmd_id, cmd.name, params_preview));

            // 5.2) Комментарий команды
            let cmd_comment = cmd.cmd_comment.as_deref().unwrap_or("(без комментария)");
            Self::_push_wrapped_text(&mut out, 4, cmd_comment.trim(), COMMENT_WRAP_LEN);
            out.push('\n');

            // 5.3) Статус команды + (опционально) ошибка одной строкой
            match res_map.get(&cmd.cmd_id) {

                // Команда исполнялась и завершилась OK
                Some(r) if r.cmd_err_msg.is_none() => {
                    out.push_str("      - Статус: ✅ OK\n\n");
                },

                // Команда исполнялась и завершилась ошибкой
                Some(r) => {
                    out.push_str("      - Статус: ⚠️ ОШИБКА\n");

                    let err_msg = r.cmd_err_msg.as_deref().unwrap_or("(empty)");
                    let err_snip = Self::_err_snippet_one_line(err_msg, ERR_SNIP_LEN);

                    out.push_str("        Ошибка: ");
                    out.push_str(&err_snip);
                    out.push_str("\n\n");
                },

                // До команды не дошли (директива остановилась раньше)
                None => {
                    out.push_str("      - Статус: ⏭ НЕ ВЫПОЛНЕНО\n\n");
                }
            }   // match

        }   // for cmd

        // Разделитель директив (визуальный).
        out.push('\n');

        let _ = report::set_comment_report(&out);

    }   // build_comment_report()

    /// Описание: Сбрасывает состояние процессора команд.
    pub(super) fn clear(&mut self) {
        self.dir_res.dir_id = 0;
        self.dir_res.dir_err_msg = None;
        self.dir_res.total_cmd = 0;
        self.dir_res.completed_cmd = 0;
        self.dir_res.cmd_results.clear();
    }   // clear()
}   // impl CommandProcessor

//--------------------------------------------------------------------------------------------------
//                  Внутренние утилиты
//--------------------------------------------------------------------------------------------------
impl CommandProcessor {

    /// Описание: Добавляет результат выполнения одной команды во внутренний список.
    ///
    /// Метод также обновляет счетчик успешно выполненных команд: если `err_msg = None`,
    /// то считается, что команда выполнена успешно и `completed_cmd` увеличивается на 1.
    ///
    /// # Параметры
    /// - `id`: Идентификатор команды `cmd_id` из директивы.
    /// - `name`: Имя команды (ключ хэндлера).
    /// - `result`: Текстовый результат выполнения (stdout/вывод хэндлера) или `None`.
    /// - `err_msg`: Текст ошибки выполнения или `None`.
    ///
    /// # Алгоритм работы
    /// - Если `err_msg.is_none()` — инкрементирует `self.dir_res.completed_cmd`.
    /// - Добавляет `CommandResult { ... }` в `self.dir_res.cmd_results`.
    ///
    /// # Побочные эффекты
    /// - Модифицирует `self.dir_res.completed_cmd`.
    /// - Дописывает элемент в `self.dir_res.cmd_results`.
    fn _add_cmd_result(
        &mut self,
        id: u32,
        name: String,
        result: Option<String>,
        err_msg: Option<String>
    ) {
        if err_msg.is_none() {
            self.dir_res.completed_cmd += 1;
        }   // if

        self.dir_res.cmd_results.push(CommandResult {
            id,
            name,
            cmd_err_msg: err_msg,
            cmd_result: result,
        });
    }   // _add_cmd_result()

    /// Возвращает “превью” params (первые `max_chars` символов).
    ///
    /// # Формат
    /// - `None` / `[]` -> `(no params)`
    /// - иначе -> join через пробел и обрезка до `max_chars`.
    fn _params_snippet(params: &Option<Vec<String>>, max_chars: usize) -> String {

        let Some(v) = params.as_ref() else {
            return "(no params)".to_string();
        };

        if v.is_empty() {
            return "(no params)".to_string();
        }   // if

        let mut s = v.join(" ");
        s = s.trim().replace('\r', "");
        s = s.replace('\n', " ");

        let snippet = glob::substring(&s, 0, Some(max_chars));
        let suffix = if s.chars().count() > max_chars { "…" } else { "" };

        format!("{}{}", snippet, suffix)

    }   // _params_snippet()

    /// Возвращает первые `max_chars` символов сообщения об ошибке, в одну строку.
    ///
    /// Специально для comment_log.md:
    /// - убираем переносы строк (заменяем на пробел),
    /// - убираем внешние бэктики (чтобы не приезжали ``` из wrap_in_fence),
    /// - обрезаем до `max_chars`.
    fn _err_snippet_one_line(err_msg: &str, max_chars: usize) -> String {

        // 1) Снять возможную fenced-обёртку самым простым способом:
        //    - удалить бэктики по краям (обычно это ``` ... ```)
        //    - это не идеальный “парсер fence”, но прост и решает основную проблему.
        let mut s = err_msg.trim().trim_matches('`').to_string();

        // 2) Нормализовать в одну строку.
        s = s.replace('\r', "");
        s = s.replace('\n', " ");
        s = s.trim().to_string();

        // 3) Обрезка.
        let snippet = glob::substring(&s, 0, Some(max_chars));
        let suffix = if s.chars().count() > max_chars { "…" } else { "" };

        format!("{}{}", snippet, suffix)

    }   // _err_snippet_one_line()
    /// Пишет текст с переносом строк по ширине `max_line_len`.
    ///
    /// # Важно
    /// - Это форматирование для удобства чтения “сырого markdown”.
    /// - Перенос делается по словам (`split_whitespace()`), поэтому множественные пробелы схлопываются.
    ///
    /// # Параметры
    /// - `indent`: Отступ слева пробелами (важно для вложенных списков).
    /// - `max_line_len`: Максимальная длина строки (в символах), включая отступ.
    fn _push_wrapped_text(out: &mut String, indent: usize, text: &str, max_line_len: usize) {

        let pad = " ".repeat(indent);

        for (line_idx, raw_line) in text.lines().enumerate() {

            if line_idx > 0 {
                out.push('\n');
            }   // if

            out.push_str(&pad);

            // Текущая длина строки в символах (включая indent).
            let mut cur_len: usize = indent;

            for word in raw_line.split_whitespace() {

                let word_len = word.chars().count();
                let need_space = cur_len > indent;

                // Если слово не помещается — перенос.
                // +1 учитываем пробел между словами.
                let extra = word_len + if need_space { 1 } else { 0 };

                if cur_len + extra > max_line_len && cur_len > indent {
                    out.push('\n');
                    out.push_str(&pad);
                    cur_len = indent;
                }   // if

                if cur_len > indent {
                    out.push(' ');
                    cur_len += 1;
                }   // if

                out.push_str(word);
                cur_len += word_len;
            }   // for word
        }   // for line
    }   // _push_wrapped_text()
}   // impl CommandProcessor (internal)