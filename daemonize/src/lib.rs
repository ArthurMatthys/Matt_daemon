mod error;
mod file_handler;
mod fork;
mod logger;
mod signal;

use file_handler::{lock, redirect_stream, unlock};
use fork::{execute_fork, ForkResult};
use libc::exit;

pub use error::{get_err, get_errno, Error, Result};
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
fn get_pid() -> Result<String> {
    unsafe {
        let pid = get_err(libc::getpid(), Error::GetPid)?;
        Ok(format!("Starting with pid {pid}\n"))
    }
}

pub struct Daemon {
    debug: bool,
    lock_file: String,
    logger: TintinReporter,
    umask: Mask,
    func: fn(TintinReporter) -> Result<()>,
}

impl Daemon {
    pub fn new(logger: TintinReporter, f: fn(TintinReporter) -> Result<()>, debug: bool) -> Daemon {
        Daemon {
            debug,
            lock_file: "/var/lock/matt_daemon.lock".to_string(),
            logger,
            umask: 0.into(),
            func: f,
        }
    }

    pub fn umask(mut self, mask: u32) -> Self {
        self.umask = mask.into();
        self
    }

    pub fn start(self) -> Result<()> {
        unsafe {
            self.logger
                .log("Entering daemon mode\n", LogInfo::Info, self.debug)?;

            self.logger.log(get_pid()?, LogInfo::Info, self.debug)?;
            match execute_fork()? {
                ForkResult::Child => (),
                ForkResult::Parent(_) => exit(libc::EXIT_SUCCESS),
            }

            get_err(libc::setsid(), Error::SetSid)?;

            match execute_fork()? {
                ForkResult::Child => (),
                ForkResult::Parent(_) => exit(libc::EXIT_SUCCESS),
            }

            self.logger
                .log("Creating lock file\n", LogInfo::Debug, self.debug)?;
            lock(self.lock_file.clone())?;

            self.logger
                .log("Changing file mode creation\n", LogInfo::Debug, self.debug)?;
            libc::umask(self.umask.inner);

            self.logger
                .log("Changing working directory\n", LogInfo::Debug, self.debug)?;
            get_err(libc::chdir(b"/\0" as *const u8 as _), Error::ChangeDir)?;

            self.logger
                .log("Closing all open files\n", LogInfo::Debug, self.debug)?;
            close_fds()?;

            self.logger
                .log("Daemon started properly\n", LogInfo::Info, self.debug)?;

            self.logger.log(
                "Redirecting standard streams to /dev/null\n",
                LogInfo::Debug,
                self.debug,
            )?;
            redirect_stream()?;
            (self.func)(self.logger.clone())?;

            self.logger
                .log("deleting lock file\n", LogInfo::Debug, self.debug)?;
            unlock(self.lock_file)?;
        }
        Ok(())
    }
}

impl Default for Daemon {
    fn default() -> Self {
        Self::new(
            TintinReporter::default(),
            |_| {
                std::thread::sleep(std::time::Duration::from_secs(10));
                Ok(())
            },
            true,
        )
    }
}
