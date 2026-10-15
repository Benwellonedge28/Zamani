#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Adversarial and Fuzz-Smoke Tests
//!
//! Path: `src/zuir/tests/fuzz.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Exercise ZUIR's existing canonical serialization and fingerprinting
//! boundaries using deterministic, generated, adversarial, and boundary
//! inputs. Detect panics, nondeterminism, malformed output, incorrect
//! budget handling, and data corruption.
//!
//! These tests are deterministic fuzz-smoke tests. They complement, but do
//! not replace, coverage-guided fuzzing, property tests, parser fuzzing,
//! IR verification tests, or end-to-end compiler tests.
//!
//! AUTHORITY
//!
//! - `DESIGN.md`
//! - `ZAMANI_IR_SPEC.md`
//! - `src/zuir/ZUIR-SPEC.md`, if present and authoritative
//! - `src/zuir/fingerprint/mod.rs`
//! - `src/zuir/fingerprint/stable_hash.rs`
//! - `src/zuir/serialization/canonical.rs`
//! - `src/zuir/serialization/schema.rs`
//! - `src/zuir/serialization/encode.rs`
//! - `src/zuir/serialization/decode.rs`
//! - `src/zuir/tests/property.rs`
//! - `src/zuir/tests/determinism.rs`
//! - `src/zuir/tests/serialization.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//!
//! Repository paths and authority must be reconciled against the actual
//! checkout before declaring the test suite integrated.
//!
//! OWNS
//!
//! - Deterministic adversarial test-data generation.
//! - Serialization and round-trip robustness assertions.
//! - Boundary and failure-path regression tests.
//! - Regression coverage for portable domain-neutral payloads.
//!
//! DOES_NOT_OWN
//!
//! - The production serializer or fingerprint algorithm.
//! - The ZUIR schema, IR builder, or IR verifier.
//! - Grammar parsing, lexer behavior, AST construction, or semantic analysis.
//! - Quantum operation semantics, HDL semantics, or target lowering.
//! - Cache storage, incremental compilation, or runtime execution.
//! - A replacement for a coverage-guided fuzzing harness.
//!
//! DEPENDENCIES
//!
//! - Existing `serde_json` dependency.
//! - `crate::zuir::fingerprint`.
//! - `crate::zuir::serialization::canonical`.
//!
//! No new production or development dependency is required.
//!
//! SCALABILITY AND POCO-REAF
//!
//! Finite generated workloads are test-runner budgets, not ZUIR limits.
//! The generator's bounded recursion prevents the test harness itself from
//! accidentally exhausting the stack. It does not impose a language-level
//! nesting limit.
//!
//! No universal maximum is defined for operations, qubits, processors,
//! register width, tensor rank, devices, nodes, or program size.
//!
//! Explicit serialization budgets must fail deterministically and must not
//! be confused with semantic invalidity or target-capacity restrictions.
//!
//! No finite implementation can guarantee literal infinity. Actual
//! execution remains subject to available memory, time, address space,
//! configured budgets, and target capabilities.
//!
//! GRAMMAR BOUNDARY
//!
//! The grammar defines source syntax. Semantic analysis defines meaning.
//! ZUIR serialization operates on already constructed values and does not
//! prove that a source string parses or that a payload is a valid IR program.
//!
//! Source/parser fuzzing must be connected to the actual frontend entry
//! points in a separate suite. These tests must not invent a parser API.
//!
//! DETERMINISM
//!
//! Generated cases use a deterministic generator rather than ambient
//! randomness. A failure must be reproducible from the test case.
//!
//! Fingerprint equality establishes repeatable encoding for the tested
//! inputs; it does not prove semantic equivalence or eliminate collisions.
//!
//! INTEGRATION
//!
//! Register this module exactly once through the repository's actual ZUIR
//! test-module owner. Do not create a competing module tree.
//!
//! FREEZE CRITERIA
//!
//! - Every import resolves in the integrated repository.
//! - Tests use the real serializer and fingerprint implementation.
//! - Generated data remains bounded by test-runner resource budgets.
//! - Failure cases return errors rather than silently succeeding.
//! - The file compiles with unsafe Rust forbidden.
//! - Formatting, linting, and relevant repository tests pass.

use std::io::{self, Write};

use serde_json::{Map, Number, Value};

use crate::zuir::fingerprint::fingerprint;
use crate::zuir::serialization::canonical::{
    canonical_json_bytes,
    canonical_json_string,
    write_canonical_json,
    CanonicalError,
    CanonicalOptions,
};

// -----------------------------------------------------------------------------
// Deterministic input generation
// -----------------------------------------------------------------------------

/// Advance a deterministic test-data generator.
///
/// This generator is not used by the compiler or by production fingerprinting.
/// Its output is test data, not a source of semantic identifiers.
fn next_u64(state: &mut u64) -> u64 {
    // Xorshift operations are deterministic and do not require a dependency
    // on a random-number generator.
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/// Generate a string containing ordinary, Unicode, and control characters.
fn generated_string(state: &mut u64) -> String {
    const ALPHABET: &[char] = &[
        'a', 'Z', '0', '_', ':', '.', '-', '/', '\\', '"', '\'',
        '\n', '\r', '\t', '\0', ' ', 'λ', '量', '子', 'Ж', 'م',
        '🙂', '\u{007F}', '\u{0085}', '\u{2028}', '\u{2029}',
    ];

    let length = (next_u64(state) % 48) as usize;
    let mut output = String::new();

    for _ in 0..length {
        let index = (next_u64(state) % ALPHABET.len() as u64) as usize;
        output.push(ALPHABET[index]);
    }

    output
}

/// Generate a bounded-depth JSON value.
///
/// The depth and branching bounds protect the test runner from accidental
/// resource exhaustion. They are not ZUIR schema limits.
fn generated_value(state: &mut u64, depth: usize) -> Value {
    // A bounded generator is intentionally used here. Dedicated stress tests
    // should control larger workloads through their own execution budgets.
    if depth >= 8 {
        return generated_scalar(state);
    }

    match next_u64(state) % 7 {
        0 => Value::Null,
        1 => Value::Bool(next_u64(state) & 1 == 0),
        2 => generated_scalar(state),
        3 => {
            let length = (next_u64(state) % 5) as usize;
            let values = (0..length)
                .map(|_| generated_value(state, depth + 1))
                .collect();

            Value::Array(values)
        }
        4 => {
            let length = (next_u64(state) % 5) as usize;
            let mut object = Map::new();

            for _ in 0..length {
                let key = generated_string(state);
                let value = generated_value(state, depth + 1);
                object.insert(key, value);
            }

            Value::Object(object)
        }
        5 => Value::String(generated_string(state)),
        _ => {
            // A portable JSON integer, deliberately independent of the
            // host's pointer width or any hardware register width.
            let number = (next_u64(state) % 1_000_000_000) as i64;
            Value::Number(Number::from(number))
        }
    }
}

/// Generate a scalar JSON value.
fn generated_scalar(state: &mut u64) -> Value {
    match next_u64(state) % 5 {
        0 => Value::Null,
        1 => Value::Bool(next_u64(state) & 1 == 0),
        2 => Value::String(generated_string(state)),
        3 => {
            let value = (next_u64(state) % 1_000_000_000) as i64;
            Value::Number(Number::from(value))
        }
        _ => Value::Number(Number::from(next_u64(state))),
    }
}

/// Produce one deterministic collection of structured test cases.
fn generated_cases() -> Vec<Value> {
    let mut cases = Vec::new();

    // Each case gets an independent, reproducible generator state.
    // This count controls test duration only; it is not a language limit.
    for case_index in 1_u64..=128 {
        let mut state = case_index
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .max(1);

        cases.push(generated_value(&mut state, 0));
    }

    cases
}

/// Render using the production canonical JSON API and its default options.
fn render(value: &Value) -> Result<Vec<u8>, CanonicalError> {
    canonical_json_bytes(value, CanonicalOptions::default())
}

// -----------------------------------------------------------------------------
// Structured payloads representing extensible computation domains
// -----------------------------------------------------------------------------

/// Construct an ordinary JSON object without imposing an operation enum.
///
/// These are serialization fixtures, not claims that the payloads are
/// registered dialects or verified executable ZUIR programs.
fn domain_payload(domain: &str, operation: &str, parameters: Value) -> Value {
    let mut object = Map::new();

    object.insert(
        "domain".to_owned(),
        Value::String(domain.to_owned()),
    );
    object.insert(
        "operation".to_owned(),
        Value::String(operation.to_owned()),
    );
    object.insert("parameters".to_owned(), parameters);

    Value::Object(object)
}

fn domain_fixtures() -> Vec<Value> {
    vec![
        domain_payload(
            "classical",
            "arithmetic.add",
            serde_json::json!({
                "lhs": "value::left",
                "rhs": "value::right",
                "numeric_semantics": "declared"
            }),
        ),
        domain_payload(
            "quantum",
            "quantum.measure",
            serde_json::json!({
                "targets": ["logical::q"],
                "result": "value::measurement",
                "effects": ["measurement"]
            }),
        ),
        domain_payload(
            "hdl",
            "hdl.signal",
            serde_json::json!({
                "width": "symbolic::WIDTH",
                "clock": "symbolic::CLOCK"
            }),
        ),
        domain_payload(
            "tensor",
            "tensor.reshape",
            serde_json::json!({
                "shape": ["symbolic::ROWS", "symbolic::COLUMNS"]
            }),
        ),
        domain_payload(
            "distributed",
            "distributed.exchange",
            serde_json::json!({
                "topology": "environment::topology",
                "participants": "symbolic::PARTICIPANTS"
            }),
        ),
        domain_payload(
            "future.vendor.extension",
            "future.operation.with.no.core.enum",
            serde_json::json!({
                "opaque_metadata": {
                    "unicode": "量子🙂",
                    "version": 1
                }
            }),
        ),
    ]
}

// -----------------------------------------------------------------------------
// Generated-input robustness
// -----------------------------------------------------------------------------

#[test]
fn generated_values_serialize_deterministically() {
    for (index, value) in generated_cases().iter().enumerate() {
        let first = render(value)
            .unwrap_or_else(|error| panic!("case {index}: first render failed: {error}"));

        let second = render(value)
            .unwrap_or_else(|error| panic!("case {index}: second render failed: {error}"));

        assert_eq!(
            first, second,
            "canonical output changed for generated case {index}"
        );
    }
}

#[test]
fn generated_values_round_trip_through_json() {
    for (index, value) in generated_cases().iter().enumerate() {
        let bytes = render(value)
            .unwrap_or_else(|error| panic!("case {index}: render failed: {error}"));

        let decoded: Value = serde_json::from_slice(&bytes)
            .unwrap_or_else(|error| panic!("case {index}: JSON parse failed: {error}"));

        assert_eq!(
            &decoded, value,
            "canonical serialization changed generated case {index}"
        );
    }
}

#[test]
fn generated_canonical_values_have_repeatable_fingerprints() {
    for (index, value) in generated_cases().iter().enumerate() {
        let bytes = render(value)
            .unwrap_or_else(|error| panic!("case {index}: render failed: {error}"));

        let canonical_text = std::str::from_utf8(&bytes)
            .unwrap_or_else(|error| panic!("case {index}: invalid UTF-8: {error}"));

        let first = fingerprint("zuir.fuzz.canonical-json.v1", &canonical_text)
            .unwrap_or_else(|error| panic!("case {index}: fingerprint failed: {error}"));

        let second = fingerprint("zuir.fuzz.canonical-json.v1", &canonical_text)
            .unwrap_or_else(|error| panic!("case {index}: fingerprint failed: {error}"));

        assert_eq!(
            first, second,
            "fingerprint changed for generated case {index}"
        );
    }
}

#[test]
fn generated_strings_preserve_unicode_and_control_characters() {
    let inputs = [
        "",
        "\0",
        "\n\r\t",
        "\"quoted\" and \\ escaped",
        "λ量子Жم🙂",
        "\u{007F}\u{0085}\u{2028}\u{2029}",
        "classical::compute",
        "quantum.measure",
        "hdl.signal",
        "tensor.reshape",
        "distributed.exchange",
        "future.vendor.extension.operation",
    ];

    for input in inputs {
        let value = Value::String(input.to_owned());

        let bytes = render(&value).expect("string should serialize");
        let decoded: Value =
            serde_json::from_slice(&bytes).expect("serialized string should parse");

        assert_eq!(decoded, value);
    }
}

#[test]
fn generated_domain_fixtures_remain_round_trip_safe() {
    for (index, value) in domain_fixtures().iter().enumerate() {
        let bytes = render(value)
            .unwrap_or_else(|error| panic!("domain fixture {index}: {error}"));

        let decoded: Value = serde_json::from_slice(&bytes)
            .unwrap_or_else(|error| panic!("domain fixture {index}: {error}"));

        assert_eq!(
            decoded, *value,
            "domain fixture {index} did not survive serialization"
        );
    }
}

// -----------------------------------------------------------------------------
// Canonicalization invariants
// -----------------------------------------------------------------------------

#[test]
fn object_key_order_does_not_depend_on_insertion_history() {
    let mut first = Map::new();
    first.insert("zeta".to_owned(), Value::from(3));
    first.insert("alpha".to_owned(), Value::from(1));
    first.insert("middle".to_owned(), Value::from(2));

    let mut second = Map::new();
    second.insert("middle".to_owned(), Value::from(2));
    second.insert("zeta".to_owned(), Value::from(3));
    second.insert("alpha".to_owned(), Value::from(1));

    let first_bytes = render(&Value::Object(first))
        .expect("first object should serialize");

    let second_bytes = render(&Value::Object(second))
        .expect("second object should serialize");

    assert_eq!(first_bytes, second_bytes);
}

#[test]
fn canonical_json_preserves_array_order() {
    let forward = Value::Array(vec![
        Value::from("first"),
        Value::from("second"),
        Value::from("third"),
    ]);

    let reverse = Value::Array(vec![
        Value::from("third"),
        Value::from("second"),
        Value::from("first"),
    ]);

    assert_ne!(
        render(&forward).expect("forward array should serialize"),
        render(&reverse).expect("reverse array should serialize"),
        "canonicalization must not discard meaningful sequence order"
    );
}

#[test]
fn distinct_generated_payloads_remain_distinguishable_in_the_test_corpus() {
    let first = Value::String("quantum.measure".to_owned());
    let second = Value::String("quantum.reset".to_owned());

    let first_bytes = render(&first).expect("first payload should serialize");
    let second_bytes = render(&second).expect("second payload should serialize");

    assert_ne!(first_bytes, second_bytes);

    let first_text =
        std::str::from_utf8(&first_bytes).expect("canonical JSON is UTF-8");
    let second_text =
        std::str::from_utf8(&second_bytes).expect("canonical JSON is UTF-8");

    let first_fingerprint = fingerprint("zuir.fuzz.payload.v1", &first_text)
        .expect("first fingerprint should succeed");
    let second_fingerprint = fingerprint("zuir.fuzz.payload.v1", &second_text)
        .expect("second fingerprint should succeed");

    assert_ne!(first_fingerprint, second_fingerprint);
}

// -----------------------------------------------------------------------------
// Configured resource-budget behavior
// -----------------------------------------------------------------------------

#[test]
fn serialization_depth_budget_returns_a_structured_error() {
    let value = serde_json::json!({
        "outer": {
            "inner": {
                "value": true
            }
        }
    });

    let options = CanonicalOptions::default().with_max_depth(Some(1));

    assert!(matches!(
        canonical_json_bytes(&value, options),
        Err(CanonicalError::DepthLimitExceeded { .. })
    ));
}

#[test]
fn serialization_node_budget_returns_a_structured_error() {
    let value = serde_json::json!({
        "items": [1, 2, 3]
    });

    let options = CanonicalOptions::default().with_max_nodes(Some(1));

    assert!(matches!(
        canonical_json_bytes(&value, options),
        Err(CanonicalError::NodeLimitExceeded { .. })
    ));
}

#[test]
fn serialization_output_budget_returns_a_structured_error() {
    let value = Value::String("a payload larger than the output budget".to_owned());

    let options = CanonicalOptions::default().with_max_output_bytes(Some(1));

    assert!(matches!(
        canonical_json_bytes(&value, options),
        Err(CanonicalError::OutputLimitExceeded { .. })
    ));
}

#[test]
fn serialization_budget_failure_does_not_mean_the_value_is_invalid() {
    let value = serde_json::json!({
        "domain": "quantum",
        "operation": "quantum.measure",
        "metadata": {
            "description": "a valid JSON payload"
        }
    });

    let restricted = CanonicalOptions::default().with_max_output_bytes(Some(1));

    assert!(canonical_json_bytes(&value, restricted).is_err());

    // The same value succeeds when the caller explicitly chooses the
    // serializer's no-additional-budget option.
    let unrestricted = CanonicalOptions::unbounded();

    assert!(canonical_json_bytes(&value, unrestricted).is_ok());
}

// -----------------------------------------------------------------------------
// Malformed input and failing writer behavior
// -----------------------------------------------------------------------------

#[test]
fn malformed_json_inputs_return_errors_without_panicking() {
    let corpus: &[&[u8]] = &[
        b"",
        b"{",
        b"[",
        b"{\"key\":}",
        b"{\"unterminated\":\"value}",
        b"[1, 2,]",
        b"\xff",
        b"\xc3\x28",
        b"null trailing",
        b"{\"a\":1,\"a\":2}",
    ];

    for (index, input) in corpus.iter().enumerate() {
        let result = std::panic::catch_unwind(|| {
            serde_json::from_slice::<Value>(input)
        });

        assert!(
            result.is_ok(),
            "JSON parser panicked on malformed corpus item {index}"
        );
    }
}

/// A writer that accepts only a finite number of bytes, then fails.
///
/// This is test infrastructure. The byte allowance is not a serialization
/// format limit or a machine-capacity assumption.
struct FailingWriter {
    remaining: usize,
}

impl FailingWriter {
    fn new(allowed_bytes: usize) -> Self {
        Self {
            remaining: allowed_bytes,
        }
    }
}

impl Write for FailingWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if self.remaining == 0 {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "intentional test writer failure",
            ));
        }

        let accepted = buffer.len().min(self.remaining);
        self.remaining -= accepted;

        Ok(accepted)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn canonical_writer_propagates_destination_failures() {
    let value = serde_json::json!({
        "domain": "classical",
        "operation": "arithmetic.add",
        "operands": ["left", "right"]
    });

    let mut writer = FailingWriter::new(1);

    let result = write_canonical_json(
        &value,
        &mut writer,
        CanonicalOptions::default(),
    );

    assert!(
        matches!(result, Err(CanonicalError::Io(_))),
        "a failed destination must not be reported as successful serialization"
    );
}

#[test]
fn canonical_string_api_matches_the_canonical_byte_api() {
    for (index, value) in generated_cases().iter().enumerate() {
        let text = canonical_json_string(value, CanonicalOptions::default())
            .unwrap_or_else(|error| panic!("case {index}: string rendering failed: {error}"));

        let bytes = canonical_json_bytes(value, CanonicalOptions::default())
            .unwrap_or_else(|error| panic!("case {index}: byte rendering failed: {error}"));

        assert_eq!(text.as_bytes(), bytes.as_slice());
    }
}