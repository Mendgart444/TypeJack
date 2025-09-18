#![cfg(test)]

use crate::transpiler_core::transpile;
use std::{fs, path::{Path, PathBuf}};

#[test]
fn test_transpiler() {
    // Path Dummy
    let test_file_name = r#"src\main.ts"#;
    let path_buf = fs::canonicalize(test_file_name).unwrap();
    let path_buf_string = path_buf.to_str().unwrap();
    
    let path_buf: PathBuf = if path_buf_string.starts_with(r#"\\?\"#) {
        PathBuf::from(&path_buf_string[4..])
    } else {
        path_buf
    };

    let entry_path: &Path = path_buf.as_path();
    let result = transpile(entry_path);

    match result {
        Ok(_) => println!("Sucess"),
        Err(_) => panic!(),
    }
}