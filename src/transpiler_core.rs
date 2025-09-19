use std::{path::Path, process::exit};

use nu_ansi_term::Color::Red;
use swc_common::{
    GLOBALS, Globals, Mark, SourceMap,
    comments::SingleThreadedComments,
    errors::{ColorConfig, Handler},
    sync::Lrc,
};
use swc_ecma_codegen::to_code_default;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax, error::Error, lexer::Lexer};
use swc_ecma_transforms_base::{fixer::fixer, hygiene::hygiene, resolver};
use swc_ecma_transforms_typescript::strip;

pub fn transpile(input: &Path) -> anyhow::Result<String> {
    let cm: Lrc<SourceMap> = Default::default();
    let handler: Handler =
        Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // load Typescript File -----------------------------------------------------------------------------------------------------------
    let fm: Lrc<swc_common::SourceFile> = cm.load_file(input).unwrap_or_else(|e| {
        eprintln!(
            "[{}] Failed to load typescript file: {}",
            Red.paint("error"),
            e
        );
        exit(5);
    });

    let comments: SingleThreadedComments = SingleThreadedComments::default();

    let lexer: Lexer<'_> = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: input.extension().is_some_and(|ext| ext == "tsx"),
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        Some(&comments),
    );

    let mut parser: Parser<Lexer<'_>> = Parser::new_from(lexer);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let module: swc_ecma_ast::Program = parser
        .parse_program()
        .map_err(|e: Error| e.into_diagnostic(&handler).emit())
        .unwrap_or_else(|_| {
            eprintln!("[{}] Failed to Parse Module", Red.paint("error"));
            exit(6);
        });

    let globals: Globals = Globals::default();
    let code: String = GLOBALS.set(&globals, || {
        let unresolved_mark: Mark = Mark::new();
        let top_level_mark: Mark = Mark::new();

        // Optionally transforms decorators here before the resolver pass
        // as it might produce runtime declarations.

        // Conduct identifier scope analysis
        let module: swc_ecma_ast::Program =
            module.apply(resolver(unresolved_mark, top_level_mark, true));

        // Remove typescript types
        let module: swc_ecma_ast::Program = module.apply(strip(unresolved_mark, top_level_mark));

        // Fix up any identifiers with the same name, but different contexts
        let module: swc_ecma_ast::Program = module.apply(hygiene());

        // Ensure that we have enough parenthesis.
        let program: swc_ecma_ast::Program = module.apply(fixer(Some(&comments)));

        to_code_default(cm, Some(&comments), &program)
    });

    Ok(code)
}
