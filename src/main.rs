/// Welcome to the source code of TypeJack!
/// Typejack is a new Typescript Framework that is easy and 100% protable!
mod test;
mod transpiler_core;

use anyhow::Ok;
// Imports ---------------------------------------
use clap::{ArgMatches, Command};
use nu_ansi_term::Color::{Blue, Green, Red};
use serde::Deserialize;
use std::{
    env, fs,
    io::{Write, stdin, stdout},
    path::Path,
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
/// TypeJack uses a typejack.toml to read the project infos like cargo.
fn load_config() -> anyhow::Result<Config> {
    let config_str: String = fs::read_to_string("typejack.toml").unwrap_or_else(|e| {
        eprintln!("[{}] Failed to read typejack.toml: {e}", Red.paint("error"));
        "".to_string()
    });

    let config: Config = toml::from_str(&config_str).unwrap_or_else(|e| {
        eprintln!("[{}] faliled to Read Toml file: {e}", Red.paint("error"));
        exit(9);
    });
    Ok(config)
}

/// this function creates a new typejack project
fn new_project() -> anyhow::Result<()> {
    let mut name_of_project: String = String::new();

    // NAME ---------------------------------------------------
    print!("Enter project name: ");
    stdout().flush().unwrap_or_else(|e| {
        eprintln!("[{}] {e}", Red.paint("error"));
        exit(1);
    });
    stdin().read_line(&mut name_of_project).unwrap_or_else(|e| {
        eprintln!("[{}] {e}", Red.paint("error"));
        exit(2);
    });

    let folder_struct: String = format!("{}/src", name_of_project.trim());

    // check if folder already exists
    if fs::exists(name_of_project.trim())? {
        eprintln!(
            "[{}] File {} already exists.",
            Red.paint("error"),
            Blue.paint(name_of_project.trim())
        );
        exit(4);
    } else {
        // toml file
        let value_of_toml: String = format!(
            "[project]\nname = \"{}\"\nentry = \"src/main.ts\"\nout_dir = \"out\"",
            name_of_project.trim()
        );

        // creates the folders: name/src
        fs::create_dir_all(&folder_struct).unwrap_or_else(|e| {
            eprintln!("[{}] {e}", Red.paint("error"));
            exit(5);
        });
        println!("[{}] Created dir", Blue.paint("info"));

        // creates the file name/src/main.ts
        fs::File::create_new(format!("{}/main.ts", folder_struct)).unwrap_or_else(|e| {
            eprintln!("[{}] {e}", Red.paint("error"));
            exit(6);
        });
        println!("[{}] Created source file", Blue.paint("info"));

        // creates the file name/typejack.toml
        fs::File::create_new(format!("{}/typejack.toml", name_of_project.trim())).unwrap_or_else(
            |e| {
                eprintln!("[{}] {e}", Red.paint("error"));
                exit(7);
            },
        );
        println!("[{}] Created configuration file", Blue.paint("info"));

        // write name/typejack.toml
        fs::write(
            format!("{}/typejack.toml", name_of_project.trim()),
            value_of_toml,
        )
        .unwrap_or_else(|e| {
            eprintln!("[{}] failed to write toml file {e}", Red.paint("error"));
            exit(8);
        });
    }

    Ok(())
}

/// This function does transpile the project
fn transpile_project() -> anyhow::Result<()> {
    println!("[{}] Building project...", Blue.paint("info"),);

    // Load configuration
    let config: Config = load_config()?;

    println!(
        "[{}] Project Name: {}",
        Blue.paint("info"),
        config.project.name
    );
    let transpiled_js_code: String = transpiler_core::transpile(Path::new(&config.project.entry))
        .unwrap_or_else(|_| {
            eprintln!("[{}] failed to transpile code.", Red.paint("error"));
            exit(7);
        });

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

    Ok(())
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
        .subcommand(Command::new("b").about("Build TypeScript files into JavaScript"))
        // new project -------------------------------------------------
        .subcommand(Command::new("new").about("Creates new TypeJack project"))
        .subcommand(Command::new("n").about("Creates new TypeJack project"))
        // End of subcommands----------------------------------
        .get_matches();

    // Handle arguments-----------------------------------
    match arg.subcommand() {
        Some(("build", _)) => {
            transpile_project()?;
        }
        Some(("new", _)) => {
            new_project()?;
        }
        _ => unreachable!(
            "Oh no! you reached an error that should not be reachable. please report your error here: https://github.com/Mendgart444/TypeJack/issues"
        ),
    }

    Ok(())
}
