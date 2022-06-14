use daemonize::*;

fn main() {
    let reporter = TintinReporter::default();
    let d = Daemon::new(&reporter).start();

    match d {
        Ok(_) => (),
        Err(e) => {
            reporter.log(format!("error creating daemon : {e}"), LogInfo::Error);
        }
    }

    println!("Hello, world!");
}
