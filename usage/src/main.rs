use clap::Parser;
use daemonize::{Daemon, LogInfo, Result, TintinReporter};
use smtp::get_smtp;

mod connections;
mod smtp;

mod usage;
use usage::MattDaemonArgs;

mod server;
use server::server;

fn main() -> Result<()> {
    let mut reporter = TintinReporter::default();
    let args = MattDaemonArgs::parse();

    match args.mail_send {
        Some(addr) => {
            if let Err(e) = get_smtp(&mut reporter, addr) {
                eprintln!("{e}");
                return Err(e);
            }
        }
        None => (),
    }

    match Daemon::new(reporter.clone(), server, false).start() {
        Ok(_) => Ok(()),
        Err(e) => {
            reporter
                .log(format!("Error : {e}\n"), LogInfo::Error, false)
                .expect("Failed to log error in daemon");
            Err(e)
        }
    }
}
