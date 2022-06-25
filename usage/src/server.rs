use crate::connections::Connections;
use daemonize::{get_err, Error, LogInfo, Result, TintinReporter};
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::os::unix::prelude::AsRawFd;
use std::process::Command;

use strum::FromRepr;

const ADDRESS: &str = "127.0.0.1:4242";
const SIZE_BUFF: usize = 400;

const EXIT: &str = "exit";
const QUIT: &str = "quit";
const SHELL: &str = "shell";

#[derive(FromRepr, PartialEq, PartialOrd, Clone)]
enum ShellMode {
    None = 1,
    Shell = 2,
    Bash = 3,
}

pub struct Client {
    stream: TcpStream,
    addr: SocketAddr,
    shell_mode: ShellMode,
}

impl Client {
    fn new(stream: TcpStream, addr: SocketAddr) -> Self {
        Self {
            stream,
            addr,
            shell_mode: ShellMode::None,
        }
    }
    fn change_shell_mode(&mut self, nb: isize) -> Result<()> {
        let tmp = ShellMode::from_repr((self.shell_mode.clone() as isize + nb) as usize)
            .ok_or(Error::ShellModeOverflow)?;
        self.shell_mode = tmp;
        Ok(())
    }

    fn get_addr(&self) -> String {
        self.addr.to_string()
    }

    fn print_prompt(&mut self) -> Result<()> {
        match self.shell_mode {
            ShellMode::None => Ok(()),
            ShellMode::Shell => self
                .stream
                .write_all(b"matt_daemon>")
                .map_err(Error::WriteToStream),
            ShellMode::Bash => self
                .stream
                .write_all(b"bash>")
                .map_err(Error::WriteToStream),
        }
    }
}

fn handle_remote_shell(msg: String, client: &mut Client, logger: &TintinReporter) -> Result<()> {
    let addr = client.get_addr();
    if msg.is_empty() {
        logger.log(format!("{addr} empty command\n"), LogInfo::Warn, false)?;
        client.print_prompt()?;
        return Ok(());
    }
    if msg == EXIT || msg == QUIT {
        let shell_type = match client.shell_mode {
            ShellMode::Bash => "bash",
            ShellMode::Shell => "shell mode",
            _ => unreachable!(),
        };

        logger.log(
            format!("{addr} exiting {shell_type}\n"),
            LogInfo::Info,
            false,
        )?;
        client.change_shell_mode(-1)?;
        client.print_prompt()?;
        return Ok(());
    }
    if msg == SHELL && client.shell_mode < ShellMode::Bash {
        logger.log(format!("{addr} now in bash mode\n"), LogInfo::Info, false)?;
        client.change_shell_mode(1)?;
        client.print_prompt()?;
        return Ok(());
    }

    let (cmd, args) = match client.shell_mode {
        ShellMode::Bash => {
            let cmd = "/bin/sh";
            let args = vec!["-c", &*msg];
            (cmd, args)
        }
        ShellMode::Shell => {
            let mut arguments = msg.split_whitespace();
            let cmd = arguments.next().ok_or(Error::NoArgumentProvided)?;
            let args = arguments.collect();
            (cmd, args)
        }
        _ => unreachable!(),
    };

    let env: HashMap<String, String> = env::vars().collect();
    let mut to_exec = Command::new(cmd);
    let to_exec = to_exec.current_dir("/").envs(env).args(args);

    let pg = to_exec.get_program().to_str().ok_or(Error::ConvertToUTF8)?;
    let mut args = format!("{pg} ");
    to_exec
        .get_args()
        .filter_map(|arg| arg.to_str())
        .for_each(|arg| args.push_str(&*format!("{arg} ")));
    let args = args.trim();

    logger.log(
        format!("{addr} is running `{args}`\n"),
        LogInfo::Info,
        false,
    )?;

    match to_exec.output() {
        Ok(res) => {
            client
                .stream
                .write_all(&res.stdout)
                .map_err(Error::WriteToStream)?;
        }
        Err(e) => {
            let err = Error::CommandFailed(e);
            return logger.log(format!("{err}\n"), LogInfo::Warn, false);
        }
    }

    client.print_prompt()?;
    Ok(())
}

fn handle_client(msg: String, client: &mut Client, logger: &TintinReporter) -> Result<bool> {
    let addr = client.get_addr();
    if msg == EXIT || msg == QUIT {
        logger.log(
            format!("{addr} asked to exit the daemon\n"),
            LogInfo::Info,
            false,
        )?;
        return Ok(true);
    }
    if msg == SHELL {
        logger.log(
            format!("{addr} is now in remote shell mode\n"),
            LogInfo::Info,
            false,
        )?;
        client.change_shell_mode(1)?;
        client.print_prompt()?;
        return Ok(false);
    }
    logger.log(format!("Read from {addr} : {msg}\n"), LogInfo::Info, false)?;
    Ok(false)
}

fn read_from_fd(
    fd: i32,
    idx: usize,
    clients: &mut [Client],
    logger: &TintinReporter,
) -> Result<bool> {
    let mut data = [0_u8; SIZE_BUFF];
    let client = clients.get_mut(idx - 1).ok_or(Error::ClientGetter)?;
    unsafe {
        let nb = get_err(
            libc::read(fd, data.as_mut_ptr() as _, SIZE_BUFF),
            Error::Read,
        )?;

        let msg =
            String::from_utf8(data[0..(nb as usize)].to_vec()).map_err(|_| Error::ConvertToUTF8)?;
        let msg = msg.trim().to_string();
        if client.shell_mode > ShellMode::None {
            let res = handle_remote_shell(msg, client, logger);
            match res {
                Err(Error::CommandFailed(_)) => {
                    eprintln!("Wrong Command");
                    Ok(false)
                }
                Ok(_) => Ok(false),
                Err(e) => Err(e),
            }
        } else {
            handle_client(msg, client, logger)
        }
    }
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
    streams: &mut Vec<Client>,
    logger: &TintinReporter,
) -> Result<()> {
    let (stream, addr) = listener.accept().map_err(Error::AcceptClient)?;
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
    streams.push(Client::new(stream, addr));
    Ok(())
}

pub fn server(logger: TintinReporter) -> Result<()> {
    let listener = TcpListener::bind(ADDRESS).map_err(Error::ClientErrorBinding)?;

    let listener_fd = listener.as_raw_fd();
    let mut fds = Connections::new();
    fds.push_from_fd(listener_fd);

    let mut clients: Vec<Client> = vec![];
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
                                add_client(&mut fds, &listener, &mut clients, &logger)?;
                            } else if read_from_fd(fd, i, &mut clients, &logger)? {
                                return Ok(());
                            }
                        }
                        if a & libc::POLLRDHUP != 0
                            || a & libc::POLLHUP != 0
                            || a & libc::POLLERR != 0
                        {
                            eprintln!("{a}");
                            let addr = clients
                                .get(i - 1)
                                .map(|client| client.get_addr())
                                .unwrap_or_else(|| "Can't find address".to_string());

                            let stream = clients.remove(i - 1);
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
