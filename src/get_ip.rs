use crate::read_file;
use chrono::prelude::{DateTime, Local};

// This function will print the Ip Address with time
pub fn log_ip() -> Result<(), Box<dyn std::error::Error>> {
    let local: DateTime<Local> = Local::now();
    let res = reqwest::blocking::get("https://ifconfig.me")?;
    println!(
        "{} - {}",
        res.text().expect("Cannot decode the body"),
        local
    );
    Ok(())
}

pub fn get_last_ip(iplogger_file: &std::path::PathBuf) -> String {
    let last_line =
        read_file::read_last_line(iplogger_file.to_str().expect("Cannot Convert to string"))
            .unwrap();
    let mut last_ip = String::from("");
    for l in last_line.split(" - ") {
        last_ip = String::from(l);
        break;
    }
    last_ip
}
