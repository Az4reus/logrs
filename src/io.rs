use std::io;
use std::io::Error;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::fs::OpenOptions;

pub fn print_file(filename: PathBuf) -> Result<(), io::Error> {
    match read_file(filename) {
        Ok(content) => {
            println!("\n {}", content);
            Ok(())
        }

        Err(e) => Err(e),
    }
}

pub fn read_file(filename: PathBuf) -> Result<String, Error> {
    let file = OpenOptions::new().read(true).open(&filename);

    match file {
        Ok(mut f) => {
            let mut content = String::new();
            f.read_to_string(&mut content)?;
            Ok(content)
        }

        Err(e) => Err(e),
    }
}

// pub fn archive(filename: String) -> Result<(), io::Error> {
//     let file = OpenOptions::new().read(true).open(&filename);

//     match file {
//         Ok(mut f) => {
//             // Read the old file
//             let mut content = String::new();
//             f.read_to_string(&mut content)?;

//             unimplemented!()
//         }
//         Err(e) => Err(e),
//     }
// }

pub fn append_to_file(filename: PathBuf, content: String) -> Result<usize, io::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&filename)?;

    file.write(content.as_bytes())
}
