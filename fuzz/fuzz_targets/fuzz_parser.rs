#![no_main]
use libfuzzer_sys::fuzz_target;
use zenith_compiler::{lexer::Lexer, parser::Parser, source_map::{FileId, SourceFile}};
use std::sync::Arc;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let sf = Arc::new(SourceFile::new("<fuzz>".to_string(), s.to_string()));
        let lex = Lexer::new(FileId::new(1), sf);
        let mut parser = Parser::new(lex);
        let _ = parser.parse_program();
    }
});
