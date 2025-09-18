mod transpiler_core;
mod test;
// Imports ---------------------------------------
use clap::{ArgMatches, Command};
use nu_ansi_term::Color::{Blue, Green, Red};
use serde::Deserialize;
use std::{
    collections::HashSet,
    env, fs,
    io::{stdin, stdout, Write},
    path::{Path, PathBuf},
    process::exit,
};

// For toml file
#[derive(Debug, Deserialize)]
struct Config {
    project: Project,
}

// for toml file
#[derive(Debug, Deserialize)]
struct Project {
    name: String,
    entry: String,
    out_dir: String,
}

/// # Toml Syntax
/// ```toml
/// [project]
/// name = "demo"
/// entry = "src/main.ts"
/// out_dir = "out"
/// ```

fn load_config() -> anyhow::Result<Config> {
    let config_str: String = fs::read_to_string("typejack.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

fn transpile_with_deps(entry_path: &Path, visited: &mut HashSet<String>) -> anyhow::Result<String> {
    let entry_path_string: String = entry_path.to_str().unwrap().to_string();

    if visited.contains(&entry_path_string) {
        return Ok(String::new());
    }
    visited.insert(entry_path_string.clone());

    if entry_path_string.is_empty() {
        eprintln!(
            "[{}] Error entry_path is empty",
            Red.paint("error")
        );
        exit(6);
    }

    if entry_path.ends_with(".js") {
        eprintln!(
            "[{}] wrong import syntax. {} does not work! please use .ts as a file extension not .js",
            Red.paint("error"),
            &entry_path.to_str().unwrap()
        );
        exit(5);
    }

    let js_code: String = transpiler_core::transpile(&entry_path)?;

    let source_code: String = fs::read_to_string(&entry_path)?;
    let mut output: String = String::new();

    for line in source_code.lines() {
        if let Some(_) = line.find("import") {
            if let Some(from_start) = line.find("from") {
                let import_path: &str = line[from_start + 4..].trim();
                let dir: &Path = Path::new(&entry_path).parent().unwrap();
                let dep_path: std::path::PathBuf = dir.join(import_path);
                let dep_path_str: &Path = dep_path.as_path();
                output += &transpile_with_deps(dep_path_str, visited)?;
            }
        }
    }
    output += &js_code;
    Ok(output)
}

fn main() -> anyhow::Result<()> {
    println!("{}", Blue.paint("TypeJack TS Framework"));
    // arg parsing
    let arg: ArgMatches = Command::new("typejack")
        // Config and infortmation-----------------------------
        .color(clap::ColorChoice::Always)
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand_required(true)
        // Subcommands-----------------------------------------
        // build project-----------------------------------------------
        .subcommand(Command::new("build").about("Build TypeScript files into JavaScript")) // Subcommand for building TS files
        // new project -------------------------------------------------
        .subcommand(Command::new("new").about("Creates new TypeJack project"))
        // End of subcommands----------------------------------
        .get_matches();

    // Handle arguments-----------------------------------
    match arg.subcommand() {
        Some(("build", _)) => {
            println!("[{}] Building project...", Blue.paint("info"),);

            // Load configuration
            let config: Config = load_config()?;
            println!(
                "[{}] Project Name: {}",
                Blue.paint("info"),
                config.project.name
            );
            let mut visited: HashSet<String> = HashSet::new();

            // create pathbuf
            let entry_abs_path: PathBuf = fs::canonicalize(&config.project.entry)?;
            let entry_abs_path_str: &str = entry_abs_path.to_str().unwrap();

            // clean up the path
            let entry_abs_path: PathBuf = if entry_abs_path_str.starts_with(r#"\\?\"#) {
                PathBuf::from(&entry_abs_path_str[4..])
            } else {
                entry_abs_path
            };

            let path: &Path = entry_abs_path.as_path();

            // traspile it
            let transpiled_js_code: String = transpile_with_deps(path, &mut visited)?;
            let js_code: String = format!("\"use strict\";\n{}", transpiled_js_code);
            let js_path: String = format!("{}/out.js", config.project.out_dir);
            
            /*----------------------------------- Write .js file ----------------------------------------------------------------*/
            if fs::exists(format!("{}/out.js", config.project.out_dir))? {
                fs::remove_file(&js_path)?;
                fs::File::create_new(&js_path)?;
                fs::write(js_path, js_code)?
            } else {
                fs::create_dir(&config.project.out_dir)?;
                // create file and write it!
                fs::File::create_new(&js_path)?;
                fs::write(js_path, js_code)?;
            }

            println!(
                "[{}] {} {}",
                Blue.paint("info"),
                Green.paint("Sucessfully builded project:"),
                config.project.name
            );
        }
        Some(("new", _)) => {
            let mut name_of_project: String = String::new();
            // NAME ---------------------------------------------------
            print!("Enter project name: ");
            stdout().flush()?;
            stdin().read_line(&mut name_of_project)?;
            let folder_struct: String = format!("{}/src", name_of_project.trim());

            // check if folder already exists
            if fs::exists(name_of_project.trim())? {
                eprintln!(
                    "[{}] File {} already exists.",
                    Red.paint("error"),
                    Blue.paint(name_of_project.trim())
                );

                exit(1)
            } else {
                // toml file
                let value_of_toml: String = format!(
                    "[project]\nname = \"{}\"\nentry = \"src/main.ts\"\nout_dir = \"out\"",
                    name_of_project.trim()
                );
                // file creation--------------------------------------------
                // creates the folders: name/src
                match fs::create_dir_all(&folder_struct) {
                    Ok(_) => println!("[{}] Created dir", Blue.paint("info")),
                    Err(e) => {
                        eprintln!("[{}] {e}", Red.paint("error"));
                        exit(2);
                    }
                }
                // creates the file name/src/main.ts
                match fs::File::create_new(format!("{}/main.ts", folder_struct)) {
                    Ok(_) => println!("[{}] Created source file", Blue.paint("info")),
                    Err(e) => {
                        eprintln!("[{}] {e}", Red.paint("error"));
                        exit(3);
                    }
                }
                // creates the file name/typejack.toml
                match fs::File::create_new(format!("{}/typejack.toml", name_of_project.trim())) {
                    Ok(_) => println!("[{}] Created configuration file", Blue.paint("info")),
                    Err(e) => {
                        eprintln!("[{}] {e}", Red.paint("error"));
                        exit(4);
                    }
                }
                // write name/typejack.toml
                fs::write(
                    format!("{}/typejack.toml", name_of_project.trim()),
                    value_of_toml,
                )
                .expect(format!("[{}] faild to write toml file", Red.paint("error")).as_str());
            }
        }
        _ => unreachable!(
            "There should not be an error. if there is an error please report on github issues!"
        ),
    }

    Ok(())
}
