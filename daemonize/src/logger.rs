use std::{fmt::Display, fs, io::Write, path::PathBuf};

use crate::error::{Error, Result};
use chrono::offset::Local;

// const LOGFILE: PathBuf = PathBuf::from("/var/log/matt_daemon/matt_daemon.log");

pub enum LogInfo {
    Debug,
    Error,
    Info,
    Warn,
}

impl Display for LogInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogInfo::Debug => write!(f, "\x1B[34mDEBUG\x1B[0m"),
            LogInfo::Error => write!(f, "\x1B[31mERROR\x1B[0m"),
            LogInfo::Info => write!(f, "\x1B[33mINFO\x1B[0m"),
            LogInfo::Warn => write!(f, "\x1B[35mWarn\x1B[0m"),
        }
    }
}

impl LogInfo {
    fn is_debug(&self) -> bool {
        matches!(self, LogInfo::Debug)
    }
}

#[derive(Clone)]
pub struct TintinReporter {
    pub logfile: PathBuf,
}

impl Default for TintinReporter {
    fn default() -> Self {
        Self {
            logfile: PathBuf::from("/var/log/matt_daemon/matt_daemon.log"),
        }
    }
}

impl TintinReporter {
    pub fn logfile(mut self, logfile: PathBuf) -> Self {
        self.logfile = logfile;
        self
    }
    pub fn log<S>(&self, msg: S, info: LogInfo, debug: bool) -> Result<()>
    where
        S: Display,
    {
        eprintln!("top");
        if debug && info.is_debug() {
            return Ok(());
        }
        fs::create_dir_all("/var/log/matt_daemon").map_err(Error::CreateDir)?;
        let mut f = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.logfile)
            .map_err(Error::LogOpen)?;

        let now = Local::now().format("%d / %m / %Y - %H : %M : %S");
        f.write(format!("[{now:}] - {info:5} : {msg}").as_bytes())
            .map_err(Error::Log)?;
        Ok(())
    }
}
