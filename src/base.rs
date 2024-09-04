use crate::data::hash_object;
use std::{
    fs::{self, read_dir},
    path::PathBuf,
};

pub fn write_tree(directory: String)-> String {
    let mut entries: Vec<(PathBuf, String, &str)> = Vec::new();
    for entry in read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let full = entry.path();
        if is_ignored(full.clone()) {
            continue;
        }
        if full.is_file() {
            let type_ = "blob";
            let contents =
                fs::read_to_string(&full).expect("Should have been able to read the file");
            let oid = hash_object!(contents);
            entries.push((full, String::from(oid), type_));
        } else if full.is_dir() {
            let type_ = "tree";
            let oid = write_tree(full.clone().into_os_string().into_string().unwrap());
            entries.push((full, String::from(oid), type_));
        }
    }
    entries
        .into_iter()
        .map(|entry| format!("{} {} {:?}", entry.2, entry.1, entry.0))
        .collect::<Vec<String>>()
        .join("\n")
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
    path.into_os_string()
        .into_string()
        .unwrap()
        .contains(".pgit")
}
