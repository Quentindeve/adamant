//! Implementation of a generic Logger

use core::{mem::MaybeUninit, fmt::{Write, Arguments}};

/// Type definition for a logging function
type LogFn = fn(msg: &str);

pub enum LogLevel {
    Info,
    Warn,
    Error,
    Panic,
}

/// Logger structure. It contains 4 `LogFn` for each log level.
pub struct Logger {
    log: LogFn,
}

impl Logger {
    pub fn new_all(log: LogFn) -> Self {
        Self {
            log
        }
    }

    pub fn log(&self, level: LogLevel, msg: &str) {
        (self.log)(msg);
    }
}

impl Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.log(LogLevel::Info, s);
        Ok(())
    }
}

pub static mut GLOBAL_LOGGER: MaybeUninit<Logger> = MaybeUninit::uninit();

pub fn set_global_logger(logger: Logger) {
    unsafe {
        GLOBAL_LOGGER = MaybeUninit::new(logger);
    }
}

pub fn _print(fmt: Arguments) {
    unsafe {
        let _ = GLOBAL_LOGGER.assume_init_mut().write_fmt(fmt);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::log::_print(format_args!($(
            $arg)*));
    }
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}