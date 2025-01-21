use std::fs;

fn main() {
    read_dir("/a");
    read_dir("/b");
    read_dir("/c");
    read_dir("/d");
    read_dir("/e");
}

fn read_dir(path: &str) {
    print!("Reading {path}: ");

    let dir = fs::read_dir(path);

    if dir.is_ok() {
        println!("Success");
    } else {
        println!("Failed. Error: {:?}", dir.err());
    }
}
