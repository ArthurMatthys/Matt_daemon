use std::{fs, io::Write, process};

use chrono::Local;

use crate::{
    error::{get_err, Error, Result},
    file_handler::unlock,
    LogInfo,
};

/// Can't find it in libc, this value has been taken from nyx::sys::signal, but it's the same as in
/// signal.h
const NSIG: libc::c_int = 32;

pub fn handle_sig(value: i32) {
    fs::create_dir_all("/var/log/matt_daemon")
        .map_err(Error::CreateDir)
        .expect("Cannot create dir to log signal input");
    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("/var/log/matt_daemon/matt_daemon.log")
        .map_err(Error::LogOpen)
        .expect("Cannot open the log file to log signal input");

    let now = Local::now().format("%d / %m / %Y - %H : %M : %S");
    let info = LogInfo::Warn;
    let msg = format!("Received signal {value}. Exiting the daemon\n");
    f.write(format!("[{now:}] - {info:5} : {msg}").as_bytes())
        .map_err(Error::Log)
        .expect("Could not log the signal input");

    unlock("/var/lock/matt_daemon.lock".to_string())
        .expect("The lock file should be set to `/var/lock/matt_daemon.lock`");

    process::exit(0);
}

pub fn set_sig_handlers() -> Result<()> {
    unsafe {
        for i in 1..NSIG {
            if i == libc::SIGKILL || i == libc::SIGSTOP {
                continue;
            }
            get_err(libc::signal(i, handle_sig as _), Error::SignalSetting)?;
        }
    }
    Ok(())
}
