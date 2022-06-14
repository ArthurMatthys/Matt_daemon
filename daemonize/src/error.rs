type Errno = libc::c_int;
pub type Result<T> = std::result::Result<T, Error>;
use std::{fmt::Display, io};

#[derive(Debug)]
pub enum Error {
    ChangeDir(Errno),
    CloseFd(Errno),
    Log(std::io::Error),
    LogOpen(std::io::Error),
    Fork(Errno),
    GetPid(Errno),
    GetPgid(Errno),
    GetSid(Errno),
    Open(Errno),
    RedirectStream(Errno),
    Rlmit(Errno),
    SetSid(Errno),
    SetSig(Errno),
    SigMask(Errno),
    SignalSetting(Errno),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ChangeDir(e) => write!(f, "Error changing directory : {e}")?,
            Error::CloseFd(e) => write!(f, "Error closing fd : {e}")?,
            Error::Log(e) => write!(f, "Error while logging : {e}")?,
            Error::LogOpen(e) => write!(f, "Error trying to open logfile : {e}")?,
            Error::Fork(e) => write!(f, "Error forking : {e}")?,
            Error::GetPid(e) => write!(f, "Can't retrieve pid : {e}")?,
            Error::GetPgid(e) => write!(f, "Can't retrieve pid : {e}")?,
            Error::GetSid(e) => write!(f, "Can't retrieve pid : {e}")?,
            Error::Open(e) => write!(f, "Error opening file : {e}")?,
            Error::RedirectStream(e) => write!(f, "Error redirecting stream : {e}")?,
            Error::Rlmit(e) => write!(f, "Error getting rlimit : {e}")?,
            Error::SetSid(e) => write!(f, "Error setting sid : {e}")?,
            Error::SetSig(e) => write!(f, "Error getting signal set : {e}")?,
            Error::SigMask(e) => write!(f, "Error setting signal mask : {e}")?,
            Error::SignalSetting(e) => write!(f, "Error setting signla handler : {e}")?,
        };
        Ok(())
    }
}

pub trait IsErr {
    fn is_err(&self) -> bool;
}
impl IsErr for i32 {
    fn is_err(&self) -> bool {
        *self == -1
    }
}
impl IsErr for usize {
    fn is_err(&self) -> bool {
        *self == usize::MAX
    }
}

pub fn get_err<V, F>(value: V, f: F) -> Result<V>
where
    V: IsErr,
    F: FnOnce(Errno) -> Error,
{
    if value.is_err() {
        Err(f(get_errno()))
    } else {
        Ok(value)
    }
}

fn get_errno() -> Errno {
    io::Error::last_os_error()
        .raw_os_error()
        .expect("Errno expected")
}
