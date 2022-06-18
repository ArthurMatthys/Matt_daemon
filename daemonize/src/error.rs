type Errno = libc::c_int;
pub type Result<T> = std::result::Result<T, Error>;
use std::{fmt::Display, io};

#[derive(Debug)]
pub enum Error {
    AlreadyLock(Errno),
    ChangeDir(Errno),
    CloseFd(Errno),
    CreateDir(std::io::Error),
    ClientErrorBinding(std::io::Error),
    DeleteLock(Errno),
    InvalidFd { fd: i32, expected: i32 },
    IssueLockFile(Errno),
    Log(std::io::Error),
    LogOpen(std::io::Error),
    Fork(Errno),
    GetPid(Errno),
    GetPgid(Errno),
    GetSid(Errno),
    MaxFdTooBig,
    Open(Errno),
    Quit,
    Read(Errno),
    RedirectStream(Errno),
    Rlmit(Errno),
    SetSid(Errno),
    SetSig(Errno),
    SigMask(Errno),
    SignalSetting(Errno),
    Sysconf(Errno),
    Unlock(Errno),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AlreadyLock(e) => write!(f, "Lock file already created : {e}")?,
            Error::ChangeDir(e) => write!(f, "Error changing directory : {e}")?,
            Error::CloseFd(e) => write!(f, "Error closing fd : {e}")?,
            Error::ClientErrorBinding(e) => write!(f, "Cannot bind to address: {e}")?,
            Error::CreateDir(e) => write!(f, "Error creating dir : {e}")?,
            Error::DeleteLock(e) => write!(f, "Error deleting lock file: {e}")?,
            Error::InvalidFd { fd, expected } => {
                write!(f, "Opening fd {fd}, it should be {expected}")?
            }
            Error::IssueLockFile(e) => write!(f, "Issue with lock file : {e}")?,
            Error::Log(e) => write!(f, "Error while logging : {e}")?,
            Error::LogOpen(e) => write!(f, "Error trying to open logfile : {e}")?,
            Error::Fork(e) => write!(f, "Error forking : {e}")?,
            Error::GetPid(e) => write!(f, "Can't retrieve pid : {e}")?,
            Error::GetPgid(e) => write!(f, "Can't retrieve pid : {e}")?,
            Error::GetSid(e) => write!(f, "Can't retrieve pid : {e}")?,
            Error::MaxFdTooBig => write!(f, "Max fd retrieved with sysconf is too big")?,
            Error::Open(e) => write!(f, "Error opening file : {e}")?,
            Error::Read(e) => write!(f, "Error reading file : {e}")?,
            Error::RedirectStream(e) => write!(f, "Error redirecting stream : {e}")?,
            Error::Rlmit(e) => write!(f, "Error getting rlimit : {e}")?,
            Error::SetSid(e) => write!(f, "Error setting sid : {e}")?,
            Error::SetSig(e) => write!(f, "Error getting signal set : {e}")?,
            Error::SigMask(e) => write!(f, "Error setting signal mask : {e}")?,
            Error::SignalSetting(e) => write!(f, "Error setting signla handler : {e}")?,
            Error::Sysconf(e) => write!(f, "Error getting value of sysconf : {e}")?,
            Error::Unlock(e) => write!(f, "Error unlocking lock file : {e}")?,
            Error::Quit => write!(f, "Quitting the daemon")?,
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
impl IsErr for i64 {
    fn is_err(&self) -> bool {
        *self == -1
    }
}
impl IsErr for isize {
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

pub fn get_errno() -> Errno {
    io::Error::last_os_error()
        .raw_os_error()
        .expect("Errno expected")
}
