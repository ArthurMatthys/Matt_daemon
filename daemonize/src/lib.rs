mod error;
mod file_handler;
mod fork;
mod logger;
mod signal;

use std::path::PathBuf;

use error::{get_err, Error, Result};
use fork::{execute_fork, ForkResult};
use libc::exit;

// use libc::pid_t;
use std::fs::File;

use file_handler::{redirect_stream, Stdio};
use logger::LogInfo;

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
fn get_info(name: &str) -> Result<String> {
    unsafe {
        let pid = get_err(libc::getpid(), Error::GetPid)?;
        let sid = get_err(libc::getsid(pid), Error::GetPid)?;
        let pgid = get_err(libc::getpgid(pid), Error::GetPid)?;
        Ok(format!(
            "{name:10} || pid : {pid} || sid : {sid} || pgid : {pgid}"
        ))
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
        logger::log(get_info("parent")?, LogInfo::Info)?;
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
            // eprintln!("closing fd :");
            // close_fds()?;
            get_err(libc::setsid(), Error::SetSid)?;

            match execute_fork()? {
                ForkResult::Child => (),
                ForkResult::Parent(_) => exit(0),
            }

            logger::log(get_info("daemon")?, LogInfo::Info)?;
            libc::umask(self.umask.inner);

            eprintln!("Yop");

            get_err(libc::chdir(b"/\0" as *const u8 as _), Error::ChangeDir)?;

            redirect_stream(self.stdin, self.stdout, self.stderr)?;
            eprintln!("Bonjour");
        }

        // eprintln!("gogogogo");
        // std::thread::sleep(std::time::Duration::from_secs(10));

        Ok(())
    }
}

impl Default for Daemon {
    fn default() -> Self {
        Self::new()
    }
}
