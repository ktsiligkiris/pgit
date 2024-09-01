use crate::data;
use std::fs::read_dir;

pub fn write_tree(directory: String) {
    for entry in read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let full = entry.path();
        if full.is_dir() {
            write_tree(full.into_os_string().into_string().unwrap());
        } else {
            println!("{:?}", full);
        }
    }
}

macro_rules! write_tree {
    ($dir: expr) => {
        write_tree($dir)
    };
    () => {
        write_tree(String::from("."))
    };
}
