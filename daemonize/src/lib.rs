mod error;
mod file_handler;
mod fork;
mod logger;
mod signal;

use error::{get_err, Error, Result};
use fork::{execute_fork, ForkResult};
use libc::exit;

// use libc::pid_t;

use file_handler::{lock, redirect_stream, unlock};
pub use logger::{LogInfo, TintinReporter};

use crate::file_handler::close_fds;

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
            "{name} || pid : {pid} || sid : {sid} || pgid : {pgid}"
        ))
    }
}

pub struct Daemon<'a> {
    lock_file: String,
    logger: &'a TintinReporter,
    umask: Mask,
    func: fn() -> Result<()>,
}

impl<'a> Daemon<'a> {
    pub fn new(logger: &TintinReporter) -> Daemon {
        Daemon {
            logger,
            lock_file: "/var/lock/matt_daemon.lock".to_string(),
            umask: 0.into(),
            func: || {
                std::thread::sleep(std::time::Duration::from_secs(10));
                Ok(())
            },
        }
    }

    pub fn umask(&mut self, mask: u32) {
        self.umask = mask.into()
    }

    pub fn start(self) -> Result<()> {
        unsafe {
            self.logger.log(get_info("parent")?, LogInfo::Info)?;
            match execute_fork()? {
                ForkResult::Child => (),
                ForkResult::Parent(_) => exit(libc::EXIT_SUCCESS),
            }

            get_err(libc::setsid(), Error::SetSid)?;

            match execute_fork()? {
                ForkResult::Child => (),
                ForkResult::Parent(_) => exit(libc::EXIT_SUCCESS),
            }
            self.logger.log(get_info("daemon")?, LogInfo::Info)?;

            self.logger.log("Creating lock file", LogInfo::Info)?;
            lock(self.lock_file.clone())?;

            self.logger
                .log("Changing file mode creation", LogInfo::Info)?;
            libc::umask(self.umask.inner);

            self.logger
                .log("Changing working directory", LogInfo::Info)?;
            get_err(libc::chdir(b"/\0" as *const u8 as _), Error::ChangeDir)?;

            self.logger.log("Closing all open files", LogInfo::Info)?;
            close_fds()?;

            self.logger
                .log("Redirecting standard streams to /dev/null", LogInfo::Info)?;
            redirect_stream()?;
            (self.func)()?;

            self.logger.log("deleting lock file", LogInfo::Info)?;
            unlock(self.lock_file)?;
        }
        Ok(())
    }
}

// impl<'a> Default for Daemon<'a> {
//     fn default() -> Self {
//         Self::new(TintinReporter::default().to_owned())
//     }
// }
