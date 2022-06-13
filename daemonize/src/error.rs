type Errno = libc::c_int;
pub type Result<T> = std::result::Result<T, Error>;
use std::io;

#[derive(Debug)]
pub enum Error {
    ChangeDir(Errno),
    CloseFd(Errno),
    Fork(Errno),
    Open(Errno),
    RedirectStream(Errno),
    Rlmit(Errno),
    SetSid(Errno),
    SetSig(Errno),
    SignalSetting(Errno),
    SigProcSet(Errno),
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
