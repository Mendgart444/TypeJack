#![allow(dead_code)]
use anyhow::Result;
use std::{fs, path::Path};

pub fn write_output(out_dir: &str, filename: &str, content: &str) -> Result<()> {
    let path: &Path = Path::new(out_dir);
    fs::create_dir_all(path)?;
    let file_path: std::path::PathBuf = path.join(filename);
    fs::write(file_path, content)?;
    Ok(())
}
