use std::fs::File;
use std::io;
use std::io::Read;

pub fn read_file_lines(filename: &str) -> Result<Vec<String>, io::Error> {
    match read_file(filename) {
        Ok(s) => Ok(s.split("\n").map(|l| l.to_string()).collect()),
        Err(e) => Err(e),
    }
}

pub fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("Error on loading file {} {}", filename, error)
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
