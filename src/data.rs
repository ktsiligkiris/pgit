use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::{self, Write};

pub const GIT_DIR: &str = ".pgit";

pub fn init() -> std::io::Result<()> {
    fs::create_dir(GIT_DIR)?;
    fs::create_dir(format!("{GIT_DIR}/objects"))?;
    Ok(())
}

pub fn hash_object(data: String, type_: String) -> String {
    let mut hasher = Sha1::new();
    let obj = format!("{type_}\x00{data}");
    hasher.update(obj.clone());
    let oid = format!("{:X}", hasher.finalize());
    let mut file = File::create(format!("{GIT_DIR}/objects/{oid}")).unwrap();
    file.write_all(obj.as_bytes()).unwrap();
    oid
}

macro_rules! hash_object {
    ($data: expr, $type_: expr) => {
        hash_object($data, $type_)
    };
    ($data: expr) => {
        hash_object($data, String::from("blob"))
    };
}

pub fn get_object(oid: &String, expected: String) -> io::Result<Vec<u8>> {
    let filepath = format!("{GIT_DIR}/objects/{oid}");
    let obj = fs::read(filepath);
    let binding = obj.unwrap();
    let mut parts = binding.split(|x| *x == 0);
    let type_ = parts.next().unwrap();
    let content = parts.next().unwrap();
    if expected.as_bytes() == type_ {
        Ok(content.to_vec())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "unexpected object type: expected {} and found {:?}",
                expected, type_,
            ),
        ))
    }
}

macro_rules! get_object {
    ($oid: expr, $expected: expr) => {
        get_object($oid, $expected)
    };
    ($oid: expr) => {
        get_object($oid, String::from("blob"))
    };
}
