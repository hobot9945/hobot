//! config.rs
//!
//! Модуль инкапсулирует логику работы с конфигурацией приложения.
//!
//! # ОТВЕТСТВЕННОСТЬ
//! - Хранение структуры настроек `AppConfig`.
//! - Загрузка конфигурации из TOML-файла при старте.
//! - Предоставление доступа к конфигурации через синглтон.
//! - Создание дефолтного конфига при отсутствии файла.

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::sync::OnceLock;

/// Путь к файлу конфигурации (относительно текущей директории запуска).
pub const CONFIG_PATH: &'static str = "config.toml";

/// Структура конфигурации приложения.
#[derive(Deserialize, Serialize, Debug)]
pub struct AppConfig {

    /// Режим "только логирование" (без реального выполнения действий).
    pub is_log_only: bool,

    /// Путь к промпту, относительно каталога исполнения. Используются обратные слэши.
    pub prompt_on_hobot: String,

    /// Число последних каталогов журналов, не переносимых в архив.
    pub unarchived_dirs: u64,

    /// Число гарантированно сохраняемых каталогов журналов.
    pub min_log_dirs_to_keep: u64,

    /// Гарантированный период хранения.
    pub log_dirs_retention_days: u64,
}   // AppConfig

impl Default for AppConfig {
    /// Создает структуру AppConfig с дефолтными значениями полей.
    fn default() -> Self {
        Self {
            is_log_only: false,
            prompt_on_hobot: "prompt\\prompt_hobot_prompt.md".to_string(),
            unarchived_dirs: 3,
            min_log_dirs_to_keep: 30,
            log_dirs_retention_days: 14
        }   // Self
    }   // default()
}   // impl Default for AppConfig

impl AppConfig {
    /// Загружает настройки из файла config.toml.
    ///
    /// # Алгоритм работы
    /// 1. Пытается прочитать файл по пути `CONFIG_PATH`.
    /// 2. Если файл существует — парсит TOML.
    /// 3. Если файла нет — создает новый с дефолтными настройками и сохраняет его.
    ///
    /// # Ошибки
    /// Возвращает `Err(String)` в случаях:
    /// - Ошибки доступа к файловой системе (чтение/запись).
    /// - Ошибки сериализации/десериализации TOML.
    pub fn load() -> Result<Self, String> {
        match fs::read_to_string(CONFIG_PATH) {
            Ok(content) => {

                // Файл прочитан, парсим, возвращаем Self
                toml::from_str(&content).map_err(|e| {
                    format!("Ошибка парсинга конфигурационного файла '{}': {}", CONFIG_PATH, e)
                })
            },  // Ok(content)

            Err(e) if e.kind() == io::ErrorKind::NotFound => {

                // Файла нет на диске. Генерируем дефолты, пишем в файл, возвращаем Self.
                let config = Self::default();

                let toml_string = toml::to_string(&config).map_err(|e| {
                    format!("Ошибка сериализации дефолтной конфигурации: {}", e)
                })?;

                fs::write(CONFIG_PATH, &toml_string).map_err(|e| {
                    format!("Не удалось записать дефолтный файл конфигурации '{}': {}", CONFIG_PATH, e)
                })?;

                Ok(config)
            },  // Err(NotFound)

            Err(e) => {
                // Другая ошибка чтения: скажем нет прав на чтение. Возвращаем ошибку.
                Err(format!("Не удалось прочитать файл конфигурации '{}': {}", CONFIG_PATH, e))
            }, // Err(other)
        }   // match read_to_string
    }   // load()
}   // impl AppConfig

/// Глобальный конфиг приложения.
/// Инициализируется один раз при старте через `init`.
static CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// Инициализирует конфигурацию.
///
/// Вызывается из `glob::initialize_glob()`. Загружает настройки и устанавливает их в синглтон.
///
/// # Ошибки
/// Паникует, если не удалось загрузить конфигурацию или она уже была инициализирована.
pub fn init() {
    let config = AppConfig::load()
        .map_err(|e| format!("Ошибка инициализации конфигурации:\n{}", e)).unwrap();

    CONFIG.set(config).unwrap();
}   // init()

/// Предоставляет доступ к глобальной конфигурации.
///
/// # Паника
/// Если конфигурация не инициализирована (ошибка этапа разработки).
///
/// # Возвращаемое значение
/// &AppConfig: Ссылка на конфигурацию.
pub fn get() -> &'static AppConfig {
    CONFIG.get().expect("Критическая ошибка: попытка доступа к конфигурации до инициализации.")
}   // get()