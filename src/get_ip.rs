use crate::read_file;
use chrono::prelude::{DateTime, Local};
use std::io::{Error, ErrorKind};
use std::time::Duration;

// This function will print the Ip Address with time
pub fn log_ip() -> Result<(), Box<dyn std::error::Error>> {
    let local: DateTime<Local> = Local::now();
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    let res = match client.get("https://ifconfig.me/ip").send() {
        Ok(res) => res,
        Err(_) => return Ok(()),
    };
    let currrent_ip = res.text().expect("Cannot parse the body");
    if currrent_ip != get_last_ip()? {
        println!("{} - {}", currrent_ip, local);
    }
    Ok(())
}

// This function return the last Ip Address logged in the file
pub fn get_last_ip() -> Result<String, Error> {
    let iplogger_file = dirs::home_dir().unwrap().join(".iplogger/log.txt");
    let last_line = match read_file::read_last_line(
        iplogger_file.to_str().expect("Cannot Convert to string"),
    ) {
        Ok(line) => line,
        Err(_) => String::from("127.0.0.1 - date"),
    };
    let ip = last_line.split(" - ").next();
    match ip {
        Some(val) => Ok(String::from(val)),
        None => Err(Error::new(ErrorKind::Other, "Cannot split the string")),
    }
}
