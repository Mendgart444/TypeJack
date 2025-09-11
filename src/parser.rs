use anyhow::Result;
use swc_common::{FileName, SourceMap, sync::Lrc};
use swc_ecma_ast::Module;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

pub fn parse_ts(source_code: &str) -> Result<Module> {
    let cm: Lrc<SourceMap> = Default::default();
    let fm: Lrc<swc_common::SourceFile> = cm.new_source_file(
        FileName::Custom("input.ts".into()).into(),
        source_code.to_string(),
    );
    // set syntax settings
    let ts_syntax: Syntax = Syntax::Typescript(TsSyntax {
        tsx: false,
        dts: false,
        no_early_errors: false,
        disallow_ambiguous_jsx_like: false,
        ..Default::default()
    });

    let mut parser: Parser<Lexer> = Parser::new(ts_syntax, StringInput::from(&*fm), None);

    let module: Module = parser
        .parse_module()
        .map_err(|e: swc_ecma_parser::error::Error| anyhow::anyhow!("[error] {:?}", e))?;

    Ok(module)
}
