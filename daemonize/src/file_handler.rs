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

pub(crate) fn redirect_stream(stdin: Stdio, stdout: Stdio, stderr: Stdio) -> Result<()> {
    unsafe {
        let null_fd = get_err(
            libc::open(b"/dev/null\0" as *const [u8; 10] as _, libc::O_RDWR),
            Error::Open,
        )?;
        eprintln!("null fd: {null_fd}");
        let redirect = |fd, std: Stdio| -> Result<()> {
            match std.inner {
                StdioImpl::Redirect(file) => {
                    let raw_fd = file.as_raw_fd();
                    get_err(libc::dup2(raw_fd, fd), Error::RedirectStream)?;
                }
                StdioImpl::DevNull => {
                    get_err(libc::dup2(null_fd, fd), Error::RedirectStream)?;
                }
            };
            Ok(())
        };
        redirect(libc::STDIN_FILENO, stdin)?;
        redirect(libc::STDOUT_FILENO, stdout)?;
        redirect(libc::STDERR_FILENO, stderr)?;
        get_err(libc::close(null_fd), Error::CloseFd)?;
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

pub(crate) unsafe fn close_fds() -> Result<()> {
    // eprintln!("gogogogo");
    // std::thread::sleep(std::time::Duration::from_secs(10));
    // let path = PathBuf::from("/proc/self/fd/");
    // eprintln!("gogogogo");
    // std::thread::sleep(std::time::Duration::from_secs(10));
    // let dir = fs::read_dir(path);
    // eprintln!("gogogogo");
    // std::thread::sleep(std::time::Duration::from_secs(10));
    let path = PathBuf::from("/proc/self/fd/");
    let dir = fs::read_dir(path);
    let fds: Vec<i32> = match dir {
        Ok(entries) => {
            entries
                .filter_map(|entry| {
                    entry
                        .ok()
                        .map(|entry| {
                            // eprintln!("entry : {entry:#?}");
                            entry
                                .file_name()
                                .into_string()
                                .map(|filename| filename.parse::<i32>().ok())
                                .ok()
                        })
                        .flatten()
                        .flatten()
                })
                .filter(|fd| ![0, 1, 2].contains(fd))
                .collect()
        }
        Err(e) => {
            eprintln!("error reading fd dir : {e}");
            (3..get_rlimit()?).collect()
        }
    };
    eprintln!("fd to close : {fds:#?}");

    for fd in fds.iter() {
        if *fd == 4 {
            continue;
        }
        eprintln!("closing fd : {fd}");
        get_err(libc::close(*fd), Error::CloseFd)?;
    }
    Ok(())
}
