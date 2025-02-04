use std::fs;

use crate::errors::CrateResult;

pub fn ls() -> CrateResult<()> {
    let entries = fs::read_dir(".")?;

    for entry in entries {
        let entry = entry?;

        println!("{}", entry.file_name().to_string_lossy());
    }

    Ok(())
}

pub fn pwd() -> CrateResult<String> {
    let current_dir = std::env::current_dir()?;

    Ok(current_dir.display().to_string())
}

pub fn cd(path: &str) -> CrateResult<()> {
    std::env::set_current_dir(path)?;

    Ok(())
}

pub fn touch(path: &str) -> CrateResult<()> {
    fs::File::create(path)?;

    Ok(())
}

pub fn rm(path: &str) -> CrateResult<()> {
    fs::remove_file(path)?;

    Ok(())
}

pub fn cat(path: &str) -> CrateResult<String> {
    let pwd = pwd()?;

    let joined_path = std::path::Path::new(&pwd).join(path);
    let contents = fs::read_to_string(joined_path)?;

    Ok(contents)
}
