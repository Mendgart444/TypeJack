#![cfg(test)]

use crate::transpiler_core::transpile;
use std::path::Path;

#[test]
fn test_transpiler() {
    // Path Dummy
    let test_file_name = r#"test\src\main.ts"#;
    let path = Path::new(test_file_name);
    let result = transpile(path);

    match result {
        Ok(_) => println!("Sucess"),
        Err(_) => panic!(),
    }
}
