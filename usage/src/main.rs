use daemonize::{get_err, Daemon, Error, LogInfo, Result, TintinReporter};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::os::unix::prelude::AsRawFd;

mod connections;
use connections::Connections;

const ADDRESS: &str = "127.0.0.1:4242";
const QUIT: [u8; 4] = *b"quit";
const SIZE_BUFF: usize = 40;

fn read_from_fd(
    fd: i32,
    idx: usize,
    streams: &mut [(TcpStream, SocketAddr)],
    logger: &TintinReporter,
) -> Result<bool> {
    let mut data = [0_u8; SIZE_BUFF];
    unsafe {
        let nb = get_err(
            libc::read(fd, data.as_mut_ptr() as _, SIZE_BUFF),
            Error::Read,
        )?;
        let addr = streams
            .get(idx - 1)
            .map(|(_, addr)| addr.to_string())
            .unwrap_or_else(|| "Can't find address".to_string());

        if nb == 4 && data[0..(nb as usize)] == QUIT {
            logger.log(
                format!("Connecting from new address : {addr}\n"),
                LogInfo::Info,
                false,
            )?;
            return Ok(true);
        }
        let msg = String::from_utf8(data.to_vec())
            .unwrap_or_else(|_| "Cant't parse message from utf8".to_string());
        logger.log(format!("Read from {addr} : {msg}\n"), LogInfo::Info, false)?;
    }
    Ok(false)
}

fn handle_revent_error(a: i16, addr: String, logger: &TintinReporter) -> Result<()> {
    if a & libc::POLLHUP != 0 {
        logger.log(format!("Hanging up from {addr}\n"), LogInfo::Warn, false)?;
    } else if a & libc::POLLERR != 0 {
        logger.log(
            format!("Error condition from {addr}\n"),
            LogInfo::Warn,
            false,
        )?;
    } else if a & libc::POLLNVAL != 0 {
        logger.log(
            format!("Invalid request: fd not open from {addr}\n"),
            LogInfo::Warn,
            false,
        )?;
    } else {
        logger.log(
            format!("Stream socket peer closed connection from {addr}\n"),
            LogInfo::Warn,
            false,
        )?;
    }
    Ok(())
}

fn add_client(
    fds: &mut Connections,
    listener: &TcpListener,
    streams: &mut Vec<(TcpStream, SocketAddr)>,
    logger: &TintinReporter,
) -> Result<()> {
    let (stream, addr) = listener.accept().unwrap();
    if fds.len() >= 3 {
        logger.log("Already 2 clients connected\n", LogInfo::Warn, false)?;
        return Ok(());
    }
    fds.push_from_fd(stream.as_raw_fd());
    logger.log(
        format!("Connecting to new address : {addr}\n"),
        LogInfo::Info,
        false,
    )?;
    streams.push((stream, addr));
    Ok(())
}

fn server(logger: TintinReporter) -> Result<()> {
    let listener = TcpListener::bind(ADDRESS).map_err(Error::ClientErrorBinding)?;

    let listener_fd = listener.as_raw_fd();
    let mut fds = Connections::new();
    fds.push_from_fd(listener_fd);

    let mut streams: Vec<(TcpStream, SocketAddr)> = vec![];
    loop {
        unsafe {
            let _ = libc::poll(fds.as_mut_ptr(), fds.len() as u64, -1);
            for (i, poll_fd) in fds.clone().iter().enumerate() {
                let fd = poll_fd.fd;
                match poll_fd.revents {
                    0 => (),
                    a => {
                        if a & libc::POLLIN != 0 && a & libc::POLLRDHUP == 0 {
                            if fd == listener_fd {
                                add_client(&mut fds, &listener, &mut streams, &logger)?;
                            } else if read_from_fd(fd, i, &mut streams, &logger)? {
                                return Ok(());
                            }
                        }
                        if a & libc::POLLRDHUP != 0
                            || a & libc::POLLHUP != 0
                            || a & libc::POLLERR != 0
                            || a & libc::POLLNVAL != 0
                        {
                            let addr = streams
                                .get(i - 1)
                                .map(|(_, addr)| addr.to_string())
                                .unwrap_or_else(|| "Can't find address".to_string());

                            let stream = streams.remove(i - 1);
                            drop(stream);
                            fds.remove(i);
                            handle_revent_error(a, addr, &logger)?;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let reporter = TintinReporter::default();
    let d = Daemon::new(reporter.clone(), server, false).start();
    match d {
        Ok(_) => (),
        Err(e) => {
            reporter
                .log(format!("Error : {e}\n"), LogInfo::Error, false)
                .expect("Failed to log error in daemon");
        }
    }
}
