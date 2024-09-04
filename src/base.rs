use crate::data::hash_object;
use std::{fs::{self, read_dir}, path::PathBuf};

pub fn write_tree(directory: String) {
    for entry in read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let full = entry.path();
        if is_ignored(full.clone()) {
            continue;
        }
        if full.is_file() {
            let contents = fs::read_to_string(full.clone()).expect("Should have been able to read the file");
            println!("{}, {:?}", hash_object!(contents), full)
        } else if full.is_dir() {
            write_tree(full.into_os_string().into_string().unwrap());
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

fn is_ignored(path: PathBuf) -> bool {
    path.into_os_string().into_string().unwrap().contains(".pgit")
}
