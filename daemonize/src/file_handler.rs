use std::{
    fs::{self, File},
    os::unix::prelude::AsRawFd,
    path::PathBuf,
};

use crate::error::{get_err, Error, Result};

enum StdioImpl {
    DevNull,
    // Keep,
    Redirect(File),
}

pub struct Stdio {
    inner: StdioImpl,
}

impl From<File> for Stdio {
    fn from(f: File) -> Self {
        Self {
            inner: StdioImpl::Redirect(f),
        }
    }
}

impl Stdio {
    pub(crate) fn dev_null() -> Stdio {
        Self {
            inner: StdioImpl::DevNull,
        }
    }
}

pub(crate) fn redirect_stream() -> Result<()> {
    unsafe {
        get_err(libc::close(libc::STDIN_FILENO), Error::CloseFd)?;
        let null_fd = get_err(
            libc::open(b"/dev/null\0" as *const [u8; 10] as _, libc::O_RDWR),
            Error::Open,
        )?;
        if null_fd != 0 {
            return Err(Error::InvalidFd {
                fd: null_fd,
                expected: libc::STDIN_FILENO,
            });
        }
        let out_fd = get_err(
            libc::dup2(libc::STDIN_FILENO, libc::STDOUT_FILENO),
            Error::RedirectStream,
        )?;
        if out_fd != libc::STDOUT_FILENO {
            return Err(Error::InvalidFd {
                fd: out_fd,
                expected: libc::STDOUT_FILENO,
            });
        }
        let err_fd = get_err(
            libc::dup2(libc::STDIN_FILENO, libc::STDERR_FILENO),
            Error::RedirectStream,
        )?;
        if err_fd != libc::STDERR_FILENO {
            return Err(Error::InvalidFd {
                fd: err_fd,
                expected: libc::STDERR_FILENO,
            });
        }
    }
    Ok(())
}
fn get_rlimit() -> Result<i32> {
    let mut rlim = libc::rlimit {
        rlim_cur: 0,
        rlim_max: u32::MAX.into(),
    };
    unsafe {
        get_err(
            libc::getrlimit(libc::RLIMIT_NOFILE, &mut rlim),
            Error::Rlmit,
        )
    }
}

fn get_max_fd() -> Result<i32> {
    unsafe {
        let ret = get_err(libc::sysconf(libc::_SC_OPEN_MAX), Error::Sysconf);
        if let Ok(max_fd) = ret {
            return max_fd.try_into().map_err(|_| Error::MaxFdTooBig);
        }
    }

    get_rlimit()
}

pub(crate) unsafe fn close_fds() -> Result<()> {
    let fds = 3..get_max_fd()?;
    get_rlimit()?;
    eprintln!("fd to close : {fds:#?}");

    fds.for_each(|fd| {
        libc::close(fd);
    });
    Ok(())
}

pub(crate) fn lock(file: String) -> Result<()> {
    unsafe {
        let fd = libc::open((file + "\0").as_ptr() as _, libc::O_RDONLY | libc::O_CREAT);

        eprintln!("fd : {fd}");

        get_err(libc::flock(fd, libc::LOCK_EX), Error::AlreadyLock).map_err(|e| match e {
            Error::AlreadyLock(libc::EWOULDBLOCK) => {
                eprintln!("Je devrais arriver ici");
                e
            }
            Error::AlreadyLock(v) => Error::IssueLockFile(v),
            _ => unreachable!(),
        })?;
    }
    Ok(())
}
pub(crate) fn unlock(file: String) -> Result<()> {
    unsafe {
        let fd = libc::open((file.clone() + "\0").as_ptr() as _, libc::O_RDONLY);

        get_err(libc::flock(fd, libc::LOCK_UN), Error::Unlock)?;

        get_err(libc::remove((file + "\0").as_ptr() as _), Error::DeleteLock)?;
    }
    Ok(())
}
