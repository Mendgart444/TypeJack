mod parser;
mod runtime;
mod transpiler;
// Imports ---------------------------------------
use clap::{ArgMatches, Command};
use serde::Deserialize;
use std::{env, fs};
use swc_ecma_ast::Module;

#[derive(Debug, Deserialize)]
struct Config {
    project: Project,
}

#[derive(Debug, Deserialize)]
struct Project {
    name: String,
    entry: String,
    out_dir: String,
}

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
        .subcommand(Command::new("build").about("Build TypeScript files into JavaScript")) // Subcommand for building TS files
        // End of subcommands----------------------------------
        .get_matches();
    // Handle arguments-----------------------------------
    match arg.subcommand() {
        Some(("build", _sub_m)) => {
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
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    Ok(())
}
