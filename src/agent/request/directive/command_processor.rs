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
//! - Ошибки команд не “вылетают” наружу: они сохраняются как данные отчёта.
//! - `Err(...)` из `process_commands()` возвращается только при инфраструктурных сбоях
//!   (в текущей реализации не используется, но контракт сохранён).
//!
//! # ГРАНИЦЫ ИНКАПСУЛЯЦИИ
//! - Внешний код (выше `directive`) не имеет доступа к структурам результатов.
//! - Единственный потребитель модуля — родительский `directive` (через `pub(super)` API).

mod test_command_processor_test;

use crate::agent::request::directive::{Command, DirectiveContext};
use crate::agent::request::report::Report;
use crate::handler::HandlerRegistry;
use crate::library;

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
    dir_err_msg: Option<String>,           // Ошибка уровня директивы (останов/парсинг/инфра).
    total_cmd: u32,                        // Общее количество команд в директиве.
    completed_cmd: u32,                    // Количество успешно выполненных команд.
    cmd_results: Vec<CommandResult>,       // Результаты команд.
}   // DirectiveResult

/// Исполнитель и сборщик отчёта по директиве.
///
/// Тип доступен только родительскому модулю `directive`.
#[derive(Debug)]
pub(super) struct CommandProcessor {
    dir_res: DirectiveResult,
}   // CommandProcessor

impl CommandProcessor {

    /// Описание: Создает новый пустой процессор команд.
    ///
    /// # Возвращаемое значение
    /// Тип: Self: Новый `CommandProcessor` в нулевом состоянии.
    pub(super) fn new() -> Self {
        Self {
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
    ///
    /// # Алгоритм работы
    /// - Записывает `dir_id` и общее число команд `total_cmd`.
    /// - Создает `HandlerRegistry`.
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
    pub(super) fn process_commands(&mut self, commands: &[Command], dir_id: u32) -> Result<(), String> {

        // 1) Зафиксировать метаданные директивы (для отчёта).
        self.dir_res.dir_id = dir_id;
        self.dir_res.total_cmd = commands.len() as u32;

        // 2) Создать реестр хэндлеров: `command_name -> fn(params) -> Result<String, String>`.
        let registry = HandlerRegistry::new();

        // 3) Исполняем команды строго последовательно, как пришли в директиве.
        for cmd in commands {

            // 3.1) Найти хэндлер по имени команды.
            let handler = registry.handlers().get(cmd.name.as_str());
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
    /// - `report`: Контекст отчёта, который будет перезаписан итоговым текстом.
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
    /// - Для печати полезной нагрузки использует `_push_payload_as_block()`, чтобы:
    ///   - корректно обрабатывать пустые строки,
    ///   - не ломать разметку при наличии ``` в выводе,
    ///   - поддерживать payload, который уже содержит fenced-блок.
    ///
    /// # Побочные эффекты
    /// - Полностью перезаписывает `report.text`.
    pub(super) fn build_report(&mut self, report: &mut Report, dir_ctx: &DirectiveContext) {

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
                "> **Статус**: ⚠️ Выполнено частично ({}/{})\n\n",
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

            let human_idx = idx + 1;
            body.push_str(&format!("### {}. `{}` (ID: {})\n", human_idx, r.name, r.id));

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

        report.text = format!("{}{}{}", opening_bracket, body, closing_bracket);
    }   // build_report()

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
}   // impl CommandProcessor (internal)