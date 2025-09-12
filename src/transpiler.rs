use anyhow::Result;
use swc_common::{SourceMap, sync::Lrc};
use swc_ecma_ast::Module;
use swc_ecma_codegen::{Emitter, text_writer::JsWriter};

pub fn transpile_ts_to_js(module: Module) -> Result<String> {
    // configurations we need
    let cm: Lrc<SourceMap> = Default::default();
    let mut buf = Vec::new();
    // write the code
    {
        let writer: Box<JsWriter<&mut Vec<u8>>> =
            Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None));
        let mut emitter: Emitter<'_, Box<JsWriter<'_, &mut Vec<u8>>>, SourceMap> = Emitter {
            cfg: Default::default(),
            comments: None,
            cm: cm.clone(),
            wr: writer,
        };
        emitter.emit_module(&module)?;
    }

    let js_code: String = String::from_utf8(buf)?;
    Ok(js_code)
}
