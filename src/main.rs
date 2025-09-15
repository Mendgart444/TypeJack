// modules
mod parser;
mod runtime;
mod transpiler;
// Imports ---------------------------------------
use clap::{ArgMatches, Command};
use nu_ansi_term::Color::{Blue, Green, Red};
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
            // read source file
            let source_code: String =
                fs::read_to_string(config.project.entry).expect("[error] no ts file found!");
            // traspile it
            let ast: Module = parser::parse_ts(&source_code)?;
            let transpiled_js_code: String = transpiler::transpile_ts_to_js(ast)?;
            let js_code: String = format!("\"use strict\";\n{}", transpiled_js_code);
            let js_path: String = format!("{}/out.js", config.project.out_dir);
            /*----------------------------------- Write .js file ----------------------------------------------------------------*/
            if fs::exists(format!("{}/out.js", config.project.out_dir)).unwrap() {
                fs::remove_file(&js_path).unwrap();
                fs::File::create_new(&js_path).unwrap();
                fs::write(js_path, js_code).unwrap()
            } else {
                fs::create_dir(&config.project.out_dir).unwrap();
                // create file and write it!
                fs::File::create_new(&js_path).unwrap();
                fs::write(js_path, js_code).unwrap();
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
            stdout().flush().unwrap();
            stdin().read_line(&mut name_of_project).unwrap();
            let folder_struct: String = format!("{}/src", name_of_project.trim());

            // check if folder already exists
            if fs::exists(name_of_project.trim()).unwrap() {
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
                        exit(1);
                    }
                }
                // creates the file name/src/main.ts
                match fs::File::create_new(format!("{}/main.ts", folder_struct)) {
                    Ok(_) => println!("[{}] Created source file", Blue.paint("info")),
                    Err(e) => {
                        eprintln!("[{}] {e}", Red.paint("error"));
                        exit(1);
                    }
                }
                // creates the file name/typejack.toml
                match fs::File::create_new(format!("{}/typejack.toml", name_of_project.trim())) {
                    Ok(_) => println!("[{}] Created configuration file", Blue.paint("info")),
                    Err(e) => {
                        eprintln!("[{}] {e}", Red.paint("error"));
                        exit(1);
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
