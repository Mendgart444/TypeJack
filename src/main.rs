#![allow(unused_imports)]
#![allow(unused_variables)]
use std::{
    env, fs::File, io::Write, path::PathBuf, sync::Arc
};
use anyhow::{Context, Result};
use clap::{Arg, ArgMatches, Command, Parser, Subcommand};
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, Globals, SourceMap, DUMMY_SP
};
use swc_ecma_ast::EsVersion;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::{lexer::Lexer, StringInput, Syntax};
use swc_ecma_transforms_typescript::strip;
use swc_ecma_visit::FoldWith;

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