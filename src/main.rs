// modules
mod parser;
mod runtime;
mod transpiler;
// Imports ---------------------------------------
use clap::{ArgMatches, Command};
use serde::Deserialize;
use std::{
    env, fs,
    io::{Write, stdin, stdout},
    process::exit,
};
use swc_ecma_ast::Module;
// For toml file
#[derive(Debug, Deserialize)]
struct Config {
    project: Project,
}
// for toml file
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
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

fn main() -> anyhow::Result<()> {
    println!("TypeJack TS Framework");
    // arg parsing
    let arg: ArgMatches = Command::new("typejack")
        // Config and infortmation-----------------------------
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
            println!("Building project...");

            // Load configuration
            let config: Config = load_config()?;
            println!("Project Name: {}", config.project.name);

            // Parse the entry file
            // TS test variable
            let ts_code: &str = "let x: number = 42;";
            let ast: Module = parser::parse_ts(ts_code)?;
            let js_code: String = transpiler::transpile_ts_to_js(ast)?;

            println!("[info] Successfully Transpiled: {}", js_code);
        }
        Some(("new", _)) => {
            let mut name_of_project: String = String::new();
            let value_of_toml: String = format!(
                "[project]\nname = \"{}\"\nentry = \"src/main.ts\"\nout_dir = \"out\"",
                name_of_project
            );
            // NAME ---------------------------------------------------
            print!("Enter project name: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut name_of_project).unwrap();
            let folder_struct: String = format!("{}/src", name_of_project.trim());

            // file creation--------------------------------------------
            // creates the folders: name/src
            match fs::create_dir_all(&folder_struct) {
                Ok(_) => println!("[info] Created dir"),
                Err(e) => {
                    // eprintln!("dir create error {}", e); // DEBUG
                    eprintln!("[error] {}", e);
                    exit(1);
                }
            }
            // creates the file name/src/main.ts
            match fs::File::create_new(format!("{}/main.ts", folder_struct)) {
                Ok(_) => println!("[info] Created source file"),
                Err(e) => {
                    // eprintln!("dir create error {}", e); // DEBUG
                    eprintln!("[error] {}", e);
                    exit(1);
                }
            }
            // creates the file name/typejack.toml
            match fs::File::create_new(format!("{}/typejack.toml", name_of_project.trim())) {
                Ok(_) => println!("[info] Writen typejack.toml file"),
                Err(e) => {
                    //eprintln!("file create error {}", e); // DEBUG
                    eprintln!("[error] {}", e);
                    exit(1);
                }
            }
            // write name/typejack.toml
            fs::write(
                format!("{}/typejack.toml", name_of_project.trim()),
                value_of_toml,
            )
            .expect("[error] faild to write toml file");
        }
        _ => unreachable!(
            "There should not be an error. if there is an error please report on github issues!"
        ),
    }

    Ok(())
}
