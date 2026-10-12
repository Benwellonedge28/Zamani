#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Integration and conformance tests for Zamani's domain parser adapters.
//!
//! File: src/frontend/parser/tests/domains.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Contract
//!
//! These tests exercise the existing domain adapters through the canonical
//! lexer Token, TokenKind, and Span types. They test syntax recognition and
//! preservation, not semantic validity or target feasibility.
//!
//! # Ownership
//!
//! - The lexer owns tokenization and source locations.
//! - Domain adapters own their documented syntax-boundary behavior.
//! - The authoritative grammar files own accepted language syntax.
//! - The canonical AST adapter owns conversion into the frontend AST.
//! - Semantic analysis owns name resolution, types, effects, capabilities,
//!   contracts, resource feasibility, and authorization.
//! - Compiler backends own lowering and target-specific realization.
//!
//! # Scalability
//!
//! Tests must not introduce hardware-capacity ceilings. Configurable limits
//! exercised here are parser-work safeguards, not language-level restrictions.
//! Any workload remains subject to available memory, address space, and
//! explicitly configured resource budgets.
//!
//! # Integration prerequisites
//!
//! - `src/frontend/parser/domains/mod.rs` declares the tested domain modules.
//! - `src/frontend/parser/tests/mod.rs` declares `mod domains;`.
//! - `src/frontend/parser/mod.rs` exposes the test module under `#[cfg(test)]`.
//! - The module tree and existing grammar/API contracts must compile together.
//!
//! These tests do not replace grammar conformance tests generated from the
//! normative ANTLR grammar files.

use crate::frontend::lexer::cursor::CursorPosition;
use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;
use crate::frontend::parser::domains::capabilities::{
    CapabilityItem, CapabilityParser, CapabilityParserLimits,
};
use crate::frontend::parser::domains::classical::ClassicalConstructKind;
use crate::frontend::parser::domains::contracts::{
    ContractItem, ContractParser, ContractParserLimits,
};
use crate::frontend::parser::domains::distributed::{
    scan_distributed_regions, DistributedScanLimits,
};
use crate::frontend::parser::domains::effects::{
    parse_effect_reference, parse_effect_set, EffectParseLimits,
};
use crate::frontend::parser::domains::hdl::{
    HdlConstructKind, HdlDomainParser, HdlParserLimits,
};
use crate::frontend::parser::domains::hybrid::{
    parse_hybrid_region, HybridElementKind, HybridSyntaxErrorKind,
};
use crate::frontend::parser::domains::quantum::{
    QuantumDomainParser, QuantumItem, QuantumParserLimits,
};
use crate::frontend::parser::domains::resources::{
    ResourceItem, ResourceParser, ResourceParserLimits,
};

/// Construct a canonical source span from byte offsets.
///
/// The test fixtures are ASCII, so byte and scalar offsets coincide. The
/// cursor calculation still handles line boundaries so future multiline
/// fixtures can reuse the helper.
fn span_for(source: &str, start: usize, end: usize) -> Span {
    assert!(start <= end, "span start must not exceed span end");
    assert!(end <= source.len(), "span must remain inside the source");
    assert!(
        source.is_char_boundary(start) && source.is_char_boundary(end),
        "span boundaries must be UTF-8 character boundaries"
    );

    fn position(source: &str, offset: usize) -> CursorPosition {
        let prefix = &source[..offset];
        let line = prefix.bytes().filter(|byte| *byte == b'\n').count() + 1;
        let column = prefix
            .rsplit('\n')
            .next()
            .map_or(1, |last_line| last_line.chars().count() + 1);

        CursorPosition::new(offset, line, column)
    }

    Span::new(position(source, start), position(source, end))
}

/// Map punctuation needed by the domain adapters to canonical token kinds.
///
/// This helper intentionally does not implement a lexer or classify domain
/// keywords. Domain words remain identifiers and retain their source spelling.
fn punctuation_kind(spelling: &str) -> TokenKind {
    match spelling {
        "(" => TokenKind::LeftParen,
        ")" => TokenKind::RightParen,
        "{" => TokenKind::LeftBrace,
        "}" => TokenKind::RightBrace,
        "[" => TokenKind::LeftBracket,
        "]" => TokenKind::RightBracket,
        "," => TokenKind::Comma,
        ";" => TokenKind::Semicolon,
        ":" => TokenKind::Colon,
        "::" => TokenKind::DoubleColon,
        "@" => TokenKind::At,
        "#" => TokenKind::Hash,
        "$" => TokenKind::Dollar,
        "?" => TokenKind::Question,
        "~" => TokenKind::Tilde,
        "+" => TokenKind::Plus,
        "-" => TokenKind::Minus,
        "*" => TokenKind::Star,
        "/" => TokenKind::Slash,
        "%" => TokenKind::Modulo,
        "=" => TokenKind::Assign,
        _ => TokenKind::Identifier,
    }
}

/// Produce a small, source-backed token stream for adapter integration tests.
///
/// This helper only supports the deliberately limited ASCII fixtures below.
/// It is not a production tokenizer and must not be used to validate lexical
/// rules. The real lexer owns that responsibility.
///
/// Every stream includes exactly one synthetic EOF token at its end.
fn tokens(source: &str) -> Vec<Token<'_>> {
    let bytes = source.as_bytes();
    let mut output = Vec::new();
    let mut index = 0;

    while index < bytes.len() {
        let byte = bytes[index];

        if byte.is_ascii_whitespace() {
            index += 1;
            continue;
        }

        let start = index;

        if byte == b'"' {
            index += 1;

            while index < bytes.len() {
                if bytes[index] == b'\\' {
                    index = (index + 2).min(bytes.len());
                    continue;
                }

                if bytes[index] == b'"' {
                    index += 1;
                    break;
                }

                index += 1;
            }

            let spelling = &source[start..index];
            output.push(Token::borrowed(
                TokenKind::StringLiteral,
                spelling,
                span_for(source, start, index),
            ));
            continue;
        }

        if byte.is_ascii_alphanumeric() || byte == b'_' {
            index += 1;

            while index < bytes.len()
                && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
            {
                index += 1;
            }

            let spelling = &source[start..index];
            let kind = if spelling.bytes().all(|ch| ch.is_ascii_digit()) {
                TokenKind::IntegerLiteral
            } else {
                TokenKind::Identifier
            };

            output.push(Token::borrowed(
                kind,
                spelling,
                span_for(source, start, index),
            ));
            continue;
        }

        if bytes.get(index..index.saturating_add(2)) == Some(b"::") {
            index += 2;
        } else {
            index += 1;
        }

        let spelling = &source[start..index];
        output.push(Token::borrowed(
            punctuation_kind(spelling),
            spelling,
            span_for(source, start, index),
        ));
    }

    output.push(Token::synthetic(
        TokenKind::Eof,
        span_for(source, source.len(), source.len()),
    ));

    output
}

fn unlimited_effect_limits() -> EffectParseLimits {
    EffectParseLimits::unlimited()
}

// -----------------------------------------------------------------------------
// Classical-domain dispatch
// -----------------------------------------------------------------------------

#[test]
fn classical_dispatch_recognizes_shared_declaration_starts() {
    assert_eq!(
        ClassicalConstructKind::from_token_kind(TokenKind::KeywordFn),
        Some(ClassicalConstructKind::Declaration)
    );
}

#[test]
fn classical_dispatch_does_not_claim_unknown_future_token_kinds() {
    // Identifier is an expression candidate in the current adapter, not a
    // guarantee that every identifier begins a complete valid expression.
    assert_eq!(
        ClassicalConstructKind::from_token_kind(TokenKind::Identifier),
        Some(ClassicalConstructKind::ExpressionStatement)
    );
}

// -----------------------------------------------------------------------------
// Quantum-domain syntax
// -----------------------------------------------------------------------------

#[test]
fn quantum_adapter_preserves_open_world_operation_names() {
    let source = "apply vendor_future_interaction(theta)(target);";
    let stream = tokens(source);
    let mut parser =
        QuantumDomainParser::new(&stream, QuantumParserLimits::default());

    let item = parser
        .parse_next()
        .expect("valid operation syntax should not fail")
        .expect("operation introducer should be recognized");

    assert!(
        matches!(item, QuantumItem::Operation(_)),
        "operation syntax should remain a quantum operation item"
    );

    let debug = format!("{item:?}");
    assert!(
        debug.contains("vendor_future_interaction"),
        "operation identity must survive syntax parsing"
    );
    assert!(
        debug.contains("target"),
        "target syntax must survive syntax parsing"
    );
}

#[test]
fn quantum_adapter_leaves_classical_input_for_the_parent_parser() {
    let source = "compute_value;";
    let stream = tokens(source);
    let mut parser =
        QuantumDomainParser::new(&stream, QuantumParserLimits::default());

    let before = parser.position();
    let item = parser
        .parse_next()
        .expect("non-quantum input should not be an error");

    assert!(item.is_none());
    assert_eq!(
        parser.position(),
        before,
        "a non-matching extension point must not consume input"
    );
}

#[test]
fn quantum_adapter_supports_configurable_construct_budgets() {
    let source = "apply operation(parameter)(target);";
    let stream = tokens(source);
    let limits = QuantumParserLimits {
        max_tokens_per_construct: 1,
        ..QuantumParserLimits::default()
    };
    let mut parser = QuantumDomainParser::new(&stream, limits);

    let result = parser.parse_next();

    assert!(
        result.is_err(),
        "an over-budget quantum construct must fail explicitly"
    );
}

#[test]
fn quantum_adapter_rejects_positions_beyond_the_token_stream() {
    let source = "apply operation(target);";
    let stream = tokens(source);
    let mut parser =
        QuantumDomainParser::new(&stream, QuantumParserLimits::default());

    assert!(
        parser.set_position(stream.len() + 1).is_err(),
        "invalid parser positions must not be accepted"
    );
}

// -----------------------------------------------------------------------------
// HDL-domain syntax
// -----------------------------------------------------------------------------

#[test]
fn hdl_adapter_recognizes_a_module_without_target_assumptions() {
    let source = "module fabric { signal data; }";
    let stream = tokens(source);
    let mut parser = HdlDomainParser::new(&stream, HdlParserLimits::default())
        .expect("default HDL limits should be valid");

    let construct = parser
        .parse_next()
        .expect("module syntax should parse")
        .expect("module introducer should be recognized");

    assert_eq!(construct.kind, HdlConstructKind::Module);
    assert!(construct.body.is_some());
    assert!(
        construct.header.iter().any(|token| token.spelling == "fabric"),
        "module identity should be preserved"
    );
}

#[test]
fn hdl_adapter_leaves_ordinary_function_syntax_unconsumed() {
    let source = "fn compute;";
    let stream = tokens(source);
    let mut parser = HdlDomainParser::new(&stream, HdlParserLimits::default())
        .expect("default HDL limits should be valid");

    let item = parser
        .parse_next()
        .expect("ordinary input should not fail at an HDL extension point");

    assert!(item.is_none());
    assert_eq!(parser.position(), 0);
}

#[test]
fn hdl_adapter_rejects_an_unclosed_module_body() {
    let source = "module fabric { signal data;";
    let stream = tokens(source);
    let mut parser = HdlDomainParser::new(&stream, HdlParserLimits::default())
        .expect("default HDL limits should be valid");

    assert!(
        parser.parse_next().is_err(),
        "an unclosed HDL construct must not be reported as successful"
    );
}

#[test]
fn hdl_adapter_rejects_zero_work_budgets() {
    let limits = HdlParserLimits {
        max_nesting_depth: 0,
        ..HdlParserLimits::default()
    };

    assert!(
        HdlDomainParser::new(&[], limits).is_err(),
        "zero nesting budget should be rejected"
    );

    let limits = HdlParserLimits {
        max_tokens_per_construct: 0,
        ..HdlParserLimits::default()
    };

    assert!(
        HdlDomainParser::new(&[], limits).is_err(),
        "zero construct-token budget should be rejected"
    );
}

// -----------------------------------------------------------------------------
// Hybrid-domain composition
// -----------------------------------------------------------------------------

#[test]
fn hybrid_adapter_preserves_domain_invocations_and_spans() {
    let source = "hybrid { quantum::measure(state); }";
    let stream = tokens(source);
    let region = parse_hybrid_region(&stream)
        .expect("balanced hybrid region should parse");

    assert_eq!(region.elements().len(), 1);
    assert_eq!(
        region.elements()[0].kind(),
        HybridElementKind::DomainInvocation
    );
    assert!(region.span().start_byte() < region.span().end_byte());
    assert!(
        region.span().contains_span(&region.elements()[0].span()),
        "element source span must lie inside the enclosing region"
    );
}

#[test]
fn hybrid_adapter_accepts_an_empty_region() {
    let source = "hybrid {}";
    let stream = tokens(source);
    let region =
        parse_hybrid_region(&stream).expect("empty hybrid region is structurally valid");

    assert!(region.is_empty());
    assert!(region.elements().is_empty());
}

#[test]
fn hybrid_adapter_rejects_missing_closing_brace() {
    let source = "hybrid { quantum::measure(state);";
    let stream = tokens(source);
    let error =
        parse_hybrid_region(&stream).expect_err("missing brace must be rejected");

    assert_eq!(error.kind(), HybridSyntaxErrorKind::UnexpectedEndOfInput);
}

#[test]
fn hybrid_adapter_rejects_trailing_nontrivia_tokens() {
    let source = "hybrid {} unexpected";
    let stream = tokens(source);
    let error =
        parse_hybrid_region(&stream).expect_err("trailing tokens must be rejected");

    assert_eq!(error.kind(), HybridSyntaxErrorKind::TrailingTokens);
}

// -----------------------------------------------------------------------------
// Resource clauses
// -----------------------------------------------------------------------------

#[test]
fn resource_adapter_preserves_portable_requirements() {
    let source = "requires qubits >= required_qubits;";
    let stream = tokens(source);
    let mut parser =
        ResourceParser::new(&stream, ResourceParserLimits::default())
            .expect("stream must contain a final EOF token");

    let items = parser
        .parse_all()
        .expect("resource requirement syntax should parse");

    assert_eq!(items.len(), 1);
    assert!(
        matches!(items.first(), Some(ResourceItem::Clause(_))),
        "resource requirements should be represented as clauses"
    );

    let debug = format!("{:?}", items[0]);
    assert!(debug.contains("required_qubits"));
    assert!(debug.contains("qubits"));
}

#[test]
fn resource_adapter_accepts_future_resource_names_without_a_catalogue() {
    let source = "requires vendor_future_resource >= needed;";
    let stream = tokens(source);
    let mut parser =
        ResourceParser::new(&stream, ResourceParserLimits::default())
            .expect("stream must contain a final EOF token");

    let items = parser
        .parse_all()
        .expect("open-world resource syntax should parse");

    assert_eq!(items.len(), 1);
    assert!(format!("{:?}", items[0]).contains("vendor_future_resource"));
}

#[test]
fn resource_adapter_rejects_a_stream_without_eof() {
    let source = "requires memory >= required_memory;";
    let mut stream = tokens(source);
    stream.pop();

    assert!(
        ResourceParser::new(&stream, ResourceParserLimits::default()).is_err(),
        "resource parser requires a final EOF token"
    );
}

#[test]
fn resource_adapter_enforces_configured_item_budgets() {
    let source = "requires memory >= required_memory;";
    let stream = tokens(source);
    let limits = ResourceParserLimits {
        max_tokens_per_item: 1,
        ..ResourceParserLimits::default()
    };
    let mut parser =
        ResourceParser::new(&stream, limits).expect("token stream should be valid");

    assert!(
        parser.parse_next().is_err(),
        "an over-budget clause must fail explicitly"
    );
}

// -----------------------------------------------------------------------------
// Capability clauses
// -----------------------------------------------------------------------------

#[test]
fn capability_adapter_accepts_open_world_capability_names() {
    let source = "requires capability(\"vendor.future.compute\");";
    let stream = tokens(source);
    let mut parser =
        CapabilityParser::new(&stream, CapabilityParserLimits::default())
            .expect("stream must contain a final EOF token");

    let items = parser
        .parse_all()
        .expect("capability requirement syntax should parse");

    assert_eq!(items.len(), 1);
    assert!(
        matches!(items.first(), Some(CapabilityItem::Clause(_))),
        "capability syntax should be represented as a clause"
    );
    assert!(format!("{:?}", items[0]).contains("vendor.future.compute"));
}

#[test]
fn capability_adapter_preserves_resource_and_compute_requirements() {
    let source = concat!(
        "requires capability(\"gpu.compute\");",
        "requires capability(\"quantum.measurement\");",
    );
    let stream = tokens(source);
    let mut parser =
        CapabilityParser::new(&stream, CapabilityParserLimits::default())
            .expect("stream must contain a final EOF token");

    let items = parser
        .parse_all()
        .expect("both capability clauses should parse");

    assert_eq!(items.len(), 2);
    assert!(format!("{items:?}").contains("gpu.compute"));
    assert!(format!("{items:?}").contains("quantum.measurement"));
}

#[test]
fn capability_adapter_rejects_a_stream_without_eof() {
    let source = "requires capability(\"future.capability\");";
    let mut stream = tokens(source);
    stream.pop();

    assert!(
        CapabilityParser::new(&stream, CapabilityParserLimits::default()).is_err(),
        "capability parser requires a final EOF token"
    );
}

#[test]
fn capability_adapter_enforces_configured_clause_budgets() {
    let source = "requires capability(\"future.capability\");";
    let stream = tokens(source);
    let limits = CapabilityParserLimits {
        max_tokens_per_clause: 1,
        ..CapabilityParserLimits::default()
    };
    let mut parser =
        CapabilityParser::new(&stream, limits).expect("token stream should be valid");

    assert!(
        parser.parse_next().is_err(),
        "an over-budget capability clause must fail explicitly"
    );
}

// -----------------------------------------------------------------------------
// Effect syntax
// -----------------------------------------------------------------------------

#[test]
fn effect_parser_preserves_qualified_effect_references() {
    let source = "quantum::Measurement";
    let stream = tokens(source);

    let (reference, end) = parse_effect_reference(
        &stream,
        0,
        unlimited_effect_limits(),
    )
    .expect("qualified effect reference should parse");

    assert_eq!(
        reference.segments(),
        &["quantum".to_owned(), "Measurement".to_owned()]
    );
    assert!(end > 0);
}

#[test]
fn effect_parser_supports_empty_and_extensible_effect_sets() {
    let empty_source = "{}";
    let empty_stream = tokens(empty_source);
    let (empty_set, _) = parse_effect_set(
        &empty_stream,
        0,
        unlimited_effect_limits(),
    )
    .expect("empty effect set should parse");

    assert!(empty_set.is_empty());

    let source = "{ IO, quantum::Measurement, vendor::FutureEffect, }";
    let stream = tokens(source);
    let (set, end) = parse_effect_set(
        &stream,
        0,
        unlimited_effect_limits(),
    )
    .expect("qualified effect set with trailing comma should parse");

    assert_eq!(set.references().len(), 3);
    assert!(end > 0);
    assert!(format!("{set:?}").contains("FutureEffect"));
}

#[test]
fn effect_parser_reports_budget_exhaustion() {
    let source = "{ IO, quantum::Measurement }";
    let stream = tokens(source);
    let limits = EffectParseLimits::new(1, 1, 1);

    assert!(
        parse_effect_set(&stream, 0, limits).is_err(),
        "the effect parser must enforce the supplied token budget"
    );
}

// -----------------------------------------------------------------------------
// Contract syntax
// -----------------------------------------------------------------------------

#[test]
fn contract_adapter_parses_standalone_contract_clauses() {
    let source = "requires (input > 0);";
    let stream = tokens(source);
    let mut parser =
        ContractParser::new(&stream, ContractParserLimits::default())
            .expect("contract token stream should be valid");

    let items = parser
        .parse_all()
        .expect("standalone contract syntax should parse");

    assert_eq!(items.len(), 1);
    assert!(
        matches!(items.first(), Some(ContractItem::Clause(_))),
        "contract statement should be represented as a clause"
    );
    assert!(format!("{:?}", items[0]).contains("input"));
}

#[test]
fn contract_adapter_rejects_an_empty_condition() {
    let source = "requires ();";
    let stream = tokens(source);
    let mut parser =
        ContractParser::new(&stream, ContractParserLimits::default())
            .expect("token stream should be valid");

    assert!(
        parser.parse_next().is_err(),
        "an empty condition must not be accepted"
    );
}

#[test]
fn contract_adapter_enforces_configured_work_budgets() {
    let source = "requires (input > 0);";
    let stream = tokens(source);
    let limits = ContractParserLimits {
        max_tokens_per_clause: 1,
        ..ContractParserLimits::default()
    };
    let mut parser =
        ContractParser::new(&stream, limits).expect("token stream should be valid");

    assert!(
        parser.parse_next().is_err(),
        "over-budget contract syntax must fail explicitly"
    );
}

// -----------------------------------------------------------------------------
// Distributed-domain structural scanning
// -----------------------------------------------------------------------------

#[test]
fn distributed_scanner_preserves_source_order_for_multiple_regions() {
    let source = "node worker {} service gateway {}";
    let stream = tokens(source);
    let result = scan_distributed_regions(&stream, DistributedScanLimits::default());

    assert_eq!(result.regions.len(), 2);
    assert!(
        result.regions[0].token_range.start < result.regions[1].token_range.start,
        "distributed regions must remain in source order"
    );
}

#[test]
fn distributed_scanner_honors_a_configured_work_budget() {
    let source = "node worker {} service gateway {}";
    let stream = tokens(source);
    let result = scan_distributed_regions(
        &stream,
        DistributedScanLimits {
            max_tokens_scanned: Some(1),
            ..DistributedScanLimits::default()
        },
    );

    assert!(
        !result.diagnostics.is_empty(),
        "budget exhaustion must be reported instead of silently ignored"
    );
}

#[test]
fn distributed_scanner_does_not_treat_string_contents_as_declarations() {
    let source = "\"node worker {}\"";
    let stream = tokens(source);
    let result = scan_distributed_regions(&stream, DistributedScanLimits::default());

    assert!(
        result.regions.is_empty(),
        "literal contents must not become distributed declarations"
    );
}

// -----------------------------------------------------------------------------
// Cross-domain and repeatability checks
// -----------------------------------------------------------------------------

#[test]
fn domain_adapter_outputs_are_deterministic_for_identical_inputs() {
    let source = "module fabric { signal data; }";
    let stream = tokens(source);

    let mut first =
        HdlDomainParser::new(&stream, HdlParserLimits::default())
            .expect("limits should be valid");
    let mut second =
        HdlDomainParser::new(&stream, HdlParserLimits::default())
            .expect("limits should be valid");

    let first_item = first
        .parse_next()
        .expect("first parse should succeed")
        .expect("module should be recognized");
    let second_item = second
        .parse_next()
        .expect("second parse should succeed")
        .expect("module should be recognized");

    assert_eq!(first_item, second_item);
}

#[test]
fn domain_adapters_do_not_encode_physical_capacity_limits() {
    // The names below are intentionally ordinary source-level identifiers.
    // They should survive parsing without the test imposing a device count,
    // qubit count, register width, tensor rank, or topology size.
    let source = concat!(
        "requires resource_count >= future_resource_count;",
        "requires capability(\"future.compute.domain\");",
    );
    let stream = tokens(source);

    let mut resource_parser =
        ResourceParser::new(&stream, ResourceParserLimits::default())
            .expect("token stream should be valid");

    let resource = resource_parser
        .parse_next()
        .expect("resource clause should parse")
        .expect("resource clause should exist");

    assert!(format!("{resource:?}").contains("future_resource_count"));
}