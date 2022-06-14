use daemonize::*;

fn main() {
    let d = Daemon::new()
        .stdout(std::fs::File::create("/tmp/test.log").unwrap())
        .start();

    match d {
        Ok(_) => (),
        Err(e) => eprintln!("error creating daemon : {e}"),
    }

    println!("Hello, world!");
}
