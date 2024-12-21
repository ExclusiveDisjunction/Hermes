use std::{fmt::Debug, fs::File};
use std::io::Write;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

use super::errors::Error;
use crate::{io_error, argument_error, operation_error};

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum LoggerLevel {
    None = 0,
    Debug = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
    Critical = 5 
}
impl Debug for LoggerLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "NONE",
                Self::Debug => "DEBUG",
                Self::Info => "INFO",
                Self::Warning => "WARNING",
                Self::Error => "ERROR",
                Self::Critical => "CRITICAL"
            }
        )
    }
}

pub struct LoggerData {
    file: Option<std::fs::File>,
    state: LoggerLevel, //Currently being used
    level: LoggerLevel //The level of the logger (what we will accept)
}
impl Default for LoggerData {
    fn default() -> Self {
        Self::new()
    }
}
impl LoggerData {
    pub fn new() -> Self {
        Self {
            file: None,
            state: LoggerLevel::None,
            level: LoggerLevel::None
        }
    }

    pub fn open(&mut self, path: &str, level: LoggerLevel) -> Result<(), Error> {
        if self.is_open() {
            Err(operation_error!("open", "already open"))
        } else if level == LoggerLevel::None {
            Err(argument_error!("level", "cannot open under level '{:?}'", LoggerLevel::None))
        } else {
            self.file = match std::fs::File::create(path) {
                Ok(f) => Some(f),
                Err(e) => return Err(io_error!(e))
            };

            self.state = LoggerLevel::None;
            self.level = level;
            Ok(())
        }
    }
    pub fn close(&mut self) {
        self.file = None;
        self.state = LoggerLevel::None;
        self.level = LoggerLevel::None;
    }
    pub fn is_open(&self) -> bool {
        self.file.is_some()
    }
    pub fn open_level(&self) -> LoggerLevel {
        self.level
    }
    fn get_file(&mut self) -> &mut File {
        if self.file.is_none() {
            panic!("get file called when the file stored in the log is None");
        }

        self.file.as_mut().unwrap()
    }
    fn is_in_log(&self) -> bool {
        self.state != LoggerLevel::None && self.file.is_some()
    }
    fn curr_log_ignored(&self) -> bool {
       self.state < self.level || self.file.is_none()
    }

    pub fn write(&mut self, obj: &impl Debug) -> Result<(), Error> {
        if !self.is_open() {
            Err(operation_error!("write", "no file currently open"))
        }
        else if !self.is_in_log() {
            Err(operation_error!("write", "not in writing mode"))
        }
        else {
            if !self.curr_log_ignored() {
                let t_file: &mut File = self.file.as_mut().unwrap();
                if let Err(e) = t_file.write(format!("{:?}", obj).as_bytes()) {
                    return Err(io_error!(e));
                }
            }

            Ok(())
        }
    }
    pub fn start_log(&mut self, level: LoggerLevel) -> Result<(), Error> {
        if !self.is_open() {
            return Err(operation_error!("start log", "log not open"));
        }
        else if level == LoggerLevel::None {
            return Err(operation_error!("start log", "cannot start a log at level {:?}", LoggerLevel::None));
        }
        else if self.is_in_log() {
            return Err(operation_error!("start log", "log already started at level {:?}", &self.state));
        }

        self.state = level;
        if !self.curr_log_ignored() {
            if let Err(e) =  self.get_file().write(format!("{:?} {:?} ", chrono::Local::now(), level).as_bytes()) {
                return Err(io_error!(e));
            }
        }

        Ok(())
    }
    pub fn end_log(&mut self) -> Result<(), Error> {
        if !self.is_open() {
            return Err(operation_error!("end log", "log not open"));
        }
        else if !self.is_in_log() {
            return Err(operation_error!("end log", "not in log"));
        }

        if let Err(e) = self.get_file().write("\n".as_bytes()) {
            return Err(io_error!(e));
        }

        self.state = LoggerLevel::None;
        Ok(())
    }
}

pub struct Logger {
    data: Arc<Mutex<LoggerData>>
}
impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
impl Logger {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(LoggerData::new()))
        }
    }

    pub fn open(&self, path: &str, level: LoggerLevel) -> Result<(), Error> {
        let mut data = self.data.lock().unwrap();
        data.open(path, level)
    }
    pub fn close(&self) {
        let mut data = self.data.lock().unwrap();
        data.close()
    }
    pub fn is_open(&self) -> bool {
        let data = self.data.lock().unwrap();
        data.is_open()
    }
    pub fn open_level(&self) -> LoggerLevel {
        let data = self.data.lock().unwrap();
        data.open_level()
    }

    pub fn write(&self, obj: &impl Debug) -> Result<(), Error> {
        let mut data = self.data.lock().unwrap();
        data.write(obj)
    }
    pub fn start_log(&self, level: LoggerLevel) -> Result<(), Error> {
        let mut data = self.data.lock().unwrap();
        data.start_log(level)
    }
    pub fn end_log(&self) -> Result<(), Error> {
        let mut data = self.data.lock().unwrap();
        data.end_log()
    }

}

lazy_static! {
    pub static ref logging: Logger = Logger::new();
}

#[macro_export]
macro_rules! logger_write {
    ($level: expr, $($arg:tt)*) => {
        {
            if !logging.is_open() { //Do nothing, so that standard error is not flooded with 'not open' errors.
                return;
            }

            #[allow(unreachable_patterns)]
            let true_level: LoggerLevel = match $level {
                LoggerLevel::None => panic!("cannot record information about a None log"),
                LoggerLevel::Debug => LoggerLevel::Debug,
                LoggerLevel::Info => LoggerLevel::Info,
                LoggerLevel::Warning => LoggerLevel::Warning,
                LoggerLevel::Error => LoggerLevel::Error,
                LoggerLevel::Critical => LoggerLevel::Critical,
                _ => panic!("the type {:?} cannot be interpreted as a valid `LoggerLevel` instance", $level)
            };
            if true_level < logging.open_level() {
                return; //Nothing to do, optimization
            }

            let contents: String = format!($($arg)*);

            if let Err(e) = logging.start_log(true_level) {
                eprintln!("log error: '{:?}'. closing log", e);
                logging.close();
                return;
            }

            if let Err(e) = logging.write(&contents) {
                eprintln!("log error: '{:?}'. closing log", e);
                logging.close();
                return;
            }

            if let Err(e) = logging.end_log() {
                eprintln!("log error: '{:?}'. closing log", e);
                logging.close();
                return;
            }
        }
    };
}
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        {
            logger_write!(LoggerLevel::Debug, $($arg)*)
        }
    }
}
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        {
            logger_write!(LoggerLevel::Info, $($arg)*)
        }
    }
}
#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        {
            logger_write!(LoggerLevel::Warning, $($arg)*)
        }
    }
}
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        {
            logger_write!(LoggerLevel::Error, $($arg)*)
        }
    }
}
#[macro_export]
macro_rules! log_critical {
    ($($arg:tt)*) => {
        {
            logger_write!(LoggerLevel::Critical, $($arg)*)
        }
    }
}

#[test]
fn test_logger_write() {
    if let Err(e) = logging.open("tmp.log", LoggerLevel::Debug) {
        panic!("unable to open log because '{:?}'", e);
    }
    format!("hello");

    logger_write!(LoggerLevel::Debug, "hello");
    logger_write!(LoggerLevel::Info, "hello");
    logger_write!(LoggerLevel::Warning, "hello");
    logger_write!(LoggerLevel::Error, "hello");
    logger_write!(LoggerLevel::Critical, "hello");

    log_debug!("hello");
    log_info!("hello");
    log_warning!("hello");
    log_error!("hello");
    log_critical!("hello");
}