use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read_last_line(path: &str) -> Result<String, Error> {
    let f = File::open(path)?;
    let file = BufReader::new(&f);
    let last_line = file.lines().last();
    match last_line {
        Some(line) => Ok(line.expect("Error")),
        None => Err(Error::new(ErrorKind::Other, "Last line cannot be fetched")),
    }
}
