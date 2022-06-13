mod error;

use std::path::PathBuf;
use std::{fs, os::unix::prelude::AsRawFd};

use error::{get_err, Error, Result};
use libc::exit;

// use libc::pid_t;
use std::fs::File;

/// Can't find it in libc, this value has been taken from nyx::sys::signal, but it's the same as in
/// signal.h
const NSIG: libc::c_int = 32;
pub fn check() {
    println!("yop");
}

pub enum ForkResult {
    Child,
    Parent(libc::pid_t),
}

enum StdioImpl {
    DevNull,
    // Keep,
    Redirect(File),
}

struct Stdio {
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
    fn dev_null() -> Stdio {
        Self {
            inner: StdioImpl::DevNull,
        }
    }
}

// struct StdioStruct{
//     inner
// }

struct Mask {
    inner: libc::mode_t,
}

impl From<u32> for Mask {
    fn from(mask: u32) -> Self {
        Mask {
            inner: mask as libc::mode_t,
        }
    }
}

pub struct Daemon {
    logfile: PathBuf,
    pid: Option<PathBuf>,
    stdin: Stdio,
    stdout: Stdio,
    stderr: Stdio,
    umask: Mask,
    workdir: PathBuf,
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

unsafe fn close_fds() -> Result<()> {
    eprintln!("gogogogo");
    std::thread::sleep(std::time::Duration::from_secs(10));
    let path = PathBuf::from("/proc/self/fd/");
    eprintln!("gogogogo");
    std::thread::sleep(std::time::Duration::from_secs(10));
    let dir = fs::read_dir(path);
    eprintln!("gogogogo");
    std::thread::sleep(std::time::Duration::from_secs(10));
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

unsafe fn reset_sig_handlers() -> Result<()> {
    for i in 0..NSIG {
        eprintln!("signal num : {i}");
        get_err(libc::signal(i, libc::SIG_DFL), Error::SignalSetting)?;
    }
    Ok(())
}

unsafe fn reset_sig_mask() -> Result<()> {
    let null = std::ptr::null_mut();
    let set = std::ptr::null_mut();
    get_err(libc::sigemptyset(set), Error::SetSig)?;
    get_err(
        libc::sigprocmask(libc::SIG_SETMASK, set, null),
        Error::SigProcSet,
    )?;
    Ok(())
}

unsafe fn execute_fork() -> Result<ForkResult> {
    let pid = get_err(libc::fork(), Error::Fork)?;
    if pid == 0 {
        Ok(ForkResult::Child)
    } else {
        Ok(ForkResult::Parent(pid))
    }
}
fn redirect_stream(stdin: Stdio, stdout: Stdio, stderr: Stdio) -> Result<()> {
    unsafe {
        let null_fd = get_err(
            libc::open(b"/dev/null\0" as *const [u8; 10] as _, libc::O_RDWR),
            Error::Open,
        )?;
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

impl Daemon {
    pub fn new() -> Daemon {
        Daemon {
            logfile: PathBuf::from("/tmp/daemon.log"),
            pid: None,
            stdin: Stdio::dev_null(),
            stdout: Stdio::dev_null(),
            stderr: Stdio::dev_null(),
            umask: 0.into(),
            workdir: PathBuf::from("/"),
        }
    }
    pub fn stdin(mut self, file: File) -> Self {
        self.stdin = file.into();
        self
    }
    pub fn stdout(mut self, file: File) -> Self {
        self.stdout = file.into();
        self
    }
    pub fn stderr(mut self, file: File) -> Self {
        self.stderr = file.into();
        self
    }
    pub fn umask(&mut self, mask: u32) {
        self.umask = mask.into()
    }
    pub fn workdir(&mut self, dir: PathBuf) {
        self.workdir = dir
    }
    pub fn start(self) -> Result<()> {
        unsafe {
            // reset_sig_handlers()?;
            // reset_sig_mask()?;

            match execute_fork()? {
                ForkResult::Child => Ok(self.daemonize()?),
                ForkResult::Parent(_) => exit(0),
            }
        }
    }

    pub fn daemonize(self) -> Result<()> {
        unsafe {
            get_err(libc::setsid(), Error::SetSid)?;

            match execute_fork()? {
                ForkResult::Child => (),
                ForkResult::Parent(_) => exit(0),
            }

            libc::umask(self.umask.inner);

            eprintln!("Yop");

            get_err(libc::chdir(b"/\0" as *const u8 as _), Error::ChangeDir)?;
            eprintln!("closing fd :");
            close_fds()?;

            redirect_stream(self.stdin, self.stdout, self.stderr)?;
            eprintln!("Bonjour");
        }

        Ok(())
    }
}

impl Default for Daemon {
    fn default() -> Self {
        Self::new()
    }
}
