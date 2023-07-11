use std::{fs::File, io::Read};

pub fn read_file(path: &str) -> (String, String) {

    let mut file = File::open(path).expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<&str> = contents.trim().split("\n").collect();

    (lines[0].to_owned(), lines[1].to_owned())
}