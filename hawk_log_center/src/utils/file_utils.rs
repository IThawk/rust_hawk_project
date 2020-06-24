use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

/// ```read file to string
/// read file from FilePath
/// ```
pub fn read_file(file_path: String) -> Result<String, String> {
    let file_opt = OpenOptions::new().read(true).open(file_path.as_str());
    info!("{:?}", file_path);
    println!("{:?}", file_opt);
    if let Ok(file) = file_opt {
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents);
        println!("{}", contents);
        return Ok(contents);
    }

    error!("read {} error", file_path);
    Err(format!("read {} error", file_path))
}

/// ```write file
/// write file with string  from FilePath
/// ```
pub fn write_file(file_path: String, context: String, append: bool) -> Result<(), String> {
    let path = PathBuf::from(file_path.as_str());
    info!("{:?}", path);
    println!("{:?}", path);
    if path.exists() {}
    let file_opt = OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .open(file_path.as_str());
    println!("read {:?} error", file_opt);
    if let Ok(mut file) = file_opt {
        if let Ok(()) = file.write_all(context.as_bytes()) {
            if let Ok(()) = file.sync_all() {
                return Ok(());
            };
        };
    }
    error!("read {} error", file_path);
    Err(format!("read {} error", file_path))
}

#[test]
fn read() {
    let a = read_file("./config/test.yml".to_string());
    if let Ok(context) = a {
        println!("{}", context)
    }
}
