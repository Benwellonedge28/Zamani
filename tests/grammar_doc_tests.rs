//! Regression test ensuring the example program documented in GRAMMAR.md
//! (section 7, "Complete Example") continues to parse without errors.
//! If this test breaks, either the parser changed in a way that
//! invalidates the documented grammar, or GRAMMAR.md needs updating to
//! match a new, intentional syntax change. Keep them in sync.

use std::sync::Arc;
use zenith_compiler::lexer::Lexer;
use zenith_compiler::parser::Parser;
use zenith_compiler::source_map::{FileId, SourceFile};

fn parse_src(src: &str) -> (usize, Vec<String>) {
    let sf = Arc::new(SourceFile::new("<grammar-doc-example>".into(), src.into()));
    let lex = Lexer::new(FileId::new(1), sf);
    let mut p = Parser::new(lex);
    let prog = p.parse_program();
    let errs = p
        .get_errors()
        .iter()
        .map(|e| e.message.clone())
        .collect::<Vec<_>>();
    (prog.statements.len(), errs)
}

#[test]
fn grammar_md_complete_example_parses_without_errors() {
    let src = r#"
language Zenith "1.0";

import stdlib.math;
use quantum::gates::*;

type PatientId = String;

struct Patient {
    id: PatientId,
    age: Int,
}

trait Greet {
    fn hello(name: String) -> String;
}

class Robot extends Machine {
    public name: String,

    new(name: String) {
        this.name = name;
    }

    fn speak() -> String {
        return "beep";
    }
}

effect QuantumDecoherence;

quantum circuit Bell {
    let q = quantum hadamard(q0);
}

nano agent Healer {
    let dose = 10;
}

remember wisdom_of_elders = 42;

fn plan(patient: Patient) -> Int {
    let past = zamani { recall(wisdom_of_elders) };
    let now = sasa { patient.age };
    let insight = learn from now;

    handle QuantumDecoherence {
        let r = perform QuantumDecoherence("noise");
    } with {
        println("recovered");
    }

    match patient.age {
        0 => 0,
        _ => now,
    }
}
"#;
    let (stmt_count, errs) = parse_src(src);
    assert!(
        errs.is_empty(),
        "GRAMMAR.md example failed to parse: {:?}",
        errs
    );
    assert_eq!(stmt_count, 12, "expected 12 top-level statements");
}

#[test]
fn builtin_call_like_keywords_are_lexed_correctly() {
    // Regression test for a lexer bug where keywords not present in an
    // internal allowlist (e.g. println, assert, panic) would have the
    // character immediately following them silently swallowed, breaking
    // `println("x")` into two unrelated statements.
    let cases = [
        r#"println("recovered");"#,
        r#"print("x");"#,
        r#"assert(true);"#,
        r#"panic("boom");"#,
    ];
    for src in cases {
        let (stmt_count, errs) = parse_src(src);
        assert!(errs.is_empty(), "failed to parse {:?}: {:?}", src, errs);
        assert_eq!(
            stmt_count, 1,
            "expected exactly one statement for {:?}",
            src
        );
    }
}
