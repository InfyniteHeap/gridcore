use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use chrono::Local;
use colored::Colorize;
use log::{Level, LevelFilter, Log, Metadata, Record};

const LOGGING_DIR: &str = "./logs";
const LOGGING_FILE_NAME: &str = "grid_log.txt";

pub struct GridLogger;
static LOGGER: GridLogger = GridLogger;

impl GridLogger {
    pub fn init() {
        log::set_logger(&LOGGER)
            .map(|_| {
                if cfg!(debug_assertions) {
                    log::set_max_level(LevelFilter::Trace)
                } else {
                    log::set_max_level(LevelFilter::Info);
                }
            })
            .unwrap()
    }

    fn log_to_console(record: &Record) {
        let formatted_message = format!(
            "[{}] [{}] [{}:{}]\n{}",
            Local::now().to_rfc3339(),
            match record.level() {
                Level::Error => "ERROR".red(),
                Level::Warn => "WARN".yellow(),
                Level::Info => "INFO".green(),
                Level::Debug => "DEBUG".blue(),
                Level::Trace => "TRACE".blue(),
            },
            record.file().unwrap_or_default(),
            record.line().unwrap_or_default(),
            record.args()
        );

        println!("{formatted_message}")
    }

    fn log_to_file(record: &Record) {
        let formatted_message = format!(
            "[{}] [{}] [{}:{}]\n{}",
            Local::now().to_rfc3339(),
            record.level(),
            record.file().unwrap_or_default(),
            record.line().unwrap_or_default(),
            record.args()
        );

        let logging_dir = Path::new(LOGGING_DIR);
        if !logging_dir.exists() {
            fs::create_dir_all(logging_dir).unwrap();
        }

        let mut logging_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(logging_dir.join(LOGGING_FILE_NAME))
            .unwrap();

        writeln!(logging_file, "{formatted_message}").unwrap();
    }
}

impl Log for GridLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if cfg!(debug_assertions) {
            metadata.level() <= Level::Trace
        } else {
            metadata.level() <= Level::Info
        }
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if cfg!(debug_assertions) {
                GridLogger::log_to_console(record);
            }
            GridLogger::log_to_file(record);
        }
    }

    fn flush(&self) {}
}
