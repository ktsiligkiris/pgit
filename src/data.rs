use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::{self, Write};

pub const GIT_DIR: &str = ".pgit";

pub fn init() -> std::io::Result<()> {
    fs::create_dir(GIT_DIR)?;
    fs::create_dir(format!("{GIT_DIR}/objects"))?;
    Ok(())
}

pub fn hash_object(data: String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data.clone());
    let oid = format!("{:X}", hasher.finalize());
    let mut file = File::create(format!("{GIT_DIR}/objects/{oid}")).unwrap();
    file.write_all(data.as_bytes()).unwrap();
    oid
}

pub fn get_object(oid: &String) -> io::Result<Vec<u8>> {
    let filepath = format!("{GIT_DIR}/objects/{oid}");
    fs::read(filepath)
}
