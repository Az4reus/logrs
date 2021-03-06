use std::io;
use std::io::{Error, ErrorKind};
use std::path::Path;
use config::Config;
use io::append_to_file;
use io::create_path;

extern crate chrono;
use self::chrono::*;

pub fn log(entry: &String, config: &Config) -> Result<(), io::Error> {
    if entry.is_empty() {
        // TODO - print program usage.
        return Err(Error::new(ErrorKind::Other, "Please provide me with data to log."));
    }

    let now = Local::now();
    let formatted_log = format_log_entry(&entry, now);
    let filename = get_file_name(now.date());
    let filepath = Path::new(&config.base_filepath).join(filename);

    match append_to_file(&filepath, formatted_log) {
        Ok(_) => {
            println!("Noted.");
            Ok(())
        }
        Err(x) => match x { 
            ref e if e.raw_os_error() == Some(2) => {
                match create_path(&config.base_filepath) {
                    Ok(()) =>  log(entry, config), 
                    Err(x) => Err(x)
                }
            }
            _ => Err(x)
        }
    }
}

fn format_log_entry(entry: &String, time: DateTime<Local>) -> String {
    let timestamp = time.format("%H:%M").to_string();

    let mut res = String::new();
    res.push_str(" ");
    res.push_str(&timestamp);
    res.push_str(" - ");
    res.push_str(&entry);
    res.push_str("\n");

    res
}

pub fn get_file_name(time_of_entry: Date<Local>) -> String {
    time_of_entry.format("%Y-%m-%d").to_string()
}
