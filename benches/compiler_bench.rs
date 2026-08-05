//! Zamani Compiler — Benchmark Suite
//!
//! Run with: cargo bench

use std::hint::black_box;

fn lex_simple(source: &str) {
    use std::sync::Arc;
    use zamani_compiler::lexer::{Lexer, TokenType};
    use zamani_compiler::source_map::{FileId, SourceFile};
    let sf = Arc::new(SourceFile::new("<bench>".to_string(), source.to_string()));
    let mut lex = Lexer::new(FileId::new(1), sf);
    loop {
        let tok = lex.next_token();
        if tok.token_type == TokenType::EOF {
            break;
        }
    }
}

fn parse_simple(source: &str) {
    use std::sync::Arc;
    use zamani_compiler::{
        lexer::Lexer,
        parser::Parser,
        source_map::{FileId, SourceFile},
    };
    let sf = Arc::new(SourceFile::new("<bench>".to_string(), source.to_string()));
    let lex = Lexer::new(FileId::new(1), sf);
    let mut parser = Parser::new(lex);
    black_box(parser.parse_program());
}

fn compile_full(source: &str) {
    black_box(zamani_compiler::compile(source).ok());
}

fn main() {
    // Micro-benchmarks — replace with criterion when added as dev-dependency
    let src_small = "let x = 1 + 2; let y = x * 3;";
    let src_medium = src_small.repeat(50);
    let src_large = src_small.repeat(500);

    let runs = 1000;

    let start = std::time::Instant::now();
    for _ in 0..runs {
        lex_simple(black_box(src_small));
    }
    println!("lex_small: {:?}/iter", start.elapsed() / runs);

    let start = std::time::Instant::now();
    for _ in 0..runs {
        parse_simple(black_box(src_small));
    }
    println!("parse_small: {:?}/iter", start.elapsed() / runs);

    let start = std::time::Instant::now();
    for _ in 100u32..(100 + runs) {
        compile_full(black_box(src_medium.as_str()));
    }
    println!("compile_medium: {:?}/iter", start.elapsed() / runs);

    let start = std::time::Instant::now();
    for _ in 0..100u32 {
        compile_full(black_box(src_large.as_str()));
    }
    println!("compile_large (100 runs): {:?}/iter", start.elapsed() / 100);
}
