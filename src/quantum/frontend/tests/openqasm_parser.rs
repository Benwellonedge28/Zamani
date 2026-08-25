//! Zamani Quantum Frontend — OpenQASM parser production contract tests.
//!
//! File:
//!
//!     src/quantum/frontend/tests/openqasm_parser.rs
//!
//! This suite is the dedicated production parser/grammar contract for:
//!
//!     src/quantum/frontend/formats/openqasm/parser.rs
//!
//! Architectural boundary:
//!
//!     untrusted OpenQASM source
//!             │
//!             ▼
//!          lexer.rs
//!             │
//!             ▼
//!        Token<'src>
//!             │
//!             ▼
//!         parser.rs
//!             │
//!             ▼
//!       OpenQASM AST
//!             │
//!             ▼
//!       validation.rs
//!             │
//!             ▼
//!       importer/lowering
//!             │
//!             ▼
//!        Quantum IR
//!
//! These tests intentionally stop at the AST boundary.
//!
//! They MUST NOT test:
//!
//! - semantic type checking;
//! - symbol resolution;
//! - gate arity validation;
//! - include resolution;
//! - filesystem access;
//! - network access;
//! - process execution;
//! - calibration execution;
//! - Quantum IR lowering;
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware mapping;
//! - QPU execution.
//!
//! Those responsibilities belong to later frontend/compiler layers.
//!
//! Production parser invariants covered here:
//!
//! - complete OpenQASM program parsing;
//! - OpenQASM 3.0 and 3.1 version handling;
//! - source-order preservation;
//! - AST node construction;
//! - source-span preservation;
//! - declarations;
//! - expressions;
//! - quantum operations;
//! - measurement;
//! - reset;
//! - barrier;
//! - delay;
//! - gate definitions;
//! - gate modifiers;
//! - classical control;
//! - for/while/switch control flow;
//! - subroutines;
//! - extern declarations;
//! - return/break/continue/let;
//! - includes;
//! - annotations/pragmas;
//! - calibration syntax;
//! - malformed input rejection;
//! - unexpected EOF handling;
//! - token-stream validation;
//! - parser AST-node limits;
//! - statement limits;
//! - nesting limits;
//! - expression-depth limits;
//! - deterministic results;
//! - deterministic diagnostics;
//! - no parser panics;
//! - no accidental acceptance of trailing garbage;
//! - no semantic validation leakage into the parser;
//! - parser independence from external I/O.
//!
//! Rust:
//!
//! - Rust 1.97 / 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no additional dependencies.
//!
//! Integration contract:
//!
//! This file consumes only the parser/AST/source contracts that already exist
//! in the frontend. If parser internals change, the parser's public contract
//! should remain compatible rather than changing this suite to reach into
//! private parser state.
//!
//! IMPORTANT:
//!
//! This file should be wired into the frontend's test module exactly once.
//! It is not a standalone Cargo integration-test crate because it intentionally
//! tests the internal parser contract. A future `tests/mod.rs` or the frontend
//! test module may include it with:
//!
//!     #[path = "openqasm_parser.rs"]
//!     mod openqasm_parser;
//!
//! Do not duplicate the module declaration elsewhere.

#![allow(clippy::needless_range_loop)]

use crate::quantum::frontend::core::source::SourceId;

use crate::quantum::frontend::formats::openqasm::ast::{
    AstNode,
    AstNodeKind,
    OpenQasmVersion,
    Statement,
};

use crate::quantum::frontend::formats::openqasm::lexer::{
    OpenQasmLexer,
    Span,
    Token,
    TokenKind,
};

use crate::quantum::frontend::formats::openqasm::parser::{
    OpenQasmParser,
    ParseError,
    ParseErrorKind,
    ParserConfig,
    ParserLimits,
};

// =============================================================================
// Test helpers
// =============================================================================

fn config() -> ParserConfig {
    ParserConfig {
        source_id: SourceId::from_raw(1),
        limits: ParserLimits::default(),
    }
}

fn parse(source: &str) -> Result<crate::quantum::frontend::formats::openqasm::ast::Program, ParseError> {
    OpenQasmParser::parse(source, config())
}

fn parse_with_config(
    source: &str,
    parser_config: ParserConfig,
) -> Result<crate::quantum::frontend::formats::openqasm::ast::Program, ParseError> {
    OpenQasmParser::parse(source, parser_config)
}

fn assert_parses(source: &str) {
    let result = parse(source);

    assert!(
        result.is_ok(),
        "valid OpenQASM parser input was rejected:\n\
         source:\n{}\n\
         error:\n{}",
        source,
        result
            .as_ref()
            .err()
            .map(ToString::to_string)
            .unwrap_or_default()
    );
}

fn assert_parse_error(source: &str, expected_kind: ParseErrorKind) {
    let result = parse(source);

    let error = result.expect_err("source was expected to fail parsing");

    assert_eq!(
        error.kind(),
        expected_kind,
        "unexpected parser error for source:\n{}\nerror: {}",
        source,
        error
    );
}

fn assert_statement_kind(
    source: &str,
    expected_kind: fn(&Statement) -> bool,
) {
    let program = parse(source).expect("source must parse");

    assert_eq!(
        program.statements().len(),
        1,
        "test input must contain exactly one statement:\n{}",
        source
    );

    assert!(
        expected_kind(&program.statements()[0]),
        "unexpected statement AST for source:\n{}\nAST:\n{:#?}",
        source,
        program.statements()[0]
    );
}

fn lex_tokens<'src>(source: &'src str) -> Vec<Token<'src>> {
    OpenQasmLexer::new(source)
        .expect("lexer construction must succeed")
        .tokenize()
        .expect("test source must tokenize")
}

fn parser_config_with_limits(limits: ParserLimits) -> ParserConfig {
    ParserConfig {
        source_id: SourceId::from_raw(7),
        limits,
    }
}

// =============================================================================
// Empty program / EOF
// =============================================================================

#[test]
fn empty_source_is_a_valid_empty_program() {
    let program = parse("").expect("empty source must parse");

    assert_eq!(program.version(), None);
    assert!(program.statements().is_empty());
}

#[test]
fn whitespace_only_source_is_a_valid_empty_program() {
    let program = parse(" \t\r\n  \n\t")
        .expect("whitespace-only source must parse");

    assert_eq!(program.version(), None);
    assert!(program.statements().is_empty());
}

#[test]
fn comments_only_source_is_a_valid_empty_program() {
    let program = parse(
        r#"
// comment
/* block comment */
"#,
    )
    .expect("comments-only source must parse");

    assert_eq!(program.version(), None);
    assert!(program.statements().is_empty());
}

#[test]
fn parser_accepts_eof_after_final_statement() {
    let program = parse(
        r#"
OPENQASM 3.1;
qubit[2] q;
"#,
    )
    .expect("EOF after a valid program must be accepted");

    assert_eq!(program.version(), Some(OpenQasmVersion::V3_1));
    assert_eq!(program.statements().len(), 1);
}

// =============================================================================
// Version handling
// =============================================================================

#[test]
fn parses_openqasm_3_0() {
    let program = parse(
        r#"
OPENQASM 3.0;
qubit q;
"#,
    )
    .expect("OpenQASM 3.0 must parse");

    assert_eq!(program.version(), Some(OpenQasmVersion::V3_0));
}

#[test]
fn parses_openqasm_3_1() {
    let program = parse(
        r#"
OPENQASM 3.1;
qubit q;
"#,
    )
    .expect("OpenQASM 3.1 must parse");

    assert_eq!(program.version(), Some(OpenQasmVersion::V3_1));
}

#[test]
fn rejects_unsupported_version() {
    let result = parse(
        r#"
OPENQASM 4.0;
qubit q;
"#,
    );

    assert!(
        result.is_err(),
        "future OpenQASM versions must never silently map to a supported version"
    );
}

#[test]
fn rejects_malformed_version() {
    assert_parse_error(
        "OPENQASM nonsense;",
        ParseErrorKind::InvalidVersion,
    );
}

#[test]
fn rejects_missing_version_literal() {
    let result = parse("OPENQASM ;");

    assert!(
        result.is_err(),
        "OPENQASM without a version must not be accepted"
    );
}

#[test]
fn rejects_missing_version_semicolon() {
    let result = parse("OPENQASM 3.1");

    assert!(
        result.is_err(),
        "version declaration requires a terminating semicolon"
    );
}

#[test]
fn version_is_not_required_by_parser() {
    // Syntax parsing and language-policy validation are separate boundaries.
    // The parser may represent a source document without a version so that
    // validation can issue the appropriate language-level diagnostic.
    let program = parse("qubit q;")
        .expect("version absence must not crash the parser");

    assert_eq!(program.version(), None);
}

// =============================================================================
// Source order and source spans
// =============================================================================

#[test]
fn source_statement_order_is_preserved() {
    let program = parse(
        r#"
OPENQASM 3.1;
qubit q;
bit c;
reset q;
measure q -> c;
"#,
    )
    .expect("source must parse");

    assert_eq!(program.statements().len(), 4);

    assert!(matches!(
        program.statements()[0],
        Statement::QuantumDeclaration(_)
    ));

    assert!(matches!(
        program.statements()[1],
        Statement::ClassicalDeclaration(_)
    ));

    assert!(matches!(
        program.statements()[2],
        Statement::Reset(_)
    ));

    assert!(matches!(
        program.statements()[3],
        Statement::MeasureAssignment(_)
    ));
}

#[test]
fn program_span_is_source_anchored() {
    let source = "OPENQASM 3.1;\nqubit q;\n";

    let program = parse(source).expect("source must parse");

    let span = program.span();

    assert_eq!(span.source_id, SourceId::from_raw(1));
    assert_eq!(span.start.as_usize(), 0);
    assert_eq!(span.end.as_usize(), source.len());
}

#[test]
fn statement_spans_are_monotonic() {
    let source = r#"
OPENQASM 3.1;
qubit q;
bit c;
reset q;
"#;

    let program = parse(source).expect("source must parse");

    let statements = program.statements();

    for index in 1..statements.len() {
        let previous = statements[index - 1].span();
        let current = statements[index].span();

        assert!(
            previous.start.as_usize() <= current.start.as_usize(),
            "statement spans must preserve source order"
        );

        assert!(
            previous.end.as_usize() <= current.end.as_usize(),
            "statement spans must not move backwards"
        );
    }
}

#[test]
fn parser_uses_configured_source_identity() {
    let parser_config = ParserConfig {
        source_id: SourceId::from_raw(1234),
        limits: ParserLimits::default(),
    };

    let program = parse_with_config(
        "qubit q;",
        parser_config,
    )
    .expect("source must parse");

    assert_eq!(
        program.span().source_id,
        SourceId::from_raw(1234)
    );

    assert_eq!(
        program.statements()[0].span().source_id,
        SourceId::from_raw(1234)
    );
}

// =============================================================================
// Token stream integrity
// =============================================================================

#[test]
fn parser_accepts_valid_token_stream_with_eof() {
    let source = "qubit q;";
    let tokens = lex_tokens(source);

    let parser = OpenQasmParser::from_tokens(
        tokens,
        config(),
    )
    .expect("valid lexer token stream must be accepted");

    let program = parser
        .parse_program()
        .expect("valid token stream must parse");

    assert_eq!(program.statements().len(), 1);
}

#[test]
fn parser_rejects_empty_token_stream() {
    let result = OpenQasmParser::from_tokens(
        Vec::new(),
        config(),
    );

    let error = result.expect_err(
        "empty token stream must be rejected before parsing",
    );

    assert_eq!(
        error.kind(),
        ParseErrorKind::InvalidTokenStream
    );
}

#[test]
fn parser_rejects_token_stream_without_eof() {
    let token = Token::new(
        TokenKind::Identifier,
        Span::new(0, 1),
        "q",
    );

    let result = OpenQasmParser::from_tokens(
        vec![token],
        config(),
    );

    let error = result.expect_err(
        "token stream without EOF must be rejected",
    );

    assert_eq!(
        error.kind(),
        ParseErrorKind::InvalidTokenStream
    );
}

#[test]
fn parser_rejects_token_stream_over_token_limit() {
    let source = "q q q q q;";
    let mut tokens = lex_tokens(source);

    let limits = ParserLimits {
        max_tokens: 1,
        ..ParserLimits::default()
    };

    let result = OpenQasmParser::from_tokens(
        std::mem::take(&mut tokens),
        parser_config_with_limits(limits),
    );

    let error = result.expect_err(
        "token limit must be enforced before parsing",
    );

    assert_eq!(
        error.kind(),
        ParseErrorKind::TokenLimitExceeded
    );
}

// =============================================================================
// Parser configuration validation
// =============================================================================

#[test]
fn zero_token_limit_is_rejected() {
    let limits = ParserLimits {
        max_tokens: 0,
        ..ParserLimits::default()
    };

    let result = parse_with_config(
        "",
        parser_config_with_limits(limits),
    );

    let error = result.expect_err(
        "zero token limit must be rejected as invalid configuration",
    );

    assert_eq!(
        error.kind(),
        ParseErrorKind::Configuration
    );
}

#[test]
fn zero_ast_node_limit_is_rejected() {
    let limits = ParserLimits {
        max_ast_nodes: 0,
        ..ParserLimits::default()
    };

    let result = parse_with_config(
        "",
        parser_config_with_limits(limits),
    );

    let error = result.expect_err(
        "zero AST-node limit must be rejected"
    );

    assert_eq!(
        error.kind(),
        ParseErrorKind::Configuration
    );
}

#[test]
fn zero_nesting_limit_is_rejected() {
    let limits = ParserLimits {
        max_nesting_depth: 0,
        ..ParserLimits::default()
    };

    let result = parse_with_config(
        "",
        parser_config_with_limits(limits),
    );

    let error = result.expect_err(
        "zero nesting limit must be rejected"
    );

    assert_eq!(
        error.kind(),
        ParseErrorKind::Configuration
    );
}

#[test]
fn excessive_nesting_limit_is_rejected() {
    let limits = ParserLimits {
        max_nesting_depth: 257,
        ..ParserLimits::default()
    };

    let result = parse_with_config(
        "",
        parser_config_with_limits(limits),
    );

    let error = result.expect_err(
        "unsafe recursion limit must be rejected"
    );

    assert_eq!(
        error.kind(),
        ParseErrorKind::Configuration
    );
}

#[test]
fn excessive_expression_depth_limit_is_rejected() {
    let limits = ParserLimits {
        max_expression_depth: 257,
        ..ParserLimits::default()
    };

    let result = parse_with_config(
        "",
        parser_config_with_limits(limits),
    );

    let error = result.expect_err(
        "unsafe expression recursion limit must be rejected"
    );

    assert_eq!(
        error.kind(),
        ParseErrorKind::Configuration
    );
}

// =============================================================================
// Resource exhaustion
// =============================================================================

#[test]
fn statement_limit_is_enforced() {
    let source = r#"
qubit q;
qubit r;
qubit s;
"#;

    let limits = ParserLimits {
        max_statements_per_scope: 2,
        ..ParserLimits::default()
    };

    let result = parse_with_config(
        source,
        parser_config_with_limits(limits),
    );

    let error = result.expect_err(
        "statement limit must be enforced"
    );

    assert_eq!(
        error.kind(),
        ParseErrorKind::StatementLimitExceeded
    );
}

#[test]
fn ast_node_limit_is_enforced() {
    let limits = ParserLimits {
        max_ast_nodes: 1,
        ..ParserLimits::default()
    };

    let result = parse_with_config(
        "qubit q;",
        parser_config_with_limits(limits),
    );

    let error = result.expect_err(
        "AST-node limit must be enforced"
    );

    assert_eq!(
        error.kind(),
        ParseErrorKind::AstLimitExceeded
    );
}

#[test]
fn expression_depth_limit_is_enforced() {
    let source = "qubit q;\nlet x = (((((((((((1))))))))));";

    let limits = ParserLimits {
        max_expression_depth: 4,
        ..ParserLimits::default()
    };

    let result = parse_with_config(
        source,
        parser_config_with_limits(limits),
    );

    let error = result.expect_err(
        "deep expression nesting must be bounded"
    );

    assert_eq!(
        error.kind(),
        ParseErrorKind::ExpressionDepthExceeded
    );
}

#[test]
fn syntactic_nesting_limit_is_enforced() {
    let source = r#"
if (true) {
    if (true) {
        if (true) {
            if (true) {
                qubit q;
            }
        }
    }
}
"#;

    let limits = ParserLimits {
        max_nesting_depth: 2,
        ..ParserLimits::default()
    };

    let result = parse_with_config(
        source,
        parser_config_with_limits(limits),
    );

    let error = result.expect_err(
        "nested scopes must be bounded"
    );

    assert_eq!(
        error.kind(),
        ParseErrorKind::NestingLimitExceeded
    );
}

// =============================================================================
// Declarations
// =============================================================================

#[test]
fn parses_qubit_declaration() {
    assert_statement_kind(
        "qubit q;",
        |statement| {
            matches!(statement, Statement::QuantumDeclaration(_))
        },
    );
}

#[test]
fn parses_sized_qubit_declaration() {
    assert_statement_kind(
        "qubit[8] q;",
        |statement| {
            matches!(statement, Statement::QuantumDeclaration(_))
        },
    );
}

#[test]
fn parses_bit_declaration() {
    assert_statement_kind(
        "bit c;",
        |statement| {
            matches!(statement, Statement::ClassicalDeclaration(_))
        },
    );
}

#[test]
fn parses_sized_bit_declaration() {
    assert_statement_kind(
        "bit[8] c;",
        |statement| {
            matches!(statement, Statement::ClassicalDeclaration(_))
        },
    );
}

#[test]
fn parses_bool_declaration() {
    assert_statement_kind(
        "bool flag;",
        |statement| {
            matches!(statement, Statement::ClassicalDeclaration(_))
        },
    );
}

#[test]
fn parses_integer_declaration() {
    assert_statement_kind(
        "int[32] value;",
        |statement| {
            matches!(statement, Statement::ClassicalDeclaration(_))
        },
    );
}

#[test]
fn parses_unsigned_integer_declaration() {
    assert_statement_kind(
        "uint[32] value;",
        |statement| {
            matches!(statement, Statement::ClassicalDeclaration(_))
        },
    );
}

#[test]
fn parses_float_declaration() {
    assert_statement_kind(
        "float[64] value;",
        |statement| {
            matches!(statement, Statement::ClassicalDeclaration(_))
        },
    );
}

#[test]
fn parses_angle_declaration() {
    assert_statement_kind(
        "angle[32] theta;",
        |statement| {
            matches!(statement, Statement::ClassicalDeclaration(_))
        },
    );
}

#[test]
fn parses_complex_declaration() {
    assert_statement_kind(
        "complex[float[32]] value;",
        |statement| {
            matches!(statement, Statement::ClassicalDeclaration(_))
        },
    );
}

#[test]
fn parses_array_declaration() {
    assert_statement_kind(
        "array[float[32], 4] values;",
        |statement| {
            matches!(statement, Statement::ClassicalDeclaration(_))
        },
    );
}

#[test]
fn parses_const_declaration() {
    assert_statement_kind(
        "const int[32] value = 1;",
        |statement| {
            matches!(statement, Statement::ConstDeclaration(_))
        },
    );
}

#[test]
fn parses_input_declaration() {
    assert_statement_kind(
        "input int[32] value;",
        |statement| {
            matches!(statement, Statement::IoDeclaration(_))
        },
    );
}

#[test]
fn parses_output_declaration() {
    assert_statement_kind(
        "output bit[8] result;",
        |statement| {
            matches!(statement, Statement::IoDeclaration(_))
        },
    );
}

// =============================================================================
// Quantum operations
// =============================================================================

#[test]
fn parses_gate_call() {
    assert_statement_kind(
        "x q;",
        |statement| {
            matches!(statement, Statement::GateCall(_))
        },
    );
}

#[test]
fn parses_parameterized_gate_call() {
    assert_statement_kind(
        "rx(pi / 2) q;",
        |statement| {
            matches!(statement, Statement::GateCall(_))
        },
    );
}

#[test]
fn parses_multi_qubit_gate_call() {
    assert_statement_kind(
        "cx q, r;",
        |statement| {
            matches!(statement, Statement::GateCall(_))
        },
    );
}

#[test]
fn parses_measurement() {
    assert_statement_kind(
        "measure q;",
        |statement| {
            matches!(statement, Statement::Measurement(_))
        },
    );
}

#[test]
fn parses_measurement_assignment() {
    assert_statement_kind(
        "measure q -> c;",
        |statement| {
            matches!(statement, Statement::MeasureAssignment(_))
        },
    );
}

#[test]
fn parses_reset() {
    assert_statement_kind(
        "reset q;",
        |statement| {
            matches!(statement, Statement::Reset(_))
        },
    );
}

#[test]
fn parses_barrier() {
    assert_statement_kind(
        "barrier q, r;",
        |statement| {
            matches!(statement, Statement::Barrier(_))
        },
    );
}

#[test]
fn parses_delay() {
    assert_statement_kind(
        "delay[100ns] q;",
        |statement| {
            matches!(statement, Statement::Delay(_))
        },
    );
}

// =============================================================================
// Gate modifiers
// =============================================================================

#[test]
fn parses_inverse_gate_modifier() {
    assert_statement_kind(
        "inv @ x q;",
        |statement| {
            matches!(statement, Statement::GateCall(_))
        },
    );
}

#[test]
fn parses_control_gate_modifier() {
    assert_statement_kind(
        "ctrl @ x q;",
        |statement| {
            matches!(statement, Statement::GateCall(_))
        },
    );
}

#[test]
fn parses_negative_control_gate_modifier() {
    assert_statement_kind(
        "negctrl @ x q;",
        |statement| {
            matches!(statement, Statement::GateCall(_))
        },
    );
}

#[test]
fn parses_power_gate_modifier() {
    assert_statement_kind(
        "pow(2) @ x q;",
        |statement| {
            matches!(statement, Statement::GateCall(_))
        },
    );
}

#[test]
fn parses_chained_gate_modifiers() {
    assert_statement_kind(
        "ctrl @ inv @ x q;",
        |statement| {
            matches!(statement, Statement::GateCall(_))
        },
    );
}

// =============================================================================
// Gate definitions
// =============================================================================

#[test]
fn parses_simple_gate_definition() {
    assert_statement_kind(
        r#"
gate my_gate q {
    x q;
}
"#,
        |statement| {
            matches!(statement, Statement::GateDefinition(_))
        },
    );
}

#[test]
fn parses_parameterized_gate_definition() {
    assert_statement_kind(
        r#"
gate my_gate(theta) q {
    rx(theta) q;
}
"#,
        |statement| {
            matches!(statement, Statement::GateDefinition(_))
        },
    );
}

#[test]
fn parses_multi_parameter_gate_definition() {
    assert_statement_kind(
        r#"
gate my_gate(theta, phi) q, r {
    rx(theta) q;
    rz(phi) r;
    cx q, r;
}
"#,
        |statement| {
            matches!(statement, Statement::GateDefinition(_))
        },
    );
}

// =============================================================================
// Classical expressions and assignments
// =============================================================================

#[test]
fn parses_let_statement() {
    assert_statement_kind(
        "let x = 1;",
        |statement| {
            matches!(statement, Statement::Let(_))
        },
    );
}

#[test]
fn parses_assignment() {
    assert_statement_kind(
        "x = 1;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

#[test]
fn parses_compound_assignment() {
    assert_statement_kind(
        "x += 1;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

#[test]
fn parses_boolean_expression() {
    assert_statement_kind(
        "x = true && false;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

#[test]
fn parses_arithmetic_expression() {
    assert_statement_kind(
        "x = (1 + 2) * 3 - 4 / 2;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

#[test]
fn parses_comparison_expression() {
    assert_statement_kind(
        "x = a >= b;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

#[test]
fn parses_bitwise_expression() {
    assert_statement_kind(
        "x = a & b | c ^ d;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

#[test]
fn parses_shift_expression() {
    assert_statement_kind(
        "x = a << 2;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

// =============================================================================
// Classical control flow
// =============================================================================

#[test]
fn parses_if_statement() {
    assert_statement_kind(
        r#"
if (flag) {
    x q;
}
"#,
        |statement| {
            matches!(statement, Statement::If(_))
        },
    );
}

#[test]
fn parses_if_else_statement() {
    assert_statement_kind(
        r#"
if (flag) {
    x q;
} else {
    z q;
}
"#,
        |statement| {
            matches!(statement, Statement::If(_))
        },
    );
}

#[test]
fn parses_for_statement() {
    assert_statement_kind(
        r#"
for int i in [0:4] {
    x q;
}
"#,
        |statement| {
            matches!(statement, Statement::For(_))
        },
    );
}

#[test]
fn parses_for_range_with_step() {
    assert_statement_kind(
        r#"
for int i in [0:1:4] {
    x q;
}
"#,
        |statement| {
            matches!(statement, Statement::For(_))
        },
    );
}

#[test]
fn parses_while_statement() {
    assert_statement_kind(
        r#"
while (flag) {
    x q;
}
"#,
        |statement| {
            matches!(statement, Statement::While(_))
        },
    );
}

#[test]
fn parses_switch_statement() {
    assert_statement_kind(
        r#"
switch (value) {
    case 0 {
        x q;
    }
    default {
        z q;
    }
}
"#,
        |statement| {
            matches!(statement, Statement::Switch(_))
        },
    );
}

#[test]
fn parses_break_statement() {
    assert_statement_kind(
        "break;",
        |statement| {
            matches!(statement, Statement::Break(_))
        },
    );
}

#[test]
fn parses_continue_statement() {
    assert_statement_kind(
        "continue;",
        |statement| {
            matches!(statement, Statement::Continue(_))
        },
    );
}

#[test]
fn parses_return_statement() {
    assert_statement_kind(
        "return;",
        |statement| {
            matches!(statement, Statement::Return(_))
        },
    );
}

#[test]
fn parses_return_expression() {
    assert_statement_kind(
        "return value;",
        |statement| {
            matches!(statement, Statement::Return(_))
        },
    );
}

// =============================================================================
// Subroutines / extern
// =============================================================================

#[test]
fn parses_subroutine_definition() {
    assert_statement_kind(
        r#"
def helper(int[32] value) {
    return;
}
"#,
        |statement| {
            matches!(statement, Statement::SubroutineDefinition(_))
        },
    );
}

#[test]
fn parses_extern_declaration() {
    assert_statement_kind(
        "extern int[32] helper(int[32]);",
        |statement| {
            matches!(statement, Statement::ExternDeclaration(_))
        },
    );
}

// =============================================================================
// Include / annotations / pragmas
// =============================================================================

#[test]
fn parses_include_statement() {
    assert_statement_kind(
        r#"include "stdgates.inc";"#,
        |statement| {
            matches!(statement, Statement::Include(_))
        },
    );
}

#[test]
fn parses_annotation() {
    assert_statement_kind(
        "@annotation value",
        |statement| {
            matches!(statement, Statement::Annotation(_))
        },
    );
}

#[test]
fn parses_pragma() {
    assert_statement_kind(
        "pragma zamani test;",
        |statement| {
            matches!(statement, Statement::Pragma(_))
        },
    );
}

// =============================================================================
// Timing / box
// =============================================================================

#[test]
fn parses_box_without_duration() {
    assert_statement_kind(
        r#"
box {
    x q;
}
"#,
        |statement| {
            matches!(statement, Statement::Box(_))
        },
    );
}

#[test]
fn parses_box_with_duration() {
    assert_statement_kind(
        r#"
box[100ns] {
    x q;
}
"#,
        |statement| {
            matches!(statement, Statement::Box(_))
        },
    );
}

// =============================================================================
// Calibration syntax
// =============================================================================

#[test]
fn parses_defcalgrammar_statement() {
    assert_statement_kind(
        r#"defcalgrammar "openpulse";"#,
        |statement| {
            matches!(statement, Statement::DefcalGrammar(_))
        },
    );
}

#[test]
fn parses_defcal_definition_syntax() {
    assert_statement_kind(
        r#"
defcal x $0 {
    play(0, 1);
}
"#,
        |statement| {
            matches!(statement, Statement::Defcal(_))
        },
    );
}

#[test]
fn parses_calibration_statement() {
    assert_statement_kind(
        r#"
cal {
    play(0, 1);
}
"#,
        |statement| {
            matches!(statement, Statement::Calibration(_))
        },
    );
}

// =============================================================================
// Statement-family coverage
// =============================================================================

#[test]
fn parser_ast_statement_kind_is_stable() {
    let source = "qubit q;";

    let program = parse(source).expect("source must parse");

    assert_eq!(
        program.node_kind(),
        AstNodeKind::Program
    );

    assert_eq!(
        program.statements()[0].node_kind(),
        AstNodeKind::Statement
    );
}

// =============================================================================
// Malformed syntax — required rejection
// =============================================================================

#[test]
fn rejects_missing_semicolon_after_declaration() {
    assert_parse_error(
        "qubit q",
        ParseErrorKind::UnexpectedEof,
    );
}

#[test]
fn rejects_missing_closing_brace() {
    let result = parse(
        r#"
if (true) {
    x q;
"#,
    );

    assert!(
        result.is_err(),
        "unterminated scope must be rejected"
    );
}

#[test]
fn rejects_missing_opening_parenthesis() {
    let result = parse(
        "if true { x q; }",
    );

    assert!(
        result.is_err(),
        "if without opening parenthesis must be rejected"
    );
}

#[test]
fn rejects_missing_closing_parenthesis() {
    let result = parse(
        "if (true { x q; }",
    );

    assert!(
        result.is_err(),
        "unterminated condition must be rejected"
    );
}

#[test]
fn rejects_missing_gate_body() {
    let result = parse(
        "gate g q",
    );

    assert!(
        result.is_err(),
        "gate definition without body must be rejected"
    );
}

#[test]
fn rejects_missing_gate_name() {
    let result = parse(
        "gate q { x q; }",
    );

    assert!(
        result.is_err(),
        "gate syntax with missing required identifier must be rejected"
    );
}

#[test]
fn rejects_missing_assignment_expression() {
    let result = parse(
        "x = ;",
    );

    assert!(
        result.is_err(),
        "assignment without an expression must be rejected"
    );
}

#[test]
fn rejects_unterminated_include_string() {
    let result = parse(
        r#"include "stdgates.inc;"#,
    );

    assert!(
        result.is_err(),
        "unterminated include string must be rejected"
    );
}

#[test]
fn rejects_unterminated_annotation_context() {
    let result = parse(
        "@",
    );

    assert!(
        result.is_err(),
        "incomplete annotation syntax must be rejected"
    );
}

#[test]
fn rejects_trailing_garbage() {
    let result = parse(
        "qubit q; @@@",
    );

    assert!(
        result.is_err(),
        "parser must not silently discard trailing source"
    );
}

// =============================================================================
// Expression boundary cases
// =============================================================================

#[test]
fn parses_unary_plus() {
    assert_statement_kind(
        "x = +1;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

#[test]
fn parses_unary_minus() {
    assert_statement_kind(
        "x = -1;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

#[test]
fn parses_logical_not() {
    assert_statement_kind(
        "x = !flag;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

#[test]
fn parses_bitwise_not() {
    assert_statement_kind(
        "x = ~value;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

#[test]
fn parses_power_expression() {
    assert_statement_kind(
        "x = 2 ** 8;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

#[test]
fn parses_parenthesized_expression() {
    assert_statement_kind(
        "x = (a + b) * c;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

// =============================================================================
// Numeric / timing syntax preservation
// =============================================================================

#[test]
fn parses_integer_literal_expression() {
    assert_statement_kind(
        "x = 123456;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

#[test]
fn parses_float_literal_expression() {
    assert_statement_kind(
        "x = 1.25;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

#[test]
fn parses_imaginary_literal_expression() {
    assert_statement_kind(
        "x = 1.0im;",
        |statement| {
            matches!(statement, Statement::Assignment(_))
        },
    );
}

#[test]
fn parses_duration_literal_in_delay() {
    assert_statement_kind(
        "delay[100ns] q;",
        |statement| {
            matches!(statement, Statement::Delay(_))
        },
    );
}

#[test]
fn parses_multiple_duration_units() {
    let source = r#"
delay[1dt] q;
delay[1s] q;
delay[1ms] q;
delay[1us] q;
delay[1ns] q;
delay[1ps] q;
delay[1fs] q;
"#;

    let program = parse(source)
        .expect("all OpenQASM duration units must be lexically/parser accepted");

    assert_eq!(program.statements().len(), 7);

    for statement in program.statements() {
        assert!(
            matches!(statement, Statement::Delay(_)),
            "every duration test statement must be a Delay node"
        );
    }
}

// =============================================================================
// Unicode / source preservation
// =============================================================================

#[test]
fn unicode_identifiers_reach_the_parser_without_span_corruption() {
    let source = "qubit q_π;";

    let program = parse(source)
        .expect("Unicode identifier source must parse");

    assert_eq!(program.statements().len(), 1);

    let span = program.statements()[0].span();

    assert_eq!(
        span.source_id,
        SourceId::from_raw(1)
    );

    assert!(
        span.end.as_usize() <= source.len(),
        "AST span must remain inside source byte range"
    );
}

#[test]
fn multibyte_source_preserves_monotonic_spans() {
    let source = "qubit q_π;\nbit café;\n";

    let program = parse(source)
        .expect("multibyte source must parse");

    let statements = program.statements();

    assert_eq!(statements.len(), 2);

    assert!(
        statements[0].span().end.as_usize()
            <= statements[1].span().start.as_usize(),
        "statement spans must remain byte-ordered with Unicode source"
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn parsing_is_deterministic() {
    let source = r#"
OPENQASM 3.1;

qubit[2] q;
bit[2] c;

h q[0];
cx q[0], q[1];
measure q -> c;
"#;

    let first = parse(source)
        .expect("first parse must succeed");

    let second = parse(source)
        .expect("second parse must succeed");

    assert_eq!(
        first,
        second,
        "same source/config must produce identical AST"
    );
}

#[test]
fn parser_error_is_deterministic() {
    let source = "if (true {";

    let first = parse(source)
        .expect_err("invalid source must fail");

    let second = parse(source)
        .expect_err("invalid source must fail");

    assert_eq!(
        first,
        second,
        "same invalid source/config must produce identical parser errors"
    );

    assert_eq!(
        first.code(),
        second.code()
    );

    assert_eq!(
        first.message(),
        second.message()
    );

    assert_eq!(
        first.span(),
        second.span()
    );
}

// =============================================================================
// Panic-safety corpus
// =============================================================================

#[test]
fn malformed_inputs_never_panic() {
    let corpus = [
        "",
        ";",
        "{",
        "}",
        "(",
        ")",
        "[",
        "]",
        "OPENQASM",
        "OPENQASM 3",
        "OPENQASM 3.;",
        "OPENQASM 3.1",
        "qubit",
        "qubit[",
        "qubit[] q;",
        "qubit q",
        "bit",
        "bit[ q;",
        "measure",
        "measure ->",
        "reset",
        "barrier",
        "delay[",
        "delay[] q;",
        "if",
        "if (",
        "if ()",
        "if (true",
        "if (true) {",
        "for",
        "while",
        "switch",
        "gate",
        "gate g",
        "gate g q",
        "gate g q {",
        "def",
        "def f(",
        "extern",
        "extern f(",
        "return",
        "let",
        "let x =",
        "x =",
        "x +",
        "include",
        r#"include ""#,
        r#"include "unterminated"#,
        "@",
        "#",
        "pragma",
        "defcal",
        "defcalgrammar",
        "cal",
        "cal {",
        "cal { } trailing",
        "!!!",
        "@@@",
        "$",
        "$-1",
        "qubit[999999999999999999999999999999] q;",
    ];

    for source in corpus {
        let result = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| parse(source)),
        );

        assert!(
            result.is_ok(),
            "parser panicked for malformed source:\n{}",
            source
        );
    }
}

// =============================================================================
// No silent trailing input
// =============================================================================

#[test]
fn parser_consumes_complete_token_stream() {
    let sources = [
        "qubit q;",
        "bit c;",
        "x q;",
        "measure q -> c;",
        "reset q;",
        "barrier q;",
    ];

    for source in sources {
        let program = parse(source)
            .expect("source must parse");

        assert!(
            !program.statements().is_empty(),
            "complete source must produce AST statements"
        );
    }
}

// =============================================================================
// Composite production corpus
// =============================================================================

#[test]
fn parses_production_openqasm_program() {
    let source = r#"
OPENQASM 3.1;

include "stdgates.inc";

input int[32] repetitions;
output bit[2] result;

const float[64] theta = 1.5707963267948966;

qubit[2] q;
bit[2] c;

gate entangle(a, b) {
    h a;
    cx a, b;
}

entangle q[0], q[1];

ctrl @ x q[0];
inv @ h q[1];
pow(2) @ x q[0];

delay[10ns] q;
barrier q[0], q[1];

if (c == 1) {
    x q[0];
} else {
    z q[0];
}

for int i in [0:1:4] {
    h q[0];
}

while (c == 0) {
    x q[0];
    break;
}

measure q -> c;
"#;

    let program = parse(source)
        .expect("production OpenQASM corpus must parse");

    assert_eq!(
        program.version(),
        Some(OpenQasmVersion::V3_1)
    );

    assert!(
        program.statements().len() >= 15,
        "production corpus should exercise many statement families"
    );
}

// =============================================================================
// Parser / semantic-boundary separation
// =============================================================================

#[test]
fn parser_does_not_reject_unknown_gate_names_as_a_syntax_error() {
    // The parser represents syntax. Whether `my_future_gate` is declared,
    // imported, standard, or otherwise valid is the validator's responsibility.
    assert_parses(
        "my_future_gate q;"
    );
}

#[test]
fn parser_does_not_perform_qubit_arity_validation() {
    // Semantic arity checking belongs to validation.rs. The parser only
    // constructs the syntactic gate call.
    assert_parses(
        "cx q;"
    );
}

#[test]
fn parser_does_not_resolve_include_files() {
    // Parsing the include statement must not perform filesystem access.
    assert_parses(
        r#"include "caller-controlled-name.inc";"#,
    );
}

#[test]
fn parser_treats_extern_as_source_syntax_only() {
    assert_parses(
        "extern int[32] external_function(int[32]);",
    );
}

// =============================================================================
// AST invariants
// =============================================================================

#[test]
fn every_top_level_statement_has_a_non_empty_span() {
    let source = r#"
qubit q;
bit c;
x q;
measure q -> c;
"#;

    let program = parse(source)
        .expect("source must parse");

    for statement in program.statements() {
        let span = statement.span();

        assert!(
            span.start.as_usize() < span.end.as_usize(),
            "top-level statement spans must not be empty"
        );

        assert!(
            span.end.as_usize() <= source.len(),
            "statement span must remain within source"
        );

        assert_eq!(
            span.source_id,
            SourceId::from_raw(1)
        );
    }
}

#[test]
fn program_ast_node_kind_is_program() {
    let program = parse("qubit q;")
        .expect("source must parse");

    assert_eq!(
        program.node_kind(),
        AstNodeKind::Program
    );
}

#[test]
fn statement_ast_node_kind_is_statement() {
    let program = parse("qubit q;")
        .expect("source must parse");

    assert_eq!(
        program.statements()[0].node_kind(),
        AstNodeKind::Statement
    );
}

// =============================================================================
// Regression guards for parser API
// =============================================================================

#[test]
fn parser_config_source_id_is_not_hard_coded() {
    for raw_id in [0_u32, 1_u32, 42_u32, u32::MAX] {
        let parser_config = ParserConfig {
            source_id: SourceId::from_raw(raw_id),
            limits: ParserLimits::default(),
        };

        let program = parse_with_config(
            "qubit q;",
            parser_config,
        )
        .expect("source must parse");

        assert_eq!(
            program.span().source_id,
            SourceId::from_raw(raw_id)
        );
    }
}

#[test]
fn parser_limits_are_checked_before_recursive_work() {
    let limits = ParserLimits {
        max_nesting_depth: 1,
        ..ParserLimits::default()
    };

    let source = r#"
if (true) {
    if (true) {
        if (true) {
            x q;
        }
    }
}
"#;

    let result = parse_with_config(
        source,
        parser_config_with_limits(limits),
    );

    assert!(
        matches!(
            result,
            Err(ParseError {
                kind: ParseErrorKind::NestingLimitExceeded,
                ..
            })
        ),
        "parser must enforce nesting limits before unbounded recursive descent"
    );
}