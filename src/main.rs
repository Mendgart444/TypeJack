#![allow(unused_imports)]
#![allow(unused_variables)]
mod parser;
mod transliper;

use std::{
    env, fs::File, io::Write, path::PathBuf, sync::Arc
};
use anyhow::{Context, Result};
use clap::{Arg, ArgMatches, Command, Parser, Subcommand};
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, Globals, SourceMap, DUMMY_SP
};

fn main() {
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
}