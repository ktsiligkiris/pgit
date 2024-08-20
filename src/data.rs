use std::fs;

pub const GIT_DIR: &str = ".pgit";

pub fn init() -> std::io::Result<()> {
    fs::create_dir(GIT_DIR)?;
    Ok(())
}
