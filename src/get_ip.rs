use crate::read_file;
use chrono::prelude::{DateTime, Local};

// This function will print the Ip Address with time
pub fn log_ip() -> Result<(), Box<dyn std::error::Error>> {
    let local: DateTime<Local> = Local::now();
    let res = reqwest::blocking::get("https://ifconfig.me")?;
    let currrent_ip = res.text().expect("Cannot parse the body");
    if currrent_ip != get_last_ip(){
        println!(
            "{} - {}",
            currrent_ip,
            local
        );
    }
    Ok(())
}

// This function return the last Ip Address logged in the file
pub fn get_last_ip() -> String {
    let iplogger_file = dirs::home_dir().unwrap().join(".iplogger/log.txt");
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
