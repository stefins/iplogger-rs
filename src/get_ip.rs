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
