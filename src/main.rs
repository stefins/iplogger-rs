extern crate daemonize;
extern crate dirs;
use daemonize::Daemonize;
use std::fs::File;
use std::fs::OpenOptions;
use std::thread;
use std::time::Duration;
mod get_ip;

fn main() {
    let home = dirs::home_dir().unwrap().join(".iplogger/log.txt");
    let stdout = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(home)
        .unwrap();
    let stderr = File::create("/tmp/iplogger.err").unwrap();
    println!("Starting Iplogger");
    let daemonize = Daemonize::new()
        .pid_file("/tmp/iplogger.pid")
        .chown_pid_file(true)
        .working_directory("/tmp")
        .umask(0o777)
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");
    match daemonize.start() {
        Ok(_) => loop {
            get_ip::log_ip().unwrap();
            thread::sleep(Duration::from_secs(60 * 20));
        },
        Err(e) => println!("Error, {}", e),
    }
}
