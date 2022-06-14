use std::{fmt::Display, fs, io::Write};

use crate::error::{Error, Result};
use chrono::offset::Local;

pub(crate) enum LogInfo {
    Debug,
    Error,
    Info,
}

impl Display for LogInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogInfo::Debug => write!(f, "\x1B[34mDEBUG\x1B[0m"),
            LogInfo::Error => write!(f, "\x1B[31mERROR\x1B[0m"),
            LogInfo::Info => write!(f, "\x1B[33mINFO\x1B[0m"),
            // LogInfo::Warn => write!(f, "\x1B[35mWarn\x1B[0m"),
        }
    }
}
pub(crate) fn log<S>(msg: S, info: LogInfo) -> Result<()>
where
    S: Display,
{
    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(std::path::Path::new("/tmp/matt_daemon.log"))
        .map_err(Error::LogOpen)?;

    let now = Local::now().format("%d / %m / %Y - %H : %M : %S");
    f.write(format!("[{now:}] - {info:5} : {msg}\n").as_bytes())
        .map_err(Error::Log)?;
    Ok(())
}
